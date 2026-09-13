mod dataset;
mod loss;
mod model;
mod self_play;
mod tensor;
mod train;
use burn::backend::Flex;
use model::HexGoModel;

fn main() {
    type Backend = Flex;

    let device = Default::default();
    let model = HexGoModel::<Backend>::new(&device);

    let input = burn::tensor::Tensor::<Backend, 2>::zeros([1, 264], &device);

    let output = model.forward(input);

    println!("policy shape: {:?}", output.policy.dims());
    println!("value shape: {:?}", output.value.dims());
}
