
#![feature(test)]

extern crate test;
extern crate art;

use art::ArtTree;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    use std::collections::BTreeMap;

    const N: u32 = 100000;

    #[bench]
    fn bench_insert_art(b: &mut Bencher) {
        let mut t = ArtTree::new();

        b.iter(|| {
            for i in 0..N {
                test::black_box(t.insert(i,i));
            }
        })
    }

    #[bench]
    fn bench_insert_btree(b: &mut Bencher) {
        let mut t = BTreeMap::new();

        b.iter(|| {
            for i in 0..N {
                test::black_box(t.insert(i,i));
            }
        })
    }
}

fn main() {
    let mut t = ArtTree::new();
    for i in 0..50000 {
        test::black_box(t.insert(i,i));
    }
}
