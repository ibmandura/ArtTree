
#![feature(box_syntax)]

mod nodes;

use nodes::{ArtNode, ArtNode4, ArtNodeTrait, MAX_PREFIX_LEN};

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

impl<'a, K: 'a + ArtKey + std::cmp::PartialEq + std::fmt::Debug, V: std::fmt::Debug> ArtTree<K, V> {
    pub fn new() -> Self {
        ArtTree {
            root: ArtNode::Empty,
            size: 0,
        }
    }

    fn break_node<N: ArtNodeTrait<K, V>, F: Fn(Box<N>) -> ArtNode<K,V>>(
        mut ptr: Box<N>,
        prefix_match_len: usize,
        depth: usize,
        key: K,
        value: V,
        to_art_node: F,
    ) -> ArtNode<K, V> {
        let mut new_node = box ArtNode4::new();

        let next_byte_leaf = key.get_byte(depth + prefix_match_len);
        let next_byte_inner = ptr.base().partial[prefix_match_len];

        new_node.n.partial_len = prefix_match_len;
        new_node.n.partial.clone_from_slice(&ptr.base().partial);

        for i in prefix_match_len+1..ptr.base().partial_len {
            // TODO: memmove this
            ptr.mut_base().partial[i - prefix_match_len - 1] = ptr.base().partial[i];
        }

        ptr.mut_base().partial_len -= prefix_match_len + 1;

        new_node.add_child(to_art_node(ptr), next_byte_inner);
        new_node.add_child(ArtNode::Leaf(key, value), next_byte_leaf);

        ArtNode::Inner4(new_node)
    }

    fn insert_inner<N: ArtNodeTrait<K, V>, F: Fn(Box<N>) -> ArtNode<K,V>>(
        mut ptr: Box<N>,
        depth: usize,
        key: K,
        value: V,
        to_art_node: F,
    ) -> ArtNode<K, V> {
        let prefix_match_len = ptr.base().compute_prefix_match(&key, depth);

        if prefix_match_len != ptr.base().partial_len {
            Self::break_node(ptr, prefix_match_len, depth, key, value, to_art_node)
        } else {
            let next_byte = key.get_byte(depth + prefix_match_len);

            if ptr.has_child(next_byte) {
                {
                    let child = ptr.find_child_mut(next_byte).unwrap();
                    Self::rec_insert(child, depth + prefix_match_len + 1, key, value);
                }
                to_art_node(ptr)
            } else if ptr.is_full() {
                ptr.grow_and_add(ArtNode::Leaf(key, value), next_byte)
            } else {
                ptr.add_child(ArtNode::Leaf(key, value), next_byte);
                to_art_node(ptr)
            }
        }
    }

    fn rec_insert(root: &mut ArtNode<K, V>, depth: usize, key: K, value: V) {
        let old_root = std::mem::replace(root, ArtNode::Empty);

        *root = match old_root {
            ArtNode::Empty => ArtNode::Leaf(key, value),

            ArtNode::Leaf(lkey, lvalue) => {
                if lkey == key {
                    ArtNode::Leaf(key, value)
                } else {
                    let mut new_node = box ArtNode4::new();

                    let mut lcp = depth;
                    let max_lcp = std::cmp::min(MAX_PREFIX_LEN, key.get_size());

                    while lcp < max_lcp && lkey.get_byte(lcp) == key.get_byte(lcp) {
                        new_node.n.partial[lcp - depth] = key.get_byte(lcp);
                        lcp += 1;
                    }

                    new_node.n.partial_len = lcp - depth;

                    let lnext = lkey.get_byte(lcp);
                    let rnext = key.get_byte(lcp);

                    let rleaf = ArtNode::Leaf(key, value);
                    let lleaf = ArtNode::Leaf(lkey, lvalue);

                    new_node.add_child(lleaf, lnext);
                    new_node.add_child(rleaf, rnext);

                    ArtNode::Inner4(new_node)
                }
            }

            ArtNode::Inner4(ptr) => Self::insert_inner(ptr, depth, key, value, |ptr| ArtNode::Inner4(ptr)),

            ArtNode::Inner16(ptr) => Self::insert_inner(ptr, depth, key, value, |ptr| ArtNode::Inner16(ptr)),

            ArtNode::Inner48(ptr) => Self::insert_inner(ptr, depth, key, value, |ptr| ArtNode::Inner48(ptr)),

            ArtNode::Inner256(ptr) => Self::insert_inner(ptr, depth, key, value, |ptr| ArtNode::Inner256(ptr)),
        };
    }

