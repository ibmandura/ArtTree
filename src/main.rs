
extern crate rand;
extern crate art;

use art::{ArtTree};
use rand::Rng;

type InsrtType = u64;

fn main() {
    let mut t: ArtTree<InsrtType, InsrtType> = ArtTree::new();

    let n = 100000;

    let mut rng = rand::thread_rng();
    let mut keys = Vec::with_capacity(n);

    for _ in 0..n {
        let k = rng.gen::<InsrtType>();
        t.insert(k,k);
        keys.push(k);
    }

    for _ in 0..20 {
        for i in 0..n {
            match t.get(&(i as InsrtType)) {
                None => panic!("Nema"),
                Some(x) => assert!(*x == i as InsrtType, "Kurcina")
            }

            match t.get(&keys[i as usize]) {
                None => panic!("Nema"),
                Some(x) => assert!(*x == keys[i as usize], "Kurcina")
            }
        }
    }

    t.print();
}
