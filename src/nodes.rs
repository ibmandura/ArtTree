
use std;
use {ArtKey};

pub const MAX_PREFIX_LEN: usize = 10;
const EMPTY_CELL: u8 = 50;

macro_rules! make_array {
    ($n:expr, $constructor:expr) => {{
        let mut items: [_; $n] = std::mem::uninitialized();
        for place in items.iter_mut() {
            std::ptr::write(place, $constructor);
        }
        items
    }}
}

#[derive(Debug)]
pub enum ArtNode<K, T> {
    Empty,
    Inner4(Box<ArtNode4<K, T>>),
    Inner16(Box<ArtNode16<K, T>>),
    Inner48(Box<ArtNode48<K, T>>),
    Inner256(Box<ArtNode256<K, T>>),
    Leaf(K, T),
}

#[derive(Debug)]
pub struct ArtNodeBase {
    pub num_children: u16,
    pub partial_len: usize,
    pub partial: [u8; MAX_PREFIX_LEN],
}

#[derive(Debug)]
pub struct ArtNode4<K, V> {
    pub n: ArtNodeBase,
    pub keys: [u8; 4],
    pub children: [ArtNode<K, V>; 4],
}

#[derive(Debug)]
pub struct ArtNode16<K, V> {
    pub n: ArtNodeBase,
    pub keys: [u8; 16],
    pub children: [ArtNode<K, V>; 16],
}

pub struct ArtNode48<K, V> {
    pub n: ArtNodeBase,
    pub keys: [u8; 256],
    pub children: [ArtNode<K, V>; 48],
}

pub struct ArtNode256<K, V> {
    pub n: ArtNodeBase,
    pub children: [ArtNode<K, V>; 256],
}

pub trait ArtNodeTrait<K, V> {
    fn add_child(&mut self, node: ArtNode<K, V>, byte: u8);

    fn is_full(&self) -> bool;

    fn grow_and_add(mut self, leaf: ArtNode<K, V>, byte: u8) -> ArtNode<K, V>;

    fn mut_base(&mut self) -> &mut ArtNodeBase;

    fn base(&self) -> &ArtNodeBase;

    fn find_child_mut(&mut self, byte: u8) -> Option<&mut ArtNode<K, V>>;

    fn find_child(&self, byte: u8) -> Option<&ArtNode<K, V>>;

    fn has_child(&self, byte: u8) -> bool;
}


/* ************************ */

impl ArtNodeBase {
    pub fn new() -> Self {
        ArtNodeBase {
            num_children: 0,
            partial_len: 0,
            partial: [0u8; MAX_PREFIX_LEN],
        }
    }

    pub fn compute_prefix_match<K: ArtKey>(&self, key: &K, depth: usize) -> usize {
        for i in 0..self.partial_len {
            if key.get_byte(i + depth) != self.partial[i] {
                return i;
            }
        }
        self.partial_len
    }
}

impl<K, V> ArtNode4<K, V> {
    pub fn new() -> Self {
        ArtNode4 {
            n: ArtNodeBase::new(),
            keys: [0; 4],
            children: Default::default(),
        }
    }
}

impl<K, V> ArtNode16<K, V> {
    pub fn new() -> Self {
        ArtNode16 {
            n: ArtNodeBase::new(),
            keys: [0; 16],
            children: Default::default(),
        }
    }
}

impl<K, V> ArtNode48<K, V> {
    pub fn new() -> Self {
        ArtNode48 {
            n: ArtNodeBase::new(),
            keys: [EMPTY_CELL; 256],
            children: unsafe { make_array!(48, ArtNode::Empty) }
        }
    }
}

impl<K, V> ArtNode256<K, V> {
    pub fn new() -> Self {
        ArtNode256 {
            n: ArtNodeBase::new(),
            children: unsafe { make_array!(256, ArtNode::Empty) }
        }
    }
}

impl<K: ArtKey, V> ArtNodeTrait<K, V> for ArtNode4<K, V> {
    fn add_child(&mut self, child: ArtNode<K, V>, byte: u8) {
        let idx = self.n.num_children as usize;
        self.children[idx] = child;
        self.keys[idx] = byte;
        self.n.num_children += 1;
    }

    fn is_full(&self) -> bool {
        self.n.num_children >= 4
    }

    fn grow_and_add(mut self, leaf: ArtNode<K, V>, byte: u8) -> ArtNode<K, V> {
        let mut new_node = box ArtNode16::new();

        new_node.n.partial_len = self.n.partial_len;
        new_node.n.partial.clone_from_slice(&self.n.partial);
        new_node.add_child(leaf, byte);

        for i in 0..4 {
            let child = std::mem::replace(&mut self.children[i as usize], ArtNode::Empty);
            new_node.add_child(child, self.keys[i as usize]);
        }

        ArtNode::Inner16(new_node)
    }

    fn mut_base(&mut self) -> &mut ArtNodeBase {
        &mut self.n
    }

    fn base(&self) -> &ArtNodeBase {
        &self.n
    }

    fn find_child_mut(&mut self, byte: u8) -> Option<&mut ArtNode<K, V>> {
        for i in 0..self.n.num_children {
            if self.keys[i as usize] == byte {
                return Some(&mut self.children[i as usize]);
            }
        }
        None
    }

    fn find_child(&self, byte: u8) -> Option<&ArtNode<K, V>> {
        for i in 0..self.n.num_children {
            if self.keys[i as usize] == byte {
                return Some(&self.children[i as usize]);
            }
        }
        None
    }

    fn has_child(&self, byte: u8) -> bool {
        for i in 0..self.n.num_children {
            if self.keys[i as usize] == byte {
                return true;
            }
        }
        false
    }
}

