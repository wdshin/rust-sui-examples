
# Design Considerations

## 2022-12-2

* DONE user, player, text_commander
* TODO character
* TODO character attributes
* TODO character race
* TODO character job warrior
* TODO monster
* TODO monster attributes
* TODO goblin 
* TODO Zone, Area, Cell
* TODO Ticker ( World, Zone, Area ) 
* TODO Player Controller
* TODO AI Controller
* TODO Command Queue
* TODO Process Commands
* TODO Packet Commander
* TODO Send back Results
* TODO Notifier
* TODO Player Event Queue
* TODO Social World
* TODO Object Browser, World Browser, Module Browser
* TODO Object Injector, Object Editor, Object Creator
* TODO What if you want to iterate over all users in this world?
* TODO What if you want to iterate over all online users in this world?
* TODO What if you want to iterate over all players in this world?
* TODO What if you want to iterate over all online players in this world?
* TODO Entity Component System
* TODO connect to local genesis node

## 2022-12-1

* user
* player
* How to structure monster ?
  * snoop target monster must be possible
    * watching
  * The immortal must be able to enter the monster.
    * Instead of an AI controller, it should be maneuverable.
  * AI controller must be attached.
    * It's good to be able to make it with only data.
  * Are monsters also players?
    * No!
    * they are Characters.
