module dmud_poc::character {

    // The first part of the module is a list of imports. 
    // The second part is a list of types. 
    // The third part is a list of functions. 
    // The fourth part is a list of tests.

    // Part 1: imports
    //use sui::url::{Self, Url};
    use std::string;
    use std::option::{Self, Option};
    //use sui::vec_map::{Self,VecMap};
    //use std::vector;

    use sui::object::{Self, ID, UID};
    use sui::event;
    use sui::transfer;
    use sui::tx_context::{Self, TxContext};
    
    //use dmud_poc::text_commander::{Self, TextCommander};
    
    // Part 2: types
    struct Character has key, store {
        id: UID,
        /// Name for the Character
        name: string::String,
        /// Short description of the Character
        short_desc: string::String,
        /// Description of the Character
        description: string::String,
        /// The current room the Character is in
        room: UID,
        /// The current location of the Character
        location: UID,
        /// The current state of the Character
        state: UID,
        /// The current health of the Character
        health: int,
        /// The current mana of the Character
        mana: int,
        /// The current stamina of the Character
        stamina: int,
        /// The current experience of the Character
        experience: int,
        /// The current level of the Character
        level: int,
        /// The current gold of the Character
        gold: int,
        /// The current silver of the Character
        //silver: int,
        // The current copper of the Character
        //copper: int,
        // The current inventory of the Character
        //inventory: UID,
        // The current equipment of the Character
        //equipment: UID,
        // The current spells of the Character
        //spells: UID,
        // The current skills of the Character
        //skills: UID,
        // The current quests of the Character
        //quests: UID,
        // The current guilds of the Character
        //guilds: UID,
        // The current factions of the Character
        //factions: UID,
        // The current party of the Character
        //party: UID,
        // The current group of the Character
        //group: UID,
        // The current combat of the Character
        //combat: UID,
        
        //text_commander : Option<TextCommander>,
    }

    // Part3 : functions

    // ===== Events =====

    struct CharacterMinted has copy, drop {
        // The Object ID of the World
        object_id: ID,
        // The creator of the Character
        creator: address,
        // The name of the Character
        name: string::String,

    }

    // ===== Private Functions =====

    // --- Initialization

    // ===== Public Functions =====
    public entry fun new(
        name: vector<u8>,
        ctx: &mut TxContext
    ) {
        let sender = tx_context::sender(ctx);

        let new_character_id = object::new(ctx);

        let character = Character {
            id: new_character_id,
            name: string::utf8(name),
        };

        event::emit(CharacterMinted {
            object_id: object::id(&player),
            creator: sender,
            name: character.name,
        });

        transfer::transfer(player, sender);
    }

    // ===== Public View Functions =====
    
    // Part 4: tests    

}
