// ^(?:(?<order>\\d+)(?<side>[ABCHX]?)\\-)?(?<name>.+?)(?:\\-(?<sideAlt>[ABCHX]?))?$
pub fn parse_map_name(mut sid: &str) -> (Option<u32>, Option<char>, &str) {
    if let Some(last_slash) = sid.rfind('/') {
        sid = &sid[last_slash + 1..];
    }
    sid = sid.trim_end_matches(".bin");

    let non_numeric = sid.find(|c: char| !c.is_numeric()).unwrap_or(0);
    let (order, sid) = sid.split_at(non_numeric);
    let order = order.parse::<u32>().ok();

    let (side, sid) = match sid.as_bytes() {
        [side @ (b'A' | b'B' | b'C' | b'H' | b'X'), b'-', ..] => (Some(*side as char), &sid[2..]),
        [b'-', ..] => (None, &sid[1..]),
        _ => (None, sid),
    };

    // todo sideAlt
    let name = sid;

    (order, side, name)
}
