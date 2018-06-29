# Mallorn Spritesheet Notes

## mallorn_pix.png

Mallorn sprite coords (X,Y):

AT:			0,0
COMMA:		10,0
DOT:		20,0
MALLORN:	30,0
LIGHT_GND:	40,0
MED_GND:	50,0
DARK_GND:	60,0
TREE:		70,0

## BrogueFont

BrogueFont5.png = cell size 18 x 28
BrogueFont4.png = cell size 15 x 22
BrogueFont3.png = cell size 12 x 18
BrogueFont2.png = cell size 10 x 16
BrogueFont1.png = cell size 8 x 12

Characters on spritesheet are organized by UTF-8 table
position is UTF-8 code.  So 'Z', for example (utf-8 0xFA, or 90)
is in position 90 on the sprite sheet.

Positions 0-31 are not used and available.
Positions 128-160 are not part of the codepage and are available,
with 128-139 used for custom glyphs.
