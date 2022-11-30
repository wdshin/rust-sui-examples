use std::str::FromStr;
use sui_sdk::types::base_types::SuiAddress;
use sui_sdk::SuiClient;
use sui_keys::keystore::{AccountKeystore, FileBasedKeystore, Keystore};

use std::time::{SystemTime};
use std::io::Write;
//use requestty;
//use requestty::Question;

use sui_client::dmud;

// #[tokio::main]
// async fn main() -> Result<(), anyhow::Error> {
    
//     // todo url 을 argument 로 가져오기
//     //let sui = SuiClient::new("https://fullnode.devnet.sui.io:443", None, None).await?;
//     let sui = SuiClient::new("https://fullnode.devnet.sui.io:443", Some("ws://127.0.0.1:9001"), None).await?;
//     //let address = SuiAddress::from_str("0xec11cad080d0496a53bafcea629fcbcfff2a9866")?;
//     println!("sui.api_version : {:?}", sui.api_version());
//     let address = SuiAddress::from_str("0xfe78ab1acc4b10abe464aeb7eaafed6125a79c42")?;
    
//     let objects = sui.read_api().get_objects_owned_by_address(address).await?;
//     println!("objects owned : {:?}", objects);
    

//     Ok(())

// }


fn prompt(name:&str) -> String {
    let mut line = String::new();
    print!("{}", name);
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut line).expect("Error: Could not read a line");
 
    return line.trim().to_string()
}
 

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // TODO read address from config file
    let sui = SuiClient::new("https://fullnode.devnet.sui.io:443", None, None).await?;

    // TODO read from home or specified path
    let keystore_path = match dirs::home_dir() {
        Some(v) => v.join(".sui").join("sui_config").join("sui.keystore"),
        None => panic!("Cannot obtain home directory path"),
    };

    let keystore = Keystore::from(FileBasedKeystore::new(&keystore_path)?);
    let addresses = keystore.addresses();

    loop {
        let input=prompt("> ");
        match input.as_str() {
            "now" => {
                let unixtime = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
                print!("Current Unix time is {:?}\n", unixtime);
            },
            "objects" => {
                let address = SuiAddress::from_str(addresses[0].to_string().as_str())?;
                let objects = sui.read_api().get_objects_owned_by_address(address).await?;
    
                let mut i = 0;
                for val  in objects.iter() {
                    println!("Object: {} {} {} {}", i, val.object_id , val.version, val.type_);
                    i+=1;
                }
            },
            "all objects" => {
 
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
            },
            "dmud" => {
                print!("Entering dMUD shell...\n");
                dmud::shell();
            },
            "exit" => {
                break;
            },
            _ => {
                print!("Unknown command: {}\n", input);
            }
        }
    }
    Ok(())
}
