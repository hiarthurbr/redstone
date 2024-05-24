use thiserror::Error;

use crate::DataResult;

////////////////////////////////////////////////////////////////////////////////
// This section was taken from https://github.com/bluss/odds/blob/master/src/range.rs
// Although it is not a direct copy, it was heavily inspired by it.
use std::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

/// **`IndexRange`** is implemented by Rust's built-in range types, produced
/// by range syntax like `..`, `a..`, `..b`, `c..d`, `..=e` or `f..=g`.
pub trait IndexRange<T = usize> {
    #[inline]
    fn index(&self) -> Option<T> {
        None
    }
    #[inline]
    fn range(&self) -> Option<(Option<T>, Option<T>, bool)> {
        None
    }
}

impl<T> IndexRange<T> for RangeFull {
    #[inline]
    fn range(&self) -> Option<(Option<T>, Option<T>, bool)> {
        Some((None, None, false))
    }
}

impl<T: Copy> IndexRange<T> for RangeFrom<T> {
    #[inline]
    fn range(&self) -> Option<(Option<T>, Option<T>, bool)> {
        Some((Some(self.start), None, false))
    }
}

impl<T: Copy> IndexRange<T> for RangeTo<T> {
    #[inline]
    fn range(&self) -> Option<(Option<T>, Option<T>, bool)> {
        Some((None, Some(self.end), false))
    }
}

impl<T: Copy> IndexRange<T> for Range<T> {
    #[inline]
    fn range(&self) -> Option<(Option<T>, Option<T>, bool)> {
        Some((Some(self.start), Some(self.end), false))
    }
}

impl<T: Copy> IndexRange<T> for RangeInclusive<T> {
    #[inline]
    fn range(&self) -> Option<(Option<T>, Option<T>, bool)> {
        Some((Some(*self.start()), Some(*self.end()), true))
    }
}

impl<T: Copy> IndexRange<T> for RangeToInclusive<T> {
    #[inline]
    fn range(&self) -> Option<(Option<T>, Option<T>, bool)> {
        Some((None, Some(self.end), true))
    }
}

macro_rules! impl_index {
    ($($t:ty),*) => {
        $(
            impl IndexRange<$t> for $t {
                #[inline]
                fn index(&self) -> Option<$t> {
                    Some(*self)
                }
            }
        )*
    };
}

impl_index!(usize, u8, u16, u32, u64, u128);

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Error)]
pub enum BitSetError {
    #[error("Attempted to grow a frozen BitSet")]
    AttemptToGrowFrozen,
}

/// This defines the word size.
/// It should be compatible with all integer types
/// ([`u8`], [`u16`], [`u32`], [`u64`], [`usize`], [`i8`], [`i16`], [`i32`], [`i64`], [`isize`]. If not, you should file a issue).
///
/// Although, it comes with a performance tradeoff.
///
/// A bigger word will make a [`BitSet`] with less memory allocations, but will have a larger memory footprint, and may use larger registers.
/// A smaller word will make a [`BitSet`] with more memory allocations, but with a smaller memory footprint, and may be faster to process.
///
/// ---
///
/// At the moment, this is set to [`usize`], so it should be using the best register for the system.
///
/// This may cause confusion though, as the [`usize`] type is not the same size on all systems.
///
/// ---
///
/// Changing this should NOT change the behavior of the [`BitSet`]
/// (but it may change the performance, memory usage, and metrics, like the [`BitSet`] size or capacity).
type Word = usize;

/// Word size in bits
pub const WORD_SIZE: usize = Word::BITS as usize;

/// This struct implements a vector of bits that grows as needed. Each
/// component of the bit set has a `boolean` value. The
/// bits of a [`BitSet`] are indexed by nonnegative integers.
/// Individual indexed bits can be examined, set, or cleared. One
/// [`BitSet`] may be used to modify the contents of another
/// [`BitSet`] through logical AND, logical inclusive OR, and
/// logical exclusive OR operations.
///
/// By default, all bits in the set initially have the value `false`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct BitSet {
    data: Vec<Word>,
    frozen: bool,
}

impl BitSet {
    /// Creates a new bit set. All bits are initially `false`.
    #[must_use]
    pub fn new() -> Self {
        BitSet::default()
    }

    /// Creates a new bitset with at least `n` bits.
    ///
    /// All bits are initially `false` and the [`BitSet`] size is frozen by default to the specified size.
    ///
    /// You can unfreeze the [`BitSet`] size with [`BitSet::unfreeze`].
    #[must_use]
    pub fn with_capacity(n: usize) -> Self {
        if n == 0 {
            return BitSet::default();
        }

        let mut data = vec![Word::default(); n / WORD_SIZE];
        data.shrink_to_fit();
        BitSet { data, frozen: true }
    }

    /// Freezes the size of the [`BitSet`] to the logical size in words of the [`BitSet`].
    ///
    /// This method is useful in situations where it is known that a [`BitSet`] will
    /// not need to be expanded to accommodate additional words.
    ///
    /// Attempting to grow a frozen [`BitSet`] will return an [`BitSetError::AttemptToGrowFrozen`] error.
    ///
    /// ---
    ///
    /// ## Example
    ///
    /// ```
    /// use crate::protocol::data_types::{BitSet, BitSetError};
    ///
    /// {
    ///   let mut bitset = BitSet::new();
    ///   assert_eq!(bitset.grow(5), Ok(()));
    ///
    ///   bitset.freeze();
    ///   assert_eq!(bitset.grow(5), Err(BitSetError::AttemptToGrowFrozen));
    /// }
    /// ```
    pub fn freeze(&mut self) {
        self.frozen = true;
    }

