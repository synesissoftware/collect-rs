// containers/unicode_point_map.rs : `UnicodePointMap`

use base_traits::{
    IsEmpty,
    Len,
};

use std::{
    collections::HashMap,
    iter::FusedIterator,
    ops as std_ops,
};


/// Constants
pub(crate) mod constants {

    #[cfg(debug_assertions)]
    pub(crate) const DEFAULT_CONTIGUOUS_CEILING : char = '\u{80}';
    #[cfg(not(debug_assertions))]
    pub(crate) const DEFAULT_CONTIGUOUS_CEILING : char = '\u{200}';

    pub(crate) const MAXIMUM_VALID_CHAR_VALUE : u32 = 0x10ffff;
}


mod util {
    #![allow(unexpected_cfgs)] // NOTE: this is to allow `target_pointer_width = "128"` for future-proof

    use super::constants::MAXIMUM_VALID_CHAR_VALUE;

    use std::mem as std_mem;


    /// Converts the given `char` value to a `usize` value that is expected,
    /// taking into account when sizeof(char) > sizeof(usize), and asserting
    /// that `c` is in the valid character range.
    #[inline]
    pub fn char_to_valid_index(c : char) -> usize {

        let c_u32 = u32::from(c);

        debug_assert!(c_u32 <= MAXIMUM_VALID_CHAR_VALUE, "parameter `c` must be a valid `char` instance, i.e. be in the range [0, {MAXIMUM_VALID_CHAR_VALUE}), but has the value {c_u32}");

        // Rust does not have a specific `usize` size defined, so we do a
        // check here in case sizeof(usize)<sizeof(char)

        #[cfg(not(any(
            target_pointer_width = "16",
            target_pointer_width = "32",
            target_pointer_width = "64",
            target_pointer_width = "128",
        )))]
        {
            compile_error!(r#"this code not valid for any `"target_pointer_width"` value other than `"16"`, `"32"`, `"64", `"128"`"#);
        }

        if cfg!(target_pointer_width = "16") {
            // since a `char` is (according to standard) always 32-bits, in
            // this circumstance we may have a character value that exceeds
            // the 16-bits of `usize`
            debug_assert!(std_mem::size_of::<usize>() < std_mem::size_of::<char>());

            let usize_max_u32 = usize::MAX as u32;

            if c_u32 > usize_max_u32 {
                usize_max_u32 as usize
            } else {
                c_u32 as usize
            }
        } else {
            // since a `char` is (according to standard) always 32-bits, in
            // this circumstance we can simply convert the character (in
            // its `u32` form) into `usize`
            debug_assert!(std_mem::size_of::<usize>() >= std_mem::size_of::<char>());

            c_u32 as usize
        }
    }
}


/// A container that measures the frequencies of the unique code points that
/// it contains.
#[derive(Debug)]
pub struct UnicodePointMap {
    /// Contiguous storage for common characters.
    vec :   Vec<isize>,
    /// Map for other characters outside the contiguous range provide by
    /// `self.vec`.
    map :   HashMap<char, isize>,
    /// The number of unique characters represented.
    len : usize,
    /// The total number of characters represented.
    total : i64,
}

// API functions

impl UnicodePointMap {
    /// Creates a new instance wherein the continguous storage portion has
    /// the extent according to the given `default_contiguous_ceiling`.
    pub fn new(
        default_contiguous_ceiling : char
    ) -> Self {
        // validate `default_contiguous_ceiling` fits in the valid range for
        // values for `char` and the valid range of storage for `usize`
        let ceiling = {
            let dcc_u32 = {
                let dcc_u32 = default_contiguous_ceiling as u32;

                if dcc_u32 > constants::MAXIMUM_VALID_CHAR_VALUE {
                    constants::MAXIMUM_VALID_CHAR_VALUE
                } else {
                    dcc_u32
                }
            };

            // Rust does not have a specific `usize` size defined, so we do a
            // check here in case sizeof(usize)<sizeof(char)
            if cfg!(target_pointer_width = "16") {
                let usize_max_u32 = usize::MAX as u32;

                if dcc_u32 > usize_max_u32 {
                    usize_max_u32 as usize
                } else {
                    dcc_u32 as usize
                }
            } else {
                dcc_u32 as usize
            }
        };

        let vec = vec![0; ceiling];
        let map = HashMap::new();
        let len = 0;
        let total = 0;

        Self {
            vec,
            map,
            len,
            total,
        }
    }
}

// Mutating methods

impl UnicodePointMap {
    /// Clears the map, removing all records and resets `#total()`.
    #[inline]
    pub fn clear(&mut self) {
        self.vec.fill(0);
        self.map.clear();
        self.len = 0;
        self.total = 0;
    }

