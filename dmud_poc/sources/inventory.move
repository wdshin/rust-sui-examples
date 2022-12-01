module dmud_poc::inventory {

    // The first part of the module is a list of imports. 
    // The second part is a list of types. 
    // The third part is a list of functions. 
    // The fourth part is a list of tests.

    // Part 1: imports
    //use sui::url::{Self, Url};
    //use std::string;
    //use sui::vec_map::{Self,VecMap};
    //use std::vector;

    use sui::object::{Self, ID, UID};
    use sui::event;
    use sui::transfer;
    use sui::tx_context::{Self, TxContext};

    //use dmud_poc::zone;

    // Part 2: types
    struct Inventory has key, store {
        id: UID,
    }

    // Part3 : functions

    // ===== Events =====

    struct InventoryMinted has copy, drop {
        // The Object ID of the World
        object_id: ID,
        // The creator of the World
        creator: address,
    }

    // ===== Private Functions =====

    // --- Initialization

    // ===== Public Functions =====
    public fun new(
        ctx: &mut TxContext
    ) {
        let sender = tx_context::sender(ctx);

        let inventory = Inventory {
            id: object::new(ctx),
        };

        event::emit(InventoryMinted {
            object_id: object::id(&inventory),
            creator: sender,
        });

        transfer::transfer(inventory, sender);
    }

    // ===== Public View Functions =====
    
    // Part 4: tests    

}
