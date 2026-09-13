use crate::game::{Game, action::Action, player::Player};

pub struct Evaluation {
    pub policy: Vec<(Action, f32)>,
    pub value: f32,
}

pub trait NeuralNetwork {
    fn evaluate(&self, game: &Game, player: Player) -> Evaluation;
}
