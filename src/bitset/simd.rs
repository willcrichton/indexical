//! A custom SIMD-accelerated bit-set.
//!
//! Implementation is largely derived from the `bitsvec` crate: <https://github.com/psiace/bitsvec>
//!
//! The main difference is I made a much more efficient iterator that computes the indices
//! of the 1-bits.
//!
//! **WARNING:** this module makes liberal use of unsafe code and has not been thoroughly vetted,
//! so use it at your own risk.

use crate::{
    bitset::BitSet,
    pointer::{ArcFamily, RcFamily, RefFamily},
};
use std::{
    mem::size_of,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXorAssign, Not},
    simd::{LaneCount, Simd, SimdElement, SupportedLaneCount},
    slice,
};

/// Capabilities of an element that represent a SIMD lane
pub trait SimdSetElement:
    SimdElement
    + BitOr<Output = Self>
    + BitAnd<Output = Self>
    + Not<Output = Self>
    + BitOrAssign
    + BitXorAssign
    + BitAndAssign
    + PartialEq
    + 'static
{
    /// The `0` value.
    const ZERO: Self;

    /// The `1` value.
    const ONE: Self;

    /// The largest value.
    const MAX: Self;

    /// Efficient bit-shift-left.
    ///
    /// # Safety
    /// `rhs < size_of::<Self>()`
    unsafe fn unchecked_shl(self, rhs: u32) -> Self;

    /// Efficient bit-shift-right.
    ///
    /// # Safety
    /// `rhs < size_of::<Self>()`
    unsafe fn unchecked_shr(self, rhs: u32) -> Self;

    /// The number of zeros before the first 1 bit, counting from LSB.
    fn trailing_zeros(self) -> u32;

    /// The number of 1 bits in the element.
    fn count_ones(self) -> u32;
}

macro_rules! simd_set_element_impl {
    ($n:ty) => {
        impl SimdSetElement for $n {
            const ZERO: Self = 0;
            const ONE: Self = 1;
            const MAX: Self = Self::MAX;

            unsafe fn unchecked_shl(self, rhs: u32) -> Self {
                unsafe { <$n>::unchecked_shl(self, rhs) }
            }

            unsafe fn unchecked_shr(self, rhs: u32) -> Self {
                unsafe { <$n>::unchecked_shr(self, rhs) }
            }

            fn trailing_zeros(self) -> u32 {
                <$n>::trailing_zeros(self)
            }

            fn count_ones(self) -> u32 {
                <$n>::count_ones(self)
            }
        }
    };
}

simd_set_element_impl!(u8);
simd_set_element_impl!(u16);
simd_set_element_impl!(u32);
simd_set_element_impl!(u64);

/// A dense bit-set with SIMD-accelerated operations.
#[derive(PartialEq, Clone)]
pub struct SimdBitset<T, const N: usize>
where
    T: SimdSetElement,
    LaneCount<N>: SupportedLaneCount,
{
    chunks: Vec<Simd<T, N>>,
    nbits: usize,
}

impl<T: SimdSetElement, const N: usize> SimdBitset<T, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    const fn chunk_size() -> usize {
        Self::lane_size() * N
    }

    const fn lane_size() -> usize {
        size_of::<T>() * 8
    }

    const fn coords(&self, index: usize) -> (usize, usize, u32) {
        let (chunk, index) = (index / Self::chunk_size(), index % Self::chunk_size());
        let (lane, index) = (index / Self::lane_size(), index % Self::lane_size());
        (chunk, lane, index as u32)
    }

    fn get(&self, chunk_idx: usize, lane_idx: usize, bit: u32) -> bool {
        debug_assert!(chunk_idx < self.chunks.len());
        debug_assert!(lane_idx < N);
        debug_assert!(bit < Self::lane_size() as u32);

        unsafe {
            let chunk = self.chunks.get_unchecked(chunk_idx);
            let lane = chunk.as_array().get_unchecked(lane_idx);
            lane.unchecked_shr(bit) & T::ONE == T::ONE
        }
    }

    fn zip_mut(&mut self, other: &Self, mut op: impl FnMut(&mut Simd<T, N>, &Simd<T, N>)) {
        debug_assert!(other.chunks.len() == self.chunks.len());

        let mut dst = self.chunks.as_mut_ptr();
        let mut src = other.chunks.as_ptr();

        unsafe {
            let dst_end = dst.add(self.chunks.len());

            while dst != dst_end {
                op(&mut *dst, &*src);
                dst = dst.add(1);
                src = src.add(1);
            }
        }
    }
}

