
extern crate test;
extern crate rand;

mod nodes;
mod art;
mod bench;

use nodes::ArtNode;

pub trait ArtKey {
    fn bytes(&self) -> &[u8];
}

pub struct ArtTree<K: ArtKey, V> {
    root: ArtNode<K, V>,
    size: usize,
}
