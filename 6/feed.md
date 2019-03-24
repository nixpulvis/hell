### 1 - When to call `actor::Player::eat`

The dealer looks at all the species of a player, and if two or more are hungry the dealer must ask the player for their eat choice. If only one species is hungry, then the dealer must ask in three cases:

- If that species is a carnivore with more than one opponent species it can attack.
- If that species is a carnivore with no opponent species to attack, and at least one other species of the same player it can attack.
- If that species has the fat tissue trait, and there is more than one food token on the board. This case follows from the rule which states that a hungry vegetarian must eat, but the player may choose the quantity of food this species requests.

A species is defined to be hungry in two cases:

- If the species **doesn't** have the fat tissue trait, then it's hungry when it's food is less that it's population.
- If the species **has** the trait fat tissue, then it is hungry when it's fat is less than it's body size.

Note, the dealer will deny the player the option for a species to attack another one of it's own species if there is exactly one choice amongst the opponents species. This is highly confusing from a user perspective, as I'd expect to be able to make this choice regardless of the state of other players. Granted in the current assignment, a player will always abstain from eating one of it's own species. In the future when this is not always the logic for a player, the deal should allow a player to make this choice.

### 2 - Applying the `message::EatChoice` to the State of the Game

The state of the game is effected in 2 main cases, assuming only valid eat choices are considered.

- If the eating species is a vegetarian then the dealer decrements the board's food by the amount the species ate, and feeds that species the same amount. If this species has the cooperation trait, then this also feeds the species to the right one food from the board, if possible.
- If the eating species is a carnivore then the dealer decrements the defending species' population by one, and feeds the attacker one. If the defender has the horns trait, the attacker species' population is also decremented by one. Any species with a population of 0 is considered extinct, and is removed from the player's species. All species in the game with the scavenger trait are fed one token from thin air.

There is one new question this has brought up. If a species has foraging, can it eat only 1 food token? The main use case for this is if I have two species I want to feed, and I've done the math to figure out that I will only be able to feed both if I feed them one each, I'd like a way to say that a foraging species only eats one token.
