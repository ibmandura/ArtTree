
use std;
use std::mem;

use {ArtKey, ArtTree};
use nodes::{ArtNode, ArtNode4, ArtNodeTrait, MAX_PREFIX_LEN};

// TODO: Decide what kind of comparison do we want, ie. PartialEq might not be necessarry.
// If we decided to compare bytes of the key (which would be correct), we could also use SIMD.
// On the other hand, user of the lib. could implement smart and quick comparison. 
//
// Propably a good way forward for now would be to use PartialEq for now
//
impl<'a, K: 'a + ArtKey + std::cmp::PartialEq, V> ArtTree<K, V> {
    pub fn new() -> Self {
        ArtTree {
            root: ArtNode::Empty,
            size: 0,
        }
    }

    fn break_node<N: ArtNodeTrait<K, V>>(
        mut ptr: Box<N>,
        prefix_match_len: usize,
        depth: usize,
        key: K,
        value: V,
    ) -> ArtNode<K, V> {
        let mut new_node = Box::new(ArtNode4::new());

        let next_byte_leaf = key.bytes()[depth + prefix_match_len];
        let next_byte_inner = ptr.base().partial[prefix_match_len];

        new_node.n.partial_len = prefix_match_len;

        unsafe {
            std::ptr::copy_nonoverlapping(
                ptr.base().partial.as_ptr(),
                new_node.n.partial.as_mut_ptr(),
                new_node.n.partial.len());

            let copy_len = ptr.base().partial_len - prefix_match_len - 1;
            let src = ptr.base().partial[prefix_match_len+1..ptr.base().partial_len].as_ptr();
            let dst = ptr.mut_base().partial[..copy_len].as_mut_ptr();
            std::ptr::copy(src, dst, copy_len);
        }
                
        ptr.mut_base().partial_len -= prefix_match_len + 1;

        new_node.add_child(ptr.to_art_node(), next_byte_inner);
        new_node.add_child(ArtNode::new_leaf(key, value), next_byte_leaf);

        ArtNode::Inner4(new_node)
    }

    fn insert_inner<N>(mut ptr: Box<N>, depth: usize, key: K, value: V) -> ArtNode<K, V>
        where N: ArtNodeTrait<K,V>
    {
        let prefix_match_len = ptr.base().compute_prefix_match(&key, depth);

        if prefix_match_len != ptr.base().partial_len {
            Self::break_node(ptr, prefix_match_len, depth, key, value)
        } else {
            let next_byte = key.bytes()[depth + prefix_match_len];

            if ptr.has_child(next_byte) {
                {
                    let child = ptr.find_child_mut(next_byte);
                    Self::insert_rec(child, depth + prefix_match_len + 1, key, value);
                }
                ptr.to_art_node()
            } else if ptr.is_full() {
                ptr.grow_and_add(ArtNode::new_leaf(key, value), next_byte)
            } else {
                ptr.add_child(ArtNode::new_leaf(key, value), next_byte);
                ptr.to_art_node()
            }
        }
    }

    fn insert_leaf(lleaf: ArtNode<K,V>, key: K, value: V, depth: usize) -> ArtNode<K,V> {
        if *lleaf.key() == key {
            return ArtNode::new_leaf(key, value);
        }

        let mut new_node = Box::new(ArtNode4::new());

        let (lnext, rnext) = {
            let lkey = lleaf.key();

            let mut lcp = depth;
            let max_lcp = std::cmp::min(MAX_PREFIX_LEN, key.bytes().len());

            while lcp < max_lcp && lkey.bytes()[lcp] == key.bytes()[lcp] {
                lcp += 1;
            }

            if lcp > depth {
                unsafe {
                    std::ptr::copy(
                        key.bytes()[depth..].as_ptr(),
                        new_node.n.partial.as_mut_ptr(),
                        lcp - depth
                    );
                }
            }

            new_node.n.partial_len = lcp - depth;

            (lkey.bytes()[lcp], key.bytes()[lcp])
        };

        let rleaf = ArtNode::new_leaf(key, value);

        new_node.add_child(lleaf, lnext);
        new_node.add_child(rleaf, rnext);

        ArtNode::Inner4(new_node)
    }

    fn insert_rec(root: &mut ArtNode<K, V>, depth: usize, key: K, value: V) {
        *root = match mem::replace(root, ArtNode::Empty) {
            ArtNode::Empty => ArtNode::new_leaf(key, value),

            ArtNode::Inner4(ptr) => Self::insert_inner(ptr, depth, key, value),

            ArtNode::Inner16(ptr) => Self::insert_inner(ptr, depth, key, value),

            ArtNode::Inner48(ptr) => Self::insert_inner(ptr, depth, key, value),

            ArtNode::Inner256(ptr) => Self::insert_inner(ptr, depth, key, value),

            leaf => Self::insert_leaf(leaf, key, value, depth),
        };
    }

