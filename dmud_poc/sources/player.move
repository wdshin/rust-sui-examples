module dmud_poc::player {
    // Part 1: imports
    // use sui::object::{Self, UID};
    // //use sui::transfer;
    // //use sui::tx_context::{Self, TxContext};

    // // Part 2: command receiver
    // struct CommandReceiver has key, store {
    //     id: UID,
    // }

    // Part 1: imports
    use sui::object::{Self, UID};
    use sui::transfer;
    use sui::tx_context::{Self, TxContext};

    // https://docs.sui.io/build/programming-with-objects/ch4-object-wrapping
    //📖For a struct type to be capable of being embedded in a Sui object struct 
    // (which will have key ability), 
    // the embedded struct type must have the store ability.

    // Part 2: struct definitions
    struct Player has key, store {
        id: UID,
        magic: u64,
        strength: u64,
    }

    struct PlayerDB has key, store {
        id: UID,
        players_created: u64,
    }

    // Part 3: module initializer to be executed when this module is published
    fun init(ctx: &mut TxContext) {
        let admin = PlayerDB {
            id: object::new(ctx),
            players_created: 0,
        };
        // transfer the forge object to the module/package publisher
        transfer::transfer(admin, tx_context::sender(ctx));
    }

    // Part 4: accessors required to read the struct attributes
    public fun magic(self: &Player): u64 {
        self.magic
    }

    public fun strength(self: &Player): u64 {
        self.strength
    }

    public fun players_created(self: &PlayerDB): u64 {
        self.players_created
    }

    // part 5: public/ entry functions (introduced later in the tutorial)
    // part 6: private functions (if any)

    #[test]
    public fun test_player_create() {
        use sui::tx_context;
        use sui::transfer;

        // create a dummy TxContext for testing
        let ctx = tx_context::dummy();

        // create a player
        let player = Player {
            id: object::new(&mut ctx),
            magic: 42,
            strength: 7,
        };

        // check if accessor functions return correct values
        assert!(magic(&player) == 42 && strength(&player) == 7, 1);

        // create a dummy address and transfer the player
        let dummy_address = @0xCAFE;
        transfer::transfer(player, dummy_address);
    }
}
