#![allow(dead_code)]
use burn::{
    Tensor,
    optim::{GradientsParams, Optimizer},
    tensor::{ElementConversion, backend::AutodiffBackend},
};

use crate::{loss::total_loss, model::HexGoModel};

pub fn train_step<B: AutodiffBackend>(
    model: HexGoModel<B>,
    input: Tensor<B, 2>,
    target_policy: Tensor<B, 2>,
    target_value: Tensor<B, 2>,
    optimizer: &mut impl Optimizer<HexGoModel<B>, B>,
    learning_rate: f64,
) -> (HexGoModel<B>, f32) {
    let output = model.forward(input);

    let loss = total_loss(output.policy, target_policy, output.value, target_value);

    let loss_value = loss.clone().into_scalar().elem::<f32>();

    let grads = loss.backward();

    let grads = GradientsParams::from_grads(grads, &model);

    let model = optimizer.step(learning_rate, model, grads);

    (model, loss_value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{dataset::TrainingSample, model::HexGoModel, tensor::samples_to_tensors};
    use burn::{backend::Autodiff, backend::Flex, optim::AdamConfig};
    use hex_go::game::action::ACTION_SIZE;

    type Backend = Autodiff<Flex>;

    #[test]
    fn train_step_reduces_loss() {
        let device = Default::default();

        let samples = vec![TrainingSample {
            state: vec![0.0; 264],
            policy: {
                let mut p = vec![0.0; ACTION_SIZE];
                p[0] = 1.0;
                p
            },
            value: 1.0,
        }];

        let (input, target_policy, target_value) = samples_to_tensors::<Backend>(&samples, &device);

        let mut model = HexGoModel::<Backend>::new(&device);
        let mut optimizer = AdamConfig::new().init();

        let mut initial_loss = None;
        let mut final_loss = 0.0;

        for step in 0..100 {
            let (new_model, loss) = train_step(
                model,
                input.clone(),
                target_policy.clone(),
                target_value.clone(),
                &mut optimizer,
                1e-3,
            );

            model = new_model;

            if step == 0 {
                initial_loss = Some(loss);
            }

            final_loss = loss;
        }

        let initial_loss = initial_loss.unwrap();

        println!("initial loss: {initial_loss}");
        println!("final loss:   {final_loss}");

        assert!(
            final_loss < initial_loss,
            "loss did not decrease: initial={initial_loss}, final={final_loss}"
        );
    }
}