    /// Unfreezes the size of the [`BitSet`].
    ///
    /// This allows the [`BitSet`] to grow as needed when new words are added.
    ///
    /// When working with a unfrozen [`BitSet`], it is garanteed that the functions will not return [`BitSetError::AttemptToGrowFrozen`].
    ///
    /// ---
    ///
    /// ## Example
    ///
    /// ```
    /// use crate::protocol::data_types::{BitSet, BitSetError};
    ///
    /// {
    ///   let mut bitset = BitSet::with_capacity(10);
    ///   assert_eq!(bitset.grow(5), Err(BitSetError::AttemptToGrowFrozen));
    ///
    ///   bitset.unfreeze();
    ///   assert_eq!(bitset.grow(5), Ok(()));
    /// }
    /// ```
    pub fn unfreeze(&mut self) {
        self.frozen = false;
    }

    /// Trims the set to the last enabled bit.
    ///
    /// ---
    ///
    /// ## Example
    ///
    /// Consider the following [`BitSet`]: `0b0100_1001_00`
    ///
    /// After calling [`BitSet::trim`], the [`BitSet`] will look like: `0b0100_1001`
    pub fn trim(&mut self) {
        for (i, word) in self.data.iter().enumerate().rev() {
            if *word == 0 {
                self.data.truncate(i + 1);
                return;
            }
        }
    }

    /// Clear all bits in the set
    #[allow(clippy::missing_panics_doc)]
    pub fn flush(&mut self) {
        for word in &mut self.data {
            *word = Word::default();
        }
    }

    /// Grows the set with the specified number of bits.
    ///
    /// ---
    ///
    /// ## Errors
    ///
    /// If the set is frozen, this function will return an [`BitSetError::AttemptToGrowFrozen`] error.
    ///
    /// ---
    ///
    /// ## Example
    ///
    /// ```
    /// use crate::protocol::data_types::{BitSet, BitSetError};
    ///
    /// {
    ///   let mut bitset = BitSet::with_capacity(10);
    ///   bitset.unfreeze();
    ///   assert_eq!(bitset.len(), 10);
    ///
    ///   bitset.grow(10);
    ///   assert_eq!(bitset.len(), 20);
    /// }
    /// ```
    pub fn grow(&mut self, nbits: usize) -> DataResult<()> {
        if nbits == 0 {
            return Ok(());
        }
        self.grow_to(self.data.len() + nbits)
    }

    /// Grows the set to the specified number of bits.
    ///
    /// This funtion does nothing if the [`BitSet`] is already bigger than the specified capacity.
    ///
    /// ---
    ///
    /// ## Errors
    ///
    /// If the provided capacity is larger than the set current capacity, and the set is frozen, this function will return an [`BitSetError::AttemptToGrowFrozen`] error.
    ///
    /// ---
    ///
    /// ## Example
    ///
    /// ```
    /// use crate::protocol::data_types::{BitSet, BitSetError};
    ///
    /// {
    ///   let mut bitset = BitSet::with_capacity(20);
    ///   bitset.unfreeze();
    ///   assert_eq!(bitset.length(), 20);
    ///
    ///   bitset.grow_to(10);
    ///   assert_eq!(bitset.length(), 20);
    /// }
    ///
    /// {
    ///   let mut bitset = BitSet::with_capacity(10);
    ///   bitset.unfreeze();
    ///
    ///   assert_eq!(bitset.length(), 10);
    ///
    ///   bitset.grow_to(20);
    ///   assert_eq!(bitset.length(), 20);
    /// }
    /// ```
    pub fn grow_to(&mut self, capacity: usize) -> DataResult<()> {
        if capacity == 0 {
            return Ok(());
        }
        if self.data.len() < capacity {
            if self.frozen {
                return Err(BitSetError::AttemptToGrowFrozen)?;
            }
            self.data
                .resize(self.data.len().max(capacity), Word::default());
        }
        Ok(())
    }

    /// Sets the specified bit to false.
    ///
    /// ---
    ///
    /// ## Errors
    ///
    /// If the bit index is bigger than the set's length, the set will be grown to the specified index.
    /// If the set is frozen, this function will then return an error.
    ///
    /// ---
    ///
    /// ## Example
    ///
    /// ```
    /// use crate::protocol::data_types::{BitSet, BitSetError};
    ///
    /// {
    ///   let mut bitset = BitSet::with_capacity(10);
    ///   assert_eq!(bitset.insert(5), Ok(()));
    ///   assert_eq!(bitset.get(5), Some(true));
    ///
    ///   assert_eq!(bitset.clear(5), Ok(()));
    ///   assert_eq!(bitset.get(5), Some(false));
    ///
    ///   assert_eq!(bitset.clear(15), Err(BitSetError::AttemptToGrowFrozen));
    /// }
    ///
    /// {
    ///   let mut bitset = BitSet::new();
    ///   assert_eq!(bitset.clear(5), Ok(()));
    ///   assert_eq!(bitset.length(), 6);
    /// }
    /// ```
    ///
    /// ```
    /// use crate::protocol::data_types::{BitSet, BitSetError};
    ///
    /// {
    ///   let mut bitset = BitSet::with_capacity(10);
    ///   assert_eq!(bitset.insert_range(2..7), Ok(()));
    ///   assert_eq!(bitset.get(5), Some(true));
    ///
    ///   assert_eq!(bitset.clear_range(..5), Ok(()));
    ///   assert_eq!(bitset.get(4), Some(false));
    ///
    ///   assert_eq!(bitset.clear_range(..15), Err(BitSetError::AttemptToGrowFrozen));
    ///   assert_eq!(bitset.get(5), Some(true));
    /// }
    ///
    /// {
    ///   let mut bitset = BitSet::new();
    ///   assert_eq!(bitset.clear_range(..5), Ok(()));
    ///   assert_eq!(bitset.length(), 6);
    /// }
    /// ```
    pub fn clear<I: IndexRange>(&mut self, index: I) -> DataResult<()> {
        self.set(index, false)
    }

