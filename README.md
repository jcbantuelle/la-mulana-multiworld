# La-Mulana Multiworld Randomizer

This project modifies La-Mulana to enable Multiworld support. It is very much in an alpha state at the moment and is likely to have both logic and gameplay bugs. If you encounter a problem, please [open an issue](https://github.com/jcbantuelle/Archipelago/issues) with as much detail as possible about what you experienced vs what the expected behavior was. Additionally, please provide the downloaded zip containing your config, rcd, and dat files.

## Requirements to Play

* La-Mulana Version 1.0.0.1 or 1.6.6.2 (Steam), unmodded
* A running Archipelago Server with the [La-Mulana world](https://github.com/jcbantuelle/Archipelago/tree/lamulana) included
* Downloaded and updated script.rcd, script_code.dat, and lm_00.sav files, and lamulana-config.toml file

## Setup Instructions

1. Download the [DLL](https://github.com/jcbantuelle/la-mulana-multiworld/blob/main/bin/LaMulanaMW.dll) and [Launcher](https://github.com/jcbantuelle/la-mulana-multiworld/blob/main/bin/la-mulana-multiworld-launch.exe), placing them in the root directory of your La-Mulana install (where `LaMulanaWin.exe` is)
1. Generate the Archipelago game, referencing the [provided sample](https://github.com/jcbantuelle/la-mulana-multiworld/blob/main/example.yaml) (Please note many of these options are not currently implemented. See Currently Unsupported Options below)
1. Download the Archipelago provided zip for your game, containing `script.rcd`, `script_code.dat`, `lm_00.sav`, and `lamulana-config.toml`
1. Open lamulana-config.toml in a text editor and update the `server_url` to the domain and port of the Archipelago server. *Do not* include the leading protocol (e.g. http://). Also update the `password` to the correct text if there is one, or delete the text and leave it empty if there isn't one.
1. Place `lamulana-config.toml` in the root of your La-Mulana install directory. Place `script.rcd` in `data/mapdata`, replacing the existing file. Place `script_code.rcd` in `data/language/en`, replacing the existing file. Place `lm_00.sav` in `data/save`, replacing the existing file if there is one.
1. Run the game using the provided launcher, `la-mulana-multiworld-launch.exe`, and load the first save file in the list to begin.

## Building from Source

The project is built in Rust. If you don't already have an environment configured, you'll need to install it and run `rustup target add i686-pc-windows-msvc` to add the correct build target. To compile the DLL, run `cargo build --release --target=i686-pc-windows-msvc` from the project root. To compile the launcher, run `cargo build --release` from `/launcher`

## Currently Unsupported Options

* RandomizeCoinChests
* RandomizeTrapItems
* RandomizeNPCs
* RandomizeSeals
* StartingLocation
* RandomizeTransitions
* RandomizeBacksideDoors

## Known Issues

* Sacred Orbs will not sell out in a shop
* Swapping your Main Hand weapon without having one equipped (if you start with a subweapon)

## Credits

Thank you to [thezerothcat](https://github.com/thezerothcat), the creator of the original La-Mulana randomizer which was the inspiration for and a constant reference for creating this.

Thank you to [Planeswater](https://github.com/Planeswater), who ported all of the randomization logic into an Archipelago world format. This was a huge lift and the project would have been significantly delayed without all of that effort.

Thank you to [smurfton](https://github.com/smurfton) for your work reverse engineering the game's binary file formats and documenting them. This reference was invaluable for enabling this project.

Thank you to Squiggly for the extensive testing and feedback.

Finally, I'm forever indebted to [worsety](https://github.com/worsety) and SeerSkye, without who this project would literally not exist. Thank you both for your constant and detailed answers to my questions, your encyclopedic knowledge of the game source, and your guidance and support throughout this process. The La-Mulana community is lucky to have you both.
