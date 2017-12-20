
#![feature(test)]

extern crate test;
extern crate art;
extern crate rand;

use test::Bencher;

use art::ArtTree;
use rand::Rng;

const N: u32 = 100000;
type InsrtType = u64;


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
