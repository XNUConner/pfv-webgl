pub mod bfs;
// pub mod dfs
// pub mod astar

use crate::graph::Graph;

pub trait Algorithm {
    fn algo_name(&self) -> String;
    fn step(&mut self);
    fn back(&mut self);
    fn reset(&mut self);
    fn update_graph(&mut self, new_graph: Graph);
    fn is_node_visited_at(&self, row: usize, col: usize) -> bool;
}

// List of algorithms available to visualizer
pub enum AlgorithmType {
    BFS,
}
