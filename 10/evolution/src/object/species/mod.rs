use std::cmp;
use std::mem::swap;
use object::*;

/// A species which evolves, feeds, grows, and fluctuates in population
/// throughout the game.
///
/// Population must be from 0 to `MAX_POPULATION`, and body size must be
/// from 0 to `MAX_BODY_SIZE`. Traits must be a collection of 0 to `MAX_TRAITS`
/// length, and each member of the collection must be unique. Food must not be
/// larger than the population, and fat is only `Some` if the species has
/// `FatTissue` trait, and must not be greater than the body size.
///
/// A species "eating" means that it is consuming one food token. This is
/// distinct from "feeding" which is a process of eating in which the species
/// and other spcies may eat.
///
/// A species "storing" means that the species is consuming some number of
/// food tokens and putting them on their fat tissue. This is only allowed
/// if the species has the trait.
///
/// # Examples
///
/// The default species.
///
/// ```
/// use evolution::object::*;
///
/// let species = Species::default();
/// assert_eq!(1, species.population());
/// assert_eq!(0, species.body_size());
/// assert!(species.traits().is_empty());
/// assert!(species.food().is_empty());
/// // TODO: `assert!(species.fat().is_empty());` really shouldn't panic.
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Species {
    population: u64,
    body_size: u64,
    traits: Vec<Trait>,
    food: Vec<FoodToken>,
    fat: Vec<FoodToken>,
}

/// Population functions.
impl Species {
    /// Returns the current population of this species. If this value is 0
    /// then this species is considered extinct.
    ///
    /// # Examples
    ///
    /// ```
    /// use evolution::object::*;
    ///
    /// let spec = Species::default();
    /// assert_eq!(1, spec.population());
    /// ```
    pub fn population(&self) -> u64 {
        self.population
    }

    /// Returns true if this species is extinct.
    ///
    /// # Examples
    ///
    /// ```
    /// use evolution::object::*;
    ///
    /// let mut spec = Species::default();
    /// spec.kill().unwrap();
    /// assert!(spec.is_extinct());
    /// ```
    pub fn is_extinct(&self) -> bool {
        self.population == 0
    }

    /// Increase the population of this species by one. This function returns
    /// an `Err` if the population is already the maximum value.
    ///
    /// # Examples
    ///
    /// ```
    /// use evolution::object::*;
    ///
    /// let mut spec = Species::default();
    /// spec.breed().expect("unable to breed");
    /// assert_eq!(2, spec.population());
    /// ```
    pub fn breed(&mut self) -> Result<u64, ()> {
        if self.population < MAX_POPULATION {
            self.population += 1;
            Ok(self.population)
        } else {
            Err(())
        }
    }

    /// Kill one this species, removing 1 food from the species if needed to
    /// maintain the invariant that a species must have food <= population.
    ///
    /// This function assumes it will never be called on an extinct species.
    /// In general it's best to never allow data structures to contain species
    /// with a population of 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use evolution::object::*;
    ///
    /// let mut spec = Species::default();
    /// spec.kill().unwrap();
    /// assert_eq!(0, spec.population());
    /// ```
    pub fn kill(&mut self) -> Result<bool, ()> {
        match self.population.checked_sub(1) {
            Some(n) => {
                self.population = n;
                self.food.truncate(n as usize);
                Ok(self.is_extinct())
            }
            None => {
                println!("hit");
                Err(())
            }
        }
    }

    /// Kills of as many of this species as needed to make the population
    /// equal to the the food. Returns true if this species went extinct.
    ///
    /// # Examples
    ///
    /// ```
    /// use evolution::object::*;
    ///
    /// let mut spec = Species::default();
    /// spec.breed().unwrap();
    /// spec.breed().unwrap();
    /// spec.eat(FoodToken).unwrap();
    /// assert_eq!(3, spec.population());
    /// spec.cull().unwrap();
    /// assert_eq!(1, spec.population());
    /// ```
    pub fn cull(&mut self) -> Result<bool, ()> {
        while self.population() > self.food().len() as u64 {
            try!(self.kill());
        }
        Ok(self.is_extinct())
    }
}

/// Body size functions.
impl Species {
    /// Returns the body size for this species.
    ///
    /// # Examples
    ///
    /// ```
    /// use evolution::object::*;
    ///
    /// let mut spec = Species::default();
    /// assert_eq!(0, spec.body_size());
    /// spec.grow().expect("unable to grow");
    /// spec.grow().expect("unable to grow");
    /// assert_eq!(2, spec.body_size());
    /// ```
    pub fn body_size(&self) -> u64 {
        self.body_size
    }

