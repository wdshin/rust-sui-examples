# a console sui-client

* move 로 만든 contract 를 rust sui sdk로 테스트 하는 클라이언트
  
* check balance 
* mint nft
* check objects
* create character in dmud
* battle 


## build & run

* mac osx

    * warning : 최신 llvm 에서 빌드 안 된다. 
    ```bash
    export CC="/usr/bin/gcc"
    export CXX="/usr/bin/g++"
    ```

    * sui client full node
      * sui 를 local 에 설치해 놓아야 한다.
      * websocket event 를 받기 위해서 필요하다.
    ```bash
    cargo run --release --bin sui-node -- --config-path fullnode.yaml
    ```
  
    * cargo build
    * cargo run
      * cargo run --bin main
      * cargo run --bin objects
      * cargo run --bin events
      * cargo run --bin transfer-coins
      * cargo run --bin mint


### move_call

```rust
    #[method(name = "moveCall")]
    async fn move_call(
        &self,
        /// the transaction signer's Sui address
        signer: SuiAddress,
        /// the Move package ID, e.g. `0x2`
        package_object_id: ObjectID,
        /// the Move module name, e.g. `devnet_nft`
        module: String,
        /// the move function name, e.g. `mint`
        function: String,
        /// the type arguments of the Move function
        type_arguments: Vec<SuiTypeTag>,
        /// the arguments to be passed into the Move function, in [SuiJson](https://docs.sui.io/build/sui-json) format
        arguments: Vec<SuiJsonValue>,
        /// gas object to be used in this transaction, the gateway will pick one from the signer's possession if not provided
        gas: Option<ObjectID>,
        /// the gas budget, the transaction will fail if the gas cost exceed the budget
        gas_budget: u64,
    ) -> RpcResult<TransactionBytes>;
```    

### publish

```rust
    /// Create an unsigned transaction to publish Move module.
    #[method(name = "publish")]
    async fn publish(
        &self,
        /// the transaction signer's Sui address
        sender: SuiAddress,
        /// the compiled bytes of a move module, the
        compiled_modules: Vec<Base64>,
        /// gas object to be used in this transaction, the gateway will pick one from the signer's possession if not provided
        gas: Option<ObjectID>,
        /// the gas budget, the transaction will fail if the gas cost exceed the budget
        gas_budget: u64,
    ) -> RpcResult<TransactionBytes>;
```


### calls


#### mint

```bash
cargo run --bin mint
sui client objects
                 Object ID                  |  Version   |                    Digest                    |   Owner Type    |               Object Type
---------------------------------------------------------------------------------------------------------------------------------------------------------------------
 0x0d575b907dddfe87ddb027a8d8bb22f410ca8680 |     1      | FZlNlQ2TGatxCzt/W0juh1yy/m8KxroqJGxyakqy+co= |  AddressOwner   | 0x9ab92191b582b3aaff4b5f0a63aead1c66805caa::devnet_nft::DevNetNFT
 0x27d583e946208df4d35745258ae24026f8c1e6e3 |     1      | pZzT3pzHaKe7RO4vPCVqLUkq0dgbDAg/Z0jOEsEtgFc= |  AddressOwner   | 0x9c60271cde5ecfa21a30d4f549cbe97a89814586::player::PlayerDB
 0x2b38c3c8292e62fa74206e7a9f46f42d604da2cb |     1      | rrtaeqO06firPDhP8jJWaFKktr+IK7VeXvbhwZTflzk= |  AddressOwner   | 0x5a1ba9135e4e32cb9b52c202c934bc84f8ae4c9e::color_object::ColorObject
 0x6549a1a12b8855d57909e5907610afe2d2abb277 |     14     | Er+49/H2ydZXvPkjoCcnoayhN6FceZzrRFuem9tQQ1A= |  AddressOwner   |      0x2::coin::Coin<0x2::sui::SUI>
 0x8a90d137b7691c898f0dd7278c6b2803a64ea2bb |     1      | nI9O7CCMq04vdsz8Q2cS1ysNqYrwf5pl6paQXouie7g= |  AddressOwner   | 0x4d3e94bb8421b4d27c2e1138fc6948dc913bfc36::devnet_nft::DevNetNFT
 0xa34de8bf29fd83e83e40edcf1476fb07aed2e943 |     1      | P0QWAYIQzm4gQeRueQQrxHsz3rzjBdf3oTYkL6ylli0= |  AddressOwner   |        0x2::devnet_nft::DevNetNFT
 0xa79c67e26d2f671777b3da7a21a24b19fd4733af |     1      | FE/8gNxoRW82X+ZzuiaTqCkTXVGx6wjzJxoE1BxmKPo= |  AddressOwner   | 0x959c67742f03e2bc90df71d24b6b6bb686dabfaf::my_module::Forge
 0xab17cf439261ca249d27f54abc7728e949278100 |     1      | aFNuxZMBDKXIrhyqPHKreIiObWo1IrmuUPoGKDsuE/Q= |  AddressOwner   |        0x2::devnet_nft::DevNetNFT
 0xbec55476731ea09ece180d1fb7ca0a8768a124df |     1      | KwCSd/f1ukPf3oxQ185CDFvzi0ZJUq9Mqx3h+giHPs0= |  AddressOwner   | 0x932f163bd806c95ed3a2fd9b9cd489fe17288149::my_nft::Forge
 0xc8bbdd6cc0c482ac43aa0c14b7a52580fe49dc9d |     1      | rRBIAeGsYuYUiaH/cX9x1oIOO9JCzdRbJpoOn0lNGGM= |  AddressOwner   |      0x2::coin::Coin<0x2::sui::SUI>
 0xd8171754667cf47584f75bbe9da32ab82228f737 |     1      | 8GhRAJo95CsUj3+ydmuG9MqIofwuF7g/yo+diutr2bo= |  AddressOwner   | 0x4d3e94bb8421b4d27c2e1138fc6948dc913bfc36::devnet_nft::DevNetNFT

sui client object --id 0xd8171754667cf47584f75bbe9da32ab82228f737
----- Move Object (0xd8171754667cf47584f75bbe9da32ab82228f737[1]) -----
Owner: Account Address ( 0xfe78ab1acc4b10abe464aeb7eaafed6125a79c42 )
Version: 1
Storage Rebate: 16
...
Previous Transaction: jmEjfUL2IMfail1maF9lN2w8CVn2oRJCnn2kwOWw0eg=
----- Data -----
type: 0x4d3e94bb8421b4d27c2e1138fc6948dc913bfc36::devnet_nft::DevNetNFT
description: 설명
id: 0xd8171754667cf47584f75bbe9da32ab82228f737
name: 이름
url: https://www.sui.io

```

### study rust crates

* [CLI](https://rust-cli.github.io/book/index.html)

