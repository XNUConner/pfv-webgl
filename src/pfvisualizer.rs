pub mod visualizerwindowhandler;
pub mod visualizerinputhandler;
pub mod dimensions;

use crate::pfvisualizer::visualizerwindowhandler::VisualizerWindowHandler;
use speedy2d::WebCanvas;

pub struct PFVisualizer {}

impl PFVisualizer {

    pub fn start() {

        let visualizer_window_handler = VisualizerWindowHandler::new();

        WebCanvas::new_for_id("my_canvas", visualizer_window_handler).unwrap();
    }

}