    /// Inserts a record for the given `c` with the given `count`.
    ///
    /// # Preconditions:
    /// - `c` - `c` must be a valid [`char`] instance, i.e. be in the range
    ///   [0, 0x110000);
    pub fn insert(
        &mut self,
        c : char,
        count : isize
    ) -> Option<isize> {
        let ix = util::char_to_valid_index(c);

        if let Some(v) = self.vec.get_mut(ix) {
            let prev = *v;

            *v = count;

            let curr = *v;

            if 0 == prev {
                if 0 != curr {
                    self.len += 1;
                }
            } else if 0 == curr {
                self.len -= 1;
            }

            self.total += (curr - prev) as i64;

            if 0 == prev {
                None
            } else {
                Some(prev)
            }
        } else {
            match self.map.get_mut(&c) {
                Some(v) => {
                    debug_assert!(0 != *v);

                    let prev = *v;

                    *v = count;

                    if *v == 0 {
                        self.map.remove(&c);

                        self.len -= 1;
                    }

                    self.total += (count - prev) as i64;

                    Some(prev)
                },
                None => {
                    if 0 != count {

                        self.len += 1;

                        self.map.insert(c, count);
                    }

                    self.total += count as i64;

                    None
                }
            }
        }
    }

    /// Updates the count by 1 of an existing record identified by `c`, or
    /// creates, with a count of 1, a new record.
    ///
    /// In the case that the resulting count of an existing record is 0 then
    /// the record is removed.
    pub fn push(
        &mut self,
        c : char,
    ) {
        let ix = util::char_to_valid_index(c);

        if let Some(v) = self.vec.get_mut(ix) {
            let prev = *v;

            *v += 1;

            if 0 == prev {
                self.len += 1;
            }

            self.total += 1;
        } else {
            match self.map.get_mut(&c) {
                Some(v) => {
                    debug_assert!(0 != *v);

                    *v += 1;
                },
                None => {
                    self.len += 1;

                    self.map.insert(c, 1);
                }
            }

            self.total += 1;
        }
    }

    /// Updates the count by `count` of an existing record identifed by
    /// `c`, or creates, with the given `count`, a new record.
    ///
    /// In the case that the resulting count of an existing record is 0 then
    /// the record is removed.
    pub fn push_n(
        &mut self,
        c : char,
        count : isize
    ) {
        if 0 != count {
            let ix = util::char_to_valid_index(c);

            if let Some(v) = self.vec.get_mut(ix) {
                let prev = *v;

                *v += count;

                let new = *v;

                if 0 == prev {
                    if 0 == new {
                    } else {
                        self.len += 1;
                    }
                } else if 0 == new {
                    self.len -= 1;
                }
            } else {
                #[allow(clippy::collapsible_else_if)]
                if let Some(v) = self.map.get_mut(&c) {
                    debug_assert!(0 != *v);

                    *v += count;

                    if 0 == *v {
                        self.map.remove(&c);

                        self.len -= 1;
                    }
                } else {
                    self.map.insert(c, count);

                    self.len += 1;
                }
            }

            self.total += count as i64;
        }
    }

    /// Removes an entry from the map, returning the count of the key if the
    /// key was previously in the map.
    ///
    /// # Preconditions:
    /// - `c` - `c` must be a valid [`char`] instance, i.e. be in the range
    ///   [0, 0x110000);
    pub fn remove(
        &mut self,
        c : &char
    ) -> Option<isize> {
        let ix = util::char_to_valid_index(*c);

        if let Some(v) = self.vec.get_mut(ix) {
            let prev = *v;

            *v = 0;

            if 0 != prev {
                self.len -= 1;

                self.total -= prev as i64;

                Some(prev)
            }
            else {
                None
            }
        } else {
            match self.map.remove(c) {
                Some(v) => {

                    self.len -= 1;
                    self.total -= v as i64;

                    Some(v)
                },
                None => {
                    None
                }
            }
        }
    }
}

// Non-mutating methods

impl UnicodePointMap {
    /// Returns the number of records the map can hold without
    /// reallocation.
    #[inline]
    pub fn capacity(&self) -> usize {
        self.vec.len() + self.map.capacity()
    }

    /// Indicates whether a record exists for the given `c`.
    ///
    /// # Preconditions:
    /// - `c` - `c` must be a valid [`char`] instance, i.e. be in the range
    ///   [0, 0x110000);
    pub fn contains_key(
        &self,
        c : &char,
    ) -> bool {
        let ix = util::char_to_valid_index(*c);

        if let Some(&count) = self.vec.get(ix) {
            count != 0
        } else {
            self.map.contains_key(c)
        }
    }

    /// Obtains the count corresponding to the given `c`, obtaining 0 in the
    /// case that no such record exists.
    ///
    /// # Parameters:
    /// - `c` - the character for which to search. Must be a valid [`char`]
    ///   value
    ///
    /// # Preconditions:
    /// - `c` - `c` must be a valid [`char`] instance, i.e. be in the range
    ///   [0, 0x110000);
    pub fn get(
        &self,
        c : &char,
    ) -> isize {
        let ix = util::char_to_valid_index(*c);

        if let Some(&count) = self.vec.get(ix) {
            count
        } else {
            self.map.get(c).copied().unwrap_or_default()
        }
    }

