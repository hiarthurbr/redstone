////////////////////////////////////////////////////////////////////////////////
// This section was taken from https://github.com/bluss/odds/blob/master/src/range.rs
// Although it is not a direct copy, it was heavily inspired by it.
use std::{
    fmt::Debug,
    ops::{
        BitAnd, BitOr, BitXor, Not, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo,
        RangeToInclusive,
    },
};

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

impl<T: Copy> IndexRange<T> for RangeFull {
    #[inline]
    fn range(&self) -> Option<(Option<T>, Option<T>, bool)> {
        Some((None, None, true))
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

impl IndexRange<usize> for usize {
    #[inline]
    fn index(&self) -> Option<usize> {
        Some(*self)
    }
}

////////////////////////////////////////////////////////////////////////////////

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
type Word = u8;

/// This struct implements a vector of bits that grows as needed. Each
/// component of the bit set has a `boolean` value. The
/// bits of a [`BitSet`] are indexed by nonnegative integers.
/// Individual indexed bits can be examined, set, or cleared. One
/// [`BitSet`] may be used to modify the contents of another
/// [`BitSet`] through logical AND, logical inclusive OR, and
/// logical exclusive OR operations.
///
/// By default, all bits in the set initially have the value `false`.
#[derive(Debug, Clone, Eq, PartialOrd, Ord, Default)]
pub struct BitSet {
    data: Vec<Word>,
}

impl BitSet {
    /// Word size in bits
    pub const WORD_SIZE: usize = Word::BITS as usize;
    const ONE: Word = 1;

    /// Creates a new bit set. All bits are initially `false`.
    #[must_use]
    pub fn new() -> Self {
        BitSet::default()
    }

    /// Creates a new bitset with at least `n` bits.
    ///
    /// All bits are initially `false`.
    #[must_use]
    pub fn with_capacity(nbits: usize) -> Self {
        if nbits == 0 {
            return BitSet::default();
        }

        let mut data = vec![Word::default(); Self::word_index(nbits) + 1];
        data.shrink_to_fit();
        BitSet { data }
    }

    /// Trims the set to the last enabled bit.
    pub fn trim(&mut self) {
        for (i, word) in self.data.iter().enumerate().rev() {
            if *word == 0 {
                self.data.truncate(i);
                break;
            }
        }
    }

    /// Clear all bits in the set
    pub fn flush(&mut self) {
        for word in &mut self.data {
            *word = Word::default();
        }
    }

    /// Grows the set with the specified number of bits.
    ///
    /// ---
    ///
    /// ## Example
    ///
    /// ```
    /// use crate::redstone_signal::data_types::BitSet;
    ///
    /// {
    ///   let mut bitset = BitSet::with_capacity(10);
    ///   let len = bitset.len();
    ///   assert_eq!(len, BitSet::word_index(bitset.capacity()));
    ///
    ///   bitset.grow(10);
    ///   assert_eq!(bitset.len(), BitSet::word_index(len * BitSet::WORD_SIZE + 10));
    /// }
    ///
    /// {
    ///   let mut bitset = BitSet::new();
    ///   assert_eq!(bitset.len(), 0);
    ///  
    ///   bitset.grow(10);
    ///   let len = bitset.len();
    ///   assert_eq!(len, BitSet::word_index(10));
    ///
    ///   bitset.grow(10);
    ///   assert_eq!(bitset.len(), BitSet::word_index(len * BitSet::WORD_SIZE + 10));
    /// }
    /// ```
    #[inline]
    pub fn grow(&mut self, nbits: usize) {
        if nbits == 0 {
            return;
        }
        let grow_to = Self::word_index(self.capacity() + nbits);
        self.data.resize_with(grow_to, Word::default);
    }

    /// Grows the set to the specified number of bits.
    ///
    /// This funtion does nothing if the [`BitSet`] is already bigger than the specified capacity.
    ///
    /// ---
    ///
    /// ## Example
    ///
    /// ```
    /// use crate::redstone_signal::data_types::BitSet;
    ///
    /// {
    ///   let mut bitset = BitSet::with_capacity(20);
    ///   assert_eq!(bitset.len(), BitSet::word_index(20) + 1);
    ///
    ///   bitset.grow_to_size(10);
    ///   assert_eq!(bitset.len(), BitSet::word_index(20) + 1);
    /// }
    ///
    /// {
    ///   let mut bitset = BitSet::with_capacity(10);
    ///
    ///   assert_eq!(bitset.len(), BitSet::word_index(10) + 1);
    ///
    ///   bitset.grow_to_size(20);
    ///   assert_eq!(bitset.len(), BitSet::word_index(20) + 1);
    /// }
    /// ```
    #[inline]
    pub fn grow_to_size(&mut self, capacity: usize) {
        if capacity == 0 {
            return;
        }
        let grow_to = Self::word_index(capacity) + 1;
        let len = self.len();
        if len < grow_to {
            self.data.resize_with(grow_to, Word::default);
        }
    }

    /// Sets the specified bit to false.
    ///
    /// ---
    ///
    /// ## Example
    ///
    /// ```
    /// use crate::redstone_signal::data_types::BitSet;
    ///
    /// {
    ///   let mut bitset = BitSet::new();
    ///   bitset.insert(5);
    ///   assert!(bitset.get(5));
    ///
    ///   bitset.clear(5);
    ///   assert!(!bitset.get(5));
    /// }
    ///
    /// {
    ///   let mut bitset = BitSet::new();
    ///   bitset.insert(2..7);
    ///   assert!(bitset.get(5));
    ///
    ///   bitset.clear(..5);
    ///   assert!(!bitset.get(4));
    ///   assert!(bitset.get(5));
    /// }
    /// ```
    pub fn clear<I: IndexRange>(&mut self, index: I) {
        self.set(index, false);
    }

    /// Sets the specified bit to true.
    ///
    /// ---
    ///
    /// ## Example
    ///
    /// ```
    /// use crate::redstone_signal::data_types::BitSet;
    ///
    /// {
    ///   let mut bitset = BitSet::new();
    ///   bitset.insert(3);
    ///   assert!(bitset.get(3));
    /// }
    ///
    /// {
    ///   let mut bitset = BitSet::new();
    ///   bitset.insert(2..7);
    ///   assert!(bitset.get(6));
    ///   assert!(!bitset.get(7));
    ///
    ///   bitset.insert(..15);
    ///   assert!(bitset.get(9));
    /// }
    ///
    /// {
    ///   let mut bitset = BitSet::new();
    ///   bitset.insert(..=5);
    ///   assert!(bitset.get(4));
    ///   assert!(bitset.get(5));
    /// }
    /// ```
    pub fn insert<I: IndexRange>(&mut self, index: I) {
        self.set(index, true);
    }

    #[must_use]
    #[inline(always)]
    pub fn word_index(ibit: usize) -> usize {
        (ibit - (ibit % Self::WORD_SIZE)) / Self::WORD_SIZE
    }

    /// Sets the specified bit to the given value.
    ///
    /// ---
    ///
    /// ## Example
    ///
    /// ```
    /// use crate::redstone_signal::data_types::BitSet;
    ///
    /// {
    ///   let mut bitset = BitSet::new();
    ///   bitset.set(5, true);
    ///   assert!(bitset.get(5));
    ///
    ///   bitset.set(5, false);
    ///   assert!(!bitset.get(5));
    /// }
    ///
    /// {
    ///   let mut bitset = BitSet::new();
    ///   bitset.set(2..7, true);
    ///   assert!(bitset.get(2));
    ///   assert!(bitset.get(5));
    ///   assert!(bitset.get(6));
    ///
    ///   assert!(!bitset.get(1));
    ///   assert!(!bitset.get(7));
    ///
    ///   bitset.set(..5, false);
    ///   assert!(!bitset.get(4));
    /// }
    ///
    /// {
    ///   let mut bitset = BitSet::new();
    ///   bitset.set(..5, true);
    ///   assert!(bitset.get(4));
    /// }
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    pub fn set<I: IndexRange>(&mut self, index: I, to: bool) {
        if let Some(index) = index.index() {
            let word_index = Self::word_index(index);
            let bit_index = index % Self::WORD_SIZE;

            self.grow_to_size(index + 1);

            let shift_pattern = Self::ONE << bit_index;

            self.data[word_index] |= shift_pattern;

            if !to {
                self.data[word_index] ^= shift_pattern;
            }
        }
        if let Some((start, end, inclusive)) = index.range() {
            let len = self.capacity();
            let start = start.unwrap_or(0);
            let end = match end {
                Some(end) if inclusive => end + 1,
                Some(end) => end,
                None if inclusive => len,
                None => len - 1,
            };
            let width = end - start;

            if inclusive {
                self.grow_to_size(end - 1);
            } else {
                self.grow_to_size(end);
            }

            let starting_word_index = Self::word_index(start);
            let ending_word_index = Self::word_index(end);
            let shift_offset = start % Self::WORD_SIZE;
            let total_offset = shift_offset + width;
            let mut remaining_offset = width;

            for (e, word_index) in (starting_word_index..=ending_word_index).enumerate() {
                let shift_pattern = if e == 0 && remaining_offset < Self::WORD_SIZE {
                    remaining_offset -= shift_offset;

                    let res = Word::MAX << shift_offset;

                    if total_offset >= Self::WORD_SIZE {
                        res
                    } else {
                        res ^ (res & (Word::MAX << total_offset))
                    }
                } else if remaining_offset >= Self::WORD_SIZE {
                    remaining_offset -= Self::WORD_SIZE;
                    Word::MAX
                } else {
                    let res = Word::MAX >> (Self::WORD_SIZE - remaining_offset);
                    remaining_offset = 0;
                    res
                };

                self.data[word_index] |= shift_pattern;
                if !to {
                    self.data[word_index] ^= shift_pattern;
                }

                if remaining_offset == 0 {
                    break;
                }
            }
        }
    }

    /// Inverts the specified bit.
    ///
    /// ---
    ///
    /// ## Example
    ///
    /// ```
    /// use crate::redstone_signal::data_types::BitSet;
    ///
    /// {
    ///   let mut bitset = BitSet::new();
    ///   bitset.flip(5);
    ///   assert!(bitset.get(5));
    ///
    ///   bitset.flip(5);
    ///   assert!(!bitset.get(5));
    /// }
    ///
    /// {
    ///   let mut bitset = BitSet::new();
    ///   bitset.flip(2..7);
    ///   assert!(bitset.get(4));
    ///   assert!(bitset.get(5));
    ///
    ///   bitset.flip(..5);
    ///   assert!(!bitset.get(4));
    ///   assert!(bitset.get(5));
    /// }
    ///
    /// {
    ///   let mut bitset = BitSet::new();
    ///   bitset.flip(..5);
    ///   assert!(bitset.get(4));
    ///   assert!(!bitset.get(5));
    /// }
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    pub fn flip<I: IndexRange>(&mut self, index: I) {
        if let Some(index) = index.index() {
            self.set(index, !self.get(index));
        }
        if let Some((start, end, inclusive)) = index.range() {
            let bitset_len = self.len();
            let len = bitset_len * Self::WORD_SIZE;
            let start = start.unwrap_or(0);
            let end = end.unwrap_or(len);

            if inclusive {
                for i in start..=end {
                    self.set(i, !self.get(i));
                }
            } else {
                for i in start..end {
                    self.set(i, !self.get(i));
                }
            }
        }
    }

    /// Sets the specified bit to the given value, returning its previous value.
    ///
    /// ---
    ///
    /// ## Example
    ///
    /// ```
    /// use crate::redstone_signal::data_types::BitSet;
    ///
    /// {
    ///   let mut bitset = BitSet::new();
    ///   assert_eq!(bitset.put(5, true), false);
    ///   assert!(bitset.get(5));
    ///
    ///   assert_eq!(bitset.put(5, true), true);
    ///   assert!(bitset.get(5));
    ///
    ///   assert_eq!(bitset.put(5, false), true);
    ///   assert!(!bitset.get(5));
    /// }
    /// ```
    pub fn put(&mut self, bit_index: usize, to: bool) -> bool {
        let previous = self.get(bit_index);

        self.set(bit_index, to);
        previous
    }

    /// Gets the value of the specified bit.
    #[must_use]
    pub fn get(&self, bit_index: usize) -> bool {
        let index = Self::word_index(bit_index);
        let bit_index = bit_index % Self::WORD_SIZE;
        let bit = (self.data.get(index).copied().unwrap_or_default() >> bit_index) & 1;
        bit == 1
    }

    /// Performs the logical operator `AND` in this set.
    ///
    /// ---
    ///
    /// ## Example
    ///
    /// ```
    /// use crate::redstone_signal::data_types::BitSet;
    ///
    /// {
    ///   let mut bitset1 = BitSet::from(5);
    ///   let bitset2 = BitSet::from(3);
    ///
    ///   assert_eq!(bitset1.and(&bitset2), BitSet::from(5 & 3));
    ///   assert_eq!(bitset1 & bitset2, BitSet::from(5 & 3));
    /// }
    /// ```
    #[must_use]
    pub fn and(&self, set: &BitSet) -> BitSet {
        let data = (0..self.data.len().max(set.data.len())).map(|i| {
            let a = self.data.get(i).copied().unwrap_or_default();
            let b = set.data.get(i).copied().unwrap_or_default();
            a & b
        });
        BitSet {
            data: data.collect(),
        }
    }

    /// Performs the logical operator `OR` in this set.
    ///
    /// ---
    ///
    /// ## Example
    ///
    /// ```
    /// use crate::redstone_signal::data_types::BitSet;
    ///
    /// {
    ///   let mut bitset1 = BitSet::from(5);
    ///   let bitset2 = BitSet::from(3);
    ///  
    ///   assert_eq!(bitset1.or(&bitset2), BitSet::from(5 | 3));
    ///   assert_eq!(bitset1 | bitset2, BitSet::from(5 | 3));
    /// }
    /// ```
    #[must_use]
    pub fn or(&self, set: &BitSet) -> BitSet {
        let data = (0..self.data.len().max(set.data.len())).map(|i| {
            let a = self.data.get(i).copied().unwrap_or_default();
            let b = set.data.get(i).copied().unwrap_or_default();
            a | b
        });
        BitSet {
            data: data.collect(),
        }
    }

    /// Performs the logical operator `XOR` in this set.
    ///
    /// ---
    ///
    /// ## Example
    ///
    /// ```
    /// use crate::redstone_signal::data_types::BitSet;
    ///
    /// {
    ///   let mut bitset1 = BitSet::from(5);
    ///   let bitset2 = BitSet::from(3);
    ///
    ///   assert_eq!(bitset1.xor(&bitset2), BitSet::from(5 ^ 3));
    ///   assert_eq!(bitset1 ^ bitset2, BitSet::from(5 ^ 3));
    /// }
    /// ```
    #[must_use]
    pub fn xor(&self, set: &BitSet) -> BitSet {
        let data = (0..self.data.len().max(set.data.len())).map(|i| {
            let a = self.data.get(i).copied().unwrap_or_default();
            let b = set.data.get(i).copied().unwrap_or_default();
            a ^ b
        });
        BitSet {
            data: data.collect(),
        }
    }

    /// Performs the logical operator `AND NOT` in this set.
    ///
    /// ---
    ///
    /// ## Example
    ///
    /// ```
    /// use crate::redstone_signal::data_types::BitSet;
    ///
    /// {
    ///   let mut bitset1 = BitSet::from(5);
    ///   let bitset2 = BitSet::from(3);
    ///
    ///   assert_eq!(bitset1.and_not(&bitset2), BitSet::from(5 & !3));
    /// }
    /// ```
    #[must_use]
    pub fn and_not(&self, set: &BitSet) -> BitSet {
        let data = (0..self.data.len().max(set.data.len())).map(|i| {
            let a = self.data.get(i).copied().unwrap_or_default();
            let b = set.data.get(i).copied().unwrap_or_default();
            a & !b
        });
        BitSet {
            data: data.collect(),
        }
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
    /// use crate::redstone_signal::data_types::BitSet;
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
    #[inline]
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
    /// use crate::redstone_signal::data_types::BitSet;
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
    ///   bitset.insert(2..9);
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
    /// use crate::redstone_signal::data_types::BitSet;
    ///
    /// {
    ///   let mut bitset = BitSet::new();
    ///
    ///   bitset.insert(10);
    ///   assert!(!bitset.is_full());
    ///
    ///   bitset.insert(..);
    ///   assert!(bitset.is_full());
    /// }
    /// ```
    #[must_use]
    pub fn is_full(&self) -> bool {
        for word in self.data.iter().rev() {
            if *word != Word::MAX {
                return false;
            }
        }
        true
    }

    /// Returns the length of the set, in words.
    ///
    /// This includes trailing zeros.
    ///
    /// ---
    ///
    /// ## Example
    ///
    /// ```
    /// use crate::redstone_signal::data_types::BitSet;
    ///
    /// {
    ///   let mut bitset = BitSet::new();
    ///   assert_eq!(bitset.len(), 0);
    ///
    ///   bitset.insert(5);
    ///   assert_eq!(bitset.len(), BitSet::word_index(5) + 1);
    /// }
    /// ```
    #[must_use]
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns the length of the set, in bits.
    ///
    /// This includes trailing zeros.
    ///
    /// ---
    ///
    /// ## Example
    ///
    /// ```
    /// use crate::redstone_signal::data_types::BitSet;
    ///
    /// {
    ///   let mut bitset = BitSet::new();
    ///   assert_eq!(bitset.capacity(), 0);
    ///
    ///   bitset.insert(5);
    ///   assert_eq!(bitset.capacity(), (BitSet::word_index(5) + 1) * BitSet::WORD_SIZE);
    /// }
    /// ```
    #[must_use]
    #[inline(always)]
    pub fn capacity(&self) -> usize {
        self.len() * Self::WORD_SIZE
    }

    /// Returns the size of the set in words, excluding trailing zeros
    ///
    /// ---
    ///
    /// ## Example
    ///
    /// ```
    /// use crate::redstone_signal::data_types::BitSet;
    ///
    /// {
    ///   let mut bitset = BitSet::new();
    ///   assert_eq!(bitset.size(), 0);
    ///   println!("{bitset:?}");
    ///   
    ///   bitset.insert(5..15);
    ///   assert_eq!(bitset.size(), BitSet::word_index(15) + 1);
    ///   
    ///   bitset.clear(10..);
    ///   assert_eq!(bitset.size(), BitSet::word_index(9) + 1);
    ///
    ///   bitset.trim();
    ///   assert_eq!(bitset.size(), BitSet::word_index(9) + 1);
    ///
    ///   bitset.flush();
    ///   assert_eq!(bitset.size(), 0);
    /// }
    /// ```
    #[must_use]
    #[inline]
    pub fn size(&self) -> usize {
        let mut size = self.len();
        for bit in self.data.iter().rev() {
            if *bit == 0 {
                size -= 1;
            } else {
                break;
            }
        }
        size
    }
}

/// This macro implements the `From<$type>`, `From<Vec<$type>>` and `From<&[$type]>` trait for [`BitSet`]
macro_rules! impl_from_iter_for_bitset {
    ($(($type:ty, $mod:ident)),*) => {
        $(
            pub mod $mod {
                use super::*;

                fn impl_data(num: $type) -> Vec<Word> {
                    let mut data: Vec<Word> = Vec::new();

                    let word: Result<Word, _> = num.try_into();

                    match word {
                        Ok(n) => data.push(n),
                        Err(_) => {
                            for shift in (0..<$type>::BITS).step_by(BitSet::WORD_SIZE) {
                                #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
                                let word: Word = (num >> shift) as Word;
                                data.push(word);
                            }
                        }
                    }

                    data
                }

                impl From<$type> for BitSet {
                    fn from(num: $type) -> Self {
                        BitSet {
                            data: impl_data(num)
                        }
                    }
                }
                impl From<&[$type]> for BitSet {
                    fn from(slice: &[$type]) -> Self {
                        let data: Vec<Word> = slice
                            .iter()
                            .map(|word| {
                                let num = *word;
                                impl_data(num)
                            })
                            .collect::<Vec<Vec<Word>>>()
                            .concat();
                        BitSet { data }
                    }
                }
                impl From<Vec<$type>> for BitSet {
                    fn from(vec: Vec<$type>) -> Self {
                        let data: Vec<Word> = vec
                            .into_iter()
                            .map(|num| {
                                impl_data(num)
                            })
                            .collect::<Vec<Vec<Word>>>()
                            .concat();
                        BitSet { data }
                    }
                }
            }
        )*
    };
}

impl_from_iter_for_bitset!(
    (u8, u8),
    (u16, u16),
    (u32, u32),
    (u64, u64),
    (u128, u128),
    (usize, usize),
    (i8, i8),
    (i16, i16),
    (i32, i32),
    (i64, i64),
    (i128, i128),
    (isize, isize)
);

fn impl_bool_data(data: &[bool]) -> Vec<Word> {
    let mut vec = Vec::new();
    for chunk in data.chunks(BitSet::WORD_SIZE) {
        let mut byte = Word::default();
        for (i, &bit) in chunk.iter().enumerate() {
            if bit {
                byte |= 1 << i;
            }
        }
        vec.push(byte);
    }
    vec
}

impl From<&[bool]> for BitSet {
    fn from(slice: &[bool]) -> Self {
        BitSet {
            data: impl_bool_data(slice),
        }
    }
}

impl From<Vec<bool>> for BitSet {
    fn from(slice: Vec<bool>) -> Self {
        BitSet {
            data: impl_bool_data(&slice),
        }
    }
}

impl BitAnd for BitSet {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.and(&rhs)
    }
}

impl BitAnd<&BitSet> for BitSet {
    type Output = BitSet;

    fn bitand(self, rhs: &BitSet) -> Self::Output {
        self.and(rhs)
    }
}

impl BitOr for BitSet {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.or(&rhs)
    }
}

impl BitOr<&BitSet> for BitSet {
    type Output = BitSet;