impl<K: ArtKey, V> ArtNodeTrait<K, V> for ArtNode16<K, V> {
    fn add_child(&mut self, child: ArtNode<K, V>, byte: u8) {
        let idx = self.n.num_children as usize;
        self.children[idx] = child;
        self.keys[idx] = byte;
        self.n.num_children += 1;
    }

    fn is_full(&self) -> bool {
        self.n.num_children >= 16
    }

    fn grow_and_add(mut self, leaf: ArtNode<K, V>, byte: u8) -> ArtNode<K, V> {
        let mut new_node = box ArtNode48::new();

        new_node.n.partial_len = self.n.partial_len;
        new_node.n.partial.clone_from_slice(&self.n.partial);
        new_node.add_child(leaf, byte);

        for i in 0..16 {
            let child = std::mem::replace(&mut self.children[i], ArtNode::Empty);
            new_node.add_child(child, self.keys[i]);
        }

        ArtNode::Inner48(new_node)
    }

    fn mut_base(&mut self) -> &mut ArtNodeBase {
        &mut self.n
    }

    fn base(&self) -> &ArtNodeBase {
        &self.n
    }

    fn find_child_mut(&mut self, byte: u8) -> Option<&mut ArtNode<K, V>> {
        // TODO: O(1)
        for i in 0..self.n.num_children {
            if self.keys[i as usize] == byte {
                return Some(&mut self.children[i as usize]);
            }
        }
        None
    }

    fn find_child(&self, byte: u8) -> Option<&ArtNode<K, V>> {
        // TODO: O(1)
        for i in 0..self.n.num_children {
            if self.keys[i as usize] == byte {
                return Some(&self.children[i as usize]);
            }
        }
        None
    }

    fn has_child(&self, byte: u8) -> bool {
        for i in 0..self.n.num_children {
            if self.keys[i as usize] == byte {
                return true;
            }
        }
        false
    }
}

impl<K: ArtKey, V> ArtNodeTrait<K, V> for ArtNode48<K, V> {
    fn add_child(&mut self, child: ArtNode<K, V>, byte: u8) {
        self.children[self.n.num_children as usize] = child;
        self.keys[byte as usize] = self.n.num_children as u8;
        self.n.num_children += 1;
    }

    fn is_full(&self) -> bool {
        self.n.num_children >= 48
    }

    fn grow_and_add(mut self, leaf: ArtNode<K, V>, byte: u8) -> ArtNode<K, V> {
        let mut new_node = box ArtNode256::new();

        new_node.n.partial_len = self.n.partial_len;
        new_node.n.partial.clone_from_slice(&self.n.partial);
        new_node.add_child(leaf, byte);

        for i in 0..256 {
            if self.keys[i] != EMPTY_CELL {
                let child = std::mem::replace(&mut self.children[self.keys[i] as usize], ArtNode::Empty);
                new_node.add_child(child, i as u8);
            }
        }

        ArtNode::Inner256(new_node)
    }

    fn mut_base(&mut self) -> &mut ArtNodeBase {
        &mut self.n
    }

    fn base(&self) -> &ArtNodeBase {
        &self.n
    }

    fn find_child_mut(&mut self, byte: u8) -> Option<&mut ArtNode<K, V>> {
        if self.keys[byte as usize] == EMPTY_CELL {
            None
        } else {
            Some(&mut self.children[self.keys[byte as usize] as usize])
        }
    }

    fn find_child(&self, byte: u8) -> Option<&ArtNode<K, V>> {
        if self.keys[byte as usize] == EMPTY_CELL {
            None
        } else {
            Some(&self.children[self.keys[byte as usize] as usize])
        }
    }

    fn has_child(&self, byte: u8) -> bool {
        self.keys[byte as usize] != EMPTY_CELL
    }
}

impl<K: ArtKey, V> ArtNodeTrait<K, V> for ArtNode256<K, V> {
    fn add_child(&mut self, child: ArtNode<K, V>, byte: u8) {
        self.n.num_children += 1;
        self.children[byte as usize] = child;
    }

    fn is_full(&self) -> bool {
        self.n.num_children >= 256
    }

    fn grow_and_add(self, _leaf: ArtNode<K, V>, _byte: u8) -> ArtNode<K, V> {
        panic!("Cannot grow 256 sized node");
    }

    fn mut_base(&mut self) -> &mut ArtNodeBase {
        &mut self.n
    }

    fn base(&self) -> &ArtNodeBase {
        &self.n
    }

    fn find_child_mut(&mut self, byte: u8) -> Option<&mut ArtNode<K, V>> {
        if let ArtNode::Empty = self.children[byte as usize] {
            None
        } else {
            Some(&mut self.children[byte as usize])
        }
    }

    fn find_child(&self, byte: u8) -> Option<&ArtNode<K, V>> {
        if let ArtNode::Empty = self.children[byte as usize] {
            None
        } else {
            Some(&self.children[byte as usize])
        }
    }

    fn has_child(&self, byte: u8) -> bool {
        if let ArtNode::Empty = self.children[byte as usize] {
            false
        } else {
            true
        }
    }
}

impl<K, T> std::fmt::Debug for ArtNode48<K, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ArtNode48{{ n: {:?}, keys, children }}", self.n)
    }
}

impl<K, T> std::fmt::Debug for ArtNode256<K, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ArtNode256{{ n: {:?}, keys, children }}", self.n)
    }
}

impl<_K, _V> Default for ArtNode<_K, _V> {
    fn default() -> Self {
        ArtNode::Empty
    }
}

