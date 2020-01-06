use std::collections::HashSet;
use std::hash::Hash;
use std::collections::vec_deque::VecDeque;
use std::cmp::Eq;
use std::iter::Iterator;

/// Adjacency iteration: Iterate over all the neighbors of a node.
///
/// Example for struct that hold neighbors as set of fields
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
/// Example for struct that contain neighbors in internal container
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
pub trait TraverseableNode<'a> {
    type Item: 'a;
    type Iter: std::iter::Iterator<Item = &'a Self::Item>;

    fn traverser(&self) -> Self::Iter;
}

pub trait Edge<Weight, Node> {
    fn weight(&self) -> Weight;
    fn node(&self) -> Node;
}

/// https://doc.rust-lang.org/std/iter/index.html#the-three-forms-of-iteration
/// 
/// We can provide dfs_iter_mut?? 
pub trait  DfsIntoIterExt<'a>: TraverseableNode<'a, Item=Self> + Sized + 'a {
    fn dfs_iter(&'a self) -> DfsIterStateHolder<'a, Self::Item>
    where
        Self::Item: Eq + Hash,
    {
        let mut visited = HashSet::new();
        visited.insert(self);
        let mut bypass_buff = VecDeque::new();
        bypass_buff.push_back(self);
        return DfsIterStateHolder {
            visited: visited,
            bypass_buff: bypass_buff,
        };
    }
}

pub struct DfsIterStateHolder<'a, Item> {
    visited: HashSet<&'a Item>,
    bypass_buff: VecDeque<&'a Item>,
}

impl<'a, I> Iterator for DfsIterStateHolder<'a, I>
where I: TraverseableNode<'a, Item=I> + Eq + Hash
{
    type Item = &'a I;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.bypass_buff.pop_front();
        match current {
            Some(v) => {
                for el in v.traverser() {
                    if self.visited.insert(el) {
                        self.bypass_buff.push_back(el);
                    }
                }
                return current;
            }
            None => {
                return None;
            }
        }
    }
}