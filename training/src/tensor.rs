#![allow(dead_code)]
use burn::tensor::{Tensor, TensorData, backend::Backend};
use hex_go::game::action::ACTION_SIZE;

use crate::dataset::TrainingSample;

pub fn samples_to_tensors<B: Backend>(
    samples: &[TrainingSample],
    device: &B::Device,
) -> (Tensor<B, 2>, Tensor<B, 2>, Tensor<B, 2>) {
    let batch_size = samples.len();

    let mut states = Vec::with_capacity(batch_size * 264);
    let mut policies = Vec::with_capacity(batch_size * ACTION_SIZE);
    let mut values = Vec::with_capacity(batch_size);

    for sample in samples {
        states.extend_from_slice(&sample.state);
        policies.extend_from_slice(&sample.policy);
        values.push(sample.value);
    }

    let states = Tensor::<B, 1>::from_data(TensorData::new(states, [batch_size * 264]), device)
        .reshape([batch_size, 264]);

    let policies = Tensor::<B, 1>::from_data(
        TensorData::new(policies, [batch_size * ACTION_SIZE]),
        device,
    )
    .reshape([batch_size, ACTION_SIZE]);

    let values = Tensor::<B, 1>::from_data(TensorData::new(values, [batch_size]), device)
        .reshape([batch_size, 1]);

    (states, policies, values)
}

#[cfg(test)]
mod test {
    use burn::backend::Flex;

    use super::*;
    #[test]
    fn samples_to_tensors_has_expected_shapes() {
        let samples = vec![
            TrainingSample {
                state: vec![0.0; 264],
                policy: vec![0.0; ACTION_SIZE],
                value: 1.0,
            },
            TrainingSample {
                state: vec![1.0; 264],
                policy: vec![0.5; ACTION_SIZE],
                value: -1.0,
            },
        ];

        let device = Default::default();

        let (state, policy, value) = samples_to_tensors::<Flex>(&samples, &device);

        assert_eq!(state.dims(), [2, 264]);
        assert_eq!(policy.dims(), [2, ACTION_SIZE]);
        assert_eq!(value.dims(), [2, 1]);
    }
}
