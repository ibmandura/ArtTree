
extern crate test;
extern crate rand;

mod nodes;
mod art;

use nodes::ArtNode;

pub trait ArtKey {
    fn bytes(&self) -> &[u8];
}

#[derive(Debug)]
pub struct ArtTree<K: ArtKey, V> {
    root: ArtNode<K, V>,
    size: usize,
}


#[cfg(test)]
mod tests {

    use super::*;

    use std::collections::BTreeMap;
    use std::collections::HashMap;
    use test::Bencher;
    use rand::Rng;

    const N: u32 = 100000;

    const NUM_STR: u32 = 40000;

    #[bench]
    fn bench_insert_art_u64(b: &mut Bencher) {
        type InsrtType = u64;
        let mut rng = rand::thread_rng();

        b.iter(|| {
            let mut t = ArtTree::new();
            for _ in 0..N {
                test::black_box(t.insert(rng.gen::<InsrtType>(),rng.gen::<InsrtType>()));
            }
        })
    }

    #[bench]
    fn bench_insert_btree_u64(b: &mut Bencher) {
        type InsrtType = u64;
        let mut rng = rand::thread_rng();

        b.iter(|| {
            let mut t = BTreeMap::new();
            for _ in 0..N {
                test::black_box(t.insert(rng.gen::<InsrtType>(),rng.gen::<InsrtType>()));
            }
        })
    }

    #[bench]
    fn bench_insert_hmap_u64(b: &mut Bencher) {
        type InsrtType = u64;
        let mut rng = rand::thread_rng();

        b.iter(|| {
            let mut t = HashMap::new();
            for _ in 0..N {
                test::black_box(t.insert(rng.gen::<InsrtType>(),rng.gen::<InsrtType>()));
            }
        })
    }

    #[bench]
    fn bench_insert_art_u32(b: &mut Bencher) {
        type InsrtType = u32;
        let mut rng = rand::thread_rng();

        b.iter(|| {
            let mut t = ArtTree::new();
            for _ in 0..N {
                test::black_box(t.insert(rng.gen::<InsrtType>(),rng.gen::<InsrtType>()));
            }
        })
    }

    #[bench]
    fn bench_insert_btree_u32(b: &mut Bencher) {
        type InsrtType = u32;
        let mut rng = rand::thread_rng();

        b.iter(|| {
            let mut t = BTreeMap::new();
            for _ in 0..N {
                test::black_box(t.insert(rng.gen::<InsrtType>(),rng.gen::<InsrtType>()));
            }
        })
    }

    #[bench]
    fn bench_insert_hmap_u32(b: &mut Bencher) {
        type InsrtType = u32;
        let mut rng = rand::thread_rng();

        b.iter(|| {
            let mut t = HashMap::new();
            for _ in 0..N {
                test::black_box(t.insert(rng.gen::<InsrtType>(),rng.gen::<InsrtType>()));
            }
        })
    }

    #[bench]
    fn bench_insert_art_len_20_string(b: &mut Bencher) {
        let mut rng = rand::thread_rng();

        b.iter(|| {
            let mut t = ArtTree::new();
            for i in 0..NUM_STR {
                let s = rng.gen_ascii_chars().take(20).collect::<String>();
                test::black_box(t.insert(s, i));
            }
        })
    }

    #[bench]
    fn bench_insert_btree_len_20_string(b: &mut Bencher) {
        let mut rng = rand::thread_rng();

        b.iter(|| {
            let mut t = BTreeMap::new();
            for i in 0..NUM_STR {
                let s = rng.gen_ascii_chars().take(20).collect::<String>();
                test::black_box(t.insert(s, i));
            }
        })
    }

    #[bench]
    fn bench_insert_hmap_len_20_string(b: &mut Bencher) {
        let mut rng = rand::thread_rng();

        b.iter(|| {
            let mut t = HashMap::new();
            for i in 0..NUM_STR {
                let s = rng.gen_ascii_chars().take(20).collect::<String>();
                test::black_box(t.insert(s, i));
            }
        })
    }

    #[bench]
    fn bench_insert_art_len_100_string(b: &mut Bencher) {
        let mut rng = rand::thread_rng();

        b.iter(|| {
            let mut t = ArtTree::new();
            for i in 0..10000 {
                let s = rng.gen_ascii_chars().take(100).collect::<String>();
                test::black_box(t.insert(s, i));
            }
        })
    }

    #[bench]
    fn bench_insert_btree_len_100_string(b: &mut Bencher) {
        let mut rng = rand::thread_rng();

        b.iter(|| {
            let mut t = BTreeMap::new();
            for i in 0..10000 {
                let s = rng.gen_ascii_chars().take(100).collect::<String>();
                test::black_box(t.insert(s, i));
            }
        })
    }

    #[bench]
    fn bench_insert_hmap_len_100_string(b: &mut Bencher) {
        let mut rng = rand::thread_rng();

        b.iter(|| {
            let mut t = HashMap::new();
            for i in 0..10000 {
                let s = rng.gen_ascii_chars().take(100).collect::<String>();
                test::black_box(t.insert(s, i));
            }
        })
    }

    #[bench]
    fn bench_insert_art_len_1000_string(b: &mut Bencher) {
        let mut rng = rand::thread_rng();

        b.iter(|| {
            let mut t = ArtTree::new();
            for i in 0..2000 {
                let s = rng.gen_ascii_chars().take(1000).collect::<String>();
                test::black_box(t.insert(s, i));
            }
        })
    }

    #[bench]
    fn bench_insert_btree_len_1000_string(b: &mut Bencher) {
        let mut rng = rand::thread_rng();

        b.iter(|| {
            let mut t = BTreeMap::new();
            for i in 0..2000 {
                let s = rng.gen_ascii_chars().take(1000).collect::<String>();
                test::black_box(t.insert(s, i));
            }
        })
    }

    #[bench]
    fn bench_insert_hmap_len_1000_string(b: &mut Bencher) {
        let mut rng = rand::thread_rng();

        b.iter(|| {
            let mut t = HashMap::new();
            for i in 0..2000 {
                let s = rng.gen_ascii_chars().take(1000).collect::<String>();
                test::black_box(t.insert(s, i));
            }
        })
    }
}

