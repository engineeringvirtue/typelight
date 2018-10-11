extern crate sciter;

use std::collections::{HashSet};
use std::hash::{Hash, BuildHasher};

pub trait ToggleableKey<T> {
    fn toggle(&mut self, item:T) -> bool;
}

impl<T,S> ToggleableKey<T> for HashSet<T, S>
    where T: Eq + Hash, S: BuildHasher
{
    fn toggle(&mut self, item:T) -> bool {
        if self.contains(&item) { self.remove(&item) } else { self.insert(item) }
    }
}