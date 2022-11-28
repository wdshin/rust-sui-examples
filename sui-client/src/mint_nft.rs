use std::str::FromStr;
use sui_json_rpc_types::SuiTypeTag;
use sui_json::SuiJsonValue;
use serde_json::json;
use sui_keys::keystore::{AccountKeystore, FileBasedKeystore, Keystore};
use sui_sdk::{
    types::{
        base_types::{ObjectID, SuiAddress},
        //crypto::Signature,
        messages::Transaction,
    },
    SuiClient,
};
use sui_types::messages::ExecuteTransactionRequestType;

// https://github.com/MystenLabs/sui/blob/main/crates/sui-sdk/examples/tic_tac_toe.rs

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let sui = SuiClient::new("https://fullnode.devnet.sui.io:443", None, None).await?;
    // Load keystore from ~/.sui/sui_config/sui.keystore
    let keystore_path = match dirs::home_dir() {
        Some(v) => v.join(".sui").join("sui_config").join("sui.keystore"),
        None => panic!("Cannot obtain home directory path"),
    };

    let my_address = SuiAddress::from_str("0xfe78ab1acc4b10abe464aeb7eaafed6125a79c42")?;
    
    // immutable 객체 리스트를 어떻게 가져오지?
    //let objects = sui.read_api().get_objects_owned_by_address(address).await?;

    let args = vec![
        SuiJsonValue::new(json!("1"))?,
        SuiJsonValue::new(json!("1"))?,
        SuiJsonValue::new(json!("1"))?
    ];
    let type_args = vec![];//SuiTypeTag::U8, SuiTypeTag::U8, SuiTypeTag::U8];
    // let type_args = vec![
    //     SuiTypeTag::from(TypeTag::Vector(TypeTag::U8)),
    //     SuiTypeTag::from(TypeTag::Vector(TypeTag::U8)),
    //     SuiTypeTag::from(TypeTag::Vector(TypeTag::U8)),
    //     ];
    // Create a move call transaction using the TransactionBuilder API.
    let create_call = sui
        .transaction_builder()
        .move_call(
            my_address,
            ObjectID::from_str("0x4d3e94bb8421b4d27c2e1138fc6948dc913bfc36")?,//self.game_package_id,
            "devnet_nft",
            "mint_to_sender",
            type_args,
            args, //["1","2","3"],//args,
            None, // The gateway server will pick a gas object belong to the signer if not provided.
            1000,
        )
        .await?;
    println!("create_call : {:?}", create_call);

    // Sign transaction
    let keystore = Keystore::from(FileBasedKeystore::new(&keystore_path)?);
    let signature = keystore.sign(&my_address, &create_call.to_bytes())?;

    // Execute the transaction
    let transaction_response = sui
        .quorum_driver()
        .execute_transaction(
            Transaction::from_data(create_call, signature).verify()?,
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;

    println!("transaction_response : {:?}", transaction_response);

    Ok(())
}

/* 
SuiClientCommands::CreateExampleNFT {
    name,
    description,
    url,
    gas,
    gas_budget,
} => {
    let args_json = json!([
        unwrap_or(&name, EXAMPLE_NFT_NAME),
        unwrap_or(&description, EXAMPLE_NFT_DESCRIPTION),
        unwrap_or(&url, EXAMPLE_NFT_URL)
    ]);
    let mut args = vec![];
    for a in args_json.as_array().unwrap() {
        args.push(SuiJsonValue::new(a.clone()).unwrap());
    }
    let (_, effects) = call_move(
        ObjectID::from(SUI_FRAMEWORK_ADDRESS),
        "devnet_nft",
        "mint",
        vec![],
        gas,
        gas_budget.unwrap_or(100_000),
        args,
        context,
    )
    .await?;
    let nft_id = effects
        .created
        .first()
        .ok_or_else(|| anyhow!("Failed to create NFT"))?
        .reference
        .object_id;
    let object_read = context.client.read_api().get_parsed_object(nft_id).await?;
    SuiClientCommandResult::CreateExampleNFT(object_read)
}
*/