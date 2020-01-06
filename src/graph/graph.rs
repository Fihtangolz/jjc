pub enum DirectCategory {
    Directed,
    Undirected
}

pub trait Graph {
    type VertexDescriptor;
    type EdgeDescriptor;
    const DIRECT_CATEGORY: DirectCategory;
}

//https://www.boost.org/doc/libs/1_72_0/libs/graph/doc/EdgeListGraph.html
trait EdgeAcessor: Graph {
    fn edges();
    fn num_edges();
}

//https://www.boost.org/doc/libs/1_72_0/libs/graph/doc/IncidenceGraph.html
trait IncidenceAcessor: Graph {
    fn out_edges();
}

//https://www.boost.org/doc/libs/1_72_0/libs/graph/doc/IncidenceGraph.html
trait BidirectionalAcessor: IncidenceAcessor {

}

pub trait Adjacency: Graph {
    type AdjacencyIter: std::iter::Iterator;
    fn adjacent_vertices(&self, vertex: Self::VertexDescriptor) -> Self::AdjacencyIter;
}

//https://www.boost.org/doc/libs/1_72_0/libs/graph/doc/MutableGraph.html
trait MutableAcessor {
    fn add_edge();
    fn remove_edge();
}

//https://www.boost.org/doc/libs/1_72_0/libs/graph/doc/VertexListGraph.html
trait VertexAcessor: Graph {
    fn vertices();
    fn num_vertices();
}