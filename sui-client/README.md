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



