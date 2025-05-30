# TODO List

- [x] Create initial project structure
- [x] Upgrade to Bevy 0.14
- [x] Fix worker movement bug - prevent workers from drifting downward after filling their inventory
- [x] Build an exe file for Windows
- [x] Add a dependabot setup
- [x] Setup test framework
- [x] Windows Installer based executable fails with invalid path to assets
- [x] Change the selection box from a red transparent rectangle to a green outline that is drawn behind the sprite
- [x] Add an icon for the game
- [x] Add a sample texture for the background
- [x] Make PR testing faster
- [x] Cleanup some code and files that are not used
- [x] Run windows tests as well in CI
- [x] Stop building windows per PR commit
- [x] Split out main.rs into multiple files
- [x] Fix formatting, and add format testing to CI
- [x] Add a tree asset
- [x] Fix nightly builds
- [x] Think about map system
- [x] Move some more things out of main.rs
- [x] Modularize some more, restructure ready for a grid system
- [x] Support pxo files for git-lfs
- [x] Research and test out tiled, and how to integrate it with bevy and rust
- [x] Research and test ltdk, and how to integrate it with bevy
- [x] Upgrade to bevy 0.15
- [x] Make repository public
- [x] Remove some old assets, make some space
- [x] Migrate assets to aseprite
- [x] Add gif support so we can switch to our silly tree (no animation working yet though)
- [x] Delete old assets and and cleanup
- [x] Create our first LDTK map and do the import using it
- [x] Consolidate map into scenes
- [x] Make the map bigger
- [x] Add a quarry to the map, for stone resources
- [x] Run egui inspector in the game
- [x] Align the map so it isn't off the screen
- [x] Spawn the entities using the ldtk way
- [x] Strip out a whole bunch of unused code now that we have changed to a tiled map, yay
- [x] Publish to wasm somewhere
- [x] Get movement working with LDTK
- [x] Add the ability to move the map somehow?
- [x] Add a grid system
- [x] Make the WASM window in the browser as large as the viewable window
- [x] Deploy to wasm when a PR is merged
- [x] Collisions and a star path finding
- [x] When you select something, it should show the name of it in the top left
- [x] Try a different house sprite
- [x] Every unit needs an inventory system
- [x] Gathering
- [x] There is a problem with the offset of the map, not sure why, but the offset of 30,29 needs to be centralised in ldtk_calibration.rs I think.
- [x] Remove the other character, lets just start with 1 for now, or maybe spawn 2 of the same type, but lets keep it simple and for now not assign a rpg class.
- [x] Fix resource gathering pathfinding to try alternative positions when a path isn't found
- [x] Linux build target
- [x] Add background music
- [x] Rethink the multi-map thing, I'm not sure we want it to really unload maps and load them in like that
- [x] Remove the Ldtk calibration resource
- [x] Bigger test map
- [ ] Fog of war
