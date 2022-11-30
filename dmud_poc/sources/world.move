module dmud_poc::world {
    // The first part of the module is a list of imports. 
    // The second part is a list of types. 
    // The third part is a list of functions. 
    // The fourth part is a list of tests.

    // Part 1: imports
    use sui::url::{Self, Url};
    use std::string;
    use sui::vec_map::{Self,VecMap};
    //use std::vector;

    use sui::object::{Self, ID, UID};
    use sui::event;
    use sui::transfer;
    use sui::tx_context::{Self, TxContext};

    // Part 2: types
    struct World has key, store {
        id: UID,

        /// Name for the world
        name: string::String,
        /// Description of the world
        description: string::String,
        /// URL for the world
        url: Url,
        /// God of the world
        god: address,
        /// Name of the God of the world
        name_of_god: string::String,
        /// Zones in the world
        zones: VecMap<ID,Zone>,
    }

    struct Zone has key, store  {
        id: UID,
    }

    // Part3 : functions

    // ===== Events =====

    struct WorldMinted has copy, drop {
        // The Object ID of the World
        object_id: ID,
        // The creator of the World
        creator: address,
        // The name of the World
        name: string::String,
    }

    // ===== Private Functions =====

    // --- Initialization
    
    // fun init(ctx: &mut TxContext) {
    //     let admin = admin();
    //     // ensure this is being initialized by the expected admin authenticator
    //     assert!(&tx_context::sender(ctx) == &admin, ENOT_ADMIN);
    //     transfer::transfer(
    //         GameAdmin {
    //             id: object::new(ctx),
    //             boars_created: 0,
    //             potions_created: 0
    //         },
    //         admin
    //     )
    // }

    // ===== Public Functions =====

    public entry fun new(
        name: vector<u8>,
        description: vector<u8>,
        url: vector<u8>,
        god: address,
        name_of_god: vector<u8>,
        ctx: &mut TxContext
    ) {
        let sender = tx_context::sender(ctx);

        //let zone0 = new_zone(ctx);

        let world = World {
            id: object::new(ctx),
            name: string::utf8(name),
            description: string::utf8(description),
            url: url::new_unsafe_from_bytes(url),
            god: god,
            name_of_god: string::utf8(name_of_god),
            zones: vec_map::empty(),
        };

        event::emit(WorldMinted {
            object_id: object::id(&world),
            creator: sender,
            name: world.name,
        });

        transfer::transfer(world, sender);
    }

    // ===== Public View Functions =====
    
    // Part 4: tests
 
}

