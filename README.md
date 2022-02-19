# La-Mulana Multiworld Randomizer

This project modifies La-Mulana to enable Multiworld support.

## Setup Instructions

1. Modify your LaMulanaWin.exe using Floating IPS, using the provided multiworld.bps file
1. Add the provided DLL, `LaMulanaMW.dll` to the same location as LaMulanaWin.exe

## Building from Source

The project is built in Rust. If you don't already have an environment configured, you'll need to install it and run `rustup target add i686-pc-windows-msvc` to add the correct build target. To compile the source to a DLL, run `cargo build --release --target=i686-pc-windows-msvc`