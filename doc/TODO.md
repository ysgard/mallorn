# Basic TODO List

1. [x] Get a blank, captioned 1024x800 window open
1. [x] Get that window to initialize and quit properly, set up a basic
       event loop.
1. [x] Capture keystrokes, implement simple logging
1. [ ] Blit a '@' to the screen from a pixmap.
1. [ ] Get that '@' to wander around the screen.
1. [ ] Set up the map and its basic data structures in a separate
       module.
1. [ ] Set up a routine to generate a basic, random map.
1. [ ] Write a viewer (in own module) that queries a map and renders
       it to the screen.
1. [ ] Set up the data structures necessary to implement entities.
1. [ ] Have the player entity appear on the screen, and allow it to
       wander around, updating the map & player entity.
1. [ ] Implement scrolling in the viewer.
1. [ ] Implement collision detection.
1. [ ] Set up a time scheduler to allow actors to each have their own
       turn, and for scheduling events.
1. [ ] Implement non-player entities, have them schedule their turns
       but do nothing, and blit their icons on the screen.
1. [ ] Implement random movement for non-enemy, non-player entities
       (fireflies, bunnies, etc..)
1. [ ] Create the Phoenix entity.  Blit it to the screen, and use
       random movement to start.  Blit it to a random location in the map.
1. [ ] Implement game views - game should be able to segue between a
       start screen, the game, a pause screen and the end game screen.
1. [ ] Blit the Mallorn tree to the map in a random location, and have the player moving
       onto it finish the game and pop up the Victory screen.
1. [ ] Create the startup screen
1. [ ] Sounds for the player moving, the Phoenix.
1. [ ] Implement hitpoints and damage.  Have the Phoenix burn the
       player if he gets too close.
1. [ ] Implement pathfinding algorithm for the Phoenix.  Have it
       slowly path to the player.
1. [ ] Set failure mode - when the Phoenix moves onto the player, or
       the player burns to death, end the game and pop up the loss
       screen.
1. [ ] Implement burning - the Phoenix sets fire to flammable objects
       it passes over.  Burning system burns objects for a set amount
       of time, before reducing it to ash for shrubs/grass and burnt
       trees for trees.
1. [ ] Entities on fire can extinguish themselves in puddles or the river.
1. [ ] Phoenix fireballs - the Phoenix can shoot fireballs with low
       accuracy at the player.
1. [ ] Create a nicer map, but having hills, clumps of trees and
       bushes, a river, etc...
1. [ ] Background music?
1. [ ] Darken the whole map to represent night.
1. [ ] Implement 'mob luminosity' - fireflies, mushrooms, the Phoenix,
       burning stuff, etc...
1. [ ] Fire spreading work - burning trees and bushes can light
       neighbours too.
1. [ ] 'Glowing' - things with luminosity flicker.
1. [ ] Strafing - the Phoenix does bombing runs on the map.
1. [ ] Phoenix state management - Phoenix appears in game at a certain
       number of turns in.  He starts in the air, swooping over the
       map (lighting things) before starting bombing runs.  After 2-3
       bombing runs it lands and starts pursuing the player, shooting
       fireballs.  If the player manages to outrun it, it takes to the
       air and starts flyig around until it spots the player, then
       repeats.
1. [ ] Better sprite sheet?
