

## prepare an address using sui client

* https://docs.sui.io/build/cli-client


* make a new address
```bash
sui client new-address ed25519
```

* addresses you can use
```bash
sui client addresses
```

* active address
```bash
sui client active-address
```

* change active address

```bash
sui client switch --address 0x913cf36f370613ed131868ac6f9da2420166062e
```

* check what the address owner has
  
```bash
sui client objects
```

* check how much gas the address owner has
```bash
sui client gas
```

* check what the object id has

```bash
sui client object --id 0x8cc9ca32dd5229271a2fa1baa9bad382e488a99c 
sui client object --id 0x8cc9ca32dd5229271a2fa1baa9bad382e488a99c --json
```

* transfer object

```
sui client transfer --to 0xf456ebef195e4a231488df56b762ac90695be2dd --object-id 0x66eaa38c8ea99673a92a076a00101ab9b3a06b55 --gas-budget 100
```

* split coin , merge coin
  

## build

```bash
sui move build
```

## test

```bash
sui move test
```

## publish

```bash
sui client publish --gas-budget 10000
```

## test in shell

```bash
sui client publish

----- Certificate ----
Transaction Hash: deSDm1d/jZ26VDdh48DRTqmj45DXUDII/SRzcwFxzko=
Transaction Signature: AA==@rWOR8xrq+bcrReTDPqDVydxrYAskWqxnEZ+cT0aT/IbSGNCT9puQasB9q2cfmyvIzkI0swYq3BpNSx4+bF1jAA==@pGNG8dAUDvgJJKMtEuPqi0rvnh+OQAPMtDs1YgMXY/g=
Signed Authorities Bitmap: RoaringBitmap<[0, 1, 3]>
Transaction Kind : Publish
----- Transaction Effects ----
Status : Success
Created Objects:
  - ID: 0x9ab92191b582b3aaff4b5f0a63aead1c66805caa , Owner: Immutable
Mutated Objects:
  - ID: 0x6549a1a12b8855d57909e5907610afe2d2abb277 , Owner: Account Address ( 0xfe78ab1acc4b10abe464aeb7eaafed6125a79c42 )
```

```bash
export PACKAGE=0x9ab92191b582b3aaff4b5f0a63aead1c66805caa
sui client call --gas-budget 1000 --package $PACKAGE --module "devnet_nft" --function "mint_to_sender" --args "name1" "desc1" "url1"

```

```
----- Certificate ----
Transaction Hash: +e+oQJnglBHkSxK3R7wX/WnozO+RAyM5GwmIE9u8Klk=
Transaction Signature: AA==@8WYWs3o0F6u34qhfayKkQm6WShcLjL07rG80EA3G20EBwQ4ycLFsqkF2D9hJzxjUxrqqT7JHBIvZlysy7bdRBQ==@pGNG8dAUDvgJJKMtEuPqi0rvnh+OQAPMtDs1YgMXY/g=
Signed Authorities Bitmap: RoaringBitmap<[0, 1, 3]>
Transaction Kind : Call
Package ID : 0x9ab92191b582b3aaff4b5f0a63aead1c66805caa
Module : devnet_nft
Function : mint_to_sender
Arguments : ["name1", "desc1", "url1"]
Type Arguments : []
----- Transaction Effects ----
Status : Success
Created Objects:
  - ID: 0x0d575b907dddfe87ddb027a8d8bb22f410ca8680 , Owner: Account Address ( 0xfe78ab1acc4b10abe464aeb7eaafed6125a79c42 )
Mutated Objects:
  - ID: 0x6549a1a12b8855d57909e5907610afe2d2abb277 , Owner: Account Address ( 0xfe78ab1acc4b10abe464aeb7eaafed6125a79c42 )
```

```
export OBJECT=0x0d575b907dddfe87ddb027a8d8bb22f410ca8680
sui client object --id $OBJECT
sui client object --id $OBJECT --json
```