    /// Indicates whether the instance contains no records.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.is_empty_()
    }
}

/// Iterator for [`UnicodePointMap`].
pub struct UnicodePointMapIter<'a> {
    /// Reference to the providing [`UnicodePointMap`] instance.
    upm : &'a UnicodePointMap,
    /// (Optional) index into the vector.
    vec_index : Option<usize>,
    /// (Optional) interator into the map.
    map_iter : Option<std::collections::hash_map::Iter<'a, char, isize>>,
}

impl FusedIterator for UnicodePointMapIter<'_> {
}

impl Iterator for UnicodePointMapIter<'_> {
    type Item = (char, isize);

    /// Advances the iterator and returns the next value.
    ///
    /// Returns `None` when iteration is finished. Further calls continue
    /// to return `None`.
    fn next(&mut self) -> Option<(char, isize)> {
        debug_assert!(self.vec_index.is_none() || self.map_iter.is_none());

        if let Some(ix) = &mut self.vec_index {
            while *ix < self.upm.vec.len() {
                let count = self.upm.vec[*ix];

                if 0 != count {
                    // NOTE: although it is possible (e.g. on 16-bit arch)
                    // for sizeof(char)>sizeof(usize) it is an invariant of
                    // the `UnicodePointMap` design that *ix cannot exceed
                    // `usize`, so following brute-force is well-defined.
                    let c = unsafe {
                        let c_usize = *ix;
                        let c_u32 = c_usize as u32;

                        char::from_u32_unchecked(c_u32)
                    };

                    *ix += 1;

                    return Some((c, count))
                } else {
                    *ix += 1;
                }
            }

            self.vec_index = None;
            self.map_iter = Some(self.upm.map.iter());
        }

        if let Some(mi) = &mut self.map_iter {
            match mi.next() {
                Some((&c, &count)) => {
                    return Some((c, count));
                },
                None => {
                    self.map_iter = None;
                }
            }
        }

        None
    }
}

impl UnicodePointMap {

    /// An iterator visiting all key-count pairs in arbitrary order. The
    /// iterator element type is `(&'a K, &'a isize)`.
    #[inline]
    pub fn iter(&self) -> UnicodePointMapIter<'_> {
        let upm = &self;
        let vec_index = Some(0);
        let map_iter = None;

        UnicodePointMapIter {
            upm,
            vec_index,
            map_iter,
        }
    }
}

impl UnicodePointMap {

    /// Obtains the number of records.
    #[inline]
    pub fn len(&self) -> usize {
        self.len_()
    }

    /// Indicates the total frequency count across all records.
    #[inline]
    pub fn total(&self) -> i64 {
        self.total_()
    }
}

// Implementation

impl UnicodePointMap {
    #[inline]
    fn is_empty_(&self) -> bool {
        0 == self.len
    }

    fn get_(
        &self,
        c : &char
    ) -> &isize {
        let ix = util::char_to_valid_index(*c);

        if let Some(count) = self.vec.get(ix) {
            count
        } else {
            match self.map.get(c) {
                Some(count) => count,
                None => &0
            }
        }
    }

    #[inline]
    fn len_(&self) -> usize {
        self.len
    }

    #[inline]
    fn total_(&self) -> i64 {
        self.total
    }
}

// Trait implementations

impl Default for UnicodePointMap {
    /// Creates an empty instance.
    #[inline]
    fn default() -> Self {
        Self::new(constants::DEFAULT_CONTIGUOUS_CEILING)
    }
}

impl<const N: usize> From<[(char, isize); N]> for UnicodePointMap {
    /// Creates an instance from an array of key + count pairs.
    fn from(value: [(char, isize); N]) -> Self {
        // TODO: consider finding max element and calling `::new()` appropriately
        let mut upm = UnicodePointMap::default();

        for (c, count) in value {
            upm.push_n(c, count);
        }

        upm
    }
}

impl<const N: usize> From<[char; N]> for UnicodePointMap {
    /// Creates an instance from an array of keys.
    fn from(value: [char; N]) -> Self {
        // TODO: consider finding max element and calling `::new()` appropriately
        let mut upm = UnicodePointMap::default();

        for c in value {
            upm.push(c);
        }

        upm
    }
}

impl FromIterator<(char, isize)> for UnicodePointMap {
    /// Creates an instance from an iterator of keys + count pairs.
    fn from_iter<T: IntoIterator<Item = (char, isize)>>(iter: T) -> Self {
        let iter = iter.into_iter();

        let mut upm = UnicodePointMap::default();

        for (c, count) in iter {
            upm.push_n(c, count);
        }

        upm
    }
}

impl FromIterator<char> for UnicodePointMap {
    /// Creates an instance from an iterator of keys.
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        let iter = iter.into_iter();

