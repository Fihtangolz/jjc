use std::collections::HashMap;
use std::hash::Hash;

use crate::graph::graph::Graph;

pub fn dijkstra_shortest_paths<G, M>(
    graph: G, 
    source: G::VertexDescriptor,
) -> HashMap<G::VertexDescriptor, usize>
where 
    G: Graph,
    G::VertexDescriptor: Eq + Hash,
{
    let table = Self::LenTable::new();
    // let item = self.next();
    // let lenght = length_accessor(item);
    // table[item] = lenght;

    return table;
}