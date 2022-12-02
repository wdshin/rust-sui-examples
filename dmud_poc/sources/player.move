module dmud_poc::player {

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
    
    use dmud_poc::text_commander::{Self, TextCommander};
    
    // Part 2: types
    struct Player has key, store {
        id: UID,
        /// Name for the service
        name: string::String,
        //
        text_commander : Option<TextCommander>,
    }

    // Part3 : functions

    // ===== Events =====

    struct PlayerMinted has copy, drop {
        // The Object ID of the World
        object_id: ID,
        // The creator of the World
        creator: address,
        // The name of the World
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

        let new_player_id = object::new(ctx);

        let new_text_commander = text_commander::new(ctx, sender, object::uid_to_inner(&new_player_id));

        let player = Player {
            id: new_player_id,
            name: string::utf8(name),
            text_commander: option::some(new_text_commander), 
        };

        event::emit(PlayerMinted {
            object_id: object::id(&player),
            creator: sender,
            name: player.name,
        });

        transfer::transfer(player, sender);
    }

    // ===== Public View Functions =====
    
    // Part 4: tests    

}
