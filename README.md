# La-Mulana Multiworld Randomizer

This project modifies La-Mulana to enable Multiworld support. It is very much in an alpha state at the moment and is likely to have both logic and gameplay bugs. If you encounter a problem, please [open an issue](https://github.com/jcbantuelle/Archipelago/issues) with as much detail as possible about what you experienced vs what the expected behavior was. Additionally, please provide the downloaded zip containing your config, rcd, and dat files.

## Requirements

La-Mulana Version 1.0

## Setup Instructions

1. Modify your LaMulanaWin.exe using Floating IPS, using the provided LaMulanaMultiworld.bps file
1. Add the provided DLL, `LaMulanaMW.dll` to the same location as LaMulanaWin.exe

## Building from Source

The project is built in Rust. If you don't already have an environment configured, you'll need to install it and run `rustup target add i686-pc-windows-msvc` to add the correct build target. To compile the source to a DLL, run `cargo build --release --target=i686-pc-windows-msvc`

## Future Plans

* Add Quality of Life Features, such as:
  * Instant read tablets for easy warping
  * Skip mantra puzzle
  * Save at Ankhs
  * Display accurate item names from shop NPCs
* Include Coin Chests and Trap items in item shuffle
* Randomize starting location
* Randomize transitions
* Randomize NPCs

## Known Issues

None atm, but there are probably a lot of unknown issues

## Credits

Thank you to [thezerothcat](https://github.com/thezerothcat), the creator of the original La-Mulana randomizer which was the inspiration for and a constant reference for creating this.

Thank you to [Planeswater](https://github.com/Planeswater), who ported all of the randomization logic into an Archipelago world format. This was a huge lift and the project would have been significantly delayed without all of that effort.

Thank you to [smurfton](https://github.com/smurfton) for your work reverse engineering the game's binary file formats and documenting them. This reference was invaluable for enabling this project.

Finally, I'm forever indebted to [worsety](https://github.com/worsety) and SeerSkye, without who this project would literally not exist. Thank you both for your constant and detailed answers to my questions, your encyclopedic knowledge of the game source, and your guidance and support throughout this process. The La-Mulana community is lucky to have you both.
