mod graph;
mod algorithm;
mod graphtraversalbundle;
mod pfvisualizer;
mod renderer;

use crate::pfvisualizer::PFVisualizer;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    PFVisualizer::start();
}
