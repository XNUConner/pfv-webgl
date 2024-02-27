use crate::graph::Graph;
use crate::graph::node::Node;
use crate::algorithm::Algorithm;
use crate::algorithm::AlgorithmType;
use crate::algorithm::bfs;


// Basic program structure (Struct structure not module hierachy)
// VisualizerWindowHandler
// \-> GraphTraversalBundle
// \-> InputHandler (Passes a reference to &GraphTraversalBundle around?)
//     \-> Keyboard
//     \-> Mouse

pub struct GraphTraversalBundle {
    graph: Graph,
    algorithm: Option< Box<dyn Algorithm> >,
}

impl GraphTraversalBundle {

    pub fn new(rows: usize, columns: usize) -> Self {
        assert!(rows > 0);
        assert!(columns > 0);

        GraphTraversalBundle {
            graph: Graph::new(rows, columns),
            algorithm: None,
        }

    }

    pub fn reset_graph(&mut self) {
        let rows = self.graph.rows();
        let cols = self.graph.columns();
        self.graph = Graph::new(rows, cols);
        self.update_algorithms_graph();
    } 

    pub fn next_algorithm_step(&mut self) {

        if let Some(alg) = &mut self.algorithm {
            alg.step();
        }

    }

    pub fn prev_algorithm_step(&mut self) {

        if let Some(alg) = &mut self.algorithm {
            alg.back();
        }

    }

    pub fn set_algorithm(&mut self, algorithm: AlgorithmType) {
        self.algorithm = match algorithm {
            AlgorithmType::BFS => Some( Box::new( bfs::Bfs::new( self.graph.clone() ))),
        }
    }
    
    pub fn set_normal(&mut self, row: usize, col: usize) {
        self.graph.set_normal(row, col);
        self.update_algorithms_graph();
    }

    pub fn set_wall(&mut self, row: usize, col: usize) {
        self.graph.set_wall(row, col);
        self.update_algorithms_graph();
    }

    pub fn set_goal(&mut self, row: usize, col: usize) {
        self.graph.set_goal(row, col);
        self.update_algorithms_graph();
    }

    pub fn set_start(&mut self, row: usize, col: usize) {
        self.graph.set_start(row, col);
        self.update_algorithms_graph();
    }

    pub fn graph_rows(&self) -> usize {
        self.graph.rows()
    }

    pub fn graph_columns(&self) -> usize {
        self.graph.columns()
    }

    pub fn node_at(&self, row: usize, col: usize) -> Node {
        self.graph.node_at(row, col)
    }

    pub fn is_node_visited_at(&self, row: usize, col: usize) -> bool {
        if let Some(alg) = &self.algorithm {
            alg.is_node_visited_at(row, col)
        } else {
            panic!("is_node_visited_at() called but no algorithm has been set!");
        }
    }

    pub fn reset_algorithm(&mut self) {
        if let Some(alg) = &mut self.algorithm {
            alg.reset();
        }
    }

    fn update_algorithms_graph(&mut self) {
        if let Some(alg) = &mut self.algorithm {
            alg.update_graph( self.graph.clone() )
        }
    }

}