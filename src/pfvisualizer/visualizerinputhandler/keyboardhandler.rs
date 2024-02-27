use speedy2d::window::VirtualKeyCode;
use crate::graphtraversalbundle::GraphTraversalBundle;
use crate::pfvisualizer::visualizerinputhandler::mousehandler::MouseHandler;
use crate::graph::node::Node;

pub struct KeyboardHandler {}

impl KeyboardHandler {

    pub fn key_pressed(vkc: VirtualKeyCode, bundle: &mut GraphTraversalBundle, mouse_handler: &mut MouseHandler) {
        
        match vkc {
            VirtualKeyCode::N => bundle.next_algorithm_step(),
            VirtualKeyCode::B => bundle.prev_algorithm_step(),
            VirtualKeyCode::R => bundle.reset_algorithm(),
            VirtualKeyCode::G => bundle.reset_graph(),
            VirtualKeyCode::Key1 => mouse_handler.select_node(Node::Start),
            VirtualKeyCode::Key2 => mouse_handler.select_node(Node::Wall),
            VirtualKeyCode::Key3 => mouse_handler.select_node(Node::Goal),
            VirtualKeyCode::Key4 => mouse_handler.select_node(Node::Normal),
            _ => {},
        }

    }

}