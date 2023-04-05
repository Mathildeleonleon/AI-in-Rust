use std::collections::hash_map::{DefaultHasher, RandomState};
use std::hash::{BuildHasher, Hash, Hasher};

/// A Bloom Filter is a probabilistic data structure aiming at testing whether an element belongs to a set or not
/// Therefore, its contract looks very close to the one of a set, for example a `HashSet`
trait BloomFilter<Item: Hash> {
    fn insert(&mut self, item: Item);
    fn contains(&self, item: &Item) -> bool;
}

/// What is the point of using a Bloom Filter if it acts like a Set?
/// Let's imagine we have a huge number of elements to store (like un unbounded data stream) a Set storing every element will most likely take up too much space, at some point.
/// As other probabilistic data structure like Count-min Sketch, the goal of a Bloom Filter is to trade off exactitude for constant space.
/// We won't have a strictly exact result of whether the value belongs to the set, but we'll use constant space instead

/// Let's start with the basic idea behind the implementation
/// Instead of storing every element and grow the set infinitely, let's use a vector with constant capacity `CAPACITY`
/// Each element of this vector would be a boolean.
/// When a new element is added, we hash its value and set the index at index `hash(item) % CAPACITY` to `true`
/// When looking for an item, we hash its value and retrieve the boolean at index `hash(item) % CAPACITY`
/// If it's `false` it's absolutely sure the item isn't present
/// If it's `true` the item may be present, or maybe another one produces the same hash
#[derive(Debug)]
struct BasicBloomFilter<const CAPACITY: usize> {
    vec: [bool;CAPACITY],
}

impl <const CAPACITY: usize> Default for BasicBloomFilter<CAPACITY> {
    fn default() -> Self {
        Self {
            vec: [false; CAPACITY]
        }
    }
}

impl<Item: Hash, const CAPACITY: usize> BloomFilter<Item> for BasicBloomFilter<CAPACITY> {
    fn insert(&mut self, item: Item) {
        let mut hasher = DefaultHasher::new();
        item.hash(&mut hasher);
        let idx = (hasher.finish() % CAPACITY as u64) as usize;
        self.vec[idx] = true;
    }

    fn contains(&self, item: &Item) -> bool {
        let mut hasher = DefaultHasher::new();
        item.hash(&mut hasher);
        let idx = (hasher.finish() % CAPACITY as u64) as usize;
        self.vec[idx]
    }
}

/// Can we improve it? Certainly, in different ways.
/// One pattern you may have identified here is that we use a "binary array" (a vector of binary values)
/// For instance, we might have [0,1,0,0,1], which is the binary representation of 9
/// This means we can immediately replace our Vector by an actual number
/// What would it mean to set a 1 at index 'i'?
/// Say we have: 000010 and we want to set 1 at the last index: it's 000010 | 000001
/// Meaning we can hash the item value, use a modulo to find the index, and do a binary or between the current number and the index
#[derive(Debug, Default)]
struct SingleBinaryBloomFilter {
    fingerprint: u128, // let's use 128 bits, the equivalent of using CAPACITY=128 in the previous example
}

fn mask_128<T: Hash>(hasher: &mut DefaultHasher, item: T) -> u128 {
    item.hash(hasher);
    let idx = (hasher.finish() % 128) as u32;
    // idx is where we want to put a 1, let's convert this into a proper binary mask
    2_u128.pow(idx)
}

impl<T: Hash> BloomFilter<T> for SingleBinaryBloomFilter {
    fn insert(&mut self, item: T) {
        self.fingerprint |= mask_128(&mut DefaultHasher::new(), &item);
    }

    fn contains(&self, item: &T) -> bool {
        (self.fingerprint & mask_128(&mut DefaultHasher::new(), item)) > 0
    }
}


/// We may have made some progress in term of CPU efficiency, using binary operators
/// But we might still run into a lot of collisions with our 128-bits number, and also, we are limited to 128 bits
/// The first thing could be to use an array, but instead of using bools we could use bytes.
/// That'd allow us to go over 128 bits, but would divide by 8 the memory footprint (booleans are stored as bytes, not bits).
/// That's one thing, and will involve dividing / shifting by 8 in different places.
///
/// But still, can we reduce the collisions furthermore?
/// We could be using multiple hash functions, hashing the same item to different indices
/// When inserting a value, we compute its hash with every hash function (`hash_i`) and perform the same operation as above (the OR with `fingerprint`)
/// Then when looking for a value, if ANY of the tests (hash then AND) returns 0 this means the value is missing from the set, otherwise it would have returned 1
pub struct MultiBinaryBloomFilter {
    filter_size: usize,
    bytes: Vec<u8>,
    hash_builders: Vec<RandomState>
}

impl MultiBinaryBloomFilter {
    pub fn with_dimensions(filter_size: usize, hash_count: usize) -> Self {
        let bytes_count = filter_size / 8 + if filter_size % 8 > 0 { 1 } else { 0 }; // we need 8 times less entries in the array, since we are using bytes. Careful that we have at least one element though
        Self {
            filter_size,
            bytes: vec![0; bytes_count],
            hash_builders: vec![RandomState::new(); hash_count],
        }
    }
}