    /// Sets the specified bit to true.
    ///
    /// ---
    ///
    /// ## Errors
    ///
    /// If the bit index is bigger than the set's length, the set will be grown to the specified index.
    /// If the set is frozen, this function will then return an error.
    ///
    /// ---
    ///
    /// ## Example
    ///
    /// ```
    /// use crate::protocol::data_types::{BitSet, BitSetError};
    ///
    /// {
    ///   let mut bitset = BitSet::with_capacity(10);
    ///   assert_eq!(bitset.insert(5), Ok(()));
    ///   assert_eq!(bitset.get(5), Some(true));
    ///
    ///   assert_eq!(bitset.insert(15), Err(BitSetError::AttemptToGrowFrozen));
    /// }
    ///
    /// {
    ///   let mut bitset = BitSet::new();
    ///   assert_eq!(bitset.insert(5), Ok(()));
    ///   assert_eq!(bitset.length(), 6);
    ///   assert_eq!(bitset.get(5), Some(true));
    /// }
    /// ```
    ///
    /// ```
    /// use crate::protocol::data_types::{BitSet, BitSetError};
    ///
    /// {
    ///   let mut bitset = BitSet::with_capacity(10);
    ///   assert_eq!(bitset.insert_range(2..7), Ok(()));
    ///   assert_eq!(bitset.get(5), Some(true));
    ///
    ///   assert_eq!(bitset.insert_range(..15), Err(BitSetError::AttemptToGrowFrozen));
    ///   assert_eq!(bitset.get(9), Some(false));
    /// }
    ///
    /// {
    ///   let mut bitset = BitSet::new();
    ///   assert_eq!(bitset.insert_range(..5), Ok(()));
    ///   assert_eq!(bitset.length(), 6);
    ///   assert_eq!(bitset.get(4), Some(true));
    /// }
    /// ```
    pub fn insert<I: IndexRange>(&mut self, index: I) -> DataResult<()> {
        self.set(index, true)
    }

    /// Sets the specified bit to the given value.
    ///
    /// ---
    ///
    /// ## Errors
    ///
    /// If the bit index is bigger than the set's length, the set will be grown to the specified index.
    /// If the set is frozen, this function will then return an error.
    ///
    /// ---
    ///
    /// ## Example
    ///
    /// ```
    /// use crate::protocol::data_types::{BitSet, BitSetError};
    ///
    /// {
    ///   let mut bitset = BitSet::with_capacity(10);
    ///   assert_eq!(bitset.set(5, true), Ok(()));
    ///   assert_eq!(bitset.get(5), Some(true));
    ///
    ///   assert_eq!(bitset.set(5, false), Ok(()));
    ///   assert_eq!(bitset.get(5), Some(false));
    ///
    ///   assert_eq!(bitset.set(15, true), Err(BitSetError::AttemptToGrowFrozen));
    /// }
    ///
    /// {
    ///   let mut bitset = BitSet::new();
    ///   assert_eq!(bitset.set(5, true), Ok(()));
    ///   assert_eq!(bitset.length(), 6);
    ///   assert_eq!(bitset.get(5), Some(true));
    /// }
    /// ```
    ///
    /// ```
    /// use crate::protocol::data_types::{BitSet, BitSetError};
    ///
    /// {
    ///   let mut bitset = BitSet::with_capacity(10);
    ///   assert_eq!(bitset.set_range(2..7, true), Ok(()));
    ///   assert_eq!(bitset.get(5), Some(true));
    ///   assert_eq!(bitset.get(1), Some(false));
    ///
    ///   assert_eq!(bitset.set_range(..5, false), Ok(()));
    ///   assert_eq!(bitset.get(4), Some(false));
    ///
    ///   assert_eq!(bitset.set_range(..15, false), Err(BitSetError::AttemptToGrowFrozen));
    ///   assert_eq!(bitset.get(5), Some(true));
    /// }
    ///
    /// {
    ///   let mut bitset = BitSet::new();
    ///   assert_eq!(bitset.set_range(..5, true), Ok(()));
    ///   assert_eq!(bitset.length(), 6);
    ///   assert_eq!(bitset.get(4), Some(true));
    /// }
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    pub fn set<I: IndexRange>(&mut self, index: I, to: bool) -> DataResult<()> {
        if let Some(index) = index.index() {
            let index = index / WORD_SIZE;
            let bit_index = index % WORD_SIZE;

            self.grow_to(index + 1)?;

            self.data[index] |= Word::from(to) << bit_index;
        }
        if let Some((start, end, inclusive)) = index.range() {
            let bitset_len = self.len();
            let len = bitset_len * WORD_SIZE;
            let start = start.unwrap_or(0);
            let end = end.unwrap_or(len);

            self.grow_to(end)?;

            let starting_word_index = start / WORD_SIZE;
            let ending_word_index = (bitset_len * WORD_SIZE) / end;
            let shift_offset = start % WORD_SIZE;
            let mut remaining_offset = end - start;

            let mut shift = |e: usize, i: usize| {
                let shift_pattern = if e == 0 {
                    remaining_offset -= shift_offset;
                    Word::MAX << shift_offset
                } else if remaining_offset >= WORD_SIZE {
                    remaining_offset -= WORD_SIZE;
                    Word::MAX
                } else {
                    Word::MAX >> remaining_offset
                };
                self.data[i] |= shift_pattern;
                if !to {
                    self.data[i] ^= shift_pattern;
                }
            };

            if inclusive {
                for (e, i) in (starting_word_index..=ending_word_index).enumerate() {
                    shift(e, i);
                }
            } else {
                for (e, i) in (starting_word_index..ending_word_index).enumerate() {
                    shift(e, i);
                }
            }
        }
        Ok(())
    }

