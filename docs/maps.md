# Maps system

So 2d maps huh.

* Tiles need to be a certain size
  * should probably be 64x64 pixels
  * One could split up a tile, but that might be for smaller items, like coins, 4 coins to a tile?
* Each tile has many layers
  * Background
    * Grass, dirt
    * Water
    * Sand
  * Foreground
    * Cliff
    * Hill
    * Building?
    * Item?
    * Character?
    * Prop?
    * Collision?
      * Does the thing cause collision, can someone walk on this tile?
  * Collision
  * Resource
  * Building
  * Unit
  * etc

Lets use some examples:

Example 1:

* Grass Background
* Tree Foreground
  * Collision: True
  * ResourceNode: Wood (3), Leaves (1000), Sap (1) etc.
  * Building: False
  * Unit: False

Example 2:

* Gravel Background
* Unit Foreground
  * Collision: True
  * ResourceNode: False
  * Building: False
  * Unit: Worker

## Tiled

So what if we just used tiled to create the maps?

* tilesets to gather all the tile images

There is direct support for tiled maps in rust and or bevy.

* <https://github.com/adrien-bon/bevy_ecs_tiled>

Wait wait wait, there is also LDtk.

* <https://github.com/Trouv/bevy_ecs_ldtk/>

Supports aesprite files directly, which might reduce some of the work needed to create the maps.

* <https://github.com/lommix/bevy_aseprite_ultra> <- looks like it's maintained>
* <https://github.com/mdenchev/bevy_aseprite>