/// Iterator over the 1-bits of a [`SimdBitset`].
pub struct SimdSetIter<'a, T, const N: usize>
where
    T: SimdSetElement,
    LaneCount<N>: SupportedLaneCount,
{
    set: &'a SimdBitset<T, N>,
    index: usize,
    chunk_iter: slice::Iter<'a, Simd<T, N>>,
    lane_iter: slice::Iter<'a, T>,
    lane: T,
}
impl<'a, T, const N: usize> SimdSetIter<'a, T, N>
where
    T: SimdSetElement,
    LaneCount<N>: SupportedLaneCount,
{
    fn new(set: &'a SimdBitset<T, N>) -> Self {
        let mut chunk_iter = set.chunks.iter();
        let chunk = chunk_iter.next().unwrap();
        let mut lane_iter = chunk.as_array().iter();
        let lane = *lane_iter.next().unwrap();

        SimdSetIter {
            set,
            index: 0,
            chunk_iter,
            lane_iter,
            lane,
        }
    }
}

impl<T, const N: usize> Iterator for SimdSetIter<'_, T, N>
where
    T: SimdSetElement,
    LaneCount<N>: SupportedLaneCount,
{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.set.nbits {
            return None;
        }

        let lane_size = SimdBitset::<T, N>::lane_size() as u32;
        while self.lane == T::ZERO {
            self.index += lane_size as usize;

            let zero_simd = Simd::splat(T::ZERO);
            let chunk_size = lane_size as usize * N;
            match self.lane_iter.next() {
                Some(lane) => {
                    self.lane = *lane;
                }
                None => loop {
                    match self.chunk_iter.next() {
                        Some(chunk) => {
                            if *chunk == zero_simd {
                                self.index += chunk_size;
                                continue;
                            }
                            self.lane_iter = chunk.as_array().iter();
                            self.lane = *self.lane_iter.next().unwrap();
                            break;
                        }
                        None => return None,
                    }
                },
            }
        }

        let zeros = self.lane.trailing_zeros();
        let idx = self.index + zeros as usize;
        self.lane ^= unsafe { T::ONE.unchecked_shl(zeros) };
        if idx >= self.set.nbits {
            self.index = self.set.nbits;
            return None;
        }
        Some(idx)
    }
}

