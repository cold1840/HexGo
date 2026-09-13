#![allow(dead_code)]

pub const INPUT_SIZE: usize = 88 * 3;
pub struct TrainingSample {
    pub state: Vec<f32>,
    pub policy: Vec<f32>,
    pub value: f32,
}