    pub fn insert(&mut self, key: K, value: V) {
        Self::rec_insert(&mut self.root, 0, key, value);
    }

    fn search_inner<N: ArtNodeTrait<K,V>>(ptr: &'a N, key: &K, depth: usize) -> Option<&'a V> {
        let lcp = ptr.base().compute_prefix_match(key, depth);

        if lcp != ptr.base().partial_len {
            return None;
        }

        if let Some(ref child) = ptr.find_child(key.get_byte(depth + lcp)) {
            return Self::search_rec(child, key, depth + lcp + 1);
        }

        None
    }

    fn search_rec(root: &'a ArtNode<K,V>, key: &K, depth: usize) -> Option<&'a V> {
        match root {
            &ArtNode::Empty => None,

            &ArtNode::Leaf(ref lkey, ref value) =>
                if *key == *lkey { Some(value) } else { None },

            &ArtNode::Inner4(ref ptr) => Self::search_inner(&**ptr, key, depth),

            &ArtNode::Inner16(ref ptr) => Self::search_inner(&**ptr, key, depth),

            &ArtNode::Inner48(ref ptr) => Self::search_inner(&**ptr, key, depth),

            &ArtNode::Inner256(ref ptr) => Self::search_inner(&**ptr, key, depth),
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        Self::search_rec(&self.root, key, 0)
    }

    fn preorder(root: &ArtNode<K, V>) {
        println!("{:?}", root);

        match *root {
            ArtNode::Inner4(ref ptr) => for child_index in 0..4 {
                Self::preorder(&ptr.children[child_index])
            },
            ArtNode::Inner16(ref ptr) => for child_index in 0..16 {
                Self::preorder(&ptr.children[child_index])
            },

            _ => {}
        }
    }

    pub fn print(&self) {
        Self::preorder(&self.root);
    }
}

impl ArtKey for u32 {
    fn get_size(&self) -> usize {
        4
    }

    fn get_byte(&self, index: usize) -> u8 {
        if index >= 4 {
            panic!("Index out o bounce");
        }
        //unsafe { std::mem::transmute::<u32, [u8; 4]>(*self)[3 - index] }
        unsafe { std::mem::transmute::<u32, [u8; 4]>(*self)[index] }
    }

    fn get_bytes(&self, buff: &mut [u8], from: usize, len: usize) {
        let bytes = unsafe { std::mem::transmute::<u32, [u8; 4]>(*self) };

        /*for i in 0..len {
            let index = i + from;
            buff[i] = bytes[3 - index];
        }*/

        /*for i in 0..len {
            let index = i + from;
            buff[i] = bytes[index];
        }*/

        buff.clone_from_slice(&bytes[from..from+len])
    }
}

impl ArtKey for u64 {
    fn get_size(&self) -> usize {
        8
    }

    fn get_byte(&self, index: usize) -> u8 {
        if index >= 8 {
            panic!("Index out o bounce");
        }
        //unsafe { std::mem::transmute::<u64, [u8; 8]>(*self)[7 - index] }
        unsafe { std::mem::transmute::<u64, [u8; 8]>(*self)[index] }
    }

    fn get_bytes(&self, buff: &mut [u8], from: usize, len: usize) {
        let bytes = unsafe { std::mem::transmute::<u64, [u8; 8]>(*self) };

        /*for i in 0..len {
            let index = i + from;
            buff[i] = bytes[7 - index];
        } */

        /*for i in 0..len {
            let index = i + from;
            buff[i] = bytes[index];
        }*/

        buff.clone_from_slice(&bytes[from..from+len])
    }
}
