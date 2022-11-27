# NFTs

* Num: Issuing the first ten natural numbers as collectible NFT's.
  * 좋은 예제는 아니지만, init, transfer , delete 기본 사용 예를 알 수 있다.
* Marketplace: An opensea-like marketplace built with shared objects.
  * dynamic_field , 템플릿을 이용한 listing , share_object 를 알 수 있다.
  * dynamic_field::add/revove 로 마켓 아이템을 추가/삭제한다.
* geniteam: NFTs representing collectible monsters and cosmetics used in a farming game.
  * object_bag,object_table,dynamic_object_field 사용 코드를 리뷰할 수 있다.
* Auction: example implementation of an [English auction](https://en.wikipedia.org/wiki/English_auction) using single-owner objects only.
  * internal library 사용법을 알 수 있다.
  * internal library 에서 public(friend) 를 사용하는 것을 알  수 있다.
  * internal library 에서 friend로 허용하는 caller module을 지정할 수 있다.
  * std::option 사용을 알 수 있다.
  * transfer::share_object 사용법을 알 수 있다.
* SharedAuction: example implementation of an [English auction](https://en.wikipedia.org/wiki/English_auction) using shared objects.
  * transfer::share_object 를 알 수 있다.
* ImageNFT: an NFT wrapping a URL pointing to an image stored off-chain (coming soon).
