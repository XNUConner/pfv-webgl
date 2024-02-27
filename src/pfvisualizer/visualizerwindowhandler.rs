use crate::graphtraversalbundle::GraphTraversalBundle;
use crate::algorithm::AlgorithmType;
use crate::pfvisualizer::dimensions::Dimensions;
use crate::pfvisualizer::visualizerinputhandler::VisualizerInputHandler;
use crate::pfvisualizer::visualizerinputhandler::keyboardhandler::KeyboardHandler;
use crate::renderer::Renderer;

const GRAPH_ROWS: usize = 40;
const GRAPH_COLUMNS: usize = 40;

pub struct VisualizerWindowHandler {
    bundle: GraphTraversalBundle,
    window_dimensions: Dimensions,
    input_handler: VisualizerInputHandler,
}

impl VisualizerWindowHandler {
    pub fn new() -> Self {

        VisualizerWindowHandler {
            bundle: GraphTraversalBundle::new(GRAPH_ROWS, GRAPH_COLUMNS),
            window_dimensions: Dimensions { height: 0, width: 0 }, // Placeholder, actually set by on_start
            input_handler: VisualizerInputHandler::new(),
        }
    }
}

use speedy2d::window::{
    WindowHandler, 
    WindowHelper, 
    MouseButton,
    WindowStartupInfo,
    VirtualKeyCode,
    KeyScancode,
};

use speedy2d::Graphics2D;
use speedy2d::dimen::Vec2;
use speedy2d::dimen::UVec2;

impl WindowHandler for VisualizerWindowHandler {

    fn on_start(&mut self, _helper: &mut WindowHelper, info: WindowStartupInfo) {
       self.window_dimensions.height = info.viewport_size_pixels().y;
       self.window_dimensions.width = info.viewport_size_pixels().x;
       self.bundle.set_algorithm(AlgorithmType::BFS);
    }

    fn on_draw(&mut self, _helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        Renderer::render_graph(&self.window_dimensions, &self.bundle, graphics);
    }

    fn on_mouse_button_down(&mut self, helper: &mut WindowHelper, button: MouseButton) {
        self.input_handler.mouse.button_clicked(button, /* depressed */ true, &mut self.bundle);
        Renderer::redraw(helper);
    }

    fn on_mouse_button_up(&mut self, helper: &mut WindowHelper, button: MouseButton) {
        self.input_handler.mouse.button_clicked(button, /* depressed */ false, &mut self.bundle);
        Renderer::redraw(helper);
    }

    fn on_mouse_move(&mut self, helper: &mut WindowHelper, position: Vec2) {
        self.input_handler.mouse.update_cursor_position(position.x, position.y, &self.window_dimensions, &mut self.bundle);
        Renderer::redraw(helper);
    }

    fn on_resize(&mut self, _helper: &mut WindowHelper, size_pixels: UVec2) {
        self.window_dimensions.width = size_pixels.x;
        self.window_dimensions.height = size_pixels.y;

    }

    fn on_key_down(&mut self, helper: &mut WindowHelper, virtual_key_code: Option<VirtualKeyCode>, _scancode: KeyScancode) {
        if let Some(vkc) = virtual_key_code {
            KeyboardHandler::key_pressed(vkc, &mut self.bundle, &mut self.input_handler.mouse);
            Renderer::redraw(helper);
        }
    }

}