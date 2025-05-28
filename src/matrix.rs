use rustc_hash::FxHashMap;
use std::{fmt, hash::Hash};

use crate::{
    IndexSet, IndexedDomain, IndexedValue, ToIndex, bitset::BitSet, pointer::PointerFamily,
};

/// An unordered collections of pairs `(R, C)`, implemented with a sparse bit-matrix.
///
/// "Sparse" means "hash map from rows to bit-sets of columns". Subsequently, only column types `C` must be indexed,
/// while row types `R` only need be hashable.
pub struct IndexMatrix<'a, R, C: IndexedValue + 'a, S: BitSet, P: PointerFamily<'a>> {
    pub(crate) matrix: FxHashMap<R, IndexSet<'a, C, S, P>>,
    empty_set: IndexSet<'a, C, S, P>,
    col_domain: P::Pointer<IndexedDomain<C>>,
}

impl<'a, R, C, S, P> IndexMatrix<'a, R, C, S, P>
where
    R: PartialEq + Eq + Hash + Clone,
    C: IndexedValue + 'a,
    S: BitSet,
    P: PointerFamily<'a>,
{
    /// Creates an empty matrix.
    pub fn new(col_domain: &P::Pointer<IndexedDomain<C>>) -> Self {
        IndexMatrix {
            matrix: FxHashMap::default(),
            empty_set: IndexSet::new(col_domain),
            col_domain: col_domain.clone(),
        }
    }

    pub(crate) fn ensure_row(&mut self, row: R) -> &mut IndexSet<'a, C, S, P> {
        self.matrix
            .entry(row)
            .or_insert_with(|| self.empty_set.clone())
    }

    /// Inserts a pair `(row, col)` into the matrix, returning true if `self` changed.
    pub fn insert<M>(&mut self, row: R, col: impl ToIndex<C, M>) -> bool {
        let col = col.to_index(&self.col_domain);
        self.ensure_row(row).insert(col)
    }

    /// Adds all elements of `from` into the row `into`.
    pub fn union_into_row(&mut self, into: R, from: &IndexSet<'a, C, S, P>) -> bool {
        self.ensure_row(into).union_changed(from)
    }

    /// Adds all elements from the row `from` into the row `into`.
    pub fn union_rows(&mut self, from: R, to: R) -> bool {
        if from == to {
            return false;
        }

        self.ensure_row(from.clone());
        self.ensure_row(to.clone());

        // SAFETY: `from` != `to` therefore this is a disjoint mutable borrow
        let [Some(from), Some(to)] =
            (unsafe { self.matrix.get_disjoint_unchecked_mut([&from, &to]) })
        else {
            unreachable!()
        };
        to.union_changed(from)
    }

    /// Returns an iterator over the elements in `row`.
    pub fn row(&self, row: &R) -> impl Iterator<Item = &C> + use<'a, '_, R, C, S, P> {
        self.matrix.get(row).into_iter().flat_map(|set| set.iter())
    }

    /// Returns an iterator over all rows in the matrix.
    pub fn rows(
        &self,
    ) -> impl Iterator<Item = (&R, &IndexSet<'a, C, S, P>)> + use<'a, '_, R, C, S, P> {
        self.matrix.iter()
    }

    /// Returns the [`IndexSet`] for a particular `row`.
    pub fn row_set(&self, row: &R) -> &IndexSet<'a, C, S, P> {
        self.matrix.get(row).unwrap_or(&self.empty_set)
    }

    /// Clears all the elements from the `row`.
    pub fn clear_row(&mut self, row: &R) {
        self.matrix.remove(row);
    }

    /// Returns the [`IndexedDomain`] for the column type.
    pub fn col_domain(&self) -> &P::Pointer<IndexedDomain<C>> {
        &self.col_domain
    }
}

impl<'a, R, C, S, P> PartialEq for IndexMatrix<'a, R, C, S, P>
where
    R: PartialEq + Eq + Hash + Clone,
    C: IndexedValue + 'a,
    S: BitSet,
    P: PointerFamily<'a>,
{
    fn eq(&self, other: &Self) -> bool {
        self.matrix == other.matrix
    }
}

impl<'a, R, C, S, P> Eq for IndexMatrix<'a, R, C, S, P>
where
    R: PartialEq + Eq + Hash + Clone,
    C: IndexedValue + 'a,
    S: BitSet,
    P: PointerFamily<'a>,
{
}

impl<'a, R, C, S, P> Clone for IndexMatrix<'a, R, C, S, P>
where
    R: PartialEq + Eq + Hash + Clone,
    C: IndexedValue + 'a,
    S: BitSet,
    P: PointerFamily<'a>,
{
    fn clone(&self) -> Self {
        Self {
            matrix: self.matrix.clone(),
            empty_set: self.empty_set.clone(),
            col_domain: self.col_domain.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        for col in self.matrix.values_mut() {
            col.clear();
        }

        for (row, col) in source.matrix.iter() {
            self.ensure_row(row.clone()).clone_from(col);
        }

        self.empty_set = source.empty_set.clone();
        self.col_domain = source.col_domain.clone();
    }
}

impl<'a, R, C, S, P> fmt::Debug for IndexMatrix<'a, R, C, S, P>
where
    R: PartialEq + Eq + Hash + Clone + fmt::Debug,
    C: IndexedValue + fmt::Debug + 'a,
    S: BitSet,
    P: PointerFamily<'a>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.rows()).finish()
    }
}

#[cfg(test)]
mod test {
    use crate::{IndexedDomain, test_utils::TestIndexMatrix};
    use std::rc::Rc;

    fn mk(s: &str) -> String {
        s.to_string()
    }

    #[test]
    fn test_indexmatrix() {
        let col_domain = Rc::new(IndexedDomain::from_iter([mk("a"), mk("b"), mk("c")]));
        let mut mtx = TestIndexMatrix::new(&col_domain);
        mtx.insert(0, mk("b"));
        mtx.insert(1, mk("c"));
        assert_eq!(mtx.row(&0).collect::<Vec<_>>(), vec!["b"]);
        assert_eq!(mtx.row(&1).collect::<Vec<_>>(), vec!["c"]);

        assert!(mtx.union_rows(0, 1));
        assert_eq!(mtx.row(&1).collect::<Vec<_>>(), vec!["b", "c"]);
    }
}