    fn bitor(self, rhs: &BitSet) -> Self::Output {
        self.or(rhs)
    }
}

impl BitXor for BitSet {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.xor(&rhs)
    }
}

impl BitXor<&BitSet> for BitSet {
    type Output = BitSet;

    fn bitxor(self, rhs: &BitSet) -> Self::Output {
        self.xor(rhs)
    }
}

impl Not for BitSet {
    type Output = Self;

    fn not(self) -> Self::Output {
        BitSet {
            data: self.data.into_iter().map(|word| !word).collect(),
        }
    }
}

impl Not for &BitSet {
    type Output = BitSet;

    fn not(self) -> Self::Output {
        BitSet {
            data: self.data.iter().map(|word| !word).collect(),
        }
    }
}

impl PartialEq for BitSet {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..self.data.len().max(other.data.len()) {
            let a = self.data.get(i).copied().unwrap_or_default();
            let b = other.data.get(i).copied().unwrap_or_default();
            if a != b {
                return false;
            }
        }
        true
    }
}

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

        bitset.insert(0);
        bitset.insert(2);
        bitset.insert(5);
        bitset.insert(10);

        assert!(bitset.get(0));
        assert!(bitset.get(2));
        assert!(bitset.get(5));
        assert!(bitset.get(10));

        assert!(!bitset.get(1));
        assert!(!bitset.get(3));
        assert!(!bitset.get(6));
        assert!(!bitset.get(9));
    }

    #[test]
    fn clear_bits() {
        let mut bitset = BitSet::default();

        bitset.insert(2..6);

        bitset.clear(3);

        assert!(!bitset.get(3));

        assert!(bitset.get(2));
        assert!(bitset.get(4));
        assert!(bitset.get(5));

        assert!(!bitset.get(6));
        assert!(!bitset.get(7));
    }

    #[test]
    fn logical_and() {
        let bitset1 = BitSet::from(5);
        let bitset2 = BitSet::from(3);

        assert_eq!(bitset1 & bitset2, BitSet::from(5 & 3));

        let bitset1 = BitSet::from(8);
        let bitset2 = BitSet::from(9);

        assert_eq!(bitset1 & bitset2, BitSet::from(8 & 9));

        let bitset1 = BitSet::from(2645);
        let bitset2 = BitSet::from(4568);

        assert_eq!(bitset1 & bitset2, BitSet::from(2645 & 4568));
    }

    #[test]
    fn logical_or() {
        let bitset1 = BitSet::from(5);
        let bitset2 = BitSet::from(3);

        assert_eq!(bitset1 | bitset2, BitSet::from(5 | 3));

        let bitset1 = BitSet::from(8);
        let bitset2 = BitSet::from(9);

        assert_eq!(bitset1 | bitset2, BitSet::from(8 | 9));

        let bitset1 = BitSet::from(2645);
        let bitset2 = BitSet::from(4568);

        assert_eq!(bitset1 | bitset2, BitSet::from(2645 | 4568));
    }

    #[test]
    fn logical_xor() {
        let bitset1 = BitSet::from(5);
        let bitset2 = BitSet::from(3);

        assert_eq!(bitset1 ^ bitset2, BitSet::from(5 ^ 3));

        let bitset1 = BitSet::from(8);
        let bitset2 = BitSet::from(9);

        assert_eq!(bitset1 ^ bitset2, BitSet::from(8 ^ 9));

        let bitset1 = BitSet::from(2645);
        let bitset2 = BitSet::from(4568);

        assert_eq!(bitset1 ^ bitset2, BitSet::from(2645 ^ 4568));
    }

    #[test]
    fn logical_and_not() {
        let bitset1 = BitSet::from(5);
        let bitset2 = BitSet::from(3);

        let result_bitset = bitset1.and_not(&bitset2);

        assert_eq!(result_bitset, BitSet::from(5 & !3));

        let bitset1 = BitSet::from(8);
        let bitset2 = BitSet::from(9);

        let result_bitset = bitset1.and_not(&bitset2);

        assert_eq!(result_bitset, BitSet::from(8 & !9));

        let bitset1 = BitSet::from(2645);
        let bitset2 = BitSet::from(4568);

        let result_bitset = bitset1.and_not(&bitset2);

        assert_eq!(result_bitset, BitSet::from(2645 & !4568));
    }
}
