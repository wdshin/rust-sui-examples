# Games

Examples of toy games built on top of Sui!

* TicTacToe: the pencil and paper classic, now on Sui, implemented using single-owner objects only.
  * const Game Status values
  * const Error Codes
  * an Event has copy, drop
  * object::uid_to_inner
  * game instance is composed of a board with marks, some game status, and some players
  * mark vs player ,game ,cell relation
  * option::borrow
  * vector::borrow
  * option::is_some
  * option::extract
* SharedTicTacToe: the pencil and paper classic, now on Sui, using shared objects.
  * board with 2 dimensional vector of u8
  * game object of a shared object
* Hero: an adventure game where an intrepid hero slays vicious boars with a magic sword and heals himself with potions.
  * hero, boar(a monster), power with a sword , strength and magic, simple battle , potion to heal
  * test_scenario::take_immutable
  * test_scenario::return_immutable
* SeaHero: a permissionless mod of the Hero game where the hero can slay sea monsters to earn RUM tokens.
  * a game token named RUM
  * game admin managed monsters,supply of game tokens
  * 
* SeaHeroHelper: a permissionless mod of the economics of the Sea Hero game. A weak hero can request help from a stronger hero, who receives a share of the monster slaying reward.
  * transfer HelpMeSlayThisMonster to get help
  * 
* RockPaperScissors (coming soon).
  * incomplete!
  * submit secret gesture(PlayerTurn), status revealing , find winner
  * 
