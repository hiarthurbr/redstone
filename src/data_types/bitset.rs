////////////////////////////////////////////////////////////////////////////////
// Taken from https://github.com/bluss/odds/blob/master/src/range.rs //
use std::ops::{Range, RangeFrom, RangeFull, RangeTo};

/// **`IndexRange`** is implemented by Rust's built-in range types, produced
/// by range syntax like `..`, `a..`, `..b` or `c..d`.
pub trait IndexRange<T = usize> {
  #[inline]
  /// Start index (inclusive)
  fn start(&self) -> Option<T> {
    None
  }
  #[inline]
  /// End index (exclusive)
  fn end(&self) -> Option<T> {
    None
  }
}

impl<T> IndexRange<T> for RangeFull { }

impl<T: Copy> IndexRange<T> for RangeFrom<T> {
  #[inline]
  fn start(&self) -> Option<T> {
    Some(self.start)
  }
}

impl<T: Copy> IndexRange<T> for RangeTo<T> {
  #[inline]
  fn end(&self) -> Option<T> {
    Some(self.end)
  }
}

impl<T: Copy> IndexRange<T> for Range<T> {
  #[inline]
  fn start(&self) -> Option<T> {
    Some(self.start)
  }
  #[inline]
  fn end(&self) -> Option<T> {
    Some(self.end)
  }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum BitSetError {
  AttemptToGrowFrozen,
}

/// This struct implements a vector of bits that grows as needed. Each
/// component of the bit set has a `boolean` value. The
/// bits of a [`BitSet`] are indexed by nonnegative integers.
/// Individual indexed bits can be examined, set, or cleared. One
/// [`BitSet`] may be used to modify the contents of another
/// [`BitSet`] through logical AND, logical inclusive OR, and
/// logical exclusive OR operations.
///
/// By default, all bits in the set initially have the value
/// `false`.
///
/// Every bit set has a current size, which is the number of bits
/// of space currently in use by the bit set. Note that the size is
/// related to the implementation of a bit set, so it may change with
/// implementation. The length of a bit set relates to logical length
/// of a bit set and is defined independently of implementation.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct BitSet {
  pub data: Vec<bool>,
  frozen: bool,
}