    /// Inverts the specified bit.
    ///
    /// ---
    ///
    /// ## Errors
    ///
    /// If the bit index is bigger than the set's length, the set will be grown to the specified index.
    /// If the set is frozen, this function will then return an error.
    ///
    /// ---
    ///
    /// ## Example
    ///
    /// ```
    /// use crate::protocol::data_types::{BitSet, BitSetError};
    ///
    /// {
    ///   let mut bitset = BitSet::with_capacity(10);
    ///   assert_eq!(bitset.flip(5), Ok(()));
    ///   assert_eq!(bitset.get(5), Some(true));
    ///
    ///   assert_eq!(bitset.flip(5), Ok(()));
    ///   assert_eq!(bitset.get(5), Some(false));
    ///
    ///   assert_eq!(bitset.flip(15), Err(BitSetError::AttemptToGrowFrozen));
    /// }
    ///
    /// {
    ///   let mut bitset = BitSet::new();
    ///   assert_eq!(bitset.flip(5), Ok(()));
    ///   assert_eq!(bitset.length(), 6);
    ///   assert_eq!(bitset.get(5), Some(true));
    /// }
    /// ```
    ///
    /// ```
    /// use crate::protocol::data_types::{BitSet, BitSetError};
    ///
    /// {
    ///   let mut bitset = BitSet::with_capacity(10);
    ///   assert_eq!(bitset.flip_range(2..7), Ok(()));
    ///   assert_eq!(bitset.get(5), Some(true));
    ///
    ///   assert_eq!(bitset.flip_range(..5), Ok(()));
    ///   assert_eq!(bitset.get(4), Some(false));
    ///
    ///   assert_eq!(bitset.flip_range(..15), Err(BitSetError::AttemptToGrowFrozen));
    ///   assert_eq!(bitset.get(5), Some(true));
    /// }
    ///
    /// {
    ///   let mut bitset = BitSet::new();
    ///   assert_eq!(bitset.flip_range(..5), Ok(()));
    ///   assert_eq!(bitset.length(), 6);
    ///   assert_eq!(bitset.get(4), Some(true));
    /// }
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    pub fn flip<I: IndexRange>(&mut self, index: I) -> DataResult<()> {
        if let Some(index) = index.index() {
            self.set(index, !self.get(index).unwrap_or(false))?;
        }
        if let Some((start, end, inclusive)) = index.range() {
            let bitset_len = self.len();
            let len = bitset_len * WORD_SIZE;
            let start = start.unwrap_or(0);
            let end = end.unwrap_or(len);

            if inclusive {
                for i in start..=end {
                    self.set(i, !self.get(i).unwrap_or(false))?;
                }
            } else {
                for i in start..end {
                    self.set(i, !self.get(i).unwrap_or(false))?;
                }
            }
        }
        Ok(())
    }

    /// Sets the specified bit to the given value, returning its previous value.
    ///
    /// ---
    ///
    /// ## Errors
    ///
    /// If the bit index is bigger than the set's length, the set will be grown to the specified index.
    /// If the set is frozen, this function will then return an error.
    ///
    /// ---
    ///
    /// ## Example
    ///
    /// ```
    /// use crate::protocol::data_types::{BitSet, BitSetError};
    ///
    /// {
    ///   let mut bitset = BitSet::with_capacity(10);
    ///   assert_eq!(bitset.put(5, true), Ok(Some(false)));
    ///   assert_eq!(bitset.get(5), Some(true));
    ///
    ///   assert_eq!(bitset.put(5, false), Ok(Some(true)));
    ///   assert_eq!(bitset.get(5), Some(false));
    ///
    ///   assert_eq!(bitset.put(15, true), Err(BitSetError::AttemptToGrowFrozen));
    /// }
    ///
    /// {
    ///   let mut bitset = BitSet::new();
    ///   assert_eq!(bitset.put(5, true), Ok(None));
    ///   assert_eq!(bitset.length(), 6);
    ///   assert_eq!(bitset.get(5), Some(true));
    /// }
    /// ```
    pub fn put(&mut self, bit_index: usize, to: bool) -> DataResult<Option<bool>> {
        let previous = self.get(bit_index);

        self.set(bit_index, to)?;
        Ok(previous)
    }