    /// Grow a species, increasing their body size by one. This function
    /// returns `Err` if the body size is already the maximum value.
    ///
    /// # Examples
    ///
    /// ```
    /// use evolution::object::*;
    ///
    /// let mut spec = Species::default();
    /// spec.grow().expect("unable to grow");
    /// assert_eq!(1, spec.body_size());
    /// ```
    pub fn grow(&mut self) -> Result<u64, ()> {
        if self.body_size < MAX_BODY_SIZE {
            self.body_size += 1;
            Ok(self.body_size)
        } else {
            Err(())
        }
    }
}

/// Trait functions.
impl Species {
    /// Returns the traits in the order they were added.
    ///
    /// # Examples
    ///
    /// ```
    /// use evolution::object::*;
    ///
    /// let mut spec = Species::default();
    /// spec.evolve(Trait::Climbing).expect("unable to evolve");
    /// assert_eq!(1, spec.traits().len());
    /// ```
    pub fn traits(&self) -> &[Trait] {
        &self.traits
    }

    /// Returns true if this species has the given trait.
    ///
    /// # Examples
    ///
    /// ```
    /// use evolution::object::*;
    ///
    /// let mut spec = Species::default();
    /// spec.evolve(Trait::Burrowing).expect("unable to evolve");
    /// assert!(spec.has_trait(Trait::Burrowing));
    /// assert!(!spec.has_trait(Trait::Climbing));
    /// ```
    pub fn has_trait(&self, t: Trait) -> bool {
        self.traits.contains(&t)
    }

    /// Give this species the given trait. This function returns `Err` when
    /// this species already has the requested trait.
    ///
    /// # Examples
    ///
    /// ```
    /// use evolution::object::*;
    ///
    /// let mut spec = Species::default();
    /// assert_eq!(0, spec.traits().len());
    /// spec.evolve(Trait::Climbing).expect("unable to evolve");
    /// spec.evolve(Trait::FatTissue).expect("unable to evolve");
    /// assert_eq!(2, spec.traits().len());
    /// assert!(spec.has_trait(Trait::Climbing));
    /// assert!(spec.has_trait(Trait::FatTissue));
    /// ```
    pub fn evolve(&mut self, t: Trait) -> Result<(), ()> {
        if self.traits.len() >= MAX_TRAITS {
            return Err(())
        }
        if self.traits().contains(&t) {
            return Err(())
        }
        self.traits.push(t);
        Ok(())
    }

    /// Swaps a trait in-place for this species. Returns an error if the given index is
    /// out-of-bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use evolution::object::*;
    ///
    /// let mut spec = Species::default();
    /// spec.evolve(Trait::Foraging).expect("unable to evolve");
    /// assert!(spec.has_trait(Trait::Foraging));
    /// assert!(spec.exchange_trait(0, Trait::Carnivore).is_ok());
    /// assert!(spec.has_trait(Trait::Carnivore));
    /// ```
    pub fn exchange_trait(&mut self, i: usize, t: Trait) -> Result<(), ()> {
        match self.traits().iter().enumerate().find(|&(_, trait_type)| { *trait_type == t }) {
            Some((idx, _)) => {
                if idx != i {
                    return Err(());
                }
            },
            None => {},
        }

        let traits_len = self.traits.len();
        if i < traits_len {
            let old_trait = &self.traits.remove(i);
            self.traits.insert(i, t);

            // NOTE: Trait::FatTissue should really hold the fat.
            if old_trait == &Trait::FatTissue {
                self.fat = vec![];
            }

            Ok(())
        } else {
            Err(())
        }
    }
}

/// Food functions.
impl Species {
    /// Returns the amount of food this species currently has.
    ///
    /// # Examples
    ///
    /// ```
    /// use evolution::object::*;
    ///
    /// let mut spec = Species::default();
    /// assert_eq!(0, spec.food().len());
    /// spec.eat(FoodToken).expect("unable to eat");
    /// assert_eq!(1, spec.food().len());
    /// ```
    pub fn food(&self) -> &[FoodToken] {
        &self.food
    }

