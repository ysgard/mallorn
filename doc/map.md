# Map Notes






## Old Map Notes

Mallorn Map Information

2.0 Tile Structure

2.1 Byte Overview
   1      1          4            1
[WORLD][TYPE][ITEMS_CREATURES][SPECIAL]

Total: 7 bytes per tile

RAM required for a 100x100 Mallorn map: 100*100*7 = ~69 Kb of memory.  Light!
RAM required for a 200x200 Mallorn map: 200*200*7 = ~273 Kb of memory.
RAM required for a 3000x3000 Mallorn map: 3000*3000*7 = 60 Mb.  This is the 
(arbitrary) largest map size we can generate.

Maximum map size is defined in defines.h as MAXIMUM_MAP_SIZE.

2.2 WORLD OVERVIEW

Bit Position	Description
[1]		IMPASSABLE - 0 PASSABLE, 1 IMPASSABLE
[2]		HINDERS - 0 NO, 1 YES
[3]		SLOWS - 0 NO, 1 YES
[4]		EXPLORED - 0 FALSE, 1 TRUE
[5]		LIT - 0 UNLIT, 1 LIT
[6]		LUMINATION - 0 FULL_LIT, 1 DIM_LIT
[7]		DIM_LIT - 0 VERY_DIM, 1 DIM
[8]		UNASSIGNED

Default: [ 00000000 ] - > 0x00

2.2.1 IMPASSABLE 0x80

	Describes whether or not the tile can be moved through.

	PASSABLE: movement is possible.
	IMPASSABLE: Can't move through.

2.2.2 HINDERS 0x40

	Minor obstruction present?

	NO - full movement
	YES - Movement cost *1.1

2.2.3 SLOWS 0x20

	Major obstruction present?
	
	NO - full movement
	YES - Movement cost *1.5
	

2.2.4 EXPLORED 0x10

	Has the tile ever been seen or walked?  To make things simple,
	we basically set this to 1 for every tile that has been LIT, and 0 by
	default.  This field determines whether a tile's terrain should be 
	drawn.
	

2.2.5 LIT 0x08

	Can the tile be seen?  Basically describes which tiles are 
	'active' and need to be drawn.

	UNLIT - tile is dark 
	LIT - tile needs to be drawn

2.2.6 LUMINATION 0x04

	Level of light of the tile.  If set, DIM_LIT will indicate the light
	level.

	FULL_LIT - Tile is fully illuminated
	DIM_LIT - Tile is partially shaded

2.3.7 DIM_LIT 0x02

	Light modifier.

	VERY_DIM - 30% luminosity
	DIM - 60% luminosity

2.2.8 Unassigned 0x01

	The remaining bit is not used at this time.


2.3 TYPE Overview

	1 byte.  This field describes the general terrain tile.  1 byte seems
	overkill for what we need, but meh.  

	1	- GRASS
	2	- MUD
	3	- SHRUB
	4	- TREE
	5	- MALLORN
	6	- ROCK

	Default -> 0x80 

2.3.1	GRASS 0x80

	Default tile.  

2.3.2	MUD 0x40

	If set, SLOWS should also be set.

2.3.3	SHRUB 0x20

	If set, HINDERS should also be set.

2.3.4 	TREE 0x10

	If set, IMPASSABLE should be set.

2.3.5	MALLORN 0x08

	Do not set IMPASSABLE, as the elf needs to be able to move onto it.

2.3.6	ROCK 0x04

	IMPASSABLE should also be set.


2.4 ITEMS_CREATURES Overview

	This is a 4-byte (default pointer size) pointer to a linked list
	containing the items and creatures that this tile contain. 

2.5 SPECIAL

	Special world conditions.  Not used at present, but hopefully can be
	used at some point for special enviromental conditions, such as...

	1 - NOTHING
	2 - SNOW
	3 - PUDDLE
	4 - BURNING

	etc...

	Default -> 0x80

2.5.1 NOTHING 0x80

	By default, nothing special is in the tile.

2.5.2 SNOW 0x40

	HINDERS should be set.

2.5.3 PUDDLE 0x20

	
2.5.4 BURNING 0x10

	Burning things should spread to nearby adjacent GRASS, SHRUB, 
	and TREES.  BURNING tiles should inflict damage.  SLOWS should also
	be set.
