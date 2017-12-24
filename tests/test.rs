
#![feature(iterator_step_by)]
#![feature(test)]

extern crate test;
extern crate art;
extern crate rand;


use art::ArtTree;
use rand::Rng;

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
fn sanity_test_u32() {
    type InsrtType = u32;
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


#[test]
fn short_string_test() {
    let mut rng = rand::thread_rng();

    let mut keys = Vec::with_capacity(100);

    let mut t = ArtTree::new();
    for i in 0..100 {
        let s = rng.gen_ascii_chars().take(50).collect::<String>();
        keys.push(s.clone());
        test::black_box(t.insert(s, i));
    }

    for i in 0..100 {
        match t.get(&keys[i]) {
            None => assert!(false),
            Some(x) => assert_eq!(*x, i),
        }
    }
}

#[test]
fn long_string_test() {
    let mut rng = rand::thread_rng();

    let mut keys = Vec::with_capacity(100);

    let mut t = ArtTree::new();
    for i in 0..100 {
        let s = rng.gen_ascii_chars().take(500).collect::<String>();
        keys.push(s.clone());
        test::black_box(t.insert(s, i));
    }

    for i in 0..100 {
        match t.get(&keys[i]) {
            None => assert!(false),
            Some(x) => assert_eq!(*x, i),
        }
    }
}

#[test]
fn delete_test() {
    let mut t = ArtTree::new();

    for i in 0..100u32 {
        t.insert(i,i);
    }

    for i in (0..100u32).step_by(2) {
        match t.remove(&i) {
            Some(x) => assert_eq!(x, i),
            None => assert!(false),
        }
    }

    for i in (1..100).step_by(2) {
        match t.get(&i) {
            Some(x) => assert_eq!(*x, i),
            None => assert!(false),
        }
    }
}
