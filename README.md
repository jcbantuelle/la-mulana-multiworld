# La-Mulana Multiworld Randomizer

This project modifies La-Mulana to enable Multiworld support. It is very much in an alpha state at the moment and is likely to have both logic and gameplay bugs. If you encounter a problem, please [open an issue](https://github.com/jcbantuelle/Archipelago/issues) with as much detail as possible about what you experienced vs what the expected behavior was. Additionally, please provide the downloaded zip containing your config, rcd, and dat files.

## Requirements to Play

* La-Mulana Version 1.0, modded with the provided BPS file
* A running Archipelago Server with the [La-Mulana world](https://github.com/jcbantuelle/Archipelago/tree/lamulana) included
* Downloaded and updated script.rcd and script_code.dat files, and lamulana-config.toml file

## Setup Instructions

1. Modify your LaMulanaWin.exe using Floating IPS, using the provided LaMulanaMultiworld.bps file
1. Add the provided DLL, `LaMulanaMW.dll` to the same location as LaMulanaWin.exe
1. Generate the Archipelago game, referencing the [provided sample](https://github.com/jcbantuelle/la-mulana-multiworld/blob/main/Cargo.toml) (Please note many of these features do not currently work)
1. Download the Archipelago provided zip for your game, containing `script.rcd`, `script_code.dat`, and `lamulana-config.toml`
1. Open lamulana-config.toml in a text editor and update the `server_url` to the domain and port of the Archipelago server. *Do not* include the leading protocol (e.g. http://). Also update the `password` to the correct text if there is one, or delete the text and leave it empty if there isn't one.
1. Place `lamulana-config.toml` in the root of your La-Mulana install directory. Place `script.rcd` in `data/mapdata`, replacing the existing file. Place `script_code.rcd` in `data/language/en`, replacing the existing file.

## Building from Source

The project is built in Rust. If you don't already have an environment configured, you'll need to install it and run `rustup target add i686-pc-windows-msvc` to add the correct build target. To compile the source to a DLL, run `cargo build --release --target=i686-pc-windows-msvc`

## Future Plans

* Add Quality of Life Features, including:
  * Skip mantra puzzle
  * Save at Ankhs
  * Display accurate item names from shop NPCs
* Include Coin Chests and Trap items in item shuffle
* Randomize starting location
* Randomize transitions
* Randomize NPCs

## Known Issues

* Mr. Fishman (original) shop is overridden by the Alternate version. The Alt Shop should open in a different location to allow access to both
* Mulbruk conversations need to be reworked
* Xelpud interactions related to the diary chest need to be reworked

## Credits

Thank you to [thezerothcat](https://github.com/thezerothcat), the creator of the original La-Mulana randomizer which was the inspiration for and a constant reference for creating this.

Thank you to [Planeswater](https://github.com/Planeswater), who ported all of the randomization logic into an Archipelago world format. This was a huge lift and the project would have been significantly delayed without all of that effort.

Thank you to [smurfton](https://github.com/smurfton) for your work reverse engineering the game's binary file formats and documenting them. This reference was invaluable for enabling this project.

Thank you to Squiggly for the extensive testing and feedback.

Finally, I'm forever indebted to [worsety](https://github.com/worsety) and SeerSkye, without who this project would literally not exist. Thank you both for your constant and detailed answers to my questions, your encyclopedic knowledge of the game source, and your guidance and support throughout this process. The La-Mulana community is lucky to have you both.