impl<T: SimdSetElement, const N: usize> BitSet for SimdBitset<T, N>
where
    LaneCount<N>: SupportedLaneCount,
    Simd<T, N>: for<'a> BitOr<&'a Simd<T, N>, Output = Simd<T, N>>,
    Simd<T, N>: for<'a> BitAnd<&'a Simd<T, N>, Output = Simd<T, N>>,
{
    fn empty(nbits: usize) -> Self {
        let n_chunks = nbits.div_ceil(Self::chunk_size());
        SimdBitset {
            chunks: vec![Simd::from([T::ZERO; N]); n_chunks],
            nbits,
        }
    }

    fn insert(&mut self, index: usize) -> bool {
        let (chunk_idx, lane_idx, bit) = self.coords(index);

        debug_assert!(chunk_idx < self.chunks.len());
        debug_assert!(lane_idx < N);
        debug_assert!(bit < Self::lane_size() as u32);

        unsafe {
            let chunk = self.chunks.get_unchecked_mut(chunk_idx);
            let lane = chunk.as_mut_array().get_unchecked_mut(lane_idx);
            *lane |= T::ONE.unchecked_shl(bit);
        }

        true
    }

    fn remove(&mut self, index: usize) -> bool {
        let (chunk_idx, lane_idx, bit) = self.coords(index);

        debug_assert!(chunk_idx < self.chunks.len());
        debug_assert!(lane_idx < N);
        debug_assert!(bit < Self::lane_size() as u32);

        unsafe {
            let chunk = self.chunks.get_unchecked_mut(chunk_idx);
            let lane = chunk.as_mut_array().get_unchecked_mut(lane_idx);
            *lane &= T::ZERO.unchecked_shl(bit);
        }

        true
    }

    fn contains(&self, index: usize) -> bool {
        let (chunk_idx, lane_idx, bit) = self.coords(index);
        self.get(chunk_idx, lane_idx, bit)
    }

    fn iter(&self) -> impl Iterator<Item = usize> {
        SimdSetIter::new(self)
    }

    fn len(&self) -> usize {
        let mut n = 0;
        for chunk in &self.chunks {
            for lane in chunk.as_array() {
                n += lane.count_ones();
            }
        }
        n as usize
    }

    fn union(&mut self, other: &Self) {
        self.zip_mut(other, |dst, src| *dst |= src);
    }

    fn intersect(&mut self, other: &Self) {
        self.zip_mut(other, |dst, src| *dst &= src);
    }

    fn subtract(&mut self, other: &Self) {
        let mut other = other.clone();
        other.invert();
        self.intersect(&other);
    }

    fn invert(&mut self) {
        for chunk in self.chunks.iter_mut() {
            for lane in chunk.as_mut_array() {
                *lane = !*lane;
            }
        }
    }

    fn clear(&mut self) {
        for chunk in self.chunks.iter_mut() {
            for lane in chunk.as_mut_array() {
                *lane = T::ZERO;
            }
        }
    }

    fn insert_all(&mut self) {
        for chunk in self.chunks.iter_mut() {
            for lane in chunk.as_mut_array() {
                *lane = T::MAX;
            }
        }
    }

    fn copy_from(&mut self, other: &Self) {
        self.zip_mut(other, |dst, src| *dst = *src);
    }
}

/// [`IndexSet`](crate::IndexSet) specialized to the [`SimdBitset`] implementation.
pub type IndexSet<T> = crate::IndexSet<'static, T, SimdBitset<u64, 4>, RcFamily>;

/// [`IndexSet`](crate::IndexSet) specialized to the [`SimdBitset`] implementation with the [`ArcFamily`].
pub type ArcIndexSet<T> = crate::IndexSet<'static, T, SimdBitset<u64, 4>, ArcFamily>;

/// [`IndexSet`](crate::IndexSet) specialized to the [`SimdBitset`] implementation with the [`RefFamily`].
pub type RefIndexSet<'a, T> = crate::IndexSet<'a, T, SimdBitset<u64, 4>, RefFamily<'a>>;

/// [`IndexMatrix`](crate::IndexMatrix) specialized to the [`SimdBitset`] implementation.
pub type IndexMatrix<R, C> = crate::IndexMatrix<'static, R, C, SimdBitset<u64, 4>, RcFamily>;

/// [`IndexMatrix`](crate::IndexMatrix) specialized to the [`SimdBitset`] implementation with the [`ArcFamily`].
pub type ArcIndexMatrix<R, C> = crate::IndexMatrix<'static, R, C, SimdBitset<u64, 4>, ArcFamily>;

/// [`IndexMatrix`](crate::IndexMatrix) specialized to the [`SimdBitset`] implementation with the [`RefFamily`].
pub type RefIndexMatrix<'a, R, C> = crate::IndexMatrix<'a, R, C, SimdBitset<u64, 4>, RefFamily<'a>>;

#[test]
fn test_simd_bitset() {
    const N: usize = 64 * 7 + 63;
    let mut bitset = SimdBitset::<u64, 4>::empty(N);

    for i in 0..N {
        bitset.clear();
        for j in 0..i {
            bitset.insert(j);
        }
        assert_eq!(
            bitset.iter().collect::<Vec<_>>(),
            (0..i).collect::<Vec<_>>()
        );
        assert_eq!(bitset.len(), i);
    }

    crate::test_utils::impl_test::<SimdBitset<u64, 4>>();
}