    /// Gets the value of the specified bit.
    ///
    /// This method returns `None` if the bit index is bigger than the set's length.
    #[must_use]
    pub fn get(&self, bit_index: usize) -> Option<bool> {
        let index = bit_index / WORD_SIZE;
        let bit_index = bit_index % WORD_SIZE;
        let bit = (self.data.get(index)? >> bit_index) & 1;
        Some(bit == 1)
    }

    /// Performs the logical operator `AND` in this set, using the specified set.
    ///
    /// ---
    ///
    /// ## Errors
    ///
    /// If the specified set is bigger than this set, an attempt to grow this set will be made.
    /// If the set is frozen, this function will then return an [`BitSetError::AttemptToGrowFrozen`] error.
    ///
    /// ---
    ///
    /// ## Example
    ///
    /// ```
    /// use crate::protocol::data_types::BitSet;
    ///
    /// {
    ///   let mut bitset1 = BitSet::from(5);
    ///   let bitset2 = BitSet::from(3);
    ///
    ///   bitset1.and(&bitset2);
    ///
    ///   assert_eq!(bitset1.data, BitSet::from(1).data);
    ///   assert_eq!(5 & 3, 1);
    /// }
    /// ```
    pub fn and(&mut self, set: &BitSet) -> DataResult<()> {
        if self.len() < set.len() {
            self.grow_to(set.len())?;
        }

        for (i, bit) in set.data.iter().enumerate() {
            self.data[i] &= *bit;
        }

        Ok(())
    }

    /// Performs the logical operator `OR` in this set, using the specified set.
    ///
    /// ---
    ///
    /// ## Errors
    ///
    /// If the specified set is bigger than this set, an attempt to grow this set will be made.
    /// If the set is frozen, this function will then return an [`BitSetError::AttemptToGrowFrozen`] error.
    ///
    /// ---
    ///
    /// ## Example
    ///
    /// ```
    /// use crate::protocol::data_types::BitSet;
    ///
    /// {
    ///   let mut bitset1 = BitSet::from(5);
    ///   let bitset2 = BitSet::from(3);
    ///  
    ///   bitset1.or(&bitset2);
    ///  
    ///   assert_eq!(bitset1.data, BitSet::from(7).data);
    ///   assert_eq!(5 | 3, 7);
    /// }
    /// ```
    pub fn or(&mut self, set: &BitSet) -> DataResult<()> {
        if self.len() < set.len() {
            self.grow_to(set.len())?;
        }

        for (i, bit) in set.data.iter().enumerate() {
            self.data[i] |= *bit;
        }

        Ok(())
    }

    /// Performs the logical operator `XOR` in this set, using the specified set.
    ///
    /// ---
    ///
    /// ## Errors
    ///
    /// If the specified set is bigger than this set, an attempt to grow this set will be made.
    /// If the set is frozen, this function will then return an [`BitSetError::AttemptToGrowFrozen`] error.
    ///
    /// ---
    ///
    /// ## Example
    ///
    /// ```
    /// use crate::protocol::data_types::BitSet;
    ///
    /// {
    ///   let mut bitset1 = BitSet::from(5);
    ///   let bitset2 = BitSet::from(3);
    ///
    ///   bitset1.xor(&bitset2);
    ///
    ///   assert_eq!(bitset1.data, BitSet::from(6).data);
    ///   assert_eq!(5 ^ 3, 6);
    /// }
    /// ```
    pub fn xor(&mut self, set: &BitSet) -> DataResult<()> {
        if self.len() < set.len() {
            self.grow_to(set.len())?;
        }

        for (i, bit) in set.data.iter().enumerate() {
            self.data[i] ^= *bit;
        }

        Ok(())
    }

    /// Performs the logical operator `AND NOT` in this set, using the specified set.
    ///
    /// ---
    ///
    /// ## Errors
    ///
    /// If the specified set is bigger than this set, an attempt to grow this set will be made.
    /// If the set is frozen, this function will then return an [`BitSetError::AttemptToGrowFrozen`] error.
    ///
    /// ---
    ///
    /// ## Example
    ///
    /// ```
    /// use crate::protocol::data_types::BitSet;
    ///
    /// {
    ///   let mut bitset1 = BitSet::from(5);
    ///   let bitset2 = BitSet::from(3);
    ///
    ///   bitset1.and_not(&bitset2);
    ///
    ///   assert_eq!(bitset1.data, BitSet::from(4).data);
    ///   assert_eq!(5 & !3, 4);
    /// }
    /// ```
    pub fn and_not(&mut self, set: &BitSet) -> DataResult<()> {
        if self.len() < set.len() {
            self.grow_to(set.len())?;
        }

        for (i, bit) in set.data.iter().enumerate() {
            self.data[i] &= !*bit;
        }

        Ok(())
    }

    /// Clones this set, and then performs the logical operator `AND` on the cloned set, using the specified set.
    ///
    /// This method does not mutate this set.
    ///
    /// If the specified set is bigger than the cloned set, the cloned set will grow, even if this set is frozen.
    ///
    /// [`BitSet`]'s returned by this method are unfrozen by default.
    ///
    /// ---
    ///
    /// ## Example
    ///
    /// ```
    /// use crate::protocol::data_types::BitSet;
    ///
    /// {
    ///   let bitset1 = BitSet::from(5);
    ///   let bitset2 = BitSet::from(3);
    ///
    ///   let result_bitset = bitset1.cloned_and(&bitset2);
    ///
    ///   assert_eq!(result_bitset.data, BitSet::from(1).data);
    ///   assert_eq!(5 & 3, 1);
    /// }
    /// ```
    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn cloned_and(&self, set: &BitSet) -> BitSet {
        let mut clone = self.clone();
        clone.unfreeze();
        clone.and(set).expect("Should never panic on grow");
        clone
    }

