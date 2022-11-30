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

    // let args = vec![
    //     SuiJsonValue::new(json!("1"))?,
    //     SuiJsonValue::new(json!("1"))?,
    //     SuiJsonValue::new(json!("1"))?
    // ];
    // let args_json = json!([
    //     unwrap_or(&name, "이름"),
    //     unwrap_or(&description, "설명"),
    //     unwrap_or(&url, "https://www.sui.io"),
    // ]);
    // name: vector<u8>,
    //     description: vector<u8>,
    //     url: vector<u8>,
    //     god: address,
    //     name_of_god: vector<u8>,

    let args_json = json!(["dmud","a genesis world","https://www.sui.io",my_address,"immortal"]);
    let mut args = vec![];
    for a in args_json.as_array().unwrap() {
        args.push(SuiJsonValue::new(a.clone()).unwrap());
    }

    let type_args = vec![];

    // Create a move call transaction using the TransactionBuilder API.
    let create_call = sui
        .transaction_builder()
        .move_call(
            my_address,
            ObjectID::from_str("0xa72f6a67a8cf6b2844354bb9c8e058b166a42422")?,//self.game_package_id,
            "space",
            "new_world",
            type_args,
            args, 
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
