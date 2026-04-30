# Launcher

## Implement connect to AP window to display updates
* Wire up button
* Create new layout
* Connect to AP
* Write data to text window
* Live update on message received

## File Gen Mods
* Add Inactive Ankh Graphic to boss rooms
* Shuffle coin chests into item pool
* Shuffle traps into item pool
* Allow Mantras to complete in any order
* Transition Rando
* Door Rando
* Start Rando
* Seal Rando
* NPC Rando

# DLL

## Bugs
* % in player name may crash game?

## Text Corruption
* Ensure text overwrite persists lifetime of string

## Make Item Delivery More Resilient
* Set flag from item popup window via write flags instead of manually setting
* Decouple from dependence on Mutex for item delivery
* Review logic for delivering Shields, Ankhs, Lamp of Time, and Sacred Orbs

## QoL Mods
* Prevent crash when swapping without main weapon
* Prevent multiple Sacred Orb purchases
* Overwrite to real item name when for another player
* Swap Map/Grail for custom item w/ custom image
* Allow any item to appear in torude scan
* Add Overlay for Ankh Jewel status when in Inventory screen
* Add Deathlink support

# Future Features

* Automatic Item Hinting
* Progressive Whips and Shields
* Track Boss Kill events (add as item check?)
* Add Randomize Shop Prices option
