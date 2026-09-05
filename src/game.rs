#![allow(dead_code)]

use crate::game::{board::BoardGraph, state::VertexState};

pub mod board;
pub mod state;

pub struct Game {
    board: BoardGraph,
    occupancy: Vec<VertexState>,
}
