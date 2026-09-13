# 2026-09-13

* Bug Fix: File Verification now works correctly on 1.0.0.1
* Feature: `RandomizeBacksideDoors` is now implemented on the Launcher side. Please note that AP Generation frequently fails with this feature enabled. A separate AP fix will be forthcoming, so the feature is not recommended yet
* Feature: Sending and receiving items is now substantially faster
* Feature: The Launcher now reports additional event logging
* Feature: The Game now reports additional event logging. Please note this can get noisy, if you're concerned about Log file size exploding you'll want to change the lamulana-config.toml setting for `log_level` from `DEBUG` to `INFO`
* Maintenance: The Launcher codebase is significantly restructured. Please report any bugs or regressions you may notice

# 2026-06-11

* Bug Fix: Gate of Illusion Coin Chest now randomizes contents
* Bug Fix: Surface Life Seal Coin Chest cover is now removed when breaking the seal
* Bug Fix: Diary Chest Xelpud Pillar Event now works correctly even after having obtained Diary
* Bug Fix: Coin Chests now behave correctly, staying closed until solved and staying open once claimed
* Bug Fix: Temple of the Sun Lights puzzle wedge hitbox has been moved to the 1.0 location across all versions
* Bug Fix: Wedjet puzzle Dais now appears correctly even after having obtained the Temple of the Sun Ankh Jewel
* Bug Fix: Main Weapon Swapping no longer crashes the game in 1.6.6.2 if you don't start with a main hand weapon
