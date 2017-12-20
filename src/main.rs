
#![feature(test)]

extern crate test;
extern crate art;
extern crate rand;

use art::ArtTree;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    use std::collections::BTreeMap;
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
}

fn main() {
    let mut t = ArtTree::new();
    for i in 0..50000 {
        test::black_box(t.insert(i as u32,i as u32));
    }
}
