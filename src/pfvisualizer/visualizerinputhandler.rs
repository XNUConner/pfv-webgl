pub mod mousehandler;
pub mod keyboardhandler;

use crate::pfvisualizer::visualizerinputhandler::mousehandler::MouseHandler;

pub struct VisualizerInputHandler {
    pub mouse: MouseHandler,
}

impl VisualizerInputHandler {
    pub fn new() -> Self {
        VisualizerInputHandler {
            mouse: MouseHandler::new(),
        }
    }
}