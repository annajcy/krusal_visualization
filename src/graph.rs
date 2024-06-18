use bevy::prelude::*;

use crate::edge::*;
use crate::union_find::*;

pub const MAX_NODE_INDEX : usize = 1000;

#[derive(Resource, Debug)]
pub struct Graph {
    pub edges : Vec<Edge>,
    pub nodes : Vec<usize>,
}

impl Default for Graph {
    fn default() -> Graph {
        Graph {
            edges : vec![],
            nodes: vec![],
        }
    }
}

impl Graph {
    pub fn new(edge_: Vec<Edge>, nodes_: Vec<u32>) -> Graph {
        Graph {edges : vec![], nodes: vec![] }
    }

    pub fn add_edge (&mut self, u : usize, v : usize, w : i32) {
        if self.nodes.contains(&u) && self.nodes.contains(&v) {
            self.edges.push(Edge::new(u, v, w));
        }
    }

    pub fn remove_edge (&mut self, u : usize, v : usize, w : i32) {
        self.edges.retain(|edge|
            edge.u != u || edge.v != v || edge.w != w
        );
    }

    pub fn add_node (&mut self, id : usize) {
        if !self.nodes.contains(&id) {
            self.nodes.push(id);
        }
    }

    pub fn remove_node (&mut self, id : usize) {
        self.edges.retain(|edge|
            edge.u != id && edge.v != id
        );
        self.nodes.retain(|node_id|
            *node_id != id
        );
    }

    pub fn node_max_id (&self) -> usize {
        *self.nodes.iter().max().unwrap()
    }

    pub fn is_connected (&self) -> bool {
        let mut uf = UnionFind::new(self.node_max_id());
        for edge in self.edges.iter() {
            uf.union(edge.u, edge.v);
        }

        let id = uf.find(self.nodes[0]);
        for node in self.nodes.iter() {
            let cid = uf.find(*node);
            if cid != id {
                return false;
            }
        }
        return true;
    }

    pub fn kruskal(&self) -> Graph {
        let mut edges = vec![];
        for edge in self.edges.iter() {
            edges.push(Edge {
                u: edge.u,
                v: edge.v,
                w: edge.w,
            })
        }
        edges.sort();

        let mut uf = UnionFind::new(MAX_NODE_INDEX);
        let mut mst = Graph::default();
        for edge in edges.iter() {
            if uf.union(edge.u, edge.v) {
                mst.add_node(edge.u);
                mst.add_node(edge.v);
                mst.add_edge(edge.u, edge.v, edge.w);
            }
        }
        mst
    }
}