        let mut upm = UnicodePointMap::default();

        for c in iter {
            upm.push(c);
        }

        upm
    }
}

impl std_ops::Index<char> for UnicodePointMap {
    type Output = isize;

    /// Performs the indexing (`container[index]`) operation.
    ///
    /// # Parameters:
    /// - `c` - the character for which to search. Must be a valid [`char`]
    ///   value
    ///
    /// # Preconditions:
    /// - `c` - `c` must be a valid [`char`] instance, i.e. be in the range
    ///   [0, 0x110000);
    ///
    /// # Panics
    ///
    /// May panic if the index is out of bounds.
    #[inline]
    fn index(
        &self,
        c : char
    ) -> &Self::Output {
        self.get_(&c)
    }
}

impl std_ops::Index<&char> for UnicodePointMap {
    type Output = isize;

    /// Performs the indexing (`container[index]`) operation.
    ///
    /// # Parameters:
    /// - `c` - the character for which to search. Must be a valid [`char`]
    ///   value
    ///
    /// # Preconditions:
    /// - `c` - `c` must be a valid [`char`] instance, i.e. be in the range
    ///   [0, 0x110000);
    ///
    /// # Panics
    ///
    /// May panic if the index is out of bounds.
    #[inline]
    fn index(
        &self,
        c : &char
    ) -> &Self::Output {
        self.get_(c)
    }
}

impl IsEmpty for UnicodePointMap {
    /// Indicates whether the instance contains no records.
    #[inline]
    fn is_empty(&self) -> bool {
        self.is_empty_()
    }
}

impl Len for UnicodePointMap {
    /// Obtains the number of records.
    #[inline]
    fn len(&self) -> usize {
        self.len_()
    }
}


