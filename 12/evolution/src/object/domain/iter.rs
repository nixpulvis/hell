use object::*;

/// A reference to a species and it's neighbors.
///
/// The first element of the tuple is the species, and the next two elements
/// are the left and right neighbors respectively.
pub type SpeciesGroup<'a> = (&'a Species, Option<&'a Species>, Option<&'a Species>);


impl<'a> IntoIterator for &'a Domain {
    type Item = SpeciesGroup<'a>;
    type IntoIter = DomainIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        DomainIter::new(self)
    }
}

pub struct DomainIter<'a> {
    domain: &'a Domain,
    index: usize,
}

impl<'a> DomainIter<'a> {
    pub fn new(domain: &'a Domain) -> DomainIter<'a> {
        DomainIter {
            domain: domain,
            index: 0,
        }
    }
}

impl<'a> Iterator for DomainIter<'a> {
    type Item = SpeciesGroup<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.domain.get(self.index).map(|s| {
            // NOTE: If the domain slice is exactly the size of a usize, this is
            // incorrect.
            let left = self.domain.get(self.index.wrapping_sub(1));
            let right = self.domain.get(self.index + 1);
            (s, left, right)
        });
        self.index += 1;
        n
    }
}
