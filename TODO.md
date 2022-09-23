# Summary

The existing Randomizer alters item locations for a single player, using RCD edits. To support Multiworld, it will need to randomize those contents for n players, ensuring that the placements on aggregate create a winnable seed for all players, then distribute the correct modified files to each player.

The Binary will need to identify when a player has received an item for someone else, and send that info to a server. The centralized server will need to process item receipts and send the information to the intended recipient's game. The Binary will need to be able to listen and update the game state to process that item reception.

# Components

## Modded EXE + DLL (1 per player participating)

### Requirements

* Read config file for server url, session id, and player id
* Parse RCD files in a way that understands *who* an item is for
  * Customize item received message to indicate who's item was obtained
* Send other player items to server
    * Read game state for item locations to determine what has been found
* Listen for items sent to the player and trigger "get item" event

### Nice to Have

* Support multiple game versions, not just 1.0

## Server for coordinating item send/receive

### Requirements

* Create Web Interface for generating seed
* Trigger Java Randomizer and capture output for distribution to players
* Allow players to register and download their customized files
* Allow Binaries to connect to server for specific session and send/receive info
* Store session data in database

## Forked Randomizer to Generate valid game files for all players

### Requirements

* Validate item distribution for winability across multiple games
* Generate customized RCD file housing recipient information
* Allow Server to generate seed

## User-Friendly App

### Requirements

* Patch EXE
* Register session to server and update RCD files

### Nice to Have

* Allow toggling between established sessions
* Generate session from app