
# dmud - 'Proof of Concepts'

1. interact with sui move contracts
2. minting some general objects
3. listening events 
4. very simple game objects
5. very simple user object
6. very simple player object
7. very simple game world
8. very simple social world
9. entity component system in sui move

## prepare an address using sui client

* https://docs.sui.io/build/cli-client


* make a new address
```bash
sui client new-address ed25519
```

* addresses you can use
```bash
sui client addresses
```

* active address
```bash
sui client active-address
```

* change active address

```bash
sui client switch --address 0x913cf36f370613ed131868ac6f9da2420166062e
```

* check what the address owner has
  
```bash
sui client objects
```

* check how much gas the address owner has
```bash
sui client gas
```

* check what the object id has

```bash
sui client object --id 0x8cc9ca32dd5229271a2fa1baa9bad382e488a99c 
sui client object --id 0x8cc9ca32dd5229271a2fa1baa9bad382e488a99c --json
```

* transfer object

```
sui client transfer --to 0xf456ebef195e4a231488df56b762ac90695be2dd --object-id 0x66eaa38c8ea99673a92a076a00101ab9b3a06b55 --gas-budget 100
```

* split coin , merge coin
  

## build

```bash
sui move build
```

## test

```bash
sui move test
```

## publish

```bash
sui client publish --gas-budget 10000
```

## test in shell

```bash
sui client publish

----- Certificate ----
Transaction Hash: deSDm1d/jZ26VDdh48DRTqmj45DXUDII/SRzcwFxzko=
Transaction Signature: AA==@rWOR8xrq+bcrReTDPqDVydxrYAskWqxnEZ+cT0aT/IbSGNCT9puQasB9q2cfmyvIzkI0swYq3BpNSx4+bF1jAA==@pGNG8dAUDvgJJKMtEuPqi0rvnh+OQAPMtDs1YgMXY/g=
Signed Authorities Bitmap: RoaringBitmap<[0, 1, 3]>
Transaction Kind : Publish
----- Transaction Effects ----
Status : Success
Created Objects:
  - ID: 0x9ab92191b582b3aaff4b5f0a63aead1c66805caa , Owner: Immutable
Mutated Objects:
  - ID: 0x6549a1a12b8855d57909e5907610afe2d2abb277 , Owner: Account Address ( 0xfe78ab1acc4b10abe464aeb7eaafed6125a79c42 )
```

```bash
export PACKAGE=0x9ab92191b582b3aaff4b5f0a63aead1c66805caa
sui client call --gas-budget 1000 --package $PACKAGE --module "devnet_nft" --function "mint_to_sender" --args "name1" "desc1" "url1"

```

```
----- Certificate ----
Transaction Hash: +e+oQJnglBHkSxK3R7wX/WnozO+RAyM5GwmIE9u8Klk=
Transaction Signature: AA==@8WYWs3o0F6u34qhfayKkQm6WShcLjL07rG80EA3G20EBwQ4ycLFsqkF2D9hJzxjUxrqqT7JHBIvZlysy7bdRBQ==@pGNG8dAUDvgJJKMtEuPqi0rvnh+OQAPMtDs1YgMXY/g=
Signed Authorities Bitmap: RoaringBitmap<[0, 1, 3]>
Transaction Kind : Call
Package ID : 0x9ab92191b582b3aaff4b5f0a63aead1c66805caa
Module : devnet_nft
Function : mint_to_sender
Arguments : ["name1", "desc1", "url1"]
Type Arguments : []
----- Transaction Effects ----
Status : Success
Created Objects:
  - ID: 0x0d575b907dddfe87ddb027a8d8bb22f410ca8680 , Owner: Account Address ( 0xfe78ab1acc4b10abe464aeb7eaafed6125a79c42 )
Mutated Objects:
  - ID: 0x6549a1a12b8855d57909e5907610afe2d2abb277 , Owner: Account Address ( 0xfe78ab1acc4b10abe464aeb7eaafed6125a79c42 )
```

