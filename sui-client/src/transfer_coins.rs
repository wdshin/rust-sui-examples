
use std::str::FromStr;
use sui_keys::keystore::{AccountKeystore, FileBasedKeystore, Keystore};
use sui_sdk::{
    types::{
        base_types::{ObjectID, SuiAddress},
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

    // sui address 는 sui client active-address 로 확인 가능
    let my_address = SuiAddress::from_str("0xfe78ab1acc4b10abe464aeb7eaafed6125a79c42")?;
    // gas_object 는 sui client objects 로 확인 가능
    // 0x6549a1a12b8855d57909e5907610afe2d2abb277 |     10     | 
    // B3phyNQtM8E5PlN12owvtXv1kGQFnz3UOpucNKuNSm8= |  AddressOwner   |      0x2::coin::Coin<0x2::sui::SUI>
    // 0xa34de8bf29fd83e83e40edcf1476fb07aed2e943 |

    let gas_object_id = ObjectID::from_str("0x6549a1a12b8855d57909e5907610afe2d2abb277")?;
    let recipient = SuiAddress::from_str("0xb6c3c65abd9b236041c0a65db20ff8b5607a313b")?;

    // Create a sui transfer transaction
    let transfer_tx = sui
        .transaction_builder()
        .transfer_sui(my_address, gas_object_id, 1000, recipient, Some(1000))
        .await?;

    // Sign transaction
    let keystore = Keystore::from(FileBasedKeystore::new(&keystore_path)?);
    let signature = keystore.sign(&my_address, &transfer_tx.to_bytes())?;

    // Execute the transaction
    let transaction_response = sui
        .quorum_driver()
        .execute_transaction(
            Transaction::from_data(transfer_tx, signature).verify()?,
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;

    println!("{:?}", transaction_response);

    Ok(())
}