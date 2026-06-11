# Launcher

## Enhancements

* Accurately report coin chests to AP logs (Currently says shell horn sent)
* Door Rando
* Start Rando
* Transition Rando
* Seal Rando
* Hell Temple Reward
* Shuffle traps into item pool
* Allow Mantras to complete in any order
* Add Inactive Ankh Graphic to boss rooms
* NPC Rando

## Code Cleanup
* Split out launcher main into multiple structs for each Window
* Split slint templates into individual files per Window

# DLL

## Bugs
* Ensure text overwrite persists lifetime of string to avoid text corruptions

## Enhancements
* Decouple item delivery from dependence on Mutex (use independent reader and writer), to speed up receiving items
* Overwrite to real item name when for another player
* Swap Map/Grail for custom item w/ custom image
* Allow any item to appear in torude scan
* Implement better filler items (weights, coins, ammo)
* Add Overlay for Ankh Jewel status when in Inventory screen
* Add Deathlink support
* Automatic Item Hinting
* Progressive Whips and Shields
* Track Boss Kill events (add as item check?)
* Add Randomize Shop Prices option