```
export OBJECT=0x0d575b907dddfe87ddb027a8d8bb22f410ca8680
sui client object --id $OBJECT
sui client object --id $OBJECT --json
```

## system design

### references

* [unreal engine 5.1](https://docs.unrealengine.com/5.1/en-US/)
  * https://docs.unrealengine.com/5.1/en-US/unreal-engine-terminology/
    * The Blueprint Visual Scripting system is a complete gameplay scripting system that uses a node-based interface to create gameplay elements from within Unreal Editor. As with many common scripting languages, it is used to define object-oriented (OO) classes or objects in the engine. As you use Unreal Engine, you'll often find that objects defined using Blueprint are colloquially referred to as "Blueprints."
    * Object : Objects are the most basic class in Unreal Engine - in other words, they act like building blocks and contain a lot of the essential functionality for your Assets. Almost everything in Unreal Engine inherits (or gets some functionality) from an Object.
    * Class : A Class defines the behaviors and properties of a particular Actor or Object in Unreal Engine. Classes are hierarchical, meaning a Class inherits information from its parent Class (that is, the Class it was derived or "sub-classed" from) and passes that information to its children. Classes can be created in C++ code or in Blueprints.
    * Actor : An Actor is any object that can be placed into a level, such as a Camera, static mesh, or player start location. Actors support 3D transformations such as translation, rotation, and scaling. They can be created (spawned) and destroyed through gameplay code (C++ or Blueprints).
    * Casting : Casting is an action that takes an Actor of a specific class and tries to treat it as if it were of a different class. Casting can succeed or fail. If casting succeeds, you can then access class-specific functionality on the Actor you cast to.
    * A Component is a piece of functionality that can be added to an Actor.
      * When you add a Component to an Actor, the Actor can use the functionality that the Component provides. For  example:
        * A Spot Light Component will make your Actor emit light like a spot light.
        * A Rotating Movement Component will make your Actor spin around.
        * An Audio Component will give your Actor the ability to play sounds.
      * Components must be attached to an Actor and can't exist by themselves.
    * Pawn : Pawns are a subclass of Actor and serve as an in-game avatar or persona (for example, the characters in a game). Pawns can be controlled by a player or by the game's AI, as non-player characters (NPCs).
      * When a Pawn is controlled by a human or AI player, it is considered to be Possessed. Conversely, when a Pawn is not controlled by a human or AI player, it is considered to be Unpossessed.
    * Character : A Character is a subclass of a Pawn Actor that is intended to be used as a player character. 
      * The Character subclass includes a collision setup, input bindings for bipedal movement, and additional code for player-controlled movement.
    * Player Controller : A Player Controller takes player input and translates it into interactions in the game. Every game has at least one Player Controller in it. A Player Controller often possesses a Pawn or Character as a representation of the player in a game.
    * AI Controller : Just as the Player Controller possesses a Pawn as a representation of the player in a game, an AI Controller possesses a Pawn to represent a non-player character (NPC) in a game. By default, Pawns and Characters will end up with a base AI Controller unless they are specifically possessed by a Player Controller or told not to create an AI Controller for themselves.
    * Player State : A Player State is the state of a participant in the game, such as a human player or a bot that is simulating a player. Non-player AI that exists as part of the game world doesn't have a Player State.
      * Some examples of player information that the Player State can contain include:
        * Name
        * Current level
        * Health
        * Score
        * Whether they are currently carrying the flag in a Capture the Flag game.
      * For multiplayer games, Player States for all players exist on all machines and can replicate data from the server to the client to keep things in sync. This is different from a Player Controller, which will only exist on the machine of the player it represents.
    * Game Mode : The Game Mode sets the rules of the game that is being played. These rules can include:
        * How players join the game.
        * Whether or not a game can be paused.
        * Any game-specific behavior such as win conditions.
      * You can set the default Game Mode in the Project Settings and override it for different Levels. Regardless of how you choose to implement it, you can only have one Game Mode for each Level.
      * In a multiplayer game, the Game Mode only exists on the server and the rules are replicated (sent) to each of the connected clients.
    * Game State : A Game State is a container that holds information you want replicated to every client in a game. In simpler terms, it is 'The State of the Game' for everyone connected.
      * Some examples of what the Game State can contain include:
        * Information about the game score.
        * Whether a match has started or not.
        * How many AI characters to spawn based on the number of players in the world.
      * For multiplayer games, there is one local instance of the Game State on each player's machine. Local Game State instances get their updated information from the server's instance of the Game State.
    * Brush : A Brush is an Actor that describes a 3D shape, such as a cube or a sphere. You can place brushes in a level to define level geometry (these are known as Binary Space Partition or BSP brushes). This is useful if you want to quickly block out a level, for example.
    * Volume : Volumes are bounded 3D spaces that have different uses based on the effects attached to them. For example:
      * Blocking Volumes are invisible and used to prevent Actors from passing through them.
      * Pain Causing Volumes cause damage over time to any Actor that overlaps them.
      * Trigger Volumes are programmed to cause events when an Actor enters or exits them.
    * Level : A Level is a gameplay area that you define. Levels contain everything a player can see and interact with, such as geometry, Pawns, and Actors.
      * Unreal Engine saves each level as a separate .umap file, which is why you will sometimes see them referred to as Maps.
    * World : A World is a container for all the Levels that make up your game. It handles the streaming of Levels and the spawning (creation) of dynamic Actors.


* [unity engine](https://docs.unity.com/)


### goal

### terminology in dmud

* User
* Player
* Entity(Object) 
* Class 
* Actor 
* Casting
  * Cast 
* Component
* Pawn
* Character
* Player Controller
* AI Controller
* Player State
* Game Mode
* Game State
* Brush
* Volume
* Level
* World
  * Zone
  * Area
  * Cell
* Universe
  
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
    * client 가 text commander 에게 검증되지 않은 명령어를 보낸다.
    * 이것을 context 에  맞게 해석하여 commander entity 로 보내고, commander entity 가 player 를 control 하도록 한다.
  * packet commander
  

* player controller
  * commander 에게 명령을 받는 entity
  * player 에게 명령을 내리는 entity
  * player 는 어떤 공간에 속해 있을 수 있다.
    * 어떤 공간에 속해 있지 않다면 void 에 속해 있다.

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

## study on move

* https://move-book.com/
## study on sui-framework's builtin data structures

* [object](https://github.com/MystenLabs/sui/blob/main/crates/sui-framework/docs/object.md)
  * ID
  * UID
* [priority_queue](https://github.com/MystenLabs/sui/blob/main/crates/sui-framework/docs/priority_queue.md)
  * commander 를 통해 받은 명령어들을 쌓아놓는데 쓰면 될 것 같다.
  * 얼마나 많이 저장할 수 있는지 궁금하다.

* [bag](https://github.com/MystenLabs/sui/blob/main/crates/sui-framework/docs/bag.md)
* [table](https://github.com/MystenLabs/sui/blob/main/crates/sui-framework/docs/table.md)


* [object_bag](https://github.com/MystenLabs/sui/blob/main/crates/sui-framework/docs/object_bag.md)
* [object_table](https://github.com/MystenLabs/sui/blob/main/crates/sui-framework/docs/object_table.md)

* [vec_map](https://github.com/MystenLabs/sui/blob/main/crates/sui-framework/docs/vec_map.md)
  * function interface 만 map 이고 모두 O(N)으로 작동함.
  * insertion order 대로 iteration 됨.
* [vec_set](https://github.com/MystenLabs/sui/blob/main/crates/sui-framework/docs/vec_set.md)
  * function interface 만 map 이고 모두 O(N)으로 작동함.
  * duplicate key 불허하는지에 대해서 보장 안 함 ㅋㅋ 


* [dynamic_field](https://github.com/MystenLabs/sui/blob/main/crates/sui-framework/docs/dynamic_field.md)
* [dynamic_object_field](https://github.com/MystenLabs/sui/blob/main/crates/sui-framework/docs/dynamic_object_field.md)
  
