# Design Notes

## Story 

The idea is that you are an Elf, caught outside in the woods at night,
hunting for berries or herbs by the light of the moon and some
phosphorescent mushrooms.  However, you have tarried too long, and
with a chill feeling, realize that dawn is coming!  Dawn is the bane
of your kind, for with the sun the Phoenix comes - grand, terrible,
wreathed in flames and possessing an insatiable desire for Elf flesh.

So you run pell-mell trying to find your way back to the Mallorn tree.
Phoenixes have a strange, inexplicable fear of the Mallorn, so if you
can just make your way back to your tree, you'll be safe!  But the
Phoenix is relentless, and worse, he sets fire to the woods, making it
harder for you to find your way to your tree.

### Characters 

Pretty simple.  It's you and the Phoenix.  As an Elf, you can run
faster than the Phoenix, but you run out of breath if you run too
fast.  The Phoenix is implacable - it sets fire to whatever it
touches, takes flight for brief periods of time to set down in
unpredictable ways.

### Level/environment design

Mallorn needs a visually pleasing and relatively complex forest in
which this lethal game of cat and mouse is to take place.  The forest
should have hills and valleys with streams running through it, patches
of bog close to the streams in which the Elf may get mired, plenty of
trees for the Phoenix to set fire to, impassable terrain (stone
boulders, steep hillsides) and, of course, the Mallorn itself.

The only real requirement for the level is that the Mallorn exists,
and that it is reacheable by the player.  Everything else is randomly
generated.

As an option - different seasons present different challenges? Snow
slows, summer sun tires, autumn rain makes bogs, spring brings
carnivorous flowers?

Terrain types include: grass, shrub, trees, mallorn, bog,
water(stream), boulders, steep hillside, mushroom.

Tile modifiers: fire, ?snow

## Time

* Mallorn is a real-time game, though being tile-based this means time
  passes in 'chunks'.
* Because it's realtime, the player can't wait, pausing only happens
  when the ESC key is pressed.
* Will take careful balancing to make sure things don't happen too
  quickly or too slowly.
  
  
### Scheduling 

* How to schedule things?
* How is time managed?



### Gameplay 

Mallorn is not turn-based, though due to its tiled nature it is
tick-based.  (By tick-based, I mean that the flow of time is not
smooth, it's more like tick - phoenix moves - tick - fire spreads -
tick phoenix moves, etc...

The player can move as quickly or as slowly as he likes, but if he
moves too quickly (defined as more than one move command evern N
milliseconds) he will start to tire.  The Elf has a health bar and a
stamina bar.  Health can be depleted by moving through fire or passing
too close to the Phoenix (or falling?).  Stamina is depleted
constantly, slowly when walking and quickly when running.  When
stamina reaches zero, the Elf is exhausted and won't move until some
more stamina is recovered.  (?mushrooms replenish stamina?)

Potentially the Elf may catch on fire, in which case only dousing
himself in a stream will put him out. (?)

The idea is that the player has to guide the Elf back to the Mallorn
tree.  Where the Mallorn tree is randomly generated at the beginning
of the game, but it should be no less than N steps away from the
player.

Trees block movement, and streams/bogs slow the player down and cost
stamina.  Trees can catch fire and spread, shrubs catch fire and
spread quickly but burn out quickly.  Trees burn out slowly.

There will be a start screen, a pause menu, and a victory/defeat
screen.

### Art 

The idea is to use the fontpage from Brogue with libtcod.  When the
major components of the game are in place a new sprite page,
specifically for Mallorn, will be used.

### Sound and Music ===

Keep this simple.  Steps for player, occasional raptor screams for
Phoenix, and fireball explosion sounds.  Anything beyond that is
gravy.  Maybe at end can implement some background music?

### User Interface, Game Controls 

Really easy - use the number pad, or the QWEASDZXC keys for movement.
Esc will pause the game and bring up the pause menu.

The start menu will offer these choices:

* Start Game
* See High Scores
* Quit

The end screen (which handles both victory and defeat) will show the
player his score and the following options:

* Start New Game
* Quit
