//! lobby2table --format csv
//! lobby2table --format raw
//! lobby2table --format draftmsg
//!
//! Read all lobby files in the current directly and copy the routing table/connection csv/discord draft message to the clipboard

use std::{
    collections::BTreeMap,
    fmt::Write,
    path::{Path, PathBuf},
    process::Command,
    str::FromStr,
};

use anyhow::{anyhow, ensure, Context, Result};
use walkdir::WalkDir;

const RESTART_PENALTY: u32 = 190;
const INCLUDE_BENCHES: bool = false;
const IGNORE_BENCH_TARGETS: &[&str] = &["E", "D", "G", "F"];

#[derive(Clone, Copy)]
enum Format {
    Table,
    Csv,
    Raw,
    DraftMsg,
}
struct Args {
    format: Format,
    placeholder: String,
    paths: Vec<PathBuf>,
    only_changed: bool,
}

fn parse_args() -> Result<Args> {
    use lexopt::prelude::*;

    let mut format = Format::Table;
    let mut paths = Vec::new();
    let mut placeholder = String::new();
    let mut only_changed = None;

    let mut parser = lexopt::Parser::from_env();
    while let Some(arg) = parser.next()? {
        match arg {
            Long("format") => {
                let val = parser.value()?.string()?;
                if val.eq_ignore_ascii_case("csv") {
                    format = Format::Csv
                } else if val.eq_ignore_ascii_case("table") {
                    format = Format::Table
                } else if val.eq_ignore_ascii_case("raw") {
                    format = Format::Raw
                } else if val.eq_ignore_ascii_case("draftmsg") {
                    format = Format::DraftMsg
                } else {
                    return Err(anyhow::anyhow!("unknown format: {val}"));
                }
            }
            Long("only-changed") => only_changed = Some(true),
            Long("placeholder") => placeholder = parser.value()?.string()?,
            Long("help") | Short('h') => {
                println!(
                    "Usage: lobby2table [--format=csv|raw|table|draftmsg] [--placeholder placeholder] PATHS..."
                );
                std::process::exit(0);
            }
            Value(val) => paths.push(val.parse()?),
            _ => return Err(arg.unexpected().into()),
        }
    }

    Ok(Args {
        format,
        placeholder,
        paths,
        only_changed: only_changed.unwrap_or(false),
    })
}

fn main() -> Result<()> {
    let mut args = parse_args()?;

    let mut in_cwd = false;
    if args.paths.is_empty() {
        args.paths.push(std::env::current_dir().unwrap());
        in_cwd = true;
    }

    for path in &args.paths {
        if !args.paths.is_empty() {
            eprintln!("{}:", path.display());
        }

        let only_paths = if !args.only_changed {
            None
        } else {
            let output = Command::new("git")
                .arg("ls-files")
                .arg("--modified")
                .arg("--others")
                .current_dir(path)
                .output()?;
            ensure!(
                output.status.success(),
                "failed to run git ls-files: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            Some(
                String::from_utf8(output.stdout)?
                    .lines()
                    .map(|line| path.join(line))
                    .collect::<Vec<_>>(),
            )
        };

        let (n, connections, _benches, prefix) =
            collect_entries(path, INCLUDE_BENCHES, only_paths)?;

        let result = match args.format {
            Format::Table => format_connections(n, connections, &args.placeholder, true)?,
            Format::Csv => format_connections(n, connections, &args.placeholder, false)?,
            Format::Raw => format_connections_raw(connections),
            Format::DraftMsg => format_connections_draftmsg(connections, prefix.as_deref()),
        };
        println!("{}", result);

        #[cfg(feature = "clipboard")]
        {
            let mut clipboard = arboard::Clipboard::new().context("failed to acquire clipboard")?;
            clipboard
                .set()
                .text(&result)
                .context("failed to set clipboard")?;
            eprintln!("copied to clipboard");
        }
    }

    if in_cwd {
        let _ = std::io::stdin().read_line(&mut String::new());
    }

    Ok(())
}