    /// Clones this set, and then performs the logical operator `OR` on the cloned set, using the specified set.
    ///
    /// This method does not mutate this set.
    ///
    /// If the specified set is bigger than the cloned set, the cloned set will grow, even if this set is frozen.
    ///
    /// [`BitSet`]'s returned by this method are unfrozen by default.
    ///
    /// ---
    ///
    /// ## Example
    ///
    /// ```
    /// use crate::protocol::data_types::BitSet;
    ///
    /// {
    ///   let bitset1 = BitSet::from(5);
    ///   let bitset2 = BitSet::from(3);
    ///  
    ///   let result_bitset = bitset1.cloned_or(&bitset2);
    ///  
    ///   assert_eq!(result_bitset.data, BitSet::from(7).data);
    ///   assert_eq!(5 | 3, 7);
    /// }
    /// ```
    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn cloned_or(&self, set: &BitSet) -> BitSet {
        let mut clone = self.clone();
        clone.unfreeze();
        clone.or(set).expect("Should never panic on grow");
        clone
    }

    /// Clones this set, and then performs the logical operator `XOR` on the cloned set, using the specified set.
    ///
    /// This method does not mutate this set.
    ///
    /// If the specified set is bigger than the cloned set, the cloned set will grow, even if this set is frozen.
    ///
    /// [`BitSet`]'s returned by this method are unfrozen by default.
    ///
    /// ---
    ///
    /// ## Example
    ///
    /// ```
    /// use crate::protocol::data_types::BitSet;
    ///
    /// {
    ///   let bitset1 = BitSet::from(5);
    ///   let bitset2 = BitSet::from(3);
    ///
    ///   let result_bitset = bitset1.cloned_xor(&bitset2);
    ///
    ///   assert_eq!(result_bitset.data, BitSet::from(6).data);
    ///   assert_eq!(5 ^ 3, 6);
    /// }
    /// ```
    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn cloned_xor(&self, set: &BitSet) -> BitSet {
        let mut clone = self.clone();
        clone.unfreeze();
        clone.xor(set).expect("Should never panic on grow");
        clone
    }

    /// Clones this set, and then performs the logical operator `AND NOT` on the cloned set, using the specified set.
    ///
    /// This method does not mutate this set.
    ///
    /// If the specified set is bigger than the cloned set, the cloned set will grow, even if this set is frozen.
    ///
    /// [`BitSet`]'s returned by this method are unfrozen by default.
    ///
    /// ---
    ///
    /// ## Example
    ///
    /// ```
    /// use crate::protocol::data_types::BitSet;
    ///
    /// {
    ///   let bitset1 = BitSet::from(5);
    ///   let bitset2 = BitSet::from(3);
    ///
    ///   let result_bitset = bitset1.cloned_and_not(&bitset2);
    ///
    ///   assert_eq!(result_bitset.data, BitSet::from(4).data);
    ///   assert_eq!(5 & !3, 4);
    /// }
    /// ```
    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn cloned_and_not(&self, set: &BitSet) -> BitSet {
        let mut clone = self.clone();
        clone.unfreeze();
        clone.and_not(set).expect("Should never panic on grow");
        clone
    }

    #[must_use]
    pub fn intersects(&self, set: &BitSet) -> bool {
        todo!()
    }

    /// Returns true if this set is a subset of the specified set.
    ///
    /// ---
    ///
    /// ## Example
    ///
    /// ```
    /// use crate::protocol::data_types::BitSet;
    ///
    /// {
    ///   todo!("Implement this")
    /// }
    /// ```
    #[must_use]
    pub fn is_subset(&self, set: &BitSet) -> bool {
        todo!()
        // self.data.iter().zip(set.data.iter()).all(|(x, y)| !x && !y)
        //     && self.data.iter().skip(set.data.len()).all(|x| !(*x))
    }

    /// Returns true if this set is a superset of the specified set.
    ///
    /// ---
    ///
    /// ## Example
    ///
    /// ```
    /// use crate::protocol::data_types::BitSet;
    ///
    /// {
    ///   todo!("Implement this")
    /// }
    /// ```
    #[must_use]
    pub fn is_superset(&self, set: &BitSet) -> bool {
        todo!()
    }

    /// Returns true if this set has no bits in common with the specified set.
    ///
    /// ---
    ///
    /// ## Example
    ///
    /// ```
    /// use crate::protocol::data_types::BitSet;
    ///
    /// {
    ///   todo!("Implement this")
    /// }
    /// ```
    #[must_use]
    pub fn is_disjoint(&self, set: &BitSet) -> bool {
        todo!()
        // self.data.iter().zip(set.data.iter()).all(|(x, y)| !(x & y))
    }

