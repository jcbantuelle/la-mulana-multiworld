# La-Mulana Easter Mod

This project modifies La-Mulana to enable in-game modifications for [thezerothcat](https://github.com/thezerothcat)'s special Easter event randomizer.

## Requirements to Use

* La-Mulana Version 1.0.0.1 or 1.6.6.2 (Steam), unmodded

## Setup Instructions

1. Download the [DLL](https://github.com/jcbantuelle/la-mulana-multiworld/blob/easter/bin/LaMulanaMW.dll) and [Launcher](https://github.com/jcbantuelle/la-mulana-multiworld/blob/easter/bin/la-mulana-multiworld-launch.exe), placing them in the root directory of your La-Mulana install (where `LaMulanaWin.exe` is)
1. Use the randomizer as normal to generate your Easter seed
1. Run the game using the provided launcher, `la-mulana-multiworld-launch.exe`

## Building from Source

The project is built in Rust. If you don't already have an environment configured, you'll need to install it and run `rustup target add i686-pc-windows-msvc` to add the correct build target. To compile the DLL, run `cargo build --release --target=i686-pc-windows-msvc` from the project root. To compile the launcher, run `cargo build --release` from `/launcher`
