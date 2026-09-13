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
use model::HexGoModel;

use crate::{self_play::generate_self_play_games, train::train_on_samples};
type Backend = Autodiff<Flex>;

const SELF_PLAY_GAMES: usize = 32;
const MCTS_ITERATIONS: usize = 32;
fn main() {
    let device = Default::default();

    let samples = generate_self_play_games(SELF_PLAY_GAMES, MCTS_ITERATIONS);

    println!("generated {} samples", samples.len());

    let mut model = HexGoModel::<Backend>::new(&device);

    let mut optimizer = AdamConfig::new().init();

    for step in 0..100 {
        let (new_model, loss) = train_on_samples(model, &mut optimizer, &samples, &device, 1e-3);

        model = new_model;

        println!("step={step}, loss={loss}");
    }
}
