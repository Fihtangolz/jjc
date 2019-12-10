#![feature(const_generics)]
use std::cmp::Eq;
use std::collections::vec_deque::VecDeque;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;
use std::iter::IntoIterator;
use std::iter::Iterator;
use std::marker::PhantomData;
use std::vec::Vec;

struct Road {
    node: *mut Node,
    lenght: usize,
}

struct Node {
    // void* data;
    roads: Vec<Road>,
    shortcut: usize,
}

struct Graph<'a> {
    head: &'a Node,
}

impl<'a> GraphTraverser<'a> for Graph<'a> {
    type Item = Node;
    type Iter = std::vec::IntoIter<&'a Self::Item>;

    fn set_rootes(&self) -> Self::Iter {
        return vec!(self.head).into_iter();
    }
}

impl<'a> TraverseableNode<'a> for Node {
    type Item = Node;
    type Iter = std::vec::IntoIter<&'a Self::Item>;

    fn traverser(&self) -> Self::Iter {
        unsafe {
            let v: Vec<_> = self.roads.iter().map(|obj| &*obj.node).collect();
            return v.into_iter();
        }
    }
}

////////////////////////////////////////////////////
pub trait GraphTraverser<'a> {
    type Item: 'a;
    type Iter: std::iter::Iterator<Item = &'a Self::Item>;

    /// Used to set the initial nodes from which the starting traversing
    fn set_rootes(&self) -> Self::Iter;

    // /// Answers the question whether the node is visited
    // fn is_visited(node: &Self::Item) -> bool;
}

pub trait TraverseableNode<'a> {
    type Item: 'a;
    type Iter: std::iter::Iterator<Item = &'a Self::Item>;

    /// The method is used for filling traverser queue
    ///
    /// Example for struct that contain another field
    ///  
    /// ```
    /// struct Node<T> {
    ///     data: *mut T,
    ///     next_left: *mut node<T>,
    ///     next_midle: *mut node<T>,
    ///     next_right: *mut node<T>,
    /// }
    ///
    /// fn filling_next(&self) {
    ///     self.next_left;
    ///     self.next_midle;
    ///     self.next_right;
    /// }
    /// ```
    ///
    /// Example for struct that contain referense in internal container
    ///
    /// ```
    /// struct Node<T> {
    ///    data: *mut T,
    ///    Vec<*mut Node<T>>
    /// }
    ///
    /// fn filling_next(&self) {
    ///     for node in self.Vec {
    ///     }
    /// }
    /// ```
    ///
    /// Example for heterogeneous graph like AST
    ///
    /// ```
    /// struct NodeA {
    ///     
    /// }
    ///
    /// struct NodeB {
    ///     
    /// }
    ///
    /// struct NodeC {
    ///
    /// }
    /// ```
    ///
    fn traverser(&self) -> Self::Iter;
}

/// https://doc.rust-lang.org/std/iter/index.html#the-three-forms-of-iteration
/// 
/// We can provide dfs_iter_mut?? 
pub trait DfsIntoIterExt<'a>: TraverseableNode<'a> {
    fn dfs_iter(&self) -> DfsIterStateHolder<'a, Self::Item>
    where
        Self::Item: Eq + Hash,
        Seff == Self::Item,
    {
        let queue: VecDeque<_> = self.traverser().collect();
        return DfsIterStateHolder {
            set: HashSet::new(),
            bypass_buff: queue,
        };
    }
}

pub struct DfsIterStateHolder<'a, Item> {
    set: HashSet<&'a Item>,
    bypass_buff: VecDeque<&'a Item>,
}

impl<'a, Item> Iterator for DfsIterStateHolder<'a, Item> {
    type Item = &'a Item;

    fn next(&mut self) -> Option<Self::Item> {
        let current_node = self.bypass_buff.pop_front();
        match current_node {
            Some(v) => {
                for el in v.traverser() {
                    if !self.set.insert(el) {
                        self.bypass_buff.push_back(el);
                    }
                }
                return current_node;
            }
            None => {
                return None;
            }
        }
    }
}

