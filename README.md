# La-Mulana Multiworld Randomizer

This project modifies La-Mulana to enable Multiworld support. It is very much in an alpha state at the moment and is likely to have both logic and gameplay bugs. If you encounter a problem, please [open an issue](https://github.com/jcbantuelle/Archipelago/issues) with as much detail as possible about what you experienced vs what the expected behavior was. Additionally, please provide the downloaded zip containing your config, rcd, and dat files.

## Requirements to Play

* La-Mulana Version 1.0.0.1, 1.6.6.1 (GOG), or 1.6.6.2 (Steam), unmodded
* A running Archipelago Server with the [La-Mulana world](https://github.com/jcbantuelle/Archipelago/tree/lamulana) included

## Setup Instructions

1. Download the [DLL](https://github.com/jcbantuelle/la-mulana-multiworld/blob/main/bin/LaMulanaMW.dll) and [Launcher](https://github.com/jcbantuelle/la-mulana-multiworld/blob/main/bin/la-mulana-multiworld-launch.exe), placing them in the root directory of your La-Mulana install (where `LaMulanaWin.exe` is)
1. Download the custom [AP World](https://github.com/jcbantuelle/la-mulana-multiworld/blob/main/bin/lamulana.apworld)
1. Generate the Archipelago game, providing the custom AP World and referencing the [provided sample](https://github.com/jcbantuelle/la-mulana-multiworld/blob/main/example.yaml) (Please note many of these options are not currently implemented. See Currently Unsupported Options below)
1. Run the provided launcher, `la-mulana-multiworld-launch.exe`, and add the connection details for the generated AP game. Once it's loaded, select "Launch Game". If you're running via Linux/Wine/Proton and the game won't launch correctly, you may need to inject the DLL using [this tool](https://github.com/jokelbaf/proton-injector) instead.

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
* HellTempleReward
* Deathlink

## Known Issues/Behaviors

There are several quirks of the current state of the project. While everything should function, there are some behaviors that might be initially surprising.

* torude scans may show a blank item popup, although the item is properly received
* There can be a minor delay (up to 30ish seconds) in item send/receive with the AP server
* AP items for another player appear as `Holy Grail (Full)` from chests, and as `Map` from shops
* All shop items default to 10g
* `Ankh Jewels` are tied to specific bosses. When you don't have the `Ankh Jewel` for a boss, that Bosses' `Ankh` will not appear
* Filler items appear as `Shell Horn` everywhere except Chests, which appear as Coin Chests

## Credits

Thank you to [thezerothcat](https://github.com/thezerothcat), the creator of the original La-Mulana randomizer which was the inspiration for and a constant reference for creating this.

Thank you to [Planeswater](https://github.com/Planeswater), who ported all of the randomization logic into an Archipelago world format. This was a huge lift and the project would have been significantly delayed without all of that effort.

Thank you to [smurfton](https://github.com/smurfton) for your work reverse engineering the game's binary file formats and documenting them. This reference was invaluable for enabling this project.

Thank you to Squiggly, Forte, DainBread, Megarush, Lurch, Roy, SallyRoses, Jen_theHuman, BlasphemousRoar, Burning Seething Jealousy, Kaz, Wizzrobe, Emmanating, MrCarter, cleartonic, Exuno, Goost, bs9594, ace, Athebyne, EpicFunkyMode, Cmil, JSG, Crownmuri, kubikill, WHYYOUHERE, Kaktus021, Knux, and KillerHUD for braving the Alpha build to provide testing and feedback.

Finally, I'm forever indebted to [worsety](https://github.com/worsety) and SeerSkye, without whom this project would literally not exist. Thank you both for your constant and detailed answers to my questions, your encyclopedic knowledge of the game source, and your guidance and support throughout this process. The La-Mulana community is lucky to have you both.
