use std::str::FromStr;
use sui_sdk::types::base_types::SuiAddress;
use sui_sdk::SuiClient;
use sui_keys::keystore::{AccountKeystore, FileBasedKeystore, Keystore};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // todo url 를 argument 로 가져오기
    let sui = SuiClient::new("https://fullnode.devnet.sui.io:443", None, None).await?;
    // todo default address 는 sui client active-address 로  가져오고
    // todo 또는 argument 로 받아서 사용하도록 변경

    let keystore_path = match dirs::home_dir() {
        Some(v) => v.join(".sui").join("sui_config").join("sui.keystore"),
        None => panic!("Cannot obtain home directory path"),
    };

    let keystore = Keystore::from(FileBasedKeystore::new(&keystore_path)?);
    let addresses = keystore.addresses();
    for val  in addresses.iter() {
        println!("Addr: {}", val.to_string());

        let address = SuiAddress::from_str(val.to_string().as_str())?;
        let objects = sui.read_api().get_objects_owned_by_address(address).await?;

        let mut i = 0;
        for val  in objects.iter() {
            println!("Object: {} {} {} {}", i, val.object_id , val.version, val.type_);
            i+=1;
        }
    }

    // todo select from the addresses
    //println!("{:?}", objects);
    Ok(())
}
