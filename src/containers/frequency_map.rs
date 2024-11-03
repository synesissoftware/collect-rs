// containers/frequency_map.rs : `FrequencyMap`

use base_traits::{
    IsEmpty,
    Len,
};

use std::{
    borrow as std_borrow,
    cmp as std_cmp,
    collections::HashMap,
    hash as std_hash,
    ops as std_ops,
};


/// A container that measures the frequencies of the unique elements it
/// contains.
///
/// # See:
/// - `stlsoft::frequency_map<>` - C++ class template in [**STLSoft**](https://github.com/synesissoftware/STLSoft-1.11);
/// - `Xqsr3::Containers::FrequencyMap` - Ruby class in [**xqsr3**](https://github.com/synesissoftware/xqsr3);
#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
pub struct FrequencyMap<K> {
    /// The map of keys and counts.
    map : HashMap<K, isize>,
    /// The total number of keys represented.
    total : isize,
}

// API functions

impl<K> FrequencyMap<K> {
    /// Creates a new empty instance.
    pub fn new() -> Self {
        let map = HashMap::new();
        let total = 0;

        Self {
            map,
            total,
        }
    }

    /// Creates a new empty instance with at least the given `capacity`.
    pub fn with_capacity(capacity: usize) -> Self {
        let map = HashMap::with_capacity(capacity);
        let total = 0;

        Self {
            map,
            total,
        }
    }
}

// Mutating methods

impl<K> FrequencyMap<K> {
    /// Clears the map, removing all records and resets `#total()`.
    #[inline]
    pub fn clear(&mut self) {
        self.map.clear();
        self.total = 0;
    }
}

impl<K: std_cmp::Eq + std_hash::Hash> FrequencyMap<K> {
    /// Moves all the records of `other` into `self`, creating or updating
    /// records as appropriate, leaving other `empty`.
    pub fn append(
        &mut self,
        other: &mut Self,
    ) {
        // benchmarking demonstrates that the `#push_n()` variant is
        // superior (at least as far as integer keys is concerned)

        #[cfg(feature = "_NEVER_TO_BE_ENABLED")]
        {
            let other_total = other.total();

            for (key, count) in other.drain() {
                self.map.entry(key).and_modify(|v| *v += count).or_insert(count);
            }

            self.map.retain(|_k, count| *count != 0);

            self.total += other_total;
        }
        #[cfg(not(feature = "_NEVER_TO_BE_ENABLED"))]
        {
            for (key, count) in other.drain() {
                self.push_n(key, count);
            }
        }
    }

    /// Clears the map, returning all key-count pairs as an iterator. Keeps
    /// the allocated memory for reuse.
    #[inline]
    pub fn drain(&mut self) -> std::collections::hash_map::Drain<'_, K, isize> {
        self.total = 0;

        self.map.drain()
    }

    /// Inserts a record with the given `key` and `count`, replacing any
    /// existing record with that `key`. In the case that `count` is 0 any
    /// existing record is removed and no record is created.
    #[inline]
    pub fn insert(
        &mut self,
        key: K,
        count: isize,
    ) -> Option<isize>
    {
        if let Some(v) = self.map.get_mut(&key) {
            self.total -= *v;

            let r = Some(*v);

            if 0 == count {
                self.map.remove(&key);
            } else {
                *v = count;
            }

            self.total += count;

            r
        } else {
            if 0 != count {
                self.map.insert(key, count);

                self.total += count;
            }

            None
        }
    }

    // pub fn merge(&mut self, )

    /// Updates the count by 1 of an existing record identified by `key`, or
    /// creates, with a count of 1, a new record.
    ///
    /// In the case that the resulting count of an existing record is 0 then
    /// the record is removed.
    pub fn push(
        &mut self,
        key : K,
    ) {
        self.map.entry(key).and_modify(|v| *v += 1).or_insert(1);

        self.total += 1;
    }

    /// Updates the count by `count` of an existing record identifed by
    /// `key`, or creates, with the given `count`, a new record.
    ///
    /// In the case that the resulting count of an existing record is 0 then
    /// the record is removed.
    pub fn push_n(
        &mut self,
        key : K,
        count: isize,
    ) {
        // Alas, we cannot use `entry()` because we need to be able to
        // delete the key in the case that `count` + the existing count
        // sums to 0

        self.total += Self::push_into_map_(&mut self.map, key, count);
    }

    /// Removes a key from the map, returning the count of the key if the
    /// key was previously in the map.
    #[inline]
    pub fn remove<Q>(
        &mut self,
        key: &Q,
    ) -> Option<isize>
    where
        K: std_borrow::Borrow<Q>,
        Q: std_hash::Hash + std_cmp::Eq + ?Sized,
    {
        let r = self.map.remove(key);

        if let Some(v) = &r {
            self.total -= v;
        }

        r
    }

    /// Removes a key from the map, returning the stored key and count if
    /// the key was previously in the map.
    pub fn remove_entry<Q>(
        &mut self,
        key: &Q,
    ) -> Option<(K, isize)>
    where
        K: std_borrow::Borrow<Q>,
        Q: std_hash::Hash + std_cmp::Eq + ?Sized,
    {
        let r = self.map.remove_entry(key);

        if let Some((_, v)) = &r {
            self.total -= v;
        };

        r
    }

    /// Reserves capacity for at least `additional` more records to be
    /// inserted in the instance.
    #[inline]
    pub fn reserve(
        &mut self,
        additional: usize,
    ) {
        self.map.reserve(additional);
    }

    /// Shrinks the capacity of the map with a lower limit.
    #[inline]
    pub fn shrink_to(
        &mut self,
        min_capacity: usize,
    ) {
        self.map.shrink_to(min_capacity)
    }

    /// Shrinks the capacity of the map as much as possible.
    #[inline]
    pub fn shrink_to_fit(&mut self) {
        self.map.shrink_to_fit()
    }
}

