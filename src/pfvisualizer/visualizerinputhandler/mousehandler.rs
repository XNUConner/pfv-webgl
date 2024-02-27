use crate::pfvisualizer::dimensions::Dimensions;
use crate::graphtraversalbundle::GraphTraversalBundle;
use crate::graph::indicies::Indicies;
use crate::graph::node::Node;

use speedy2d::window::MouseButton;

pub struct MouseHandler {
        cursor_x: f32,
        cursor_y: f32,
        leftmouse_depressed: bool,
        rightmouse_depressed: bool,
        hovered_graph_indicies: Option<Indicies>,
        selected_node: Node,
}

impl MouseHandler {

    pub fn new() -> Self {
        MouseHandler {
            cursor_x: 0.0,
            cursor_y: 0.0,
            leftmouse_depressed: false,
            rightmouse_depressed: false,
            hovered_graph_indicies: None,
            selected_node: Node::Wall,
        }
    }

    pub fn select_node(&mut self, node: Node) {
        self.selected_node = node;
    }

    pub fn update_cursor_position(&mut self, x: f32, y: f32, window_dimensions: &Dimensions, bundle: &mut GraphTraversalBundle) {
        self.cursor_x = x;
        self.cursor_y = y;
        self.hovered_graph_indicies = self.get_hovered_indicies(window_dimensions, bundle);

        if self.leftmouse_depressed || self.rightmouse_depressed {

            if let Some(_indicies) = &self.hovered_graph_indicies {
                self.button_clicked(MouseButton::Left, true, bundle);
            }

        }

    }

    pub fn button_clicked(&mut self, button: MouseButton, depressed: bool, bundle: &mut GraphTraversalBundle) {

        match button {

            MouseButton::Left => {
                self.leftmouse_depressed = depressed;

                if depressed {
                    if let Some(indicies) = &self.hovered_graph_indicies {

                        match self.selected_node {
                            Node::Wall => bundle.set_wall(indicies.row, indicies.col),
                            Node::Normal => bundle.set_normal(indicies.row, indicies.col),
                            Node::Goal => bundle.set_goal(indicies.row, indicies.col),
                            Node::Start => bundle.set_start(indicies.row, indicies.col),
                        }

                    }
                }

            },

            _ => {},

        };

    }

    // Helper func
    fn get_hovered_indicies(&self, window_dimensions: &Dimensions, bundle: &GraphTraversalBundle) -> Option<Indicies> {

        let graph_rows = bundle.graph_rows() as u32;
        let graph_columns = bundle.graph_columns() as u32;

        let ratio_x = window_dimensions.width as f32 / graph_rows as f32;
        let ratio_y = window_dimensions.height as f32 / graph_columns as f32;

        let col = (self.cursor_x / ratio_x) as u32;
        let row = (self.cursor_y / ratio_y) as u32;

        if row >= graph_rows || col >= graph_columns {
            // Cursor out of bounds
            return None;
        }

        Some(
            Indicies {
                row: row as usize,
                col: col as usize,
            }
        )

    }
}