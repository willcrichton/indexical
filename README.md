# Indexical: Human-Friendly Indexed Collections

Indexical is a library for conveniently and efficiently working with indexed collections of objects.
"Indexed" means that the domain of objects is finite, and you can assign a numeric index to each object.
This enables the use of efficient data structures like bit-sets.

Indexical is a layer on top of existing bit-set libraries like [`bitvec`](https://github.com/ferrilab/bitvec)
and [`rustc_index::bit_set`](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_index/bit_set/index.html).
Those data structures only "understand" raw indexes, not the objects represented by the index.
Indexical provides utilities for converting between the object domain and the index domain.

## Example

```rust,no_run
use indexical::{IndexedDomain, IndexedValue, RcIndexSet};
use std::rc::Rc;

// 1. Define a custom type.
#[derive(PartialEq, Eq, Clone, Hash)]
pub struct MyString(String);

// 2. Define a new index for your custom type.
indexical::define_index_type! {
    pub struct StringIndex for MyString = u32;
}

// 3. Create an immutable indexed domain from a collection of objects.
// The domain should be wrapped in an Rc, Arc, or &-ref.
let domain = Rc::new(IndexedDomain::from_iter([
    MyString(String::from("Hello")), MyString(String::from("world"))
]));

// 4. Now you can make a set. Notice how you can pass either a `MyString`
// or a `StringIndex` to `set.insert(..)` and `set.contains(..)`.
let mut set = RcIndexSet::new(&domain);
set.insert(MyString(String::from("Hello")));
set.insert(StringIndex::from_usize(1));
assert!(set.contains(&MyString(String::from("world"))));
```