    /// Returns true if the set is empty
    ///
    /// An empty set is a set with 0 length, not a set with all bits set to false. In that case, you might want [`BitSet::is_clear`]
    ///
    /// ---
    ///
    /// ## Example
    ///
    /// ```
    /// use crate::protocol::data_types::{BitSet, BitSetError};
    ///
    /// {
    ///   let mut bitset = BitSet::new();
    ///   assert!(bitset.is_empty());
    ///
    ///   // Setting the 5th bit to false will grow the set to 6 bits, making it not empty
    ///   bitset.clear(5);
    ///   assert!(!bitset.is_empty());
    ///   assert!(bitset.is_clear());
    ///
    ///   bitset.trim();
    ///   assert!(bitset.is_empty());
    /// }
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.data.len() == 0
    }

    /// Returns true if all bits in the set are set to false
    ///
    /// This is not the same as [`BitSet::is_empty`], which returns true if the set has 0 length.
    ///
    /// ---
    ///
    /// ## Example
    ///
    /// ```
    /// use crate::protocol::data_types::{BitSet, BitSetError};
    ///
    /// {
    ///   let mut bitset = BitSet::new();
    ///   
    ///   bitset.insert(5);
    ///   assert!(!bitset.is_clear());
    ///
    ///   bitset.clear(5);
    ///   assert!(bitset.is_clear());
    ///
    ///   bitset.insert_range(2..9);
    ///   assert!(!bitset.is_clear());
    ///
    ///   bitset.flush();
    ///   assert!(bitset.is_clear());
    /// }
    /// ```
    #[must_use]
    pub fn is_clear(&self) -> bool {
        for word in &self.data {
            if *word != 0 {
                return false;
            }
        }
        true
    }

    /// Returns true if all bits in the set are set to true
    ///
    /// ---
    ///
    /// ## Example
    ///
    /// ```
    /// use crate::protocol::data_types::BitSet;
    ///
    /// {
    ///   let mut bitset = BitSet::new();
    ///
    ///   bitset.insert(10);
    ///   assert!(!bitset.is_full());
    ///
    ///   bitset.insert_range(..);
    ///   assert_eq!(bitset.data, vec![true; bitset.length()]);
    ///   assert!(bitset.is_full());
    /// }
    /// ```
    #[must_use]
    pub fn is_full(&self) -> bool {
        for word in &self.data {
            if *word != Word::MAX {
                return false;
            }
        }
        true
    }

    /// Returns the length of the set
    ///
    /// This includes trailing zeros.
    ///
    /// ---
    ///
    /// ## Example
    ///
    /// ```
    /// use crate::protocol::data_types::BitSet;
    ///
    /// {
    ///   let mut bitset = BitSet::new();
    ///   assert_eq!(bitset.length(), 0);
    ///
    ///   bitset.insert(5);
    ///   assert_eq!(bitset.length(), 6);
    /// }
    /// ```
    #[must_use]
    pub fn len(&self) -> usize {
        self.data.len() * WORD_SIZE
    }

    /// Returns the size of the set in words, excluding trailing zeros
    ///
    /// ---
    ///
    /// ## Example
    ///
    /// ```
    /// use crate::protocol::data_types::BitSet;
    ///
    /// {
    ///   let mut bitset = BitSet::new();
    ///   assert_eq!(bitset.size(), 0);
    ///   
    ///   bitset.insert_range(5..15);
    ///   assert_eq!(bitset.size(), 15);
    ///   
    ///   bitset.clear_range(10..);
    ///   assert_eq!(bitset.size(), 10);
    ///
    ///   bitset.trim();
    ///   assert_eq!(bitset.size(), 10);
    /// }
    /// ```
    #[must_use]
    pub fn size(&self) -> usize {
        let mut size = self.len();
        for bit in self.data.iter().rev() {
            if *bit == 0 {
                break;
            }
            size -= 1;
        }
        size
    }
}

/// This macro implements the From<$type> trait for [`BitSet`]
macro_rules! impl_from_iter_for_bitset {
    ($type:ty, $size:expr) => {
        impl From<$type> for BitSet {
            fn from(num: $type) -> Self {
                let mut data: Vec<Word> = Vec::new();

                let n: Result<Word, _> = num.try_into();

                match n {
                    Ok(n) => data.push(n),
                    Err(_) => {
                        for shift in (0..$size).step_by(WORD_SIZE) {
                            #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
                            let word = (num >> shift) as Word;
                            data.push(word);
                        }
                    }
                }

                BitSet { frozen: true, data }
            }
        }
        impl From<&[$type]> for BitSet {
            fn from(slice: &[$type]) -> Self {
                let data: Vec<Word> = slice
                    .iter()
                    .map(|word| {
                        let mut data: Vec<Word> = Vec::new();

                        let n: Result<Word, _> = (*word).try_into();

                        match n {
                            Ok(n) => data.push(n),
                            Err(_) => {
                                for shift in (0..$size).step_by(WORD_SIZE) {
                                    #[allow(
                                        clippy::cast_sign_loss,
                                        clippy::cast_possible_truncation
                                    )]
                                    let word = (word >> shift) as Word;
                                    data.push(word);
                                }
                            }
                        }
                        data
                    })
                    .collect::<Vec<Vec<Word>>>()
                    .concat();
                BitSet { frozen: true, data }
            }
        }
        impl From<Vec<$type>> for BitSet {
            fn from(vec: Vec<$type>) -> Self {
                let data: Vec<Word> = vec
                    .into_iter()
                    .map(|word| {
                        let mut data: Vec<Word> = Vec::new();

                        let n: Result<Word, _> = word.try_into();

                        match n {
                            Ok(n) => data.push(n),
                            Err(_) => {
                                for shift in (0..$size).step_by(WORD_SIZE) {
                                    #[allow(
                                        clippy::cast_sign_loss,
                                        clippy::cast_possible_truncation
                                    )]
                                    let word = (word >> shift) as Word;
                                    data.push(word);
                                }
                            }
                        }
                        data
                    })
                    .collect::<Vec<Vec<Word>>>()
                    .concat();
                BitSet { frozen: true, data }
            }
        }
    };
}

