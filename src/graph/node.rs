#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)] 
pub enum Node {
    Normal,
    Wall,
    Goal,
    Start,
}