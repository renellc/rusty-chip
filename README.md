# rusty-chip
A CHIP-8 emulator written in Rust.

This is just a fun toy project that I did to get practice writing in Rust. All ROMs can be loaded (yet only some are actually playable for now), though you might have to experiment in game to figure out the controls. There is still some testing and bug fixing I need to do, but I'll probably do these in my spare time.

## Controls
On the keyboard, the controls are:

| | | | |
|--|--|--|--|
| 1 | 2 | 3 | 4 |
| Q | W | E | R |
| A | S | D | F |
| Z | X | C | V |



## Playable ROMs
- CONNECT4
- PONG/PONG2
- MISSILE
- MAZE
- TICTAC

The unplayable ROMs will still display something in a window, but the game might not function properly.

## Running
Assuming you have Rust and Cargo installed, all you have to do is type

`cargo run -- chip8roms/ROM_NAME`

and the game will load up. You can alternatively build the release version by typing `cargo build --release`. You can then put the release on your `PATH` and launch the program from anywhere.
