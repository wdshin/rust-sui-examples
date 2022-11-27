# sui-client

* move 로 만든 contract 를 rust sui sdk로 테스트 하는 클라이언트
  
* check balance 
* mint nft
* check objects
* create character in dmud
* battle 


## build & run

* mac osx

    * sui client full node
    ```bash
    cargo run --release --bin sui-node -- --config-path fullnode.yaml
    ```

    * warning : 최신 llvm 에서 빌드 안 된다.  
    ```bash
    export CC="/usr/bin/gcc"
    export CXX="/usr/bin/g++"
    ```
    * cargo build
    * cargo run



