
#![feature(iterator_step_by)]

extern crate art;

use art::{ArtTree};

fn main() {
    let mut t = ArtTree::new();

    let n = 10 as u32;
    for i in 0..n {
        t.insert(i,i);
    }

    for i in 0..n {
        assert!(t.get(&i).is_some());
    }

    for i in (0..n).step_by(2) {
        match t.remove(&i) {
            Some(x) => assert_eq!(x, i),
            None => assert!(false),
        }
    }

    for i in 0..n {
        if i % 2 == 1 {
            match t.get(&i) {
                Some(x) => assert_eq!(*x, i),
                None => assert!(false),
            }
        } else if t.get(&i).is_some() {
            assert!(false);
        }
    }
}
