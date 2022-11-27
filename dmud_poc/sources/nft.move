// module dmud_poc::sword_forge {
//     // Part 1: imports
//     use sui::object::{Self, UID};
//     use sui::transfer;
//     use sui::tx_context::{Self, TxContext};

//     // Part 2: struct definitions
//     struct Sword has key, store {
//         id: UID,
//         magic: u64,
//         strength: u64,
//     }

//     struct Forge has key, store {
//         id: UID,
//         swords_created: u64,
//     }

//     // Part 3: module initializer to be executed when this module is published
//     fun init(ctx: &mut TxContext) {
//         let admin = Forge {
//             id: object::new(ctx),
//             swords_created: 0,
//         };
//         // transfer the forge object to the module/package publisher
//         transfer::transfer(admin, tx_context::sender(ctx));
//     }

//     // Part 4: accessors required to read the struct attributes
//     public fun magic(self: &Sword): u64 {
//         self.magic
//     }

//     public fun strength(self: &Sword): u64 {
//         self.strength
//     }

//     public fun swords_created(self: &Forge): u64 {
//         self.swords_created
//     }

//     // part 5: public/ entry functions (introduced later in the tutorial)
//     // part 6: private functions (if any)
//     #[test]
//     public fun test_sword_create() {
//         use sui::tx_context;
//         use sui::transfer;

//         // create a dummy TxContext for testing
//         let ctx = tx_context::dummy();

//         // create a sword
//         let sword = Sword {
//             id: object::new(&mut ctx),
//             magic: 42,
//             strength: 7,
//         };

//         // check if accessor functions return correct values
//         assert!(magic(&sword) == 42 && strength(&sword) == 7, 1);

//                 // create a dummy address and transfer the sword
//         let dummy_address = @0xCAFE;
//         transfer::transfer(sword, dummy_address);
//     }
// }