impl_from_iter_for_bitset!(u8, u8::BITS);
impl_from_iter_for_bitset!(u16, u16::BITS);
impl_from_iter_for_bitset!(u32, u32::BITS);
impl_from_iter_for_bitset!(u64, u64::BITS);
impl_from_iter_for_bitset!(u128, u128::BITS);
impl_from_iter_for_bitset!(usize, usize::BITS);

impl_from_iter_for_bitset!(i8, i8::BITS);
impl_from_iter_for_bitset!(i16, i16::BITS);
impl_from_iter_for_bitset!(i32, i32::BITS);
impl_from_iter_for_bitset!(i64, i64::BITS);
impl_from_iter_for_bitset!(i128, i128::BITS);
impl_from_iter_for_bitset!(isize, isize::BITS);

#[cfg(test)]
mod test {
    use super::BitSet;
    #[test]
    fn initial_state() {
        let bitset = BitSet::default();
        assert_eq!(bitset.len(), 0);
        assert_eq!(bitset.data, vec![]);
    }

    #[test]
    fn set_bits() {
        let mut bitset = BitSet::default();

        bitset.insert(0).unwrap();
        bitset.insert(2).unwrap();
        bitset.insert(5).unwrap();
        bitset.insert(10).unwrap();

        assert!(bitset.get(0).unwrap());
        assert!(bitset.get(2).unwrap());
        assert!(bitset.get(5).unwrap());
        assert!(bitset.get(10).unwrap());

        assert!(!bitset.get(1).unwrap());
        assert!(!bitset.get(3).unwrap());
        assert!(!bitset.get(6).unwrap());
        assert!(!bitset.get(9).unwrap());

        assert_eq!(bitset.get(11), None);
    }

    #[test]
    fn clear_bits() {
        let mut bitset = BitSet::default();

        bitset.insert(2..6).unwrap();

        bitset.clear(3).unwrap();

        assert!(!bitset.get(3).unwrap());

        assert!(bitset.get(2).unwrap());
        assert!(bitset.get(4).unwrap());
        assert!(bitset.get(5).unwrap());
    }

    #[test]
    fn logical_and() {
        let bitset1 = BitSet::from(5);
        let bitset2 = BitSet::from(3);

        let result_bitset = bitset1.cloned_and(&bitset2);

        assert_eq!(result_bitset.data, BitSet::from(1).data);

        let bitset1 = BitSet::from(8);
        let bitset2 = BitSet::from(9);

        let result_bitset = bitset1.cloned_and(&bitset2);

        assert_eq!(result_bitset.data, BitSet::from(8).data);

        let bitset1 = BitSet::from(2645);
        let bitset2 = BitSet::from(4568);

        let result_bitset = bitset1.cloned_and(&bitset2);

        assert_eq!(result_bitset.data, BitSet::from(80).data);
    }

    #[test]
    fn logical_or() {
        let bitset1 = BitSet::from(5);
        let bitset2 = BitSet::from(3);

        let result_bitset = bitset1.cloned_or(&bitset2);

        assert_eq!(result_bitset.data, BitSet::from(7).data);

        let bitset1 = BitSet::from(8);
        let bitset2 = BitSet::from(9);

        let result_bitset = bitset1.cloned_or(&bitset2);

        assert_eq!(result_bitset.data, BitSet::from(9).data);

        let bitset1 = BitSet::from(2645);
        let bitset2 = BitSet::from(4568);

        let result_bitset = bitset1.cloned_or(&bitset2);

        assert_eq!(result_bitset.data, BitSet::from(7133).data);
    }

    #[test]
    fn logical_xor() {
        let bitset1 = BitSet::from(5);
        let bitset2 = BitSet::from(3);

        let result_bitset = bitset1.cloned_xor(&bitset2);

        assert_eq!(result_bitset.data, BitSet::from(6).data);

        let bitset1 = BitSet::from(8);
        let bitset2 = BitSet::from(9);

        let result_bitset = bitset1.cloned_xor(&bitset2);

        assert_eq!(result_bitset.data, BitSet::from(1).data);

        let bitset1 = BitSet::from(2645);
        let bitset2 = BitSet::from(4568);

        let result_bitset = bitset1.cloned_xor(&bitset2);

        assert_eq!(result_bitset.data, BitSet::from(7053).data);
    }

    #[test]
    fn logical_and_not() {
        let bitset1 = BitSet::from(5);
        let bitset2 = BitSet::from(3);

        let result_bitset = bitset1.cloned_and_not(&bitset2);

        assert_eq!(result_bitset.data, BitSet::from(4).data);

        let bitset1 = BitSet::from(8);
        let bitset2 = BitSet::from(9);

        let result_bitset = bitset1.cloned_and_not(&bitset2);

        assert_eq!(result_bitset.data, BitSet::from(0).data);

        let bitset1 = BitSet::from(2645);
        let bitset2 = BitSet::from(4568);

        let result_bitset = bitset1.cloned_and_not(&bitset2);

        assert_eq!(result_bitset.data, BitSet::from(2565).data);
    }
}
