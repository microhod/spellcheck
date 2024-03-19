use bit_vec::BitVec;
use fasthash::FastHasher;
use std::hash;

pub struct BloomFilter {
    filter: BitVec,
}

impl BloomFilter {
    pub fn new(size: usize) -> BloomFilter {
        return BloomFilter {
            filter: BitVec::from_elem(size, false),
        };
    }

    pub fn add(&mut self, str: &str) {
        for index in self.hash(str) {
            self.filter.set(index, true);
        }
    }

    pub fn query(&self, str: &str) -> bool {
        for index in self.hash(str) {
            match self.filter.get(index) {
                None => return false,
                Some(got) => {
                    if got == false {
                        return false;
                    }
                }
            }
        }

        return true;
    }

    fn hash(&self, str: &str) -> [usize; 2] {
        return [
            self.run_hasher(FnvHasher::new(), str),
            self.run_hasher(fasthash::Murmur3HasherExt::new(), str),
        ];
    }

    fn run_hasher(&self, mut hasher: impl hash::Hasher, str: &str) -> usize {
        hasher.write(str.as_bytes());
        return hasher.finish() as usize % self.filter.len();
    }
}

// https://en.wikipedia.org/wiki/Fowler%E2%80%93Noll%E2%80%93Vo_hash_function
const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;
const FNV_PRIME: u64 = 0x100000001b3;

struct FnvHasher(u64);

impl FnvHasher {
    fn new() -> FnvHasher {
        return FnvHasher(FNV_OFFSET_BASIS);
    }
}

impl hash::Hasher for FnvHasher {
    fn write(&mut self, bytes: &[u8]) {
        let mut hash = self.0;

        for byte in bytes {
            hash = hash.wrapping_mul(FNV_PRIME);
            hash ^= u64::from(*byte);
        }

        self.0 = hash;
    }

    fn finish(&self) -> u64 {
        return self.0;
    }
}