impl BitSet {
  /// Creates a new bit set. All bits are initially `false`.
  #[must_use] pub fn new() -> Self {
    BitSet::default()
  }

  /// Creates a new bit set with the specified initial size.
  /// 
  /// All bits are initially `false` and the [`BitSet`] size is frozen by default to the specified size.
  /// 
  /// You can unfreeze the [`BitSet`] size with [`BitSet::unfreeze`].
  #[must_use] pub fn with_capacity(nbits: usize) -> Self {
    if nbits == 0 {
      return BitSet::default();
    }

    let mut data = vec![false; nbits];
    data.shrink_to_fit();
    BitSet {
      data,
      frozen: true,
    }
  }

  /// Freezes the size of the [`BitSet`] to the logical size in words of the [`BitSet`].
  /// 
  /// This method is useful in situations where it is known that a [`BitSet`] will
  /// not need to be expanded to accommodate additional words.
  /// 
  /// Attempting to grow a frozen [`BitSet`] will result in an error.
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
  pub fn freeze(&mut self) { self.frozen = true; }

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
  pub fn unfreeze(&mut self) { self.frozen = false; }

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
    for (i, bit) in self.data.iter().enumerate().rev() {
      if *bit {
        self.data.truncate(i + 1);
        return;
      }
    }
    self.data.clear();
  }

  /// Clear all bits in the set
  /// 
  /// Same as calling [`BitSet::clear_range`]`(..)`
  #[allow(clippy::missing_panics_doc)]
  pub fn flush(&mut self) { self.clear_range(..).expect("Should never attempt to grow") }

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
  ///   assert_eq!(bitset.length(), 10);
  /// 
  ///   bitset.grow(10);
  ///   assert_eq!(bitset.length(), 20);
  /// }
  /// ```
  pub fn grow(&mut self, nbits: usize) -> Result<(), BitSetError> {
    if nbits == 0 { return Ok(()) }
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
  pub fn grow_to(&mut self, capacity: usize) -> Result<(), BitSetError> {
    if capacity == 0 { return Ok(()) }
    if self.data.len() < capacity {
      if self.frozen {
        return Err(BitSetError::AttemptToGrowFrozen);
      }
      self.data.resize(self.data.len().max(capacity), false);
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
  pub fn clear(&mut self, bit_index: usize) -> Result<(), BitSetError> {
    self.set(bit_index, false)
  }

  /// Sets the specified range of bits to false.
  /// 
  /// ---
  /// 
  /// ## Errors
  /// 
  /// If the end of the bit range is bigger than the set's length, the set will be grown to the specified index.
  /// If the set is frozen, this function will then return an error. In this case, no values are changed.
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
  pub fn clear_range<R: IndexRange>(&mut self, range: R) -> Result<(), BitSetError> {
    self.set_range(range, false)
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
  pub fn insert(&mut self, bit_index: usize) -> Result<(), BitSetError> {
    self.set(bit_index, true)
  }

  /// Sets the specified range of bits to true.
  /// 
  /// ---
  /// 
  /// ## Errors
  /// 
  /// If the end of the bit range is bigger than the set's length, the set will be grown to the specified index.
  /// If the set is frozen, this function will then return an error. In this case, no values are changed.
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
  pub fn insert_range<R: IndexRange>(&mut self, range: R) -> Result<(), BitSetError> {
    self.set_range(range, true)
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
  pub fn set(&mut self, bit_index: usize, to: bool) -> Result<(), BitSetError> {
    self.grow_to(bit_index + 1)?;
    self.data[bit_index] = to;
    Ok(())
  }

  /// Sets the specified range of bits to the given value.
  /// 
  /// ---
  /// 
  /// ## Errors
  /// 
  /// If the end of the bit range is bigger than the set's length, the set will be grown to the specified index.
  /// If the set is frozen, this function will then return an error. In this case, no values are changed.
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
  pub fn set_range<R: IndexRange>(&mut self, range: R, to: bool) -> Result<(), BitSetError> {
    self.grow_to(range.end().unwrap_or(self.data.len()) + 1)?;
    for i in range.start().unwrap_or(0)..range.end().unwrap_or(self.data.len()) {
      self.data[i] = to;
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
  pub fn flip(&mut self, bit_index: usize) -> Result<(), BitSetError> {
    self.grow_to(bit_index + 1)?;

    self.data[bit_index] = !self.data[bit_index];
    Ok(())
  }

  /// Inverts the specified range of bits.
  /// 
  /// ---
  /// 
  /// ## Errors
  /// 
  /// If the end of the bit range is bigger than the set's length, the set will be grown to the specified index.
  /// If the set is frozen, this function will then return an error. In this case, no values are changed.
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
  pub fn flip_range<R: IndexRange>(&mut self, range: R) -> Result<(), BitSetError> {
    let len = range.end().unwrap_or(self.data.len());
    self.grow_to(len + 1)?;

    for i in range.start().unwrap_or(0)..len {
      self.data[i] = !self.data[i];
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
  pub fn put(&mut self, bit_index: usize, to: bool) -> Result<Option<bool>, BitSetError> {
    let previous = self.get(bit_index);
    self.grow_to(bit_index + 1)?;

    self.data[bit_index] = to;
    Ok(previous)
  }

  /// Gets the value of the specified bit.
  /// 
  /// This method returns `None` if the bit index is bigger than the set's length.
  #[must_use] pub fn get(&self, bit_index: usize) -> Option<bool> {
    if bit_index >= self.length() {
      return None;
    }
    Some(self.data[bit_index])
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
  pub fn and(&mut self, set: &BitSet) -> Result<(), BitSetError> {
    if self.length() < set.length() {
      self.grow_to(set.length())?;
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
  pub fn or(&mut self, set: &BitSet) -> Result<(), BitSetError> {
    if self.length() < set.length() {
      self.grow_to(set.length())?;
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
  pub fn xor(&mut self, set: &BitSet) -> Result<(), BitSetError> {
    if self.length() < set.length() {
      self.grow_to(set.length())?;
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
  pub fn and_not(&mut self, set: &BitSet) -> Result<(), BitSetError> {
    if self.length() < set.length() {
      self.grow_to(set.length())?;
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
  #[must_use] pub fn cloned_and(&self, set: &BitSet) -> BitSet {
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
  #[must_use] pub fn cloned_or(&self, set: &BitSet) -> BitSet {
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
  #[must_use] pub fn cloned_xor(&self, set: &BitSet) -> BitSet {
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
  #[must_use] pub fn cloned_and_not(&self, set: &BitSet) -> BitSet {
    let mut clone = self.clone();
    clone.unfreeze();
    clone.and_not(set).expect("Should never panic on grow");
    clone
  }

  // Maybe implement this later?
  // #[must_use] pub fn intersects(&self, set: &BitSet) -> bool { todo!() }

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
  #[must_use] pub fn is_subset(&self, set: &BitSet) -> bool {
    self.data
      .iter()
      .zip(set.data.iter())
      .all(|(x, y)| !x && !y)
      && self.data.iter().skip(set.data.len()).all(|x| !(*x))
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
  #[must_use] pub fn is_superset(&self, set: &BitSet) -> bool {
    set.is_subset(self)
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
  #[must_use] pub fn is_disjoint(&self, set: &BitSet) -> bool {
    self.data
      .iter()
      .zip(set.data.iter())
      .all(|(x, y)| !(x & y))
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
  #[must_use] pub fn is_empty(&self) -> bool { self.data.len() == 0 }

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
  #[must_use] pub fn is_clear(&self) -> bool {
    for bit in &self.data {
      if *bit {
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
  #[must_use] pub fn is_full(&self) -> bool {
    for bit in &self.data {
      if !*bit {
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
  #[must_use] pub fn length(&self) -> usize { self.data.len() }

  /// Returns the size of the set, excluding trailing zeros
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
  #[must_use] pub fn size(&self) -> usize {
    let mut size = self.length();
    for bit in self.data.iter().rev() {
      if *bit {
        break;
      }
      size -= 1;
    }
    size
  }
}

/// This macro implements the From<$type> trait for [`BitSet`]
macro_rules! impl_from_iter_for_bitset {
  ($type:ty, $number:expr) => {
    impl From<Vec<$type>> for BitSet {
      fn from(vec: Vec<$type>) -> Self {
        let data: Vec<bool> = vec.into_iter().map(|word| {
          let mut bits: Vec<bool> = Vec::new();
          for i in 0..$number {
            bits.push((word & (1 << i)) != 0);
          }
          bits
        }).collect::<Vec<Vec<bool>>>().concat();
        BitSet {
          frozen: true,
          data,
        }
      }
    }
    impl From<&[$type]> for BitSet {
      fn from(slice: &[$type]) -> Self {
        let data: Vec<bool> = slice.iter().map(|word| {
          let mut bits: Vec<bool> = Vec::new();
          for i in 0..$number {
            bits.push((word & (1 << i)) != 0);
          }
          bits
        }).collect::<Vec<Vec<bool>>>().concat();
        BitSet {
          frozen: true,
          data,
        }
      }
    }
    impl From<$type> for BitSet {
      fn from(num: $type) -> Self {
        let mut bits: Vec<bool> = Vec::new();
        for i in 0..$number {
          bits.push((num & (1 << i)) != 0);
        }
        BitSet {
          frozen: true,
          data: bits,
        }
      }
    }
  };
}

impl_from_iter_for_bitset!(u8, 8);
impl_from_iter_for_bitset!(u16, 16);
impl_from_iter_for_bitset!(u32, 32);
impl_from_iter_for_bitset!(u64, 64);
impl_from_iter_for_bitset!(u128, 128);
impl_from_iter_for_bitset!(usize, usize::BITS);

impl_from_iter_for_bitset!(i8, 8);
impl_from_iter_for_bitset!(i16, 16);
impl_from_iter_for_bitset!(i32, 32);
impl_from_iter_for_bitset!(i64, 64);
impl_from_iter_for_bitset!(i128, 128);
impl_from_iter_for_bitset!(isize, isize::BITS);

#[cfg(test)]
mod test {
  use super::BitSet;
  #[test]
  fn initial_state() {
    let bitset = BitSet::default();
    assert_eq!(bitset.length(), 0);
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

    bitset.insert_range(2..6).unwrap();

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