    pub fn insert(&mut self, key: K, value: V) {
        Self::insert_rec(&mut self.root, 0, key, value);
        self.size += 1;
    }

    #[inline]
    fn search_inner<N: ArtNodeTrait<K,V>>(ptr: &'a N, key: &K, depth: usize) -> Option<&'a V> {
        let lcp = ptr.base().compute_prefix_match(key, depth);

        if lcp != ptr.base().partial_len {
            return None;
        }

        if let Some(ref child) = ptr.find_child(key.bytes()[depth + lcp]) {
            return Self::search_rec(child, key, depth + lcp + 1);
        }

        None
    }

    fn search_rec(root: &'a ArtNode<K,V>, key: &K, depth: usize) -> Option<&'a V> {
        match root {
            &ArtNode::Empty => None,

            &ArtNode::LeafLarge(ref ptr) => if ptr.0 == *key {
                Some(&ptr.1)
            } else {
                None
            }
            
            &ArtNode::LeafLargeKey(ref key_ptr, ref small_value) => if **key_ptr == *key {
                Some(small_value.reference())
            } else {
                None
            }

            &ArtNode::LeafLargeValue(ref small_key, ref value_ptr) => if *small_key.reference() == *key {
                Some(value_ptr)
            } else {
                None
            }

            &ArtNode::LeafSmall(ref small_key, ref small_value) => if *small_key.reference() == *key {
                Some(small_value.reference())
            } else {
                None
            }

            &ArtNode::Inner4(ref ptr) => Self::search_inner(&**ptr, key, depth),

            &ArtNode::Inner16(ref ptr) => Self::search_inner(&**ptr, key, depth),

            &ArtNode::Inner48(ref ptr) => Self::search_inner(&**ptr, key, depth),

            &ArtNode::Inner256(ref ptr) => Self::search_inner(&**ptr, key, depth),
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        Self::search_rec(&self.root, key, 0)
    }

    fn remove_leaf(leaf: ArtNode<K,V>, key: &K) -> Option<V> {
        if *key == *leaf.key() {
            Some(leaf.value())
        } else {
            None
        }
    }

    fn remove_inner<N>(mut ptr: Box<N>, depth: usize, key: &K)-> (ArtNode<K,V>, Option<V>)
        where N: ArtNodeTrait<K,V>
    {
        let prefix_match_len = ptr.base().compute_prefix_match(key, depth);
        let next_byte = key.bytes()[depth + prefix_match_len];

        if prefix_match_len != ptr.base().partial_len || !ptr.has_child(next_byte) {
            (ptr.to_art_node(), None)
        } else {
            let ret = Self::remove_rec(ptr.find_child_mut(next_byte), depth + prefix_match_len + 1, key);

            if let Some(&ArtNode::Empty) = ptr.find_child(next_byte) {
                // TODO: This is weird API, clean_child is called after the child has already been removed.
                //       Why does remove_child return should_shrink? 
                //       Do this for now, but lets focus on this sometimes.
                //
                let should_shrink = ptr.clean_child(next_byte);

                if should_shrink {
                    // TODO: After shrink happens, we should recalculate partial
                    return (ptr.shrink(), ret);
                }
            }

            (ptr.to_art_node(), ret)
        }
    }

    fn remove_rec(root: &mut ArtNode<K,V>, depth: usize, key: &K) -> Option<V> {
        let (new_root, ret) = match mem::replace(root, ArtNode::Empty) {
            ArtNode::Empty => (ArtNode::Empty, None),

            ArtNode::Inner4(ptr) => Self::remove_inner(ptr, depth, key),

            ArtNode::Inner16(ptr) => Self::remove_inner(ptr, depth, key),

            ArtNode::Inner48(ptr) => Self::remove_inner(ptr, depth, key), 

            ArtNode::Inner256(ptr) => Self::remove_inner(ptr, depth, key),

            leaf => (ArtNode::Empty, Self::remove_leaf(leaf, key)),
        };

        *root = new_root;
        ret
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.size -= 1;
        Self::remove_rec(&mut self.root, 0, key)
    }
}

impl ArtKey for u32 {
    fn bytes(&self) -> &[u8] {
        let ptr = self as *const u32 as *const u8;
        unsafe { std::slice::from_raw_parts(ptr, 4) }
    }
}

impl ArtKey for u64 {
    fn bytes(&self) -> &[u8] {
        let ptr = self as *const u64 as *const u8;
        unsafe { std::slice::from_raw_parts(ptr, 8) }
    }
}

impl ArtKey for std::string::String {
    fn bytes(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl ArtKey for std::vec::Vec<u8> {
    fn bytes(&self) -> &[u8] {
        self.as_slice()
    }
}