    /// Returns true if this species can eat one or more food tokens.
    ///
    /// # Examples
    ///
    /// ```
    /// use evolution::object::*;
    ///
    /// let mut spec = Species::default();
    /// assert!(spec.can_eat());
    /// spec.eat(FoodToken).expect("unable to eat");
    /// assert!(!spec.can_eat());
    /// ```
    pub fn can_eat(&self) -> bool {
        self.population() > (self.food().len() as u64)
    }

    /// Returns the number of eats a player's species should perform for a
    /// feed.
    pub fn eats(&self) -> usize {
        if self.has_trait(Trait::Foraging) {
            2
        } else {
            1
        }
    }

    /// Directs this species to eat a single food token. If the species was
    /// unable to eat the food token, the food token is returned in `Err`,
    /// otherwise `Ok(())` is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use evolution::object::*;
    ///
    /// let mut spec = Species::default();
    /// assert_eq!(0, spec.food().len());
    /// spec.eat(FoodToken).unwrap();
    /// assert_eq!(1, spec.food().len());
    /// ```
    pub fn eat(&mut self, food_token: FoodToken) -> Result<(), FoodToken> {
        if (self.food().len() as u64) < self.population() {
            self.food.push(food_token);
            Ok(())
        } else {
            Err(food_token)
        }
    }

    /// Return all the food from this species.
    pub fn take_food(&mut self) -> Vec<FoodToken> {
        let mut food = vec![];
        self.population = self.food().len() as u64;
        swap(&mut self.food, &mut food);
        food
    }
}

/// Fat functions.
impl Species {
    /// Returns the amount of fat food stored.
    ///
    /// # Examples
    ///
    /// Calling `fat()` on a species **with** fat tissue.
    ///
    /// ```
    /// use evolution::object::*;
    ///
    /// let mut species = Species::default();
    /// assert_eq!(0, species.fat().len());
    /// species.evolve(Trait::FatTissue).unwrap();
    /// species.grow().unwrap();
    /// species.store(vec![FoodToken]).unwrap();
    /// assert_eq!(1, species.fat().len());
    /// ```
    pub fn fat(&self) -> &[FoodToken] {
        &self.fat
    }

