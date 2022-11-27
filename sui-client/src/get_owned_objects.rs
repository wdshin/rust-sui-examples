use std::str::FromStr;
use sui_sdk::types::base_types::SuiAddress;
use sui_sdk::SuiClient;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // todo url 를 argument 로 가져오기
    let sui = SuiClient::new("https://fullnode.devnet.sui.io:443", None, None).await?;
    // todo default address 는 sui client active-address 로  가져오고
    // todo 또는 argument 로 받아서 사용하도록 변경
    let address = SuiAddress::from_str("0xfe78ab1acc4b10abe464aeb7eaafed6125a79c42")?;
    let objects = sui.read_api().get_objects_owned_by_address(address).await?;
    println!("{:?}", objects);
    Ok(())
}