impl<K> FrequencyMap<K> {
    /// Retains only the records specified by the predicate.
    #[inline]
    pub fn retain<F>(
        &mut self,
        f: F,
    )
    where
        F: Fn(&K, isize) -> bool,
    {
        let mut dropped_total = 0;

        self.map.retain(|key, count| {
            if f(key, *count) {
                true
            } else {
                dropped_total += *count;

                false
            }
        });

        self.total -= dropped_total;
    }
}

// Non-mutating methods

impl<K : std_cmp::Eq + std_hash::Hash> FrequencyMap<K> {
    /// Indicates whether a record exists for the given `key`.
    #[inline]
    pub fn contains_key<Q>(
        &self,
        key: &Q,
    ) -> bool
    where
        K: std_borrow::Borrow<Q>,
        Q: std_hash::Hash + std_cmp::Eq + ?Sized,
    {
        self.map.contains_key(key)
    }

    /// Obtains the count corresponding to the given `key`, obtaining 0 in
    /// the case that no such record exists.
    #[inline]
    pub fn get<Q>(
        &self,
        key: &Q,
    ) -> isize
    where
        K: std_borrow::Borrow<Q>,
        Q: std_hash::Hash + std_cmp::Eq + ?Sized,
    {
        *self.get_(key)
    }
}

impl<K> FrequencyMap<K> {
    /// An iterator visiting all key-count pairs in arbitrary order. The
    /// iterator element type is `(&'a K, &'a isize)`.
    #[inline]
    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, K, isize> {
        self.map.iter()
    }
}

impl<K> FrequencyMap<K> {
    /// Returns the number of records the map can hold without
    /// reallocation.
    #[inline]
    pub fn capacity(&self) -> usize {
        self.map.capacity()
    }
}

impl<K> FrequencyMap<K> {
    /// Indicates whether the instance contains no records.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.is_empty_()
    }

    /// Obtains the number of records.
    #[inline]
    pub fn len(&self) -> usize {
        self.len_()
    }

    /// Indicates the total frequency count across all records.
    #[inline]
    pub fn total(&self) -> isize {
        self.total_()
    }
}

// Implementation

impl<K : std_cmp::Eq + std_hash::Hash> FrequencyMap<K> {
    fn get_<Q>(
        &self,
        key: &Q,
    ) -> &isize
    where
        K: std_borrow::Borrow<Q>,
        Q: std_hash::Hash + std_cmp::Eq + ?Sized,
    {
        match self.map.get(key) {
            Some(count) => count,
            None => &0,
        }
    }

    fn push_into_map_(
        map : &mut HashMap<K, isize>,
        key : K,
        count: isize,
    ) -> isize /* total_delta */ {
        if 0 == count {
            0
        } else {
            if let Some(v) = map.get_mut(&key) {

                *v += count;

                if 0 == *v {
                    map.remove(&key);
                }
            } else {

                map.insert(key, count);
            }

            count
        }
    }
}

impl<K> FrequencyMap<K> {
    #[inline]
    fn is_empty_(&self) -> bool {
        0 == self.total
    }

    #[inline]
    fn len_(&self) -> usize {
        self.map.len()
    }

    #[inline]
    fn total_(&self) -> isize {
        self.total
    }
}

// Trait implementations

impl<K: std_cmp::Eq + std_hash::Hash> FromIterator<K> for FrequencyMap<K> {
    /// Creates an instance from an iterator of keys.
    fn from_iter<T: IntoIterator<Item = K>>(iter: T) -> Self {
        let iter = iter.into_iter();
        let (min_size, max_size) = iter.size_hint();
        let capacity = match max_size {
            Some(max_size) => max_size,
            None => min_size,
        };

        let mut map = HashMap::with_capacity(capacity);
        let mut total = 0;

        for key in iter {
            map.entry(key).and_modify(|v| *v += 1).or_insert(1);
            total += 1;
        }

        Self {
            map,
            total,
        }
    }
}

impl<K: std_cmp::Eq + std_hash::Hash, const N : usize> From<[(K, isize); N]> for FrequencyMap<K> {
    /// Creates an instance from an array of key + count pairs.
    fn from(value: [(K, isize); N]) -> Self {
        let mut map = HashMap::with_capacity(N);
        let mut total = 0;

        for (key, count) in value {
            total += Self::push_into_map_(&mut map, key, count);
        }

        Self {
            map,
            total,
        }
    }
}

