# Launcher

## Enhancements
* Add Inactive Ankh Graphic to boss rooms
* Shuffle coin chests into item pool
* Shuffle traps into item pool
* Allow Mantras to complete in any order
* Transition Rando
* Door Rando
* Start Rando
* Seal Rando
* NPC Rando

## Bugs
* 4-boss shop item not appearing

## Code Cleanup
* Split out launcher main into multiple structs for each Window
* Split slint templates into individual files per Window

# DLL

## Bugs
* Ensure text overwrite persists lifetime of string to avoid text corruptions
* Review logic for delivering Shields, Ankhs, Lamp of Time, and Sacred Orbs, status is not always mapping to proper delivery
* Report of some items not appearing in AP server logs
* Duplicate items received from server

## Enhancements
* Decouple item delivery from dependence on Mutex (use independent reader and writer), to speed up receiving items
* Prevent crash when swapping without main weapon
* Prevent multiple Sacred Orb purchases
* Overwrite to real item name when for another player
* Swap Map/Grail for custom item w/ custom image
* Allow any item to appear in torude scan
* Add Overlay for Ankh Jewel status when in Inventory screen
* Add Deathlink support
* Automatic Item Hinting
* Progressive Whips and Shields
* Track Boss Kill events (add as item check?)
* Add Randomize Shop Prices option
