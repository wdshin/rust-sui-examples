module dmud_poc::text_commander {
    
    // The first part of the module is a list of imports. 
    // The second part is a list of types. 
    // The third part is a list of functions. 
    // The fourth part is a list of tests.

    // Part 1: imports
    //use sui::url::{Self, Url};
    use std::string;
    use std::vector;

    use sui::object::{Self, ID, UID};
    //use sui::object::{Self, UID};
    //use sui::event;
    //use sui::transfer;
    use sui::tx_context::{Self, TxContext};

    // Part 2: types
    struct TextCommander has key, store {
        id: UID,
        creator : address,
        owner : address,
        player_id : ID,
        commands : vector<string::String>,
    }

    // Part3 : functions

    // ===== Events =====


    // ===== Private Functions =====

    // --- Initialization

    // ===== Public Functions =====

    public fun new(ctx: &mut TxContext, owner : address , player_id: ID) : TextCommander {

        let sender = tx_context::sender(ctx);

        let commander = TextCommander {
            id: object::new(ctx),
            creator : sender,
            owner : owner,
            player_id : player_id,
            commands : vector::empty(),
        };

        commander
    }

    

    // ===== Public View Functions =====
    
    // Part 4: tests
     


}