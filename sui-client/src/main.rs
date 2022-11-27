use std::str::FromStr;
use sui_sdk::types::base_types::SuiAddress;
use sui_sdk::SuiClient;
//use futures::StreamExt;
//use sui_sdk::rpc_types::SuiEventFilter;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    //let sui = SuiClient::new("https://fullnode.devnet.sui.io:443", None, None).await?;
    let sui = SuiClient::new("https://fullnode.devnet.sui.io:443", Some("ws://127.0.0.1:9001"), None).await?;
    //let address = SuiAddress::from_str("0xec11cad080d0496a53bafcea629fcbcfff2a9866")?;
    let address = SuiAddress::from_str("0xfe78ab1acc4b10abe464aeb7eaafed6125a79c42")?;
    

    let objects = sui.read_api().get_objects_owned_by_address(address).await?;
    println!("{:?}", objects);
    
    // TODO - call sui transaction
    // TODO - what is the gas object id?
     // Create a move call transaction using the TransactionBuilder API.
    //  let create_game_call = self
    //  .client
    //  .transaction_builder()
    //  .move_call(
    //      player_x,
    //      self.game_package_id,
    //      "shared_tic_tac_toe",
    //      "create_game",
    //      vec![],
    //      vec![
    //          SuiJsonValue::from_str(&player_x.to_string())?,
    //          SuiJsonValue::from_str(&player_o.to_string())?,
    //      ],
    //      None, // The gateway server will pick a gas object belong to the signer if not provided.
    //      1000,
    //  )
    //  .await?;

    //let sui = SuiClient::new("https://fullnode.devnet.sui.io:443", Some("ws://127.0.0.1:9001"), None).await?;
    // let mut subscribe_all = sui.event_api().subscribe_event(SuiEventFilter::All(vec![])).await?;
    // loop {
    //     println!("{:?}", subscribe_all.next().await);
    // }

    Ok(())

}
