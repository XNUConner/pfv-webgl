pub mod indicies;
pub mod node;

use crate::graph::indicies::Indicies;
use crate::graph::node::Node;

#[derive(Clone)]
pub struct Graph {
    columns: usize,
    rows: usize,
    nodes: Vec<Vec<Node>>,
    start: Indicies,
}

impl Graph {
    pub fn new(rows: usize, columns: usize) -> Self {

        let mut g = Graph { 
            rows, 
            columns, 
            nodes: vec![vec![Node::Normal; columns]; rows],
            start: Indicies { row: 0, col: 0 }, /* tmp value */
        };

        let center_row = (rows / 2) - 1;
        let center_col = (rows / 2) - 1;
        g.set_start( center_row, center_col );

        g

    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn columns(&self) -> usize {
        self.columns
    }

    pub fn node_at(&self, row: usize, col: usize) -> Node {
        assert!(row < self.rows && col < self.columns);
        self.nodes[row][col].clone()
    }

    pub fn set_start(&mut self, new_row: usize, new_col: usize) {
        if self.node_at(new_row, new_col) != Node::Normal { return; }
        // Get current position of start
        let row = self.start.row;
        let col = self.start.col;
        self.change_node(row, col, Node::Normal);
        self.change_node(new_row, new_col, Node::Start);
        self.start = Indicies { row: new_row, col: new_col};
    }

    pub fn get_start_pos(&self) -> Indicies {
        self.start.clone()
    }

    pub fn set_wall(&mut self, row: usize, col: usize) {
        if self.node_at(row, col) != Node::Normal { return; }
        self.change_node(row, col, Node::Wall);
    }

    pub fn set_goal(&mut self, row: usize, col: usize) {
        if self.node_at(row, col) != Node::Normal{ return; }
        self.change_node(row, col, Node::Goal);
    }

    pub fn set_normal(&mut self, row: usize, col: usize) {
        if self.node_at(row, col) == Node::Start { return; }
        self.change_node(row, col, Node::Normal);
    }

    fn change_node(&mut self, row: usize, col: usize, node: Node) {
        assert!(row < self.rows && col < self.columns);

        self.nodes[row][col] = node;

    }
}