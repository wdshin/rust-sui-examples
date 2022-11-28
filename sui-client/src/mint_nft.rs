use std::str::FromStr;
use sui_keys::keystore::{AccountKeystore, FileBasedKeystore, Keystore};
use sui_sdk::{
    types::{
        base_types::{ObjectID, SuiAddress},
        //crypto::Signature,
        messages::Transaction,
    },
    SuiClient,
};

// https://github.com/MystenLabs/sui/blob/main/crates/sui-sdk/examples/tic_tac_toe.rs

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let sui = SuiClient::new("https://fullnode.devnet.sui.io:443", None, None).await?;
    // Load keystore from ~/.sui/sui_config/sui.keystore
    let keystore_path = match dirs::home_dir() {
        Some(v) => v.join(".sui").join("sui_config").join("sui.keystore"),
        None => panic!("Cannot obtain home directory path"),
    };

    let address = SuiAddress::from_str("0xfe78ab1acc4b10abe464aeb7eaafed6125a79c42")?;
    
    // Create a move call transaction using the TransactionBuilder API.
    let create_call = sui
        .transaction_builder()
        .move_call(
            address,
            ObjectID::from_str("0xab17cf439261ca249d27f54abc7728e949278100")?,//self.game_package_id,
            "dmud_poc",
            "devnet_nft",
            vec![], //Vec<SuiTypeTag>
            vec![], //Vec<SuiJsonValue>,
            None, // The gateway server will pick a gas object belong to the signer if not provided.
            1000,
        )
        .await?;

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