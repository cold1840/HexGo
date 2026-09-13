#![allow(dead_code)]
use burn::tensor::{Tensor, backend::Backend, loss::cross_entropy_with_logits};

pub fn policy_loss<B: Backend>(logits: Tensor<B, 2>, target: Tensor<B, 2>) -> Tensor<B, 1> {
    cross_entropy_with_logits(logits, target)
}

pub fn value_loss<B: Backend>(prediction: Tensor<B, 2>, target: Tensor<B, 2>) -> Tensor<B, 2> {
    (prediction - target).powf_scalar(2.0)
}

pub fn total_loss<B: Backend>(
    policy_logits: Tensor<B, 2>,
    target_policy: Tensor<B, 2>,
    value_prediction: Tensor<B, 2>,
    target_value: Tensor<B, 2>,
) -> Tensor<B, 1> {
    let policy = policy_loss(policy_logits, target_policy).mean();
    let value = value_loss(value_prediction, target_value).mean();

    policy + value
}