#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::UnicodePointMap;

    use base_traits::{
        IsEmpty,
        Len,
    };


    #[test]
    fn TEST_Default() {
        let upm = UnicodePointMap::default();

        assert!(upm.is_empty());
        assert_eq!(0, upm.len());

        assert_eq!(0, upm[&'\0']);
        assert_eq!(0, upm[&'a']);
        assert_eq!(0, upm[&'b']);
        assert_eq!(0, upm[&'c']);
        assert_eq!(0, upm[&'0']);
        assert_eq!(0, upm[&'A']);
        assert_eq!(0, upm[&'🐻']);

        {
            let ie : &dyn IsEmpty = &upm;

            assert!(ie.is_empty());
        }

        {
            let l : &dyn Len = &upm;

            assert_eq!(0, l.len());
        }
    }

    #[test]
    fn TEST_push() {
        let mut upm = UnicodePointMap::default();

        assert!(upm.is_empty());
        assert_eq!(0, upm.len());
        assert_eq!(0, upm.total());

        assert_eq!(0, upm[&'\0']);
        assert_eq!(0, upm[&'a']);
        assert_eq!(0, upm[&'b']);
        assert_eq!(0, upm[&'c']);
        assert_eq!(0, upm[&'0']);
        assert_eq!(0, upm[&'A']);
        assert_eq!(0, upm[&'🐻']);
        assert_eq!(0, upm[&'🐼']);

        assert!(!upm.contains_key(&'\0'));
        assert!(!upm.contains_key(&'a'));
        assert!(!upm.contains_key(&'b'));
        assert!(!upm.contains_key(&'c'));
        assert!(!upm.contains_key(&'0'));
        assert!(!upm.contains_key(&'A'));
        assert!(!upm.contains_key(&'🐻'));
        assert!(!upm.contains_key(&'🐼'));

        upm.push('a');
        upm.push('b');
        upm.push('a');

        assert!(!upm.is_empty());
        assert_eq!(2, upm.len());
        assert_eq!(3, upm.total());

        assert_eq!(0, upm[&'\0']);
        assert_eq!(2, upm[&'a']);
        assert_eq!(1, upm[&'b']);
        assert_eq!(0, upm[&'c']);
        assert_eq!(0, upm[&'0']);
        assert_eq!(0, upm[&'A']);
        assert_eq!(0, upm[&'🐻']);
        assert_eq!(0, upm[&'🐼']);

        assert!(!upm.contains_key(&'\0'));
        assert!(upm.contains_key(&'a'));
        assert!(upm.contains_key(&'b'));
        assert!(!upm.contains_key(&'c'));
        assert!(!upm.contains_key(&'0'));
        assert!(!upm.contains_key(&'A'));
        assert!(!upm.contains_key(&'🐻'));
        assert!(!upm.contains_key(&'🐼'));

        upm.push('🐻');
        upm.push('a');
        upm.push('🐻');

        assert!(!upm.is_empty());
        assert_eq!(3, upm.len());
        assert_eq!(6, upm.total());

        assert_eq!(0, upm['\0']);
        assert_eq!(3, upm['a']);
        assert_eq!(1, upm['b']);
        assert_eq!(0, upm['c']);
        assert_eq!(0, upm['0']);
        assert_eq!(0, upm['A']);
        assert_eq!(2, upm['🐻']);
        assert_eq!(0, upm['🐼']);

        assert!(!upm.contains_key(&'\0'));
        assert!(upm.contains_key(&'a'));
        assert!(upm.contains_key(&'b'));
        assert!(!upm.contains_key(&'c'));
        assert!(!upm.contains_key(&'0'));
        assert!(!upm.contains_key(&'A'));
        assert!(upm.contains_key(&'🐻'));
        assert!(!upm.contains_key(&'🐼'));

        upm.clear();

        assert!(upm.is_empty());
        assert_eq!(0, upm.len());
        assert_eq!(0, upm.total());

        assert_eq!(0, upm['\0']);
        assert_eq!(0, upm['a']);
        assert_eq!(0, upm['b']);
        assert_eq!(0, upm['c']);
        assert_eq!(0, upm['0']);
        assert_eq!(0, upm['A']);
        assert_eq!(0, upm['🐻']);
        assert_eq!(0, upm['🐼']);

        assert!(!upm.contains_key(&'\0'));
        assert!(!upm.contains_key(&'a'));
        assert!(!upm.contains_key(&'b'));
        assert!(!upm.contains_key(&'c'));
        assert!(!upm.contains_key(&'0'));
        assert!(!upm.contains_key(&'A'));
        assert!(!upm.contains_key(&'🐻'));
        assert!(!upm.contains_key(&'🐼'));
    }

    #[test]
    fn TEST_push_n() {
        let mut upm = UnicodePointMap::default();

        assert!(upm.is_empty());
        assert_eq!(0, upm.len());
        assert_eq!(0, upm.total());

        assert_eq!(0, upm[&'\0']);
        assert_eq!(0, upm[&'a']);
        assert_eq!(0, upm[&'b']);
        assert_eq!(0, upm[&'c']);
        assert_eq!(0, upm[&'0']);
        assert_eq!(0, upm[&'A']);
        assert_eq!(0, upm[&'🐻']);
        assert_eq!(0, upm[&'🐼']);

        upm.push_n('a', 2);
        upm.push_n('b', 1);
        upm.push_n('c', 0);
        upm.push_n('d', 1);
        upm.push_n('d', -1);

        assert!(!upm.is_empty());
        assert_eq!(2, upm.len());
        assert_eq!(3, upm.total());

        assert_eq!(0, upm[&'\0']);
        assert_eq!(2, upm[&'a']);
        assert_eq!(1, upm[&'b']);
        assert_eq!(0, upm[&'c']);
        assert_eq!(0, upm[&'0']);
        assert_eq!(0, upm[&'A']);
        assert_eq!(0, upm[&'🐻']);
        assert_eq!(0, upm[&'🐼']);

        upm.push_n('🐻', 2);
        upm.push_n('a', 1);

        assert!(!upm.is_empty());
        assert_eq!(3, upm.len());
        assert_eq!(6, upm.total());

        assert_eq!(0, upm['\0']);
        assert_eq!(3, upm['a']);
        assert_eq!(1, upm['b']);
        assert_eq!(0, upm['c']);
        assert_eq!(0, upm['0']);
        assert_eq!(0, upm['A']);
        assert_eq!(2, upm['🐻']);
        assert_eq!(0, upm['🐼']);
    }

    #[test]
    fn TEST_From_KEYS_1() {
        let upm = UnicodePointMap::from([
            // insert list
            'a', 'b', 'c', 'd', 'a', 'f', '0', '1', '🐻',
        ]);

        assert!(!upm.is_empty());
        assert_eq!(8, upm.len());
        assert_eq!(9, upm.total());

        assert_eq!(0, upm['\0']);
        assert_eq!(2, upm['a']);
        assert_eq!(1, upm['b']);
        assert_eq!(1, upm['c']);
        assert_eq!(1, upm['d']);
        assert_eq!(0, upm['e']);
        assert_eq!(1, upm['f']);
        assert_eq!(0, upm['g']);
        assert_eq!(1, upm['0']);
        assert_eq!(1, upm['1']);
        assert_eq!(0, upm['2']);
        assert_eq!(0, upm['A']);
        assert_eq!(1, upm['🐻']);
        assert_eq!(0, upm['🐼']);
    }

    #[test]
    fn TEST_From_PAIRS_1() {
        let upm = UnicodePointMap::from([
            // insert list
            ('a', 2),
            ('b', 1),
            ('c', 1),
            ('d', 1),
            ('f', 1),
            ('0', 1),
            ('1', 1),
            ('🐻', 1),
        ]);

        assert!(!upm.is_empty());
        assert_eq!(8, upm.len());
        assert_eq!(9, upm.total());

        assert_eq!(0, upm['\0']);
        assert_eq!(2, upm['a']);
        assert_eq!(1, upm['b']);
        assert_eq!(1, upm['c']);
        assert_eq!(1, upm['d']);
        assert_eq!(0, upm['e']);
        assert_eq!(1, upm['f']);
        assert_eq!(0, upm['g']);
        assert_eq!(1, upm['0']);
        assert_eq!(1, upm['1']);
        assert_eq!(0, upm['2']);
        assert_eq!(0, upm['A']);
        assert_eq!(1, upm['🐻']);
        assert_eq!(0, upm['🐼']);
    }

    #[test]
    fn TEST_FromIterator_KEYS_1() {
        let iter = ('a'..='f').into_iter().chain('d'..='h').into_iter();

        let upm = UnicodePointMap::from_iter(iter);

        assert!(!upm.is_empty());
        assert_eq!(8, upm.len());
        assert_eq!(11, upm.total());

        assert_eq!(0, upm['\0']);
        assert_eq!(1, upm['a']);
        assert_eq!(1, upm['b']);
        assert_eq!(1, upm['c']);
        assert_eq!(2, upm['d']);
        assert_eq!(2, upm['e']);
        assert_eq!(2, upm['f']);
        assert_eq!(1, upm['g']);
        assert_eq!(1, upm['h']);
        assert_eq!(0, upm['i']);
        assert_eq!(0, upm['0']);
        assert_eq!(0, upm['1']);
        assert_eq!(0, upm['2']);
        assert_eq!(0, upm['A']);
        assert_eq!(0, upm['🐻']);
        assert_eq!(0, upm['🐼']);
    }

    #[test]
    fn TEST_FromIterator_PAIR0S_1() {
        let v = vec![
            // insert list
            ('a', 1),
            ('b', 1),
            ('c', 1),
            ('d', 2),
            ('e', 2),
            ('f', 2),
            ('g', 1),
            ('h', 1),
            ('i', 0),
            ('j', 0),
            ('k', 0),
        ];

        let iter = v.into_iter();

        let upm = UnicodePointMap::from_iter(iter);

        assert!(!upm.is_empty());
        assert_eq!(8, upm.len());
        assert_eq!(11, upm.total());

        assert_eq!(0, upm['\0']);
        assert_eq!(1, upm['a']);
        assert_eq!(1, upm['b']);
        assert_eq!(1, upm['c']);
        assert_eq!(2, upm['d']);
        assert_eq!(2, upm['e']);
        assert_eq!(2, upm['f']);
        assert_eq!(1, upm['g']);
        assert_eq!(1, upm['h']);
        assert_eq!(0, upm['i']);
        assert_eq!(0, upm['0']);
        assert_eq!(0, upm['1']);
        assert_eq!(0, upm['2']);
        assert_eq!(0, upm['A']);
        assert_eq!(0, upm['🐻']);
        assert_eq!(0, upm['🐼']);
    }

    #[test]
    fn TEST_remove_1() {
        let mut upm = UnicodePointMap::new('\u{80}');

        upm.push_n('a', 2);
        upm.push_n('b', 1);
        upm.push_n('c', 0);
        upm.push_n('d', 1);
        upm.push_n('d', -1);
        upm.push_n('🐻', 2);
        upm.push_n('a', 1);

        assert!(!upm.is_empty());
        assert_eq!(3, upm.len());
        assert_eq!(6, upm.total());

        assert_eq!(0, upm['\0']);
        assert_eq!(3, upm['a']);
        assert_eq!(1, upm['b']);
        assert_eq!(0, upm['c']);
        assert_eq!(0, upm['0']);
        assert_eq!(0, upm['A']);
        assert_eq!(2, upm['🐻']);
        assert_eq!(0, upm['🐼']);

        {
            let r = upm.remove(&'a');

            assert_eq!(Some(3), r);

            assert!(!upm.is_empty());
            assert_eq!(2, upm.len());
            assert_eq!(3, upm.total());
        }

        {
            let r = upm.remove(&'🐼');

            assert_eq!(None, r);

            assert!(!upm.is_empty());
            assert_eq!(2, upm.len());
            assert_eq!(3, upm.total());
        }

        {
            let r = upm.remove(&'🐻');

            assert_eq!(Some(2), r);

            assert!(!upm.is_empty());
            assert_eq!(1, upm.len());
            assert_eq!(1, upm.total());
        }

        {
            let r = upm.remove(&'b');

            assert_eq!(Some(1), r);

            assert!(upm.is_empty());
            assert_eq!(0, upm.len());
            assert_eq!(0, upm.total());
        }
    }

    #[test]
    fn TEST_insert_1() {
        let mut upm = UnicodePointMap::from([
            // insert list
            ('a', 2),
            ('b', 1),
            ('c', 1),
            ('d', 1),
            ('f', 1),
            ('0', 1),
            ('1', 1),
            ('🐻', 1),
        ]);

        assert!(!upm.is_empty());
        assert_eq!(8, upm.len());
        assert_eq!(9, upm.total());

        assert_eq!(0, upm['\0']);
        assert_eq!(2, upm['a']);
        assert_eq!(1, upm['b']);
        assert_eq!(1, upm['c']);
        assert_eq!(1, upm['d']);
        assert_eq!(0, upm['e']);
        assert_eq!(1, upm['f']);
        assert_eq!(0, upm['g']);
        assert_eq!(0, upm['x']);
        assert_eq!(0, upm['y']);
        assert_eq!(0, upm['z']);
        assert_eq!(1, upm['0']);
        assert_eq!(1, upm['1']);
        assert_eq!(0, upm['2']);
        assert_eq!(0, upm['A']);
        assert_eq!(1, upm['🐻']);
        assert_eq!(0, upm['🐼']);

        {
            let r = upm.insert('z', 0);

            assert_eq!(None, r);

            assert!(!upm.is_empty());
            assert_eq!(8, upm.len());
            assert_eq!(9, upm.total());

            assert_eq!(0, upm['\0']);
            assert_eq!(2, upm['a']);
            assert_eq!(1, upm['b']);
            assert_eq!(1, upm['c']);
            assert_eq!(1, upm['d']);
            assert_eq!(0, upm['e']);
            assert_eq!(1, upm['f']);
            assert_eq!(0, upm['g']);
            assert_eq!(0, upm['x']);
            assert_eq!(0, upm['y']);
            assert_eq!(0, upm['z']);
            assert_eq!(1, upm['0']);
            assert_eq!(1, upm['1']);
            assert_eq!(0, upm['2']);
            assert_eq!(0, upm['A']);
            assert_eq!(1, upm['🐻']);
            assert_eq!(0, upm['🐼']);
        }

        {
            let r = upm.insert('z', -2);

            assert_eq!(None, r);

            assert!(!upm.is_empty());
            assert_eq!(9, upm.len());
            assert_eq!(7, upm.total());

            assert_eq!(0, upm['\0']);
            assert_eq!(2, upm['a']);
            assert_eq!(1, upm['b']);
            assert_eq!(1, upm['c']);
            assert_eq!(1, upm['d']);
            assert_eq!(0, upm['e']);
            assert_eq!(1, upm['f']);
            assert_eq!(0, upm['g']);
            assert_eq!(0, upm['x']);
            assert_eq!(0, upm['y']);
            assert_eq!(-2, upm['z']);
            assert_eq!(1, upm['0']);
            assert_eq!(1, upm['1']);
            assert_eq!(0, upm['2']);
            assert_eq!(0, upm['A']);
            assert_eq!(1, upm['🐻']);
            assert_eq!(0, upm['🐼']);
        }

        {
            let r= upm.insert('a', 0);

            assert_eq!(Some(2), r);

            assert!(!upm.is_empty());
            assert_eq!(8, upm.len());
            assert_eq!(5, upm.total());

            assert_eq!(0, upm['\0']);
            assert_eq!(0, upm['a']);
            assert_eq!(1, upm['b']);
            assert_eq!(1, upm['c']);
            assert_eq!(1, upm['d']);
            assert_eq!(0, upm['e']);
            assert_eq!(1, upm['f']);
            assert_eq!(0, upm['g']);
            assert_eq!(0, upm['x']);
            assert_eq!(0, upm['y']);
            assert_eq!(-2, upm['z']);
            assert_eq!(1, upm['0']);
            assert_eq!(1, upm['1']);
            assert_eq!(0, upm['2']);
            assert_eq!(0, upm['A']);
            assert_eq!(1, upm['🐻']);
            assert_eq!(0, upm['🐼']);
        }

        {
            let r= upm.insert('b', 0);

            assert_eq!(Some(1), r);

            assert!(!upm.is_empty());
            assert_eq!(7, upm.len());
            assert_eq!(4, upm.total());

            assert_eq!(0, upm['\0']);
            assert_eq!(0, upm['a']);
            assert_eq!(0, upm['b']);
            assert_eq!(1, upm['c']);
            assert_eq!(1, upm['d']);
            assert_eq!(0, upm['e']);
            assert_eq!(1, upm['f']);
            assert_eq!(0, upm['g']);
            assert_eq!(0, upm['x']);
            assert_eq!(0, upm['y']);
            assert_eq!(-2, upm['z']);
            assert_eq!(1, upm['0']);
            assert_eq!(1, upm['1']);
            assert_eq!(0, upm['2']);
            assert_eq!(0, upm['A']);
            assert_eq!(1, upm['🐻']);
            assert_eq!(0, upm['🐼']);
        }

        {
            let r= upm.insert('🐻', 1);

            assert_eq!(Some(1), r);

            assert!(!upm.is_empty());
            assert_eq!(7, upm.len());
            assert_eq!(4, upm.total());

            assert_eq!(0, upm['\0']);
            assert_eq!(0, upm['a']);
            assert_eq!(0, upm['b']);
            assert_eq!(1, upm['c']);
            assert_eq!(1, upm['d']);
            assert_eq!(0, upm['e']);
            assert_eq!(1, upm['f']);
            assert_eq!(0, upm['g']);
            assert_eq!(0, upm['x']);
            assert_eq!(0, upm['y']);
            assert_eq!(-2, upm['z']);
            assert_eq!(1, upm['0']);
            assert_eq!(1, upm['1']);
            assert_eq!(0, upm['2']);
            assert_eq!(0, upm['A']);
            assert_eq!(1, upm['🐻']);
            assert_eq!(0, upm['🐼']);
        }

        {
            let r= upm.insert('🐻', -1);

            assert_eq!(Some(1), r);

            assert!(!upm.is_empty());
            assert_eq!(7, upm.len());
            assert_eq!(2, upm.total());

            assert_eq!(0, upm['\0']);
            assert_eq!(0, upm['a']);
            assert_eq!(0, upm['b']);
            assert_eq!(1, upm['c']);
            assert_eq!(1, upm['d']);
            assert_eq!(0, upm['e']);
            assert_eq!(1, upm['f']);
            assert_eq!(0, upm['g']);
            assert_eq!(0, upm['x']);
            assert_eq!(0, upm['y']);
            assert_eq!(-2, upm['z']);
            assert_eq!(1, upm['0']);
            assert_eq!(1, upm['1']);
            assert_eq!(0, upm['2']);
            assert_eq!(0, upm['A']);
            assert_eq!(-1, upm['🐻']);
            assert_eq!(0, upm['🐼']);
        }
    }

    #[test]
    fn TEST_UnicodePointMap_EXAMPLE_1() {
        let upm = UnicodePointMap::from_iter("The quick brown fox jumps over the lazy dog".chars().into_iter());

        assert_eq!(1, upm['a']);
        assert_eq!(1, upm['b']);
        assert_eq!(1, upm['c']);
        assert_eq!(1, upm['d']);
        assert_eq!(3, upm['e']);
        assert_eq!(1, upm['f']);
        assert_eq!(1, upm['g']);
        assert_eq!(2, upm['h']);
        assert_eq!(1, upm['i']);
        assert_eq!(1, upm['j']);
        assert_eq!(1, upm['k']);
        assert_eq!(1, upm['l']);
        assert_eq!(1, upm['m']);
        assert_eq!(1, upm['n']);
        assert_eq!(4, upm['o']);
        assert_eq!(1, upm['p']);
        assert_eq!(1, upm['q']);
        assert_eq!(2, upm['r']);
        assert_eq!(1, upm['s']);
        assert_eq!(1, upm['t']);
        assert_eq!(2, upm['u']);
        assert_eq!(1, upm['v']);
        assert_eq!(1, upm['w']);
        assert_eq!(1, upm['x']);
        assert_eq!(1, upm['y']);
        assert_eq!(1, upm['z']);
        assert_eq!(8, upm[' ']);
        assert_eq!(1, upm['T']);

        assert_eq!(0, upm['0']);
        assert_eq!(0, upm['-']);
        assert_eq!(0, upm['_']);
        assert_eq!(0, upm['.']);
        assert_eq!(0, upm[',']);
    }

    #[test]
    fn TEST_UnicodePointMap_fuse_1() {

        // empty
        {
            let upm = UnicodePointMap::default();

            let mut iter = upm.iter();

            assert_eq!(None, iter.next());
            assert_eq!(None, iter.next());
            assert_eq!(None, iter.next());
            assert_eq!(None, iter.next());
            assert_eq!(None, iter.next());
            assert_eq!(None, iter.next());
        }

        // empty fused
        {
            let upm = UnicodePointMap::default();

            let mut iter = upm.iter().fuse();

            assert_eq!(None, iter.next());
            assert_eq!(None, iter.next());
            assert_eq!(None, iter.next());
            assert_eq!(None, iter.next());
            assert_eq!(None, iter.next());
            assert_eq!(None, iter.next());
        }

        // non-empty
        {
            let upm = UnicodePointMap::from(['b', 'a', 'a', 'f']);

            let mut iter = upm.iter();

            assert_eq!(Some(('a', 2)), iter.next());
            assert_eq!(Some(('b', 1)), iter.next());
            assert_eq!(Some(('f', 1)), iter.next());
            assert_eq!(None, iter.next());
            assert_eq!(None, iter.next());
            assert_eq!(None, iter.next());
        }

        // non-empty fused
        {
            let upm = UnicodePointMap::from(['b', 'a', 'a', 'f']);

            let mut iter = upm.iter().fuse();

            assert_eq!(Some(('a', 2)), iter.next());
            assert_eq!(Some(('b', 1)), iter.next());
            assert_eq!(Some(('f', 1)), iter.next());
            assert_eq!(None, iter.next());
            assert_eq!(None, iter.next());
            assert_eq!(None, iter.next());
        }
    }
}


// ///////////////////////////// end of file //////////////////////////// //