impl <Item: Hash> BloomFilter<Item> for MultiBinaryBloomFilter {
    fn insert(&mut self, item: Item) {
        for builder in &self.hash_builders {
            let mut hasher = builder.build_hasher();
            item.hash(&mut hasher);
            let hash = hasher.finish();
            let index = hash % self.filter_size as u64;
            let byte_index = index as usize / 8; // this is this byte that we need to modify
            let bit_index = (index % 8) as u8; // we cannot only OR with value 1 this time, since we have 8 bits
            self.bytes[byte_index] |= 1 << bit_index;
        }
    }

    fn contains(&self, item: &Item) -> bool {
        for builder in &self.hash_builders {
            let mut hasher = builder.build_hasher();
            item.hash(&mut hasher);
            let hash = hasher.finish();
            let index = hash % self.filter_size as u64;
            let byte_index = index as usize / 8; // this is this byte that we need to modify
            let bit_index = (index % 8) as u8; // we cannot only OR with value 1 this time, since we have 8 bits
            if self.bytes[byte_index] & (1 << bit_index) == 0 {
                return false;
            }
        }
        true
    }
}


#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use quickcheck::{Arbitrary, Gen};
    use quickcheck_macros::quickcheck;
    use crate::data_structures::probabilistic::bloom_filters::{BasicBloomFilter, SingleBinaryBloomFilter, BloomFilter, MultiBinaryBloomFilter};

    #[derive(Debug, Clone)]
    struct TestSet {
        to_insert: HashSet<i32>,
        to_test: Vec<i32>
    }

    impl Arbitrary for TestSet {
        fn arbitrary(g: &mut Gen) -> Self {
            let mut qty = usize::arbitrary(g) % 5_000;
            if qty < 50 {
                qty += 50; // won't be perfectly uniformly distributed
            }
            let mut to_insert = HashSet::with_capacity(qty);
            let mut to_test = Vec::with_capacity(qty);
            for _ in 0..(qty) {
                to_insert.insert(i32::arbitrary(g));
                to_test.push(i32::arbitrary(g));
            }
            TestSet { to_insert, to_test }
        }
    }

    #[quickcheck]
    fn basic_filter_must_not_return_false_negative(TestSet { to_insert, to_test }: TestSet) {
        let mut basic_filter = BasicBloomFilter::<10_000>::default();
        for item in &to_insert {
            basic_filter.insert(*item);
        }
        for other in to_test {
            if !basic_filter.contains(&other) {
                assert!(!to_insert.contains(&other))
            }
        }
    }

    #[quickcheck]
    fn binary_filter_must_not_return_false_negative(TestSet { to_insert, to_test }: TestSet) {
        let mut binary_filter = SingleBinaryBloomFilter::default();
        for item in &to_insert {
            binary_filter.insert(*item);
        }
        for other in to_test {
            if !binary_filter.contains(&other) {
                assert!(!to_insert.contains(&other))
            }
        }
    }

    #[quickcheck]
    fn a_basic_filter_of_capacity_128_is_the_same_as_a_binary_filter(TestSet { to_insert, to_test }: TestSet) {
        let mut basic_filter = BasicBloomFilter::<128>::default(); // change 32 to anything else here, and the test won't pass
        let mut binary_filter = SingleBinaryBloomFilter::default();
        for item in &to_insert {
            basic_filter.insert(*item);
            binary_filter.insert(*item);
        }
        for other in to_test {
            // Since we use the same DefaultHasher::new(), and both have size 32, we should have exactly the same results
            assert_eq!(basic_filter.contains(&other), binary_filter.contains(&other));
        }
    }

    const FALSE_POSITIVE_MAX: f64 = 0.05;

    #[quickcheck]
    fn a_multi_binary_bloom_filter_must_not_return_false_negatives(TestSet { to_insert, to_test }: TestSet) {
        let n = to_insert.len();
        if n ==0 { // avoid dividing by 0 when adjusting the size
            return;
        }
        // See Wikipedia for those formula
        let optimal_filter_size = (-(n as f64) * FALSE_POSITIVE_MAX.ln() / (2.0_f64.ln().powi(2))).ceil() as usize;
        let optimal_hash_count = ((optimal_filter_size as f64 / n as f64) * 2.0_f64.ln()).ceil() as usize;
        let mut binary_filter = MultiBinaryBloomFilter::with_dimensions(optimal_filter_size, optimal_hash_count);
        for item in &to_insert {
            binary_filter.insert(*item);
        }
        let tests = to_test.len();
        let mut false_positives = 0;
        for other in to_test {
            if !binary_filter.contains(&other) {
                assert!(!to_insert.contains(&other))
            } else if !to_insert.contains(&other) {
                // false positive
                false_positives += 1;
            }
        }
        let fp_rate = false_positives as f64 / tests as f64;
        assert!(fp_rate < 0.25); // Why not FALSE_POSITIVE_MAX?
    }

}