fn format_connections_raw(connections: Connections) -> String {
    connections
        .iter()
        .flat_map(|(from, to)| to.iter().map(|(to, value)| (*from, *to, *value)))
        .fold(String::new(), |mut out, (from, to, value)| {
            let _ = writeln!(&mut out, "{from},{to},{value}");
            out
        })
}

fn frames_to_finaltime(frames: u32) -> String {
    let ms = frames * 17;
    let s = ms / 1000;
    let min = s / 60;

    format!("{}:{:0>2}.{:0>3}({frames})", min, s % 60, ms % 1000)
}

fn format_connections_draftmsg(connections: Connections, prefix: Option<&str>) -> String {
    connections
        .iter()
        .flat_map(|(from, to)| to.iter().map(|(to, value)| (*from, *to, *value)))
        .fold(String::new(), |mut out, (from, to, time)| {
            let file = match prefix {
                Some(prefix) => format!("{prefix}_{from}-{to}.tas"),
                None => format!("{from}-{to}.tas"),
            };
            let _ = writeln!(&mut out, "{file} draft in {}", frames_to_finaltime(time));
            out
        })
}

fn format_connections(
    n: u32,
    connections: Connections,
    placeholder: &str,
    with_brackets: bool,
) -> Result<String> {
    let mut text = String::new();

    let empty = BTreeMap::new();

    for from in 0..=n {
        let row = connections.get(&from).unwrap_or(&empty);
        let row = (0..=n)
            .map(|to| {
                if to == from {
                    return "0".to_string();
                }

                if to == 0 {
                    return RESTART_PENALTY.to_string();
                }

                match row.get(&to) {
                    Some(time) => time.to_string(),
                    None => placeholder.into(),
                }
            })
            .collect::<Vec<_>>()
            .join(",");

        let _ = if with_brackets {
            writeln!(&mut text, "[{row}]")
        } else {
            writeln!(&mut text, "{row}")
        };
    }

    Ok(text)
}

type Connections = BTreeMap<u32, BTreeMap<u32, u32>>;

