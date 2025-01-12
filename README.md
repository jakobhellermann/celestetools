# celestetools

## `annotate_celeste_map`

### Installation
Setup rust using https://rustup.rs.
```sh
cargo install --git https://github.com/jakobhellermann/celestetools annotate_celeste_map
```

### Usage

```sh
Usage: annotate_celeste_map [OPTIONS] <MAP> -o <OUTPUT> --cct-recordings [<FILTER>...] --lobby-entrances <LOBBY_ENTRANCES>

Arguments:
  <MAP>  path to the .png map export from Lönn

Options:
      -o <OUTPUT>            Write annotated png to <OUTPUT>
      --top-left <TOP_LEFT>  tile position x,y of the topleft corner of the map
      --open                 Open file after annotating

Annotations:
      --cct-recordings [<FILTER>...]
          Annotate with the movement of recent physics inspector recordings.
          <FILTER> can be empty to select all recordings,
          or 'city' to only match recordings in the given chapter,
          or '1,2,5' to include specific recent recordings.
      --lobby-entrances <LOBBY_ENTRANCES>
          path to .csv file with columns 'index,name,xpos,ypos'
```

**CCT Recordings**

<img src="./docs/cct_recording.png" height="400">

1. Download @viddie's [Celeste Consistency Tracker](https://gamebanana.com/mods/358978) mod.
2. Enable `Mod Options -> Consistency Tracker -> Physics Inspector Settings -> Recording Physics Enabled`
    - Optionally, increase the `Max Recent Recordings` from the default of 10
3. Run your TASes, or play the game. Every `console load`, savestate, or death, the current recording gets saved.
   
   The recordings can be found (and deleted) in `Celeste/ConsistencyTracker/physics-recordings/recent-recordings`.
4. Open the map in Lönn and do `Map -> Save Map Image` in the menu, save as e.g. `map.png`


Then, run
```sh
annotate_celeste_map map.png -o annotated.png --cct-recordings        # annotate with every recent path
annotate_celeste_map map.png -o annotated.png --cct-recordings --open # annotate and open in image viewer afterwards
annotate_celeste_map map.png -o annotated.png --cct-recordings city   # annotate with every path from a chapter matching the name 'city'
annotate_celeste_map map.png -o annotated.png --cct-recordings 0,1    # annotate with the last two paths

# ^ these will only work if your path visits the leftmost, rightmost, bottommost and topmost room in the map.
# If that isn't the case, you need to figure out at which tile position the topleft corner of the map begins.

annotate_celeste_map map.png -o annotated.png --cct-recordings --top-left 0,-401
```

To figure out the offset, hover the **leftmost** room in the debug map and copy the room's x position "320x180  <span style="color:red">**0**</span>,0  0,0", then find the **topmost** room and copy the room's y position "320x180 480,<span style="color:red">**-401**</span> 3840,-3208"
This will result in an offset of `--top-left 0,-401`.

<img src="./docs/room_pos_xy.png" height="100">

### Lobby Entrances

```sh
annotate_celeste_map gallery_lobby.png -o out.png --lobby-entrances entrances.csv --top-left 87,-84
```
with `entrances.csv` containing
```csv
0,"Start",712,1032
1,"Blueberry Jelly Valley",1573,912
2,"Ruins of Temple",1085,864
3,"Afterglow Summit",1157,480
...
```

<img src="./docs/lobby_entrances.png" height="300">

## lobby2table



```sh
cargo install --git https://github.com/jakobhellermann/celestetools tools --bin lobby2table
```

```sh
USAGE: lobby2table [--format=csv|raw|table|draftmsg] [--placeholder placeholder (e.g. 60000)] PATHS...
```


Write `lobby2table` into the path in the explorer and press enter:

<img src="./docs/lobby2table.png" height="300">

<img src="./docs/lobby2table_output.png" height="300">


Alternatively, you can do `lobby2table --format draftmsg` for
```
beg_0-1.tas draft in 0:02.873(169)
beg_0-8.tas draft in 0:04.828(284)
beg_0-9.tas draft in 0:03.995(235)
beg_0-11.tas draft in 0:03.332(196)
beg_1-2.tas draft in 0:04.369(257)
...
```

## `celesteloader`

Rust crate for parsing celeste map files, mod zips and dialog files.

Source in [./crates/celesteloader](./crates/celesteloader/).