
use std::hash::{Hash};
use std::collections::HashMap;
use std::iter::Map;
use std::collections::hash_map::{
    Keys,
    Occupied,
    Vacant,
};
use std::slice::{
    Items,
    MutItems,
};
use std::fmt;

/// **DiGraph<N, E>** is a directed graph, with generic node values **N** and
/// edge weights **E**.
///
/// It uses an adjacency list representation, i.e. using *O(|V| + |E|)* space.
///
/// The node type must be suitable as a hash table key (implementing **Eq
/// + Hash**) as well as being a simple type.
///
#[deriving(Clone)]
pub struct DiGraph<N: Eq + Hash, E> {
    nodes: HashMap<N, Vec<(N, E)>>,
}

impl<N, E> fmt::Show for DiGraph<N, E> where N: Eq + Hash + fmt::Show, E: fmt::Show
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.nodes.fmt(f)
    }
}

impl<N, E> DiGraph<N, E> where N: Copy + Eq + Hash
{
    /// Create a new **DiGraph**.
    pub fn new() -> DiGraph<N, E>
    {
        DiGraph {
            nodes: HashMap::new(),
        }
    }

    /// Add node **n** to the graph.
    pub fn add_node(&mut self, n: N) -> N {
        self.nodes.insert(n, Vec::new());
        n
    }

    /// Return **true** if node **n** was removed.
    pub fn remove_node(&mut self, n: N) -> bool {
        match self.nodes.remove(&n) {
            None => false,
            Some(..) => {
                for (_, edges) in self.nodes.iter_mut() {
                    match edges.iter().position(|&(elt, _)| elt == n) {
                        // Use swap_remove because order doesn't matter
                        Some(index) => { edges.swap_remove(index); }
                        None => {}
                    }
                }
                true
            }
        }
    }

    /// Return **true** if the node is contained in the graph.
    pub fn contains_node(&self, n: N) -> bool {
        self.nodes.contains_key(&n)
    }

    /// Add a directed edge from **a** to **b** to the graph.
    ///
    /// Return **true** if edge did not previously exist.
    pub fn add_edge(&mut self, a: N, b: N, edge: E) -> bool
    {
        // We need both lookups anyway to assert sanity, so
        // add nodes if they don't already exist
        //
        // make sure the endpoint exists in the map
        match self.nodes.entry(b) {
            Vacant(ent) => { ent.set(Vec::new()); }
            _ => {}
        }

        match self.nodes.entry(a) {
            Occupied(ent) => {
                // Add edge only if it isn't already there
                let edges = ent.into_mut();
                if edges.iter().position(|&(elt, _)| elt == b).is_none() {
                    edges.push((b, edge));
                    true
                } else {
                    false
                }
            }
            Vacant(ent) => {
                ent.set(vec![(b, edge)]);
                true
            }
        }
    }

    /// Remove edge from **a** to **b** from the graph.
    ///
    /// Return **None** if the edge didn't exist.
    pub fn remove_edge(&mut self, a: N, b: N) -> Option<E>
    {
        match self.nodes.entry(a) {
            Occupied(mut ent) => {
                match ent.get().iter().position(|&(elt, _)| elt == b) {
                    Some(index) => {
                        ent.get_mut().swap_remove(index).map(|(_, edge)| edge)
                    }
                    None => None,
                }
            }
            Vacant(..) => None,
        }
    }

    /// Return **true** if the directed edge from **a** to **b** is contained in the graph.
    pub fn contains_edge(&mut self, a: N, b: N) -> bool
    {
        match self.nodes.get(&a) {
            None => false,
            Some(sus) => sus.iter().any(|&(elt, _)| elt == b),
        }
    }