fn collect_entries(
    path: &Path,
    include_benches: bool,
    only_paths: Option<Vec<PathBuf>>,
) -> Result<(u32, Connections, Vec<BenchNode>, Option<String>)> {
    let dir = WalkDir::new(path);

    let mut nodes = Vec::new();
    let mut benches = Vec::new();

    let mut prefix = None;

    for entry in dir {
        let entry = entry?;
        let path = entry.path();

        if let Some(only_paths) = &only_paths {
            if !only_paths.iter().any(|p| p == path) {
                continue;
            }
        }

        if path.extension().map_or(true, |ext| ext != "tas") {
            continue;
        }

        anyhow::ensure!(
            entry.metadata()?.is_file(),
            "{} is not a file",
            path.display()
        );

        let stem = path
            .file_stem()
            .unwrap()
            .to_str()
            .ok_or_else(|| anyhow!("non-UTF8 path: {}", path.display()))?;
        let node = node_path(stem).ok_or_else(|| anyhow!("invalid filename: {stem}"))?;

        let prefix = prefix.get_or_insert(node.prefix.map(ToOwned::to_owned));
        ensure!(
            node.prefix == prefix.as_deref(),
            "Lobby prefix '{}' is not the same as '{}'",
            node.prefix.unwrap_or("none"),
            prefix.as_deref().unwrap_or("none")
        );

        let start = node
            .start
            .parse::<Location>()
            .context("failed to parse start node")?;
        let end = node
            .end
            .parse::<Location>()
            .context("failed to parse end node")?;

        let text = std::fs::read_to_string(path)
            .with_context(|| format!("could not read {}", path.display()))?;
        let time = extract_node_time(&text)
            .with_context(|| format!("could not extract time from {}", path.display()))?;

        ensure!(
            !text.contains("\n***"),
            "{} contains a breakpoint",
            path.display()
        );

        match (start, end) {
            (Location::Map(start), Location::Map(end)) => {
                let node = Node { start, end, time };
                nodes.push(node);
            }
            (Location::Bench(start), Location::Map(end)) => {
                benches.push(BenchNode::From { start, end, time });
            }
            (Location::Map(start), Location::Bench(end)) => {
                benches.push(BenchNode::To { start, end, time });
            }
            (Location::Bench(_), Location::Bench(_)) => {
                // are these useful?
            }
        }
    }

    let n: u32 = nodes
        .iter()
        .map(|node| node.start.max(node.end))
        .max()
        .ok_or_else(|| anyhow::anyhow!("no nodes present"))?;

    let mut map = BTreeMap::<u32, BTreeMap<u32, u32>>::new();
    for node in nodes {
        map.entry(node.start)
            .or_default()
            .insert(node.end, node.time);
    }

    if include_benches {
        let bench_connections: Vec<_> = benches
            .iter()
            .filter_map(|bench| match bench {
                BenchNode::To { start, end, time } => Some((start, end, time)),
                _ => None,
            })
            .flat_map(|(start, end, time)| {
                let indirect_connections =
                    benches
                        .iter()
                        .filter_map(move |bench_node| match bench_node {
                            BenchNode::From {
                                start: other_start,
                                end: other_end,
                                time: other_time,
                            } => {
                                if IGNORE_BENCH_TARGETS.contains(&other_start.as_str()) {
                                    return None;
                                }

                                if other_start != end && *other_end != *start {
                                    Some((*start, end, other_start, *other_end, time + other_time))
                                } else {
                                    None
                                }
                            }
                            BenchNode::To { .. } => None,
                        });

                let saving_time = indirect_connections
                    .filter(|(start, _, _, end, time)| {
                        let time_with_menuing = *time; // TODO
                        let direct_time = map.get(start).and_then(|targets| targets.get(end));
                        match direct_time {
                            Some(direct_time) if time_with_menuing < *direct_time => true,
                            Some(_) => false,
                            None => true,
                        }
                    })
                    .inspect(|(start, via1, via2, end, time)| {
                        eprintln!("using {start}-{via1}-{via2}-{end}: {time}");
                    })
                    .map(|(start, _, _, end, time)| (start, end, time));

                saving_time
            })
            .collect();

        for (start, end, time) in bench_connections {
            map.entry(start).or_default().insert(end, time);
        }
    }

    Ok((n, map, benches, prefix.unwrap_or(None)))
}

fn extract_node_time(text: &str) -> Result<u32> {
    let last_line = text
        .lines()
        .rev()
        .find(|line| {
            // TODO: use regex?
            !line.is_empty()
                && line.starts_with('#')
                && line.contains(':')
                && line.contains('.')
                && line.ends_with(')')
        })
        .ok_or_else(|| anyhow!("could not find time comment"))?;

    let (_, frames) = last_line
        .trim_end_matches(')')
        .rsplit_once('(')
        .ok_or_else(|| anyhow!("last line '{last_line}' does not contain time"))?;
    let frames = frames.parse()?;

    Ok(frames)
}

#[derive(Debug)]
struct NodePath<'a> {
    prefix: Option<&'a str>,
    start: &'a str,
    end: &'a str,
}
impl std::fmt::Display for NodePath<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.prefix {
            Some(prefix) => write!(f, "{}_{}-{}", prefix, self.start, self.end),
            None => write!(f, "{}-{}", self.start, self.end),
        }
    }
}

fn node_path(stem: &str) -> Option<NodePath> {
    let (prefix, rest) = stem
        .split_once('_')
        .map_or((None, stem), |(prefix, rest)| (Some(prefix), rest));

    let (from, rest) = rest.split_once('-')?;
    let to = rest;

    Some(NodePath {
        prefix,
        start: from,
        end: to,
    })
}

#[derive(Debug, Clone, Copy)]
struct Node {
    start: u32,
    end: u32,
    time: u32,
}

#[derive(Debug, Clone)]
enum Location {
    Bench(String),
    Map(u32),
}
impl FromStr for Location {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<u32>() {
            Ok(id) => Ok(Location::Map(id)),
            Err(_) => Ok(Location::Bench(s.to_owned())),
        }
    }
}

#[derive(Debug)]
enum BenchNode {
    From { start: String, end: u32, time: u32 },
    To { start: u32, end: String, time: u32 },
}
