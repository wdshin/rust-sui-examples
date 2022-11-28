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