    /// Returns true if this species can store the given number of food tokens.
    ///
    /// # Examples
    ///
    /// ```
    /// // In debugging this test, trying to find the line number I noticed the
    /// // line number reported by `cargo test` was 2 larger than the actual
    /// // line number for the test failure. I'm guessing it's for something
    /// // like this.
    /// //
    /// // #[test]
    /// // fn can_store_0() {
    /// use evolution::object::*;
    ///
    /// let mut spec = Species::default();
    /// assert_eq!(None, spec.can_store());
    /// spec.evolve(Trait::FatTissue).unwrap();
    /// spec.grow().unwrap();
    /// assert_eq!(Some(1), spec.can_store());
    /// assert!(spec.store(vec![FoodToken]).is_ok());
    /// // }
    /// ```
    pub fn can_store(&self) -> Option<u64> {
        if self.has_trait(Trait::FatTissue) {
            let amount = self.body_size() - (self.fat().len() as u64);
            if amount > 0 {
                Some(amount)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Store the given food tokens to this species fat. This function returns
    /// an error if the species cannot store the food, either because it
    /// doesn't have the trait, or because too much food was given.
    pub fn store(&mut self, food_tokens: Vec<FoodToken>) -> Result<(), ()> {
        if (self.fat().len() as u64) + (food_tokens.len() as u64) <= self.body_size() {
            self.fat.extend(food_tokens);
            Ok(())
        } else {
            Err(())
        }
    }

    /// Move food from fat to food. This function can only be called on
    /// species with the `FatTissue` trait.
    ///
    /// # Examples
    ///
    /// ```
    /// use evolution::object::*;
    ///
    /// let mut spec = Species::default();
    /// spec.evolve(Trait::FatTissue).unwrap();
    /// spec.grow().unwrap();
    /// spec.store(vec![FoodToken]).unwrap();
    ///
    /// assert_eq!(0, spec.food().len());
    /// assert_eq!(1, spec.fat().len());
    /// spec.digest_fat();
    /// assert_eq!(1, spec.food().len());
    /// assert_eq!(0, spec.fat().len());
    /// ```
    pub fn digest_fat(&mut self) {
        if self.has_trait(Trait::FatTissue) {
            let current_fat = self.fat().len() as u64;
            let amount = cmp::min(self.population() - (self.food().len() as u64), current_fat);
            for _ in 0..amount {
                let token = self.fat.pop().unwrap();
                self.food.push(token);
            }
        }
    }
}

/// Attacking functions.
impl Species {
    /// Returns true if this species can attack the given target species,
    /// given it's neighbors. The traits of each card dictate the result.
    ///
    /// # Undefined Behavior
    ///
    /// The behavior of this function is not defined for attacking extinct
    /// target species.
    ///
    /// # Examples
    ///
    /// ```
    /// use evolution::object::*;
    ///
    /// let mut attacker = Species::default();
    /// attacker.evolve(Trait::Carnivore).expect("unable to evolve");
    /// let defender = Species::default();
    /// // attacker can attack defender with no neighbors to help.
    /// assert!(attacker.can_attack(&defender, None, None));
    /// ```
    pub fn can_attack(&self,
                      target: &Species,
                      left: Option<&Species>,
                      right: Option<&Species>) -> bool
    {
        // This is the body size we use for checks for attacks.
        let self_body_size = if self.has_trait(Trait::PackHunting) {
            self.population() + self.body_size()
        } else {
            self.body_size()
        };

        let blocked_by_burrowing = {
            target.has_trait(Trait::Burrowing) &&
            (target.food().len() as u64) == target.population()
        };

        let blocked_by_climbing = {
            target.has_trait(Trait::Climbing) &&
            !self.has_trait(Trait::Climbing)
        };

        let blocked_by_hard_shell = {
            target.has_trait(Trait::HardShell) &&
            self_body_size < target.body_size() + HARD_SHELL_PROTECTION
        };

        let blocked_by_herding = {
            target.has_trait(Trait::Herding) &&
            self.population() <= target.population()
        };

        let blocked_by_symbiosis = {
            if target.has_trait(Trait::Symbiosis) {
                if let Some(r) = right {
                    r.body_size() > target.body_size()
                } else {
                    false
                }
            } else {
                false
            }
        };

        let blocked_by_warning_call = {
            match (self.has_trait(Trait::Ambush), left, right) {
                (false, Some(s), _) if s.has_trait(Trait::WarningCall) => true,
                (false, _, Some(s)) if s.has_trait(Trait::WarningCall) => true,
                _ => false,
            }
        };

        self.has_trait(Trait::Carnivore) &&
        !(blocked_by_burrowing ||
          blocked_by_climbing ||
          blocked_by_hard_shell ||
          blocked_by_herding ||
          blocked_by_symbiosis ||
          blocked_by_warning_call)
    }
}

impl Default for Species {
    fn default() -> Self {
        Species {
            population: 1,
            body_size: 0,
            traits: Vec::new(),
            food: Vec::new(),
            fat: Vec::new(),
        }
    }
}

#[cfg(feature = "wire")]
mod wire;

#[cfg(test)]
mod tests {
    use object::*;

    #[test]
    fn has_trait() {
        let mut spec = Species::default();
        assert!(!spec.has_trait(Trait::Carnivore));
        spec.evolve(Trait::Carnivore).expect("not able to evolve");
        assert!(spec.has_trait(Trait::Carnivore));
    }

    #[test]
    fn evolve() {
        let mut species = Species::default();

        assert!(!species.has_trait(Trait::Carnivore));

        species.evolve(Trait::Carnivore).expect("couldn't evolve");

        assert!(species.has_trait(Trait::Carnivore));
    }

    #[test]
    fn evolve_invalid_duplicate_trait() {
        let mut species = Species::default();
        species.evolve(Trait::Carnivore).expect("couldn't evolve");

        assert!(species.has_trait(Trait::Carnivore));

        assert!(species.evolve(Trait::Carnivore).is_err());
    }

    #[test]
    fn evolve_invalid_too_many_traits() {
        let mut species = Species::default();
        species.evolve(Trait::Carnivore).expect("couldn't evolve");
        species.evolve(Trait::Burrowing).expect("couldn't evolve");
        species.evolve(Trait::Ambush).expect("couldn't evolve");

        assert!(species.evolve(Trait::FatTissue).is_err());
    }

    #[test]
    fn breed() {
        let mut spec = Species::default();
        for i in 1..MAX_POPULATION {
            assert!(spec.breed().is_ok());
            assert_eq!(i+1, spec.population());
        }
        assert!(spec.breed().is_err());
    }

    #[test]
    fn kill() {
        let mut spec = Species::default();
        spec.breed().expect("can't bread");

        assert_eq!(2, spec.population());
        assert_eq!(false, spec.kill().unwrap());
        assert_eq!(true, spec.kill().unwrap());
    }

    #[test]
    fn food_bounds() {
        let mut spec = Species::default();
        spec.eat(FoodToken).expect("should be able to eat");
        for _ in 0..3 {
            spec.breed().expect("should breed at least thrice");
            spec.eat(FoodToken).expect("should be able to eat");
        }

        assert_eq!(4, spec.population());
        assert_eq!(4, spec.food().len());

        spec.kill().unwrap();
        assert_eq!(3, spec.food().len());

        spec.kill().unwrap();
        assert_eq!(2, spec.food().len());

        spec.kill().unwrap();
        assert_eq!(1, spec.food().len());
    }

    #[test]
    fn extinct() {
        let mut spec = Species::default();

        assert!(!spec.is_extinct());
        assert_eq!(1, spec.population());

        spec.kill().unwrap();

        assert!(spec.is_extinct());
    }

    #[test]
    fn grow() {
        let mut spec = Species::default();
        for i in 0..MAX_BODY_SIZE {
            assert!(spec.grow().is_ok());
            assert_eq!(i+1, spec.body_size());
        }
        assert!(spec.grow().is_err());
    }

    #[test]
    fn eat() {
        let mut spec = Species::default();
        spec.breed().unwrap();
        spec.breed().unwrap();

        assert_eq!(0, spec.food().len());

        assert!(spec.eat(FoodToken).is_ok());
        assert!(spec.eat(FoodToken).is_ok());

        assert_eq!(2, spec.food().len());
    }

    #[test]
    fn digest_fat_on_fat_tissue_species_with_fat() {
        let mut spec = Species::default();
        spec.evolve(Trait::FatTissue).unwrap();
        spec.grow().unwrap();
        spec.store(vec![FoodToken]).unwrap();

        assert_eq!(0, spec.food().len());
        assert_eq!(1, spec.fat().len());

        spec.digest_fat();

        assert_eq!(1, spec.food().len());
        assert_eq!(0, spec.fat().len());
    }

    #[test]
    fn digest_fat_on_fat_tissue_species_without_fat() {
        let mut spec = Species::default();
        spec.evolve(Trait::FatTissue).unwrap();

        assert_eq!(0, spec.food().len());
        assert_eq!(0, spec.fat().len());

        spec.digest_fat();

        assert_eq!(0, spec.food().len());
        assert_eq!(0, spec.fat().len());
    }

    #[test]
    #[ignore] // TODO: When we always have a fat store.
    fn digest_fat_on_non_fat_tissue_species() {
        let mut spec = Species::default();

        assert_eq!(0, spec.food().len());
        assert_eq!(0, spec.fat().len());

        spec.digest_fat();

        assert_eq!(0, spec.food().len());
        assert_eq!(0, spec.fat().len());
    }

    #[test]
    fn cull() {
        let mut spec = Species::default();
        spec.breed().unwrap();
        spec.eat(FoodToken).unwrap();

        assert_eq!(2, spec.population());
        assert_eq!(1, spec.food().len());

        assert_eq!(false, spec.cull().unwrap());
        assert_eq!(1, spec.population());
    }

    #[test]
    fn cull_to_extinction() {
        let mut spec = Species::default();

        assert_eq!(1, spec.population());
        assert_eq!(0, spec.food().len());

        spec.cull().unwrap();

        assert!(spec.is_extinct());
    }

    #[test]
    fn take_food() {
        let mut spec = Species::default();
        spec.breed().unwrap();
        spec.eat(FoodToken).unwrap();
        spec.eat(FoodToken).unwrap();

        assert_eq!(2, spec.food().len());
        assert_eq!(vec![FoodToken, FoodToken], spec.take_food());
    }

    #[test]
    fn can_attack_default_to_default() {
        let attacker = Species::default();
        let victim = Species::default();

        assert!(!attacker.can_attack(&victim, None, None));
    }

    #[test]
    fn can_attack_carnivore_to_default() {
        let mut attacker = Species::default();
        attacker.evolve(Trait::Carnivore).expect("not able to evolve");
        let victim = Species::default();

        assert!(attacker.can_attack(&victim, None, None));
    }

    #[test]
    fn can_attack_carnivore_to_any_pop() {
        let mut attacker = Species::default();
        attacker.evolve(Trait::Carnivore).expect("not able to evolve");
        let mut victim = Species::default();
        victim.breed().expect("failed to breed");

        assert!(attacker.can_attack(&victim, None, None));

        for _ in 0..5 {
            attacker.breed().expect("failed to breed");
        }

        assert!(attacker.can_attack(&victim, None, None));
    }

    #[test]
    fn can_attack_carnivore_to_any_bod() {
        let mut attacker = Species::default();
        attacker.evolve(Trait::Carnivore).expect("not able to evolve");
        let mut victim = Species::default();
        victim.grow().expect("not able to grow");

        assert!(attacker.can_attack(&victim, None, None));

        for _ in 0..5 {
            attacker.grow().expect("not able to grow");
        }

        assert!(attacker.can_attack(&victim, None, None));
    }

    #[test]
    fn can_attack_ambush_and_warning_call() {
        let mut attacker = Species::default();
        attacker.evolve(Trait::Carnivore).expect("not able to evolve");
        let mut caller = Species::default();
        caller.evolve(Trait::WarningCall).expect("not able to evolve");
        let left = Species::default();
        let right = Species::default();

        assert!(attacker.can_attack(&caller, Some(&left), Some(&right)));
        assert!(!attacker.can_attack(&left, None, Some(&caller)));
        assert!(!attacker.can_attack(&right, Some(&caller), None));

        attacker.evolve(Trait::Ambush).expect("not able to evolve");

        assert!(attacker.can_attack(&caller, Some(&left), Some(&right)));
        assert!(attacker.can_attack(&left, None, Some(&caller)));
        assert!(attacker.can_attack(&right, Some(&caller), None));
    }

    #[test]
    fn can_attack_burrowing() {
        let mut board = Board::default();
        board.push_food(FoodToken);
        let mut attacker = Species::default();
        attacker.evolve(Trait::Carnivore).expect("not able to evolve");
        let mut burrower = Species::default();
        burrower.evolve(Trait::Burrowing).expect("not able to evolve");

        while (burrower.food().len() as u64) < burrower.population() {
            burrower.eat(FoodToken).expect("not able to feed");
        }

        assert!(!attacker.can_attack(&burrower, None, None));
    }

    #[test]
    fn can_attack_climbing() {
        let mut attacker = Species::default();
        attacker.evolve(Trait::Carnivore).expect("not able to evolve");
        let mut climber = Species::default();
        climber.evolve(Trait::Climbing).expect("not able to evolve");

        assert!(!attacker.can_attack(&climber, None, None));

        attacker.evolve(Trait::Climbing).expect("not able to evolve");
        assert!(attacker.can_attack(&climber, None, None));
    }

    #[test]
    fn can_attack_hard_shell() {
        let mut attacker = Species::default();
        attacker.evolve(Trait::Carnivore).expect("not able to evolve");
        let mut hard_shell = Species::default();
        hard_shell.evolve(Trait::HardShell).expect("not able to evolve");

        assert!(!attacker.can_attack(&hard_shell, None, None));

        for _ in 0..3 {
            attacker.grow().expect("not able to grow");
        }
        assert!(!attacker.can_attack(&hard_shell, None, None));

        attacker.grow().expect("not able to grow");
        assert!(attacker.can_attack(&hard_shell, None, None));
    }

    #[test]
    fn can_attack_herding() {
        let mut attacker = Species::default();
        attacker.evolve(Trait::Carnivore).expect("not able to evolve");
        let mut herder = Species::default();
        herder.evolve(Trait::Herding).expect("not able to evolve");

        assert!(!attacker.can_attack(&herder, None, None));

        attacker.breed().expect("failed to breed");
        assert!(attacker.can_attack(&herder, None, None));

        herder.breed().expect("failed to breed");
        herder.breed().expect("failed to breed");
        assert!(!attacker.can_attack(&herder, None, None));
    }

    #[test]
    fn can_attack_pack_hunting() {
        let mut attacker = Species::default();
        attacker.evolve(Trait::Carnivore).expect("not able to evolve");
        attacker.evolve(Trait::PackHunting).expect("not able to evolve");
        let mut hard_shell = Species::default();
        hard_shell.evolve(Trait::HardShell).expect("not able to evolve");

        assert!(!attacker.can_attack(&hard_shell, None, None));

        attacker.breed().expect("failed to breed");
        attacker.breed().expect("failed to breed");
        assert!(!attacker.can_attack(&hard_shell, None, None));

        attacker.breed().expect("failed to breed");
        assert!(attacker.can_attack(&hard_shell, None, None));
    }

    #[test]
    fn can_attack_symbiosis() {
        let mut attacker = Species::default();
        attacker.evolve(Trait::Carnivore).expect("not able to evolve");
        let mut victim = Species::default();
        victim.evolve(Trait::Symbiosis).expect("not able to evolve");
        let mut right = Species::default();

        assert!(attacker.can_attack(&victim, None, Some(&right)));

        right.grow().expect("not able to grow");
        assert!(!attacker.can_attack(&victim, None, Some(&right)));
    }

    #[test]
    fn can_store_no_fat_tissue() {
        let species = Species::default();
        assert_eq!(None, species.can_store());
    }

    #[test]
    fn can_store_fat_tissue() {
        let mut species = Species::default();

        assert_eq!(None, species.can_store());

        species.evolve(Trait::FatTissue).expect("not able to evolve");
        assert_eq!(None, species.can_store());

        species.grow().expect("not able to grow");
        assert_eq!(Some(1), species.can_store());

        species.grow().expect("not able to grow");
        assert_eq!(Some(2), species.can_store());

        species.store(vec![FoodToken]).expect("not able to store");
        assert_eq!(Some(1), species.can_store());
    }

    #[test]
    fn can_store_fat_tissue_to_full() {
        let mut species = Species::default();
        species.evolve(Trait::FatTissue).expect("not able to evolve");
        let size = 5;
        for _ in 0..size {
            species.grow().expect("not able to grow");
        }

        assert_eq!(Some(size), species.can_store());
    }

    #[test]
    fn exchange_trait_adds_fat_tissue() {
        let mut species = Species::default();
        species.evolve(Trait::Ambush).unwrap();

        assert!(species.fat().is_empty());

        species.exchange_trait(0, Trait::FatTissue).unwrap();

        assert!(species.has_trait(Trait::FatTissue));
        assert!(species.fat().is_empty());
    }

    #[test]
    fn exchange_trait_removes_fat_tissue() {
        let mut species = Species::default();
        species.evolve(Trait::FatTissue).unwrap();
        species.grow().unwrap();
        species.store(vec![FoodToken]).unwrap();

        assert!(!species.fat().is_empty());

        species.exchange_trait(0, Trait::Ambush).unwrap();

        assert!(!species.has_trait(Trait::FatTissue));
        assert!(species.fat().is_empty());
    }

    #[test]
    fn exchange_trait_replaces_fat_tissue() {
        let mut species = Species::default();
        species.evolve(Trait::FatTissue).unwrap();
        species.grow().unwrap();
        species.store(vec![FoodToken]).unwrap();

        assert!(!species.fat().is_empty());

        species.exchange_trait(0, Trait::FatTissue).unwrap();

        assert!(species.has_trait(Trait::FatTissue));
        assert!(species.fat().is_empty());
    }

    #[test]
    fn exchange_trait_swaps_in_place() {
        let mut species = Species::default();
        species.evolve(Trait::Ambush).unwrap();
        species.evolve(Trait::FatTissue).unwrap();

        assert_eq!(vec![Trait::Ambush, Trait::FatTissue], species.traits());

        species.exchange_trait(0, Trait::Climbing).expect("couldn't exchange trait");

        assert_eq!(vec![Trait::Climbing, Trait::FatTissue], species.traits());
    }

    #[test]
    fn exchange_trait_checks_duplicates() {
        let mut species = Species::default();
        species.evolve(Trait::Ambush).unwrap();
        species.evolve(Trait::FatTissue).unwrap();

        assert_eq!(vec![Trait::Ambush, Trait::FatTissue], species.traits());

        assert!(species.exchange_trait(0, Trait::FatTissue).is_err());
    }

    #[test]
    fn exchange_trait_allows_identical_index_duplicates() {
        let mut species = Species::default();
        species.evolve(Trait::Carnivore).unwrap();

        assert!(species.exchange_trait(0, Trait::Carnivore).is_ok());
        assert!(species.has_trait(Trait::Carnivore));
    }

    #[test]
    fn exchange_trait_index_bounds() {
        let mut species = Species::default();
        species.evolve(Trait::Ambush).expect("couldn't evolve");

        assert!(species.exchange_trait(1, Trait::Burrowing).is_err());
        assert!(species.has_trait(Trait::Ambush));
        assert!(!species.has_trait(Trait::Burrowing));
    }
}
