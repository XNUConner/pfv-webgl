use std::collections::BTreeSet;
use std::collections::HashSet;

use crate::graph::indicies::Indicies;
use crate::graph::Graph;
use crate::graph::node::Node;

use crate::algorithm::Algorithm;

pub struct Bfs {
    graph: Graph,
    visited: BTreeSet<Indicies>,
    to_visit: HashSet<Indicies>,
    pub iteration: usize,
    pub finished: bool,
}

impl Algorithm for Bfs {

    fn algo_name(&self) -> String {
        "BFS".to_string()
    }

    fn update_graph(&mut self, new_graph: Graph) {
        let need_iteration = self.iteration;

        self.visited.clear();
        self.to_visit.clear();
        self.to_visit.insert( new_graph.get_start_pos() );
        self.iteration = 0;
        self.finished = false;
        self.graph = new_graph;

        while !self.finished && self.iteration != need_iteration {
            self.step();
        }

    }

    fn step(&mut self) {
        if self.finished { return; }

        let mut new_set = HashSet::new();

        // Avoid borrow checker issue
        let copy_set = self.to_visit.clone();
        for pos in copy_set {

            // Visit node's neighbors
            for neighbor in self.adjacent_indicies(&pos) {
                new_set.insert(neighbor.clone());
            }

            // Mark node visited
            self.visited.insert(pos);

        }

        self.finished = new_set.is_empty();
        self.to_visit = new_set;
        self.iteration += 1;

    }

    // INEFFICIENT WORKAROUND
    // RECALCULATES ENTIRE GRAPH INSTEAD OF JUST HAVING BFS HOLD FRAMES
    fn back(&mut self) {

        if self.iteration == 0 {
            return;
        }

        let need_iteration = self.iteration - 1; 
        self.visited.clear();
        self.to_visit.clear();
        self.to_visit.insert( self.graph.get_start_pos() );
        self.iteration = 0;
        self.finished = false;

        while !self.finished && self.iteration != need_iteration {
            self.step();
        }

    }

    fn reset(&mut self) {
        self.visited.clear();
        self.to_visit.clear();
        self.to_visit.insert( self.graph.get_start_pos() );
        self.iteration = 0;
        self.finished = false;
    }

    fn is_node_visited_at(&self, row: usize, col: usize) -> bool {
        self.already_visited( &Indicies { row, col } )
    }

}

impl Bfs {
    pub fn new(graph: Graph) -> Self {

        let mut set = HashSet::new();
        set.insert( graph.get_start_pos() );

        Bfs {
            graph, 
            visited: BTreeSet::new(),
            to_visit: set,
            iteration: 0,
            finished: false,
        }

    }

    fn adjacent_indicies(&mut self, pos: &Indicies) -> Vec<Indicies> {

        if self.is_wall( &pos ) { return vec![]; }
        if self.already_visited( &pos ) { return vec![]; }

        // Process adjacent Indicies
        let mut adjacent = vec![];
        self.process_top_pos(pos, &mut adjacent);
        self.process_bottom_pos(pos, &mut adjacent);
        self.process_right_pos(pos, &mut adjacent);
        self.process_left_pos(pos, &mut adjacent);

        return adjacent;
    }

    fn is_wall(&self, pos: &Indicies) -> bool {
        self.graph.node_at(pos.row, pos.col) == Node::Wall
    }

    fn already_visited(&self, pos: &Indicies) -> bool {
        self.visited.contains( pos )
    }

    fn process_top_pos(&self, pos: &Indicies, vec: &mut Vec<Indicies>) {
        self.process_pos(pos.row != 0, -1, 0, pos, vec);
    }

    fn process_bottom_pos(&self, pos: &Indicies, vec: &mut Vec<Indicies>) {
        self.process_pos(pos.row != self.graph.rows()-1, 1, 0, pos, vec);
    }

    fn process_right_pos(&self, pos: &Indicies, vec: &mut Vec<Indicies>) {
        self.process_pos(pos.col != 0, 0, -1, pos, vec);
    }

    fn process_left_pos(&self, pos: &Indicies, vec: &mut Vec<Indicies>) {
        self.process_pos(pos.col != self.graph.columns()-1, 0, 1, pos, vec);
    }

    fn process_pos(&self, bounds_check_expression: bool, row_offset: i32, col_offset: i32, pos: &Indicies, vec: &mut Vec<Indicies>) {
        if bounds_check_expression == false { return; }

        let row = (pos.row as i32 + row_offset) as usize;
        let col = (pos.col as i32 + col_offset) as usize;

        let adjacent_position = Indicies { row: row, col: col };
        if !self.already_visited( &adjacent_position ) {
            vec.push(adjacent_position);
        }
    }
}