impl<K: std_cmp::Eq + std_hash::Hash, const N : usize> From<[K; N]> for FrequencyMap<K> {
    /// Creates an instance from an array of keys.
    fn from(value: [K; N]) -> Self {
        let mut map = HashMap::with_capacity(N);
        let total = N as isize;

        for key in value {
            map.entry(key).and_modify(|v| *v += 1).or_insert(1);
        }

        Self {
            map,
            total,
        }
    }
}

impl<K, Q> std_ops::Index<&Q> for FrequencyMap<K>
where
    K: std_cmp::Eq + std_hash::Hash + std_borrow::Borrow<Q>,
    Q: std_cmp::Eq + std_hash::Hash + ?Sized,
{
    type Output = isize;

    #[inline]
    fn index(
        &self,
        key : &Q
    ) -> &Self::Output {
        self.get_(key)
    }
}

impl<K> IsEmpty for FrequencyMap<K> {
    /// Indicates whether the instance contains no records.
    #[inline]
    fn is_empty(&self) -> bool {
        self.is_empty_()
    }
}

impl<K> Len for FrequencyMap<K> {
    /// Obtains the number of records.
    #[inline]
    fn len(&self) -> usize {
        self.len_()
    }
}


#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::FrequencyMap;

    use std::collections::HashMap;


    #[test]
    fn TEST_FrequencyMap_Default() {

        let fm = FrequencyMap::<i32>::default();

        assert!(fm.is_empty());
        assert_eq!(0, fm.len());
        assert_eq!(0, fm.total());

        assert_eq!(0, fm.get(&0));
        assert_eq!(0, fm.get(&10));
        assert_eq!(0, fm.get(&100));
        assert_eq!(0, fm.get(&101));
        assert_eq!(0, fm.get(&102));

        {
            let ie : &dyn base_traits::IsEmpty = &fm;

            assert!(ie.is_empty());
        }

        {
            let l : &dyn base_traits::Len = &fm;

            assert_eq!(0, l.len());
        }
}

    #[test]
    fn TEST_FrequencyMap_new() {

        let fm = FrequencyMap::<i32>::new();

        assert!(fm.is_empty());
        assert_eq!(0, fm.len());
        assert_eq!(0, fm.total());

        assert_eq!(0, fm.get(&0));
        assert_eq!(0, fm.get(&10));
        assert_eq!(0, fm.get(&100));
        assert_eq!(0, fm.get(&101));
        assert_eq!(0, fm.get(&102));
    }

    #[test]
    fn TEST_FrequencyMap_with_capacity_1() {

        let mut fm = FrequencyMap::<i32>::with_capacity(1000);

        assert!(fm.is_empty());
        assert!(fm.capacity() >= 1000);
        assert_eq!(0, fm.len());
        assert_eq!(0, fm.total());

        assert_eq!(0, fm.get(&0));
        assert_eq!(0, fm.get(&10));
        assert_eq!(0, fm.get(&100));
        assert_eq!(0, fm.get(&101));
        assert_eq!(0, fm.get(&102));

        fm.shrink_to(500);

        assert!(fm.capacity() >= 500 && fm.capacity() <= 1000, "capacity is {} but should be in range [500, 1000]", fm.capacity());

        fm.shrink_to_fit();

        assert!(fm.capacity() < 100, "capacity is {} but should be in 0 or close to it", fm.capacity());
    }

    #[test]
    fn TEST_FrequencyMap_reserve_1() {

        let mut fm = FrequencyMap::<i32>::new();

        fm.reserve(1000);

        assert!(fm.is_empty());
        assert!(fm.capacity() >= 1000);
        assert_eq!(0, fm.len());
        assert_eq!(0, fm.total());

        assert_eq!(0, fm.get(&0));
        assert_eq!(0, fm.get(&10));
        assert_eq!(0, fm.get(&100));
        assert_eq!(0, fm.get(&101));
        assert_eq!(0, fm.get(&102));

        fm.shrink_to(500);

        fm.reserve(100);

        assert!(fm.capacity() >= 500 && fm.capacity() <= 1000, "capacity is {} but should be in range [500, 1000]", fm.capacity());

        fm.shrink_to_fit();

        assert!(fm.capacity() < 100, "capacity is {} but should be in 0 or close to it", fm.capacity());
    }

    #[test]
    fn TEST_FrequencyMap_From_KEYS_1() {

        let ar : [i32; 0] = [];
        let fm = FrequencyMap::<i32>::from(ar);

        assert!(fm.is_empty());
        assert_eq!(0, fm.len());
        assert_eq!(0, fm.total());

        assert_eq!(0, fm.get(&0));
        assert_eq!(0, fm.get(&10));
        assert_eq!(0, fm.get(&100));
        assert_eq!(0, fm.get(&101));
        assert_eq!(0, fm.get(&102));
    }

    #[test]
    fn TEST_FrequencyMap_From_KEYS_2() {

        let fm = FrequencyMap::<i32>::from([
            // insert list
            2,
            17,
            123,
            123,
        ]);

        assert!(!fm.is_empty());
        assert_eq!(3, fm.len());
        assert_eq!(4, fm.total());

        assert_eq!(0, fm.get(&0));
        assert_eq!(0, fm.get(&1));
        assert_eq!(1, fm.get(&2));
        assert_eq!(0, fm.get(&3));
        assert_eq!(0, fm.get(&10));
        assert_eq!(1, fm.get(&17));
        assert_eq!(0, fm.get(&100));
        assert_eq!(2, fm.get(&123));
        assert_eq!(0, fm.get(&101));
        assert_eq!(0, fm.get(&102));
    }

    #[test]
    fn TEST_FrequencyMap_From_PAIRS_1() {

        let ar : [(i32, isize); 0] = [];
        let fm = FrequencyMap::<i32>::from(ar);

        assert!(fm.is_empty());
        assert_eq!(0, fm.len());
        assert_eq!(0, fm.total());

        assert_eq!(0, fm.get(&0));
        assert_eq!(0, fm.get(&10));
        assert_eq!(0, fm.get(&100));
        assert_eq!(0, fm.get(&101));
        assert_eq!(0, fm.get(&102));
    }

    #[test]
    fn TEST_FrequencyMap_From_PAIRS_2() {

        let fm = FrequencyMap::<i32>::from([
            // insert list
            (2, 0),
            (2, 1),
            (3, 0),
            (17, 1),
            (123, 2),
        ]);

        assert!(!fm.is_empty());
        assert_eq!(3, fm.len());
        assert_eq!(4, fm.total());

        assert_eq!(0, fm.get(&0));
        assert_eq!(0, fm.get(&1));
        assert_eq!(1, fm.get(&2));
        assert_eq!(0, fm.get(&3));
        assert_eq!(0, fm.get(&10));
        assert_eq!(1, fm.get(&17));
        assert_eq!(0, fm.get(&100));
        assert_eq!(2, fm.get(&123));
        assert_eq!(0, fm.get(&101));
        assert_eq!(0, fm.get(&102));
    }

    #[test]
    fn TEST_FrequencyMap_From_PAIRS_3() {

        let fm = FrequencyMap::<i32>::from([
            // insert list
            (2, 0),
            (2, 1),
            (3, 0),
            (4, 1),
            (4, -1),
            (17, 1),
            (123, 2),
        ]);

        assert!(!fm.is_empty());
        assert_eq!(3, fm.len());
        assert_eq!(4, fm.total());

        assert!(!fm.contains_key(&0));
        assert!(!fm.contains_key(&1));
        assert!(fm.contains_key(&2));
        assert!(!fm.contains_key(&3));
        assert!(!fm.contains_key(&4));
        assert!(!fm.contains_key(&5));
        assert!(!fm.contains_key(&15));
        assert!(!fm.contains_key(&16));
        assert!(fm.contains_key(&17));
        assert!(!fm.contains_key(&18));
        assert!(!fm.contains_key(&122));
        assert!(fm.contains_key(&123));
        assert!(!fm.contains_key(&124));

        assert_eq!(0, fm.get(&0));
        assert_eq!(0, fm.get(&1));
        assert_eq!(1, fm.get(&2));
        assert_eq!(0, fm.get(&3));
        assert_eq!(0, fm.get(&4));
        assert_eq!(0, fm.get(&10));
        assert_eq!(1, fm.get(&17));
        assert_eq!(0, fm.get(&100));
        assert_eq!(2, fm.get(&123));
        assert_eq!(0, fm.get(&101));
        assert_eq!(0, fm.get(&102));
    }

    #[test]
    fn TEST_FrequencyMap_FromIterator_1() {

        let iter = (0..0).into_iter();

        let fm = FrequencyMap::<i32>::from_iter(iter);

        assert!(fm.is_empty());
        assert_eq!(0, fm.len());
        assert_eq!(0, fm.total());

        assert_eq!(0, fm[&0]);
        assert_eq!(0, fm[&10]);
        assert_eq!(0, fm[&100]);
        assert_eq!(0, fm[&101]);
        assert_eq!(0, fm[&102]);
    }

    #[test]
    fn TEST_FrequencyMap_FromIterator_2() {

        let iter = (1..10).into_iter();

        let fm = FrequencyMap::<i32>::from_iter(iter);

        assert!(!fm.is_empty());
        assert_eq!(9, fm.len());
        assert_eq!(9, fm.total());

        assert_eq!(0, fm.get(&0));
        assert_eq!(1, fm.get(&1));
        assert_eq!(1, fm.get(&2));
        assert_eq!(1, fm.get(&3));
        assert_eq!(1, fm.get(&4));
        assert_eq!(1, fm.get(&5));
        assert_eq!(1, fm.get(&6));
        assert_eq!(1, fm.get(&7));
        assert_eq!(1, fm.get(&8));
        assert_eq!(1, fm.get(&9));
        assert_eq!(0, fm.get(&10));
        assert_eq!(0, fm.get(&100));
        assert_eq!(0, fm.get(&101));
        assert_eq!(0, fm.get(&102));
    }

    #[test]
    fn TEST_FrequencyMap_FromIterator_3() {

        let iter = (1..10).into_iter().chain((2..5).into_iter());

        let fm = FrequencyMap::<i32>::from_iter(iter);

        assert!(!fm.is_empty());
        assert_eq!(9, fm.len());
        assert_eq!(12, fm.total());

        assert_eq!(0, fm.get(&0));
        assert_eq!(1, fm.get(&1));
        assert_eq!(2, fm.get(&2));
        assert_eq!(2, fm.get(&3));
        assert_eq!(2, fm.get(&4));
        assert_eq!(1, fm.get(&5));
        assert_eq!(1, fm.get(&6));
        assert_eq!(1, fm.get(&7));
        assert_eq!(1, fm.get(&8));
        assert_eq!(1, fm.get(&9));
        assert_eq!(0, fm.get(&10));
        assert_eq!(0, fm.get(&100));
        assert_eq!(0, fm.get(&101));
        assert_eq!(0, fm.get(&102));
    }

    #[test]
    fn TEST_FrequencyMap_insert_1() {

        let mut fm = FrequencyMap::<i32>::default();

        assert!(fm.is_empty());
        assert_eq!(0, fm.len());
        assert_eq!(0, fm.total());

        {
            let r = fm.insert(1, 0);

            assert_eq!(None, r);
        }

        assert!(fm.is_empty());
        assert_eq!(0, fm.len());
        assert_eq!(0, fm.total());

        {
            let r = fm.insert(1, 123);

            assert_eq!(None, r);
        }

        assert!(!fm.is_empty());
        assert_eq!(1, fm.len());
        assert_eq!(123, fm.total());

        assert_eq!(123, fm[&1]);

        {
            let r = fm.insert(1, -123);

            assert_eq!(Some(123), r);
        }

        assert!(!fm.is_empty());
        assert_eq!(1, fm.len());
        assert_eq!(-123, fm.total());

        assert_eq!(-123, fm[&1]);

        {
            let r = fm.insert(1, 0);

            assert_eq!(Some(-123), r);
        }

        assert!(fm.is_empty());
        assert_eq!(0, fm.len());
        assert_eq!(0, fm.total());

        assert_eq!(0, fm[&1]);
    }

    #[test]
    fn TEST_FrequencyMap_push_1() {

        let mut fm = FrequencyMap::<i32>::default();

        assert!(fm.is_empty());
        assert_eq!(0, fm.len());
        assert_eq!(0, fm.total());

        assert_eq!(0, fm.get(&0));
        assert_eq!(0, fm.get(&10));
        assert_eq!(0, fm.get(&100));
        assert_eq!(0, fm.get(&101));
        assert_eq!(0, fm.get(&102));

        fm.push(101);
        fm.push(102);
        fm.push(101);

        assert!(!fm.is_empty());
        assert_eq!(2, fm.len());
        assert_eq!(3, fm.total());

        assert_eq!(0, fm.get(&0));
        assert_eq!(0, fm.get(&10));
        assert_eq!(0, fm.get(&100));
        assert_eq!(2, fm.get(&101));
        assert_eq!(1, fm.get(&102));
    }

    #[test]
    fn TEST_FrequencyMap_push_n_1() {

        let mut fm = FrequencyMap::<i32>::default();

        assert!(fm.is_empty());
        assert_eq!(0, fm.len());
        assert_eq!(0, fm.total());

        assert_eq!(0, fm.get(&0));
        assert_eq!(0, fm.get(&10));
        assert_eq!(0, fm.get(&100));
        assert_eq!(0, fm.get(&101));
        assert_eq!(0, fm.get(&102));

        fm.push_n(101, 2);
        fm.push_n(102, 1);

        assert!(!fm.is_empty());
        assert_eq!(2, fm.len());
        assert_eq!(3, fm.total());

        assert_eq!(0, fm.get(&0));
        assert_eq!(0, fm.get(&10));
        assert_eq!(0, fm.get(&100));
        assert_eq!(2, fm.get(&101));
        assert_eq!(1, fm.get(&102));
    }

    #[test]
    fn TEST_FrequencyMap_push_n_2() {

        let mut fm = FrequencyMap::<i32>::default();

        assert!(fm.is_empty());
        assert_eq!(0, fm.len());
        assert_eq!(0, fm.total());

        assert!(!fm.contains_key(&0));
        assert!(!fm.contains_key(&1));
        assert!(!fm.contains_key(&2));

        fm.push_n(1, 1);

        assert!(!fm.is_empty());
        assert_eq!(1, fm.len());
        assert_eq!(1, fm.total());

        assert_eq!(0, fm[&0]);
        assert_eq!(1, fm[&1]);
        assert_eq!(0, fm[&2]);

        assert!(!fm.contains_key(&0));
        assert!(fm.contains_key(&1));
        assert!(!fm.contains_key(&2));

        fm.push_n(1, 1);

        assert!(!fm.is_empty());
        assert_eq!(1, fm.len());
        assert_eq!(2, fm.total());

        assert_eq!(0, fm[&0]);
        assert_eq!(2, fm[&1]);
        assert_eq!(0, fm[&2]);

        assert!(!fm.contains_key(&0));
        assert!(fm.contains_key(&1));
        assert!(!fm.contains_key(&2));

        fm.push_n(1, 100);

        assert!(!fm.is_empty());
        assert_eq!(1, fm.len());
        assert_eq!(102, fm.total());

        assert_eq!(0, fm[&0]);
        assert_eq!(102, fm[&1]);
        assert_eq!(0, fm[&2]);

        assert!(!fm.contains_key(&0));
        assert!(fm.contains_key(&1));
        assert!(!fm.contains_key(&2));

        fm.push_n(1, -90);

        assert!(!fm.is_empty());
        assert_eq!(1, fm.len());
        assert_eq!(12, fm.total());

        assert_eq!(0, fm[&0]);
        assert_eq!(12, fm[&1]);
        assert_eq!(0, fm[&2]);

        assert!(!fm.contains_key(&0));
        assert!(fm.contains_key(&1));
        assert!(!fm.contains_key(&2));

        fm.push_n(1, -20);

        assert!(!fm.is_empty());
        assert_eq!(1, fm.len());
        assert_eq!(-8, fm.total());

        assert_eq!(0, fm[&0]);
        assert_eq!(-8, fm[&1]);
        assert_eq!(0, fm[&2]);

        assert!(!fm.contains_key(&0));
        assert!(fm.contains_key(&1));
        assert!(!fm.contains_key(&2));

        fm.push_n(1, 8);

        assert!(fm.is_empty());
        assert_eq!(0, fm.len());
        assert_eq!(0, fm.total());

        assert_eq!(0, fm[&0]);
        assert_eq!(0, fm[&1]);
        assert_eq!(0, fm[&2]);

        assert!(!fm.contains_key(&0));
        assert!(!fm.contains_key(&1));
        assert!(!fm.contains_key(&2));
    }

    #[test]
    fn TEST_FrequencyMap_remove_1() {

        let mut fm = FrequencyMap::<i32>::from([
            // insert list
            2,
            17,
            123,
            123,
        ]);

        assert!(!fm.is_empty());
        assert_eq!(3, fm.len());
        assert_eq!(4, fm.total());

        assert_eq!(0, fm.get(&0));
        assert_eq!(0, fm.get(&1));
        assert_eq!(1, fm.get(&2));
        assert_eq!(0, fm.get(&3));
        assert_eq!(0, fm.get(&10));
        assert_eq!(1, fm.get(&17));
        assert_eq!(0, fm.get(&100));
        assert_eq!(2, fm.get(&123));
        assert_eq!(0, fm.get(&101));
        assert_eq!(0, fm.get(&102));

        {
            let r = fm.remove(&-1);

            assert_eq!(None, r);
        }

        assert!(!fm.is_empty());
        assert_eq!(3, fm.len());
        assert_eq!(4, fm.total());

        assert_eq!(0, fm.get(&0));
        assert_eq!(0, fm.get(&1));
        assert_eq!(1, fm.get(&2));
        assert_eq!(0, fm.get(&3));
        assert_eq!(0, fm.get(&10));
        assert_eq!(1, fm.get(&17));
        assert_eq!(0, fm.get(&100));
        assert_eq!(2, fm.get(&123));
        assert_eq!(0, fm.get(&101));
        assert_eq!(0, fm.get(&102));

        {
            let r = fm.remove(&17);

            assert_eq!(Some(1), r);
        }

        assert!(!fm.is_empty());
        assert_eq!(2, fm.len());
        assert_eq!(3, fm.total());

        assert_eq!(0, fm.get(&0));
        assert_eq!(0, fm.get(&1));
        assert_eq!(1, fm.get(&2));
        assert_eq!(0, fm.get(&3));
        assert_eq!(0, fm.get(&10));
        assert_eq!(0, fm.get(&17));
        assert_eq!(0, fm.get(&100));
        assert_eq!(2, fm.get(&123));
        assert_eq!(0, fm.get(&101));
        assert_eq!(0, fm.get(&102));

        {
            let r = fm.remove(&123);

            assert_eq!(Some(2), r);
        }

        assert!(!fm.is_empty());
        assert_eq!(1, fm.len());
        assert_eq!(1, fm.total());

        assert_eq!(0, fm.get(&0));
        assert_eq!(0, fm.get(&1));
        assert_eq!(1, fm.get(&2));
        assert_eq!(0, fm.get(&3));
        assert_eq!(0, fm.get(&10));
        assert_eq!(0, fm.get(&17));
        assert_eq!(0, fm.get(&100));
        assert_eq!(0, fm.get(&123));
        assert_eq!(0, fm.get(&101));
        assert_eq!(0, fm.get(&102));

        {
            let r = fm.remove(&2);

            assert_eq!(Some(1), r);
        }

        assert!(fm.is_empty());
        assert_eq!(0, fm.len());
        assert_eq!(0, fm.total());

        assert_eq!(0, fm.get(&0));
        assert_eq!(0, fm.get(&1));
        assert_eq!(0, fm.get(&2));
        assert_eq!(0, fm.get(&3));
        assert_eq!(0, fm.get(&10));
        assert_eq!(0, fm.get(&17));
        assert_eq!(0, fm.get(&100));
        assert_eq!(0, fm.get(&123));
        assert_eq!(0, fm.get(&101));
        assert_eq!(0, fm.get(&102));
    }

    #[test]
    fn TEST_FrequencyMap_remove_entry_1() {

        let mut fm = FrequencyMap::<i32>::from([
            // insert list
            2,
            17,
            123,
            123,
        ]);

        assert!(!fm.is_empty());
        assert_eq!(3, fm.len());
        assert_eq!(4, fm.total());

        assert_eq!(0, fm.get(&0));
        assert_eq!(0, fm.get(&1));
        assert_eq!(1, fm.get(&2));
        assert_eq!(0, fm.get(&3));
        assert_eq!(0, fm.get(&10));
        assert_eq!(1, fm.get(&17));
        assert_eq!(0, fm.get(&100));
        assert_eq!(2, fm.get(&123));
        assert_eq!(0, fm.get(&101));
        assert_eq!(0, fm.get(&102));

        {
            let r = fm.remove_entry(&-1);

            assert_eq!(None, r);
        }

        assert!(!fm.is_empty());
        assert_eq!(3, fm.len());
        assert_eq!(4, fm.total());

        assert_eq!(0, fm.get(&0));
        assert_eq!(0, fm.get(&1));
        assert_eq!(1, fm.get(&2));
        assert_eq!(0, fm.get(&3));
        assert_eq!(0, fm.get(&10));
        assert_eq!(1, fm.get(&17));
        assert_eq!(0, fm.get(&100));
        assert_eq!(2, fm.get(&123));
        assert_eq!(0, fm.get(&101));
        assert_eq!(0, fm.get(&102));

        {
            let r = fm.remove_entry(&17);

            assert_eq!(Some((17, 1)), r);
        }

        assert!(!fm.is_empty());
        assert_eq!(2, fm.len());
        assert_eq!(3, fm.total());

        assert_eq!(0, fm.get(&0));
        assert_eq!(0, fm.get(&1));
        assert_eq!(1, fm.get(&2));
        assert_eq!(0, fm.get(&3));
        assert_eq!(0, fm.get(&10));
        assert_eq!(0, fm.get(&17));
        assert_eq!(0, fm.get(&100));
        assert_eq!(2, fm.get(&123));
        assert_eq!(0, fm.get(&101));
        assert_eq!(0, fm.get(&102));

        {
            let r = fm.remove_entry(&123);

            assert_eq!(Some((123, 2)), r);
        }

        assert!(!fm.is_empty());
        assert_eq!(1, fm.len());
        assert_eq!(1, fm.total());

        assert_eq!(0, fm.get(&0));
        assert_eq!(0, fm.get(&1));
        assert_eq!(1, fm.get(&2));
        assert_eq!(0, fm.get(&3));
        assert_eq!(0, fm.get(&10));
        assert_eq!(0, fm.get(&17));
        assert_eq!(0, fm.get(&100));
        assert_eq!(0, fm.get(&123));
        assert_eq!(0, fm.get(&101));
        assert_eq!(0, fm.get(&102));

        {
            let r = fm.remove_entry(&2);

            assert_eq!(Some((2, 1)), r);
        }

        assert!(fm.is_empty());
        assert_eq!(0, fm.len());
        assert_eq!(0, fm.total());

        assert_eq!(0, fm.get(&0));
        assert_eq!(0, fm.get(&1));
        assert_eq!(0, fm.get(&2));
        assert_eq!(0, fm.get(&3));
        assert_eq!(0, fm.get(&10));
        assert_eq!(0, fm.get(&17));
        assert_eq!(0, fm.get(&100));
        assert_eq!(0, fm.get(&123));
        assert_eq!(0, fm.get(&101));
        assert_eq!(0, fm.get(&102));
    }

    #[test]
    fn TEST_FrequencyMap_append_1() {

        let mut fm = FrequencyMap::<i32>::from([
            // insert list
            2,
            17,
            123,
            123,
        ]);
        let mut fm2 = FrequencyMap::<i32>::from([
            // insert list
            (2, 1),
            (17, -1),
            (18, 81),
            (123, -1),
            (124, 421),
        ]);

        assert!(!fm.is_empty());
        assert_eq!(3, fm.len());
        assert_eq!(4, fm.total());

        assert_eq!(0, fm.get(&0));
        assert_eq!(0, fm.get(&1));
        assert_eq!(1, fm.get(&2));
        assert_eq!(0, fm.get(&3));
        assert_eq!(0, fm.get(&10));
        assert_eq!(1, fm.get(&17));
        assert_eq!(0, fm.get(&18));
        assert_eq!(0, fm.get(&19));
        assert_eq!(0, fm.get(&100));
        assert_eq!(0, fm.get(&101));
        assert_eq!(0, fm.get(&102));
        assert_eq!(2, fm.get(&123));
        assert_eq!(0, fm.get(&124));
        assert_eq!(0, fm.get(&125));

        assert!(!fm2.is_empty());
        assert_eq!(5, fm2.len());
        assert_eq!(501, fm2.total());

        fm.append(&mut fm2);

        assert!(fm2.is_empty());
        assert_eq!(0, fm2.len());
        assert_eq!(0, fm2.total());

        assert!(!fm.is_empty());
        assert_eq!(4, fm.len());
        assert_eq!(505, fm.total());

        assert_eq!(0, fm.get(&0));
        assert_eq!(0, fm.get(&1));
        assert_eq!(2, fm.get(&2));
        assert_eq!(0, fm.get(&3));
        assert_eq!(0, fm.get(&10));
        assert_eq!(0, fm.get(&17));
        assert_eq!(81, fm.get(&18));
        assert_eq!(0, fm.get(&19));
        assert_eq!(0, fm.get(&100));
        assert_eq!(0, fm.get(&101));
        assert_eq!(0, fm.get(&102));
        assert_eq!(1, fm.get(&123));
        assert_eq!(421, fm.get(&124));
        assert_eq!(0, fm.get(&125));
    }

    #[test]
    fn TEST_FrequencyMap_drain_1() {

        let mut fm = FrequencyMap::<i32>::from([
            // insert list
            (2, 1),
            (2, 1),
            (17, 1),
            (17, -1),
            (18, 81),
            (123, 1),
            (123, 1),
            (123, -1),
            (124, 421),
        ]);

        assert!(!fm.is_empty());
        assert_eq!(4, fm.len());
        assert_eq!(505, fm.total());

        assert_eq!(0, fm.get(&0));
        assert_eq!(0, fm.get(&1));
        assert_eq!(2, fm.get(&2));
        assert_eq!(0, fm.get(&3));
        assert_eq!(0, fm.get(&10));
        assert_eq!(0, fm.get(&17));
        assert_eq!(81, fm.get(&18));
        assert_eq!(0, fm.get(&19));
        assert_eq!(0, fm.get(&100));
        assert_eq!(0, fm.get(&101));
        assert_eq!(0, fm.get(&102));
        assert_eq!(1, fm.get(&123));
        assert_eq!(421, fm.get(&124));
        assert_eq!(0, fm.get(&125));

        let hm = HashMap::from_iter(fm.drain());

        assert!(fm.is_empty());
        assert_eq!(0, fm.len());
        assert_eq!(0, fm.total());

        {
            let expected = HashMap::from([
                // insert list
                (2, 2),
                (18, 81),
                (123, 1),
                (124, 421),
            ]);

            assert_eq!(expected, hm);
        }
    }

    #[test]
    fn TEST_FrequencyMap_retain_1() {

        let mut fm = FrequencyMap::<i32>::from([
            // insert list
            2,
            17,
            123,
            123,
        ]);

        assert!(!fm.is_empty());
        assert_eq!(3, fm.len());
        assert_eq!(4, fm.total());

        assert_eq!(0, fm.get(&0));
        assert_eq!(0, fm.get(&1));
        assert_eq!(1, fm.get(&2));
        assert_eq!(0, fm.get(&3));
        assert_eq!(0, fm.get(&10));
        assert_eq!(1, fm.get(&17));
        assert_eq!(0, fm.get(&100));
        assert_eq!(2, fm.get(&123));
        assert_eq!(0, fm.get(&101));
        assert_eq!(0, fm.get(&102));

        fm.retain(|&key, _count| key < 1000);

        assert!(!fm.is_empty());
        assert_eq!(3, fm.len());
        assert_eq!(4, fm.total());

        assert_eq!(0, fm.get(&0));
        assert_eq!(0, fm.get(&1));
        assert_eq!(1, fm.get(&2));
        assert_eq!(0, fm.get(&3));
        assert_eq!(0, fm.get(&10));
        assert_eq!(1, fm.get(&17));
        assert_eq!(0, fm.get(&100));
        assert_eq!(2, fm.get(&123));
        assert_eq!(0, fm.get(&101));
        assert_eq!(0, fm.get(&102));

        fm.retain(|_k, count| count > 0);

        assert!(!fm.is_empty());
        assert_eq!(3, fm.len());
        assert_eq!(4, fm.total());

        assert_eq!(0, fm.get(&0));
        assert_eq!(0, fm.get(&1));
        assert_eq!(1, fm.get(&2));
        assert_eq!(0, fm.get(&3));
        assert_eq!(0, fm.get(&10));
        assert_eq!(1, fm.get(&17));
        assert_eq!(0, fm.get(&100));
        assert_eq!(2, fm.get(&123));
        assert_eq!(0, fm.get(&101));
        assert_eq!(0, fm.get(&102));

        fm.retain(|_k, count| count > 1);

        assert!(!fm.is_empty());
        assert_eq!(1, fm.len());
        assert_eq!(2, fm.total());

        assert_eq!(0, fm.get(&0));
        assert_eq!(0, fm.get(&1));
        assert_eq!(0, fm.get(&2));
        assert_eq!(0, fm.get(&3));
        assert_eq!(0, fm.get(&10));
        assert_eq!(0, fm.get(&17));
        assert_eq!(0, fm.get(&100));
        assert_eq!(2, fm.get(&123));
        assert_eq!(0, fm.get(&101));
        assert_eq!(0, fm.get(&102));
    }

    #[test]
    fn TEST_FrequencyMap_EXAMPLE_1() {

        let mut fm = FrequencyMap::default();

        fm.push("cat");
        fm.push("dog");
        fm.push("dog");

        assert_eq!(1, fm.get("cat"));
        assert_eq!(2, fm.get("dog"));
        assert_eq!(0, fm.get("mouse"));
    }
}


// ///////////////////////////// end of file //////////////////////////// //

