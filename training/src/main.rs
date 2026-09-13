mod dataset;
mod loss;
mod model;
mod self_play;
mod tensor;
mod train;
use burn::{
    backend::{Autodiff, Flex},
    optim::AdamConfig,
};
use hex_go::{
    ai::neural_mcts::{DummyNetwork, NeuralMcts},
    board_layout::BoardDefinition,
    game::Game,
};
use model::HexGoModel;

use crate::{self_play::play_game, train::train_on_samples};
type Backend = Autodiff<Flex>;
fn main() {
    let device = Default::default();

    let board = BoardDefinition::compact().graph().clone();

    let mut game = Game::new(board);

    let mut mcts = NeuralMcts::new(DummyNetwork);

    let samples = play_game(&mut game, &mut mcts);

    println!("generated {} samples", samples.len());

    let mut model = HexGoModel::<Backend>::new(&device);

    let mut optimizer = AdamConfig::new().init();

    for step in 0..100 {
        let (new_model, loss) = train_on_samples(model, &mut optimizer, &samples, &device, 1e-3);

        model = new_model;

        println!("step={step}, loss={loss}");
    }
}
