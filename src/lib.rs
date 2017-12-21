
#![feature(box_syntax)]

mod nodes;
mod art;

use nodes::ArtNode;

pub trait ArtKey {
    fn get_size(&self) -> usize;
    fn get_byte(&self, index: usize) -> u8;
    fn get_bytes(&self, buff: &mut [u8], from: usize, len: usize);
}

#[derive(Debug)]
pub struct ArtTree<K: ArtKey, V> {
    root: ArtNode<K, V>,
    size: usize,
}


extern crate test;
extern crate rand;

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use std::collections::HashMap;
    use test::Bencher;
    use rand::Rng;

    const N: u32 = 50000;
    type InsrtType = u64;

    #[bench]
    fn bench_insert_art(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let mut t = ArtTree::new();

        b.iter(|| {
            for _ in 0..N {
                test::black_box(t.insert(rng.gen::<InsrtType>(),rng.gen::<InsrtType>()));
            }
        })
    }

    #[bench]
    fn bench_insert_btree(b: &mut Bencher) {
        let mut t = BTreeMap::new();
        let mut rng = rand::thread_rng();

        b.iter(|| {
            for _ in 0..N {
                test::black_box(t.insert(rng.gen::<InsrtType>(),rng.gen::<InsrtType>()));
            }
        })
    }

    #[bench]
    fn bench_insert_hmap(b: &mut Bencher) {
        let mut t = HashMap::new();
        let mut rng = rand::thread_rng();

        b.iter(|| {
            for _ in 0..N {
                test::black_box(t.insert(rng.gen::<InsrtType>(),rng.gen::<InsrtType>()));
            }
        })
    }
}
