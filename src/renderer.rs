use crate::graphtraversalbundle::GraphTraversalBundle;

use crate::graph::node::Node;

use crate::pfvisualizer::dimensions::Dimensions;

use speedy2d::Graphics2D;
use speedy2d::color::Color;
use speedy2d::dimen::Vector2;
use speedy2d::window::WindowHelper;

pub struct Renderer {}

impl Renderer {
    pub fn render_graph(dimensions: &Dimensions, bundle: &GraphTraversalBundle, graphics: &mut Graphics2D) {

        // go through the nodes in the graph, render each node
        let graph_rows = bundle.graph_rows();
        let graph_columns = bundle.graph_columns();

        for row in 0..graph_rows {
            for col in 0..graph_columns {
                let node = bundle.node_at(row, col);
                let visited = bundle.is_node_visited_at(row, col);
                Self::draw_node_at(&node, row, col, graph_rows, graph_columns, dimensions, visited, graphics);
            }
        }

    }

    pub fn redraw(helper: &mut WindowHelper) {
        helper.request_redraw();
    }

    fn get_y_for_row(row: usize, quad_height: f32) -> f32 {
        row as f32 * quad_height
    }

    fn get_x_for_col(col: usize, quad_width: f32) -> f32 {
        col as f32 * quad_width
    }

    fn draw_node_at(node: &Node, row: usize, col: usize, graph_rows: usize, graph_columns: usize, window_dimensions: &Dimensions, visited: bool, graphics: &mut Graphics2D) {

        let quad_color = Self::get_color_for_node(node, visited);

        // Ideally this function doesn't need to be run for every individual node, only when the window is resized and store the dimensions somewhere else.
        // It would require renderer to be be a struct that can be constructed
        // And a function like Renderer.update_window_dimensions() which would recalculate the quad
        let (quad_height, quad_width) = Self::calculate_dimensions_for_quad(window_dimensions, graph_rows, graph_columns);

        let y = Self::get_y_for_row(row, quad_height);
        let x = Self::get_x_for_col(col, quad_width);

        Self::draw_quad_at(y, x, quad_height, quad_width, quad_color, graphics);

    }

    fn get_color_for_node(node: &Node, visited: bool) -> Color {

        match node {

            Node::Normal => {

                if visited { 
                    Color::GRAY
                } else {
                    Color::WHITE
                }

            },

            Node::Wall => Color::BLACK,
            Node::Goal => Color::GREEN,
            Node::Start => Color::YELLOW,
        }

    }

    fn calculate_dimensions_for_quad(window_dimensions: &Dimensions, graph_rows: usize, graph_columns: usize) -> (/* height */ f32, /* width */ f32) {
        let quad_width = window_dimensions.width as f32 / graph_rows as f32;
        let quad_height = window_dimensions.height as f32 / graph_columns as f32;

        (quad_height, quad_width)
    }

    fn draw_quad_at(y: f32, x: f32, height: f32, width: f32, color: Color, graphics: &mut Graphics2D) {
        let quad = Self::create_quad(height, width, y, x);
        graphics.draw_quad(quad, color);
    }

    fn create_quad(height: f32, width: f32, y: f32, x: f32) -> [Vector2<f32>; 4] {
    
        [
            Vector2::new(width + x, height + y),
            Vector2::new(width + x, y),
            Vector2::new(x, y),
            Vector2::new(x, height + y),
        ]

    }

}