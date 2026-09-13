use crate::game::{Game, board::VertexId, player::Player};

pub struct Evaluation {
    pub policy: Vec<(VertexId, f32)>,
    pub value: f32,
}

pub trait NeuralNetwork {
    fn evaluate(&self, game: &Game, player: Player) -> Evaluation;
}
