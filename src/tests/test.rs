
#![feature(test)]

extern crate test;
extern crate art;
extern crate rand;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    use std::collections::BTreeMap;
    use std::collections::HashMap;
    use art::ArtTree;
    use rand::Rng;

    const N: u32 = 100000;
    type InsrtType = u64;

    #[bench]
    fn bench_insert_art(b: &mut Bencher) {
        let mut t = ArtTree::new();
        let mut rng = rand::thread_rng();

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

    #[test]
    fn sanity_test() {
        type InsrtType = u64;
        let mut t = ArtTree::new();

        let mut rng = rand::thread_rng();

        let n = 5011;

        let mut keys = Vec::new();
        for _ in 0..n {
            keys.push(rng.gen::<InsrtType>());
        }

        for i in 0..n {
            test::black_box(t.insert(keys[i], keys[i]));
        }

        for i in 0..n {
            match t.get(&keys[i]) {
                None => assert!(false),
                Some(x) => assert_eq!(*x, keys[i]),
            }
        }
    }

    #[test]
    fn sanity_seq_test() {
        let mut t = ArtTree::new();

        let n = 5011 as u32;

        for i in 0..n {
            test::black_box(t.insert(i, i));
        }

        for i in 0..n {
            match t.get(&i) {
                None => assert!(false),
                Some(x) => assert_eq!(*x, i),
            }
        }
    }
}

fn main() { }