    /// Return an iterator over the nodes of the graph.
    ///
    /// Iterator element type is **&'a N**.
    pub fn nodes<'a>(&'a self) -> Nodes<'a, N, E>
    {
        Nodes{iter: self.nodes.keys()}
    }

    /// Return an iterator over the nodes that are connected with **from** by edges.
    ///
    /// If the node **from** does not exist in the graph, return an empty iterator.
    ///
    /// Iterator element type is **&'a N**.
    pub fn neighbors(&self, from: N) -> Neighbors<N, E>
    {
        fn fst<'a, N: Copy, E>(t: &'a (N, E)) -> &'a N
        {
            &t.0
        }

        Neighbors{iter: self.edges(from).map(fst)}
    }

    /// Return an iterator over the nodes that are connected with **from** by edges,
    /// paired with the edge weight.
    ///
    /// If the node **from** does not exist in the graph, return an empty iterator.
    ///
    /// Iterator element type is **&'a (N, E)**.
    pub fn edges<'a>(&'a self, from: N) -> Items<'a, (N, E)>
    {
        match self.nodes.get(&from) {
            Some(edges) => edges.iter(),
            None => [].iter(),
        }
    }

    /// Return an iterator over the nodes that are connected with **from** by edges,
    /// paired with the edge weight.
    ///
    /// If the node **from** does not exist in the graph, return an empty iterator.
    ///
    /// Iterator element type is **&'a mut (N, E)**.
    pub fn edges_mut<'a>(&'a mut self, from: N) -> MutItems<'a, (N, E)>
    {
        match self.nodes.get_mut(&from) {
            Some(edges) => edges.iter_mut(),
            None => [].iter_mut(),
        }
    }

    /// Return a reference to the edge weight connecting **a** with **b**, or
    /// **None** if the edge does not exist in the graph.
    pub fn edge<'a>(&'a self, a: N, b: N) -> Option<&'a E>
    {
        match self.nodes.get(&a) {
            Some(succ) => {
                succ.iter()
                    .find(|&&(ref n, _)| n == &b)
                    .map(|&(_, ref edge)| edge)
            }
            None => None,
        }
    }

    /// Return a mutable reference to the edge weight connecting **a** with **b**, or
    /// **None** if the edge does not exist in the graph.
    pub fn edge_mut<'a>(&'a mut self, a: N, b: N) -> Option<&'a mut E>
    {
        match self.nodes.get_mut(&a) {
            Some(succ) => {
                succ.iter_mut()
                    .find(|&&(ref n, _)| n == &b)
                    .map(|&(_, ref mut edge)| edge)
            }
            None => None,
        }
    }

}

impl<N, E> DiGraph<N, E> where N: Copy + Eq + Hash, E: Clone
{
    /// Add a directed edges from **a** to **b** and from **b** to **a** to the
    /// graph.
    ///
    /// Return **true** if at least one of the edges did not previously exist.
    pub fn add_diedge(&mut self, a: N, b: N, edge: E) -> bool
    {
        self.add_edge(a, b, edge.clone()) |
        self.add_edge(b, a, edge)
    }

    /// Return a cloned graph with all edges reversed.
    pub fn reversed(&self) -> DiGraph<N, E>
    {
        let mut g = DiGraph::new();
        for &node in self.nodes() {
            for &(other, ref edge) in self.edges(node) {
                g.add_edge(other, node, edge.clone());
            }
        }
        g
    }
}

macro_rules! iterator_methods(
    ($elt_type:ty) => (
        #[inline]
        fn next(&mut self) -> Option<$elt_type>
        {
            self.iter.next()
        }

        #[inline]
        fn size_hint(&self) -> (uint, Option<uint>)
        {
            self.iter.size_hint()
        }
    )
)

pub struct Nodes<'a, N: 'a, E: 'a> {
    iter: Keys<'a, N, Vec<(N, E)>>
}

impl<'a, N: 'a, E: 'a> Iterator<&'a N> for Nodes<'a, N, E>
{
    iterator_methods!(&'a N)
}

type MapPtr<'a, From, To, Iter> = Map<&'a From, &'a To, Iter, for<'b> fn(&'b From) -> &'b To>;

pub struct Neighbors<'a, N: 'a, E: 'a> {
    iter: MapPtr<'a, (N, E), N, Items<'a, (N, E)>>,
}

impl<'a, N: 'a, E: 'a> Iterator<&'a N> for Neighbors<'a, N, E>
{
    iterator_methods!(&'a N)
}

