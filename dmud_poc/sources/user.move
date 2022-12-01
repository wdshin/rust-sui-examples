
// * User
//   * ! is directly connected to a wallet.
//   * ! should be created(minted) right after connecting to a Universe.
//   * ! has informations about the outside.
//     * has primary PFP
//       * has other NFTs
//     * membership NFTs
//     * real name
//     * nick name
//     * email
//     * blockchain Wallet address
//     * device ids
//   * ! can connect to a World.
//   * ! can create a Player after connecting to a World.
//   * ! has(owns) several Players(NFTs)
//     * A Player belongs to one World
//     * A Player can have several Commanders
//   * ? can connect to Chatting Rooms(Channels)

module dmud_poc::user {

    // The first part of the module is a list of imports. 
    // The second part is a list of types. 
    // The third part is a list of functions. 
    // The fourth part is a list of tests.

    // Part 1: imports
    //use sui::url::{Self, Url};
    use std::string;
    //use sui::vec_map::{Self,VecMap};
    //use std::vector;

    use sui::object::{Self, ID, UID};
    use sui::event;
    use sui::transfer;
    use sui::tx_context::{Self, TxContext};

    //use dmud_poc::zone;

    // Part 2: types
    struct User has key, store {
        id: UID,
        /// Name for the service
        name: string::String,
    }

    // Part3 : functions

    // ===== Events =====

    struct UserMinted has copy, drop {
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

        //let zone0 = new_zone(ctx);

        let user = User {
            id: object::new(ctx),
            name: string::utf8(name),
        };

        event::emit(UserMinted {
            object_id: object::id(&user),
            creator: sender,
            name: user.name,
        });

        transfer::transfer(user, sender);
    }

    // ===== Public View Functions =====
    
    // Part 4: tests    

}