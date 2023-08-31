use index_vec::IndexVec;
use std::{collections::HashMap, fmt};

use crate::IndexedValue;

pub struct IndexedDomain<T: IndexedValue> {
  domain: IndexVec<T::Index, T>,
  reverse_map: HashMap<T, T::Index>,
}

impl<T: IndexedValue> IndexedDomain<T> {
  pub fn new(domain: IndexVec<T::Index, T>) -> Self {
    let reverse_map = domain
      .iter_enumerated()
      .map(|(idx, value)| (value.clone(), idx))
      .collect();
    IndexedDomain {
      domain,
      reverse_map,
    }
  }

  pub fn value(&self, index: T::Index) -> &T {
    &self.domain[index]
  }

  pub fn index(&self, value: &T) -> T::Index {
    self.reverse_map[value]
  }

  pub fn contains(&self, value: &T) -> bool {
    self.reverse_map.contains_key(value)
  }

  pub fn insert(&mut self, value: T) -> T::Index {
    self.domain.push(value)
  }

  pub fn as_vec(&self) -> &IndexVec<T::Index, T> {
    &self.domain
  }

  pub fn len(&self) -> usize {
    self.domain.len()
  }

  pub fn is_empty(&self) -> bool {
    self.len() == 0
  }

  pub fn ensure(&mut self, value: &T) -> T::Index {
    if !self.contains(value) {
      self.insert(value.clone())
    } else {
      self.index(value)
    }
  }
}

impl<T: IndexedValue> FromIterator<T> for IndexedDomain<T> {
  fn from_iter<Iter: IntoIterator<Item = T>>(iter: Iter) -> Self {
    let domain = iter.into_iter().collect();
    IndexedDomain::new(domain)
  }
}

impl<T: IndexedValue + fmt::Debug> fmt::Debug for IndexedDomain<T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{:?}", self.domain)
  }
}

#[test]
fn test_domain() {
  let d = IndexedDomain::from_iter(["a", "b"]);
  let a = d.index(&"a");
  let b = d.index(&"b");
  assert_eq!(d.value(a), &"a");
  assert_eq!(d.value(b), &"b");
  assert!(d.contains(&"a"));
  assert!(!d.contains(&"c"));
  assert_eq!(d.len(), 2);
}
