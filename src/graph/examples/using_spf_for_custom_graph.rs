
use std::vec::Vec;
use std::collections::HashMap;
use std::hash::Hash;
use std::cmp::Eq;

use jjc::algorithm::traverser::{
    TraverseableNode, 
    DfsIntoIterExt, 
    Edge
};

struct Road {
    node: *mut Node,
    lenght: usize,
}

struct Node {
    roads: Vec<Road>,
}

impl Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::ptr::hash(self, state);
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        return std::ptr::eq(self, other);
    }
}

impl Eq for Node {}

impl<'a> TraverseableNode<'a> for Node {
    type Item = Road;
    type Iter = std::slice::Iter<'a, Self::Item>;

    fn traverser(&self) -> Self::Iter {
        return self.roads.iter(); 
    }
}

impl<'a> DfsIntoIterExt<'a> for Node {} 

fn creat_test_graph() -> Node {
    let mut graph: [Node; 6] = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };

    graph[0] = Node{
        roads: vec![
            Road{node: &mut graph[1] as *mut Node, lenght: 7},
            Road{node: &mut graph[2] as *mut Node, lenght: 9},
            Road{node: &mut graph[5] as *mut Node, lenght: 14},
        ],
    };

    graph[1] = Node{
        roads: vec![
            Road{node: &mut graph[0] as *mut Node, lenght: 7},
            Road{node: &mut graph[2] as *mut Node, lenght: 9},
            Road{node: &mut graph[3] as *mut Node, lenght: 14},
        ],
    };

    graph[2] = Node{
        roads: vec![
            Road{node: &mut graph[3] as *mut Node, lenght: 7},
            Road{node: &mut graph[1] as *mut Node, lenght: 9},
            Road{node: &mut graph[0] as *mut Node, lenght: 14},
            Road{node: &mut graph[5] as *mut Node, lenght: 14},
        ],
    };

    graph[3] = Node{
        roads: vec![
            Road{node: &mut graph[4] as *mut Node, lenght: 7},
            Road{node: &mut graph[2] as *mut Node, lenght: 9},
            Road{node: &mut graph[1] as *mut Node, lenght: 14},
        ],
    };

    graph[4] = Node{
        roads: vec![
            Road{node: &mut graph[3] as *mut Node, lenght: 7},
            Road{node: &mut graph[5] as *mut Node, lenght: 9},
        ],
    };

    graph[5] = Node{
        roads: vec![
            Road{node: &mut graph[4] as *mut Node, lenght: 7},
            Road{node: &mut graph[2] as *mut Node, lenght: 9},
            Road{node: &mut graph[0] as *mut Node, lenght: 14},
        ],
    };

    return graph[0];
}

fn main() {
    let mut root = creat_test_graph();
    println!("{}", root.spf());
}