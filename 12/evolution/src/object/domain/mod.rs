use std::ops::{Deref, Index, IndexMut};
use object::*;

/// A collection of species for a player.
///
/// A domain is explicitly ordered due to the nature of species with traits
/// that effect their neighbors. There is no limit to the size of a domain
/// directly, although in practice the size is limited by the number of cards
/// any one player may use to create a new species.
///
/// Iterating over a domain is a common use case, and is very convenient. The
/// iterator yields not only each species, but also thier neighbors.
///
/// Accessing a species is also simple. A domain implements `Index` so by
/// passing a species index, you can access a species directly.
///
/// # Examples
///
/// ```
/// use evolution::object::*;
///
/// // Create an empty domain.
/// let mut domain = Domain::default();
///
/// // Add a few species.
/// domain.add(Placement::Left);
/// domain.add(Placement::Left);
/// domain.add(Placement::Left);
///
/// // Iterate over each of the species.
/// for (species, left, _) in &domain {
///     if let Some(l) = left {
///         println!("{:?} has a left neighbor {:?}", species, l);
///     }
/// }
///
/// // Get the second species in the domain.
/// assert_eq!(1, domain[1].population());
/// ```
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Domain(Vec<Species>);

impl Domain {
    /// Add a new species to the player, given the placement for it. A
    /// placement indicates where to add the new species relative to the other
    /// species in the domain. The added species will have the default species
    /// attributes.
    ///
    /// # Examples
    ///
    /// ```
    /// use evolution::object::*;
    ///
    /// let mut domain = Domain::default();
    /// domain.add(Placement::Right);
    /// domain.add(Placement::Left);
    /// assert_eq!(2, domain.len());
    /// ```
    pub fn add(&mut self, placement: Placement) -> &mut Species {
        let new = Species::default();
        let index = match placement {
            Placement::Left => {
                self.0.insert(0, new);
                0
            },
            Placement::Right => {
                self.0.push(new);
                self.len() - 1
            },
        };
        &mut self.0[index]
    }

    /// Kill one population of the given species in this domain, returning
    /// true if the species went extinct.
    pub fn kill(&mut self, species_idx: usize) -> Result<bool, ()> {
        if try!(self[species_idx].kill()) {
            self.0.remove(species_idx);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Cull all the species in this domain, removing extinct species in the
    /// process. This function returns the number of species that went
    /// extinct.
    pub fn cull(&mut self) -> Result<usize, ()> {
        let mut extinctions = 0;
        for species in self.0.iter_mut() {
            if try!(species.cull()) {
                extinctions += 1;
            }
        }
        self.0.retain(|s| !s.is_extinct());
        Ok(extinctions)
    }

    /// Take all of the food from all of the species and return it.
    pub fn take_food(&mut self) -> Vec<FoodToken> {
        self.0.iter_mut().fold(vec![], |mut food, species| {
            food.extend(species.take_food());
            food
        })
    }
}

impl Deref for Domain {
    type Target = [Species];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Vec<Species>> for Domain {
    fn from(species: Vec<Species>) -> Domain {
        Domain(species)
    }
}

impl From<Domain> for Vec<Species> {
    fn from(domain: Domain) -> Vec<Species> {
        domain.0
    }
}

impl Index<usize> for Domain {
    type Output = Species;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Domain {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

/// Iteration over the domain with neighbors.
mod iter;

#[cfg(feature = "wire")]
mod wire;

#[cfg(test)]
mod tests {
    use object::*;

    #[test]
    fn add() {
        let mut domain = Domain::default();

        assert_eq!(0, domain.len());

        domain.add(Placement::Right);

        assert_eq!(1, domain.len());
    }

    #[test]
    fn kill() {
        let mut domain = Domain::default();
        domain.add(Placement::Right);
        domain.add(Placement::Right);
        domain[1].breed().unwrap();

        assert_eq!(2, domain.len());
        assert_eq!(1, domain[0].population());
        assert_eq!(2, domain[1].population());

        assert!(domain.kill(0).unwrap());
        assert!(!domain.kill(0).unwrap());

        assert_eq!(1, domain.len());
        assert_eq!(1, domain[0].population());
    }

    #[test]
    fn cull() {
        let mut domain = Domain::default();
        domain.add(Placement::Right);
        domain.add(Placement::Right);
        domain[0].breed().unwrap();
        domain[0].eat(FoodToken).unwrap();

        assert_eq!(1, domain.cull().unwrap());
        assert_eq!(1, domain[0].population());
    }

    #[test]
    fn take_food() {
        let mut domain = Domain::default();
        domain.add(Placement::Right);
        domain.add(Placement::Right);
        domain[0].breed().unwrap();
        domain[0].eat(FoodToken).unwrap();
        domain[0].eat(FoodToken).unwrap();
        domain[1].eat(FoodToken).unwrap();

        assert_eq!(vec![FoodToken, FoodToken, FoodToken], domain.take_food());
    }

    #[test]
    fn from_vec() {
        let small_species = Species::default();
        let mut medium_species = Species::default();
        medium_species.grow().unwrap();
        let mut large_species = Species::default();
        large_species.grow().unwrap();
        large_species.grow().unwrap();
        let species_vec = vec![
            small_species,
            medium_species,
            large_species,
        ];
        let domain = Domain::from(species_vec);

        assert_eq!(3, domain.len());
        assert_eq!(0, domain[0].body_size());
        assert_eq!(1, domain[1].body_size());
        assert_eq!(2, domain[2].body_size());
    }

    #[test]
    fn to_vec() {
        let mut domain = Domain::default();
        domain.add(Placement::Right);
        domain.add(Placement::Right);
        domain[1].grow().unwrap();
        domain.add(Placement::Right);
        domain[2].grow().unwrap();
        domain[2].grow().unwrap();

        let species_vec = Vec::<_>::from(domain);

        assert_eq!(3, species_vec.len());
        assert_eq!(0, species_vec[0].body_size());
        assert_eq!(1, species_vec[1].body_size());
        assert_eq!(2, species_vec[2].body_size());
    }
}
