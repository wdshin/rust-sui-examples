
  
### Responsiblities and Relationships among Entities

* User
  * ! is directly connected to a wallet.
  * ! should be created(minted) right after connecting to a Universe.
  * ! has informations about the outside.
    * has primary PFP
      * has other NFTs
    * has membership NFTs
    * has real name
    * has email
    * has blockchain wallet address
    * has device ids
  * ! can connect to a World.
  * ! can create a Player after connecting to a World.
  * ! has(owns) several Players(NFTs)
    * A Player belongs to one World
    * A Player can have several Commanders
  * ? can connect to Chatting Rooms(Channels)
* Player
  * is an mortal.
  * is connected to a User.
  * ! has Items(NFTs) of the World.
  * ! has Coins of the World.
  * ! has a PlayerState of the World.
  * ? has informations in the World.
  * ? has Achievements of the World.
  * ? can do Social Actions on other Players.
  * ? can have Social Relationships between other Players.

* Immortal (aka Wizard)
  * can create and edit the World.

* World
  * ? has Zones.
    * a Zone has Levels.
    * has an Entry(Starting) Level
  * ? has an Entry(Starting) Zone.
  * ! has a Game Mode.
  * ! has a Game State.
  * has at least one zone, one level , one game mode and one game state.

* ? Level 
  * has a parent Zone.
  * should be edited online by immortals.
  * ? should be imported from Level Editor
    * ! from dMUD Level Editor
    * from Unreal Engine , Unity Engine , 

  * Static Actors 
  * Dynamic Actor Spawner
  * Ticker
  * loads by loading static actors, dynamic actors 
  * starts by Ticker.


### input entities ( contracts )

* commander on chain
  * text commander on chain
    * The client sends an unverified command to the text commander.
    * This is interpreted according to the context and sent to the commander entity, and the commander entity controls the player.
  * packet commander
  

* player controller
  * entity that receives orders from the commander
  * An entity that issues commands to the player
  * A player can belong to any space.
    * If it does not belong to a space, it belongs to void.

### game entities

#### live actors

* player

#### static actors

#### game space entities

* world 
  * composed of some zones
* zone 
  * one processing unit that deals with live objects in ecs(Entity Component System) . 
  * composed of some areas
* area , room( in text mud )
  * player belongs to one area
  * synchronizing unit 
* cell
  * 2D = quad , 3D = voxel


### output entities

* events for player
