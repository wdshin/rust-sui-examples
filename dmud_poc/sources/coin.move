// module dmud_poc::coin {
//     use sui::coin;
//     use sui::transfer;
//     use sui::tx_context::{Self, TxContext};

//     /// The type identifier of coin. The coin will have a type
//     /// tag of kind: `Coin<package_object::coin::DMudCoin>`
//     /// Make sure that the name of the type matches the module's name.
//     struct DMudCoin has drop {}

//     /// Module initializer is called once on module publish. A treasury
//     /// cap is sent to the publisher, who then controls minting and burning
//     fun init(witness: DMudCoin, ctx: &mut TxContext) {
//         transfer::transfer(
//             // second parameter defines decimals of the Coin: 6
//             coin::create_currency(witness, 6, ctx),
//             tx_context::sender(ctx)
//         )
//     }
// }
