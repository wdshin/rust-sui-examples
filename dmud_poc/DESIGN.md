
# Design Considerations

## 2022-12-2

* DONE user, player, text_commander
* TODO character
* TODO character attributes
* TODO character race
* TODO character job warrior
* TODO monster
* TODO monster attributes
* TODO goblin 
* TODO Zone, Area, Cell
* TODO Ticker ( World, Zone, Area ) 
* TODO Player Controller
* TODO AI Controller
* TODO Command Queue
* TODO Process Commands
* TODO Packet Commander
* TODO Send back Results
* TODO Notifier
* TODO Player Event Queue
* TODO Social World
* TODO Object Browser, World Browser, Module Browser
* TODO Object Injector, Object Editor, Object Creator
* TODO 이 월드에 속한 모든 유저를 이터레이션 하고 싶다면?
* TODO 이 월드에 속한 모든 온라인 유저를 이터레이션 하고 싶다면?
* TODO 이 월드에 속한 모든 플레이어를 이터레이션 하고 싶다면?
* TODO 이 월드에 속한 모든 온라인 플레이어를 이터레이션 하고 싶다면?


## 2022-12-1

* user
* player
* monster 는 어떻게 구조화 할까?
  * snoop target monster 가능해야 함
    * 관전
  * immortal 이 monster 로 들어갈 수 있어야 함.
    * AI controller 대신에 조종가능해야 함.
  * AI controller 가 붙어야 함.
    * 데이터만으로 만들 수 있는게 좋다.
  * monster 도 player 인가?
    * No!
    * Character 이다.
