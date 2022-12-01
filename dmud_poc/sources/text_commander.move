
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
    use sui::event;
    use sui::transfer;
    use sui::tx_context::{Self, TxContext};

    // Part 2: types
    struct TextCommander has key, store {
        id: UID,
        creator : address,
        owner : address,
        player : address,
        commands : vector<string::String>,
    }

    // Part3 : functions

    // ===== Events =====

    struct TextCommanderMinted has copy, drop {
        // The Object ID of the World
        object_id: ID,
        creator : address,
        owner : address,
        player : address,
    }

    // ===== Private Functions =====

    // --- Initialization

    // ===== Public Functions =====

    public fun new(ctx: &mut TxContext, owner : address , player: address) {
        let sender = tx_context::sender(ctx);

        let commander = TextCommander {
            id: object::new(ctx),
            creator : sender,
            owner : owner,
            player : player,
            commands : vector::empty(),
        };

        event::emit(TextCommanderMinted {
            object_id: object::id(&commander),
            creator: sender,
            owner : owner,
            player : player,
        });

        transfer::transfer(commander, owner);
    }

    // ===== Public View Functions =====
    
    // Part 4: tests
     


}