use burn::{
    module::Module,
    nn::conv::{ConvTranspose2d, ConvTranspose2dConfig},
    tensor::{backend::Backend, Tensor},
};

#[derive(Module, Debug)]
struct Net<B: Backend> {
    conv1: ConvTranspose2d<B>,
    conv2: ConvTranspose2d<B>,
}

impl<B: Backend> Net<B> {
    /// Create a new model from the given record.
    pub fn new_with(record: NetRecord<B>) -> Self {
        let conv1 = ConvTranspose2dConfig::new([2, 2], [2, 2]).init_with(record.conv1);
        let conv2 = ConvTranspose2dConfig::new([2, 2], [2, 2]).init_with(record.conv2);
        Self { conv1, conv2 }
    }

    /// Forward pass of the model.
    pub fn forward(&self, x: Tensor<B, 4>) -> Tensor<B, 4> {
        let x = self.conv1.forward(x);

        self.conv2.forward(x)
    }
}

#[cfg(test)]
mod tests {
    type Backend = burn_ndarray::NdArray<f32>;

    use burn::record::{FullPrecisionSettings, HalfPrecisionSettings, Recorder};
    use burn_import::pytorch::PyTorchFileRecorder;

    use super::*;

    fn conv_transpose2d(record: NetRecord<Backend>, precision: usize) {
        let device = Default::default();

        let model = Net::<Backend>::new_with(record);

        let input = Tensor::<Backend, 4>::from_data(
            [[
                [[0.024_595_8, 0.25883394], [0.93905586, 0.416_715_5]],
                [[0.713_979_7, 0.267_644_3], [0.990_609, 0.28845078]],
            ]],
            &device,
        );

        let output = model.forward(input);

        let expected = Tensor::<Backend, 4>::from_data(
            [[
                [
                    [0.04547675, 0.01879685, -0.01636661, 0.00310803],
                    [0.02090115, 0.01192738, -0.048_240_2, 0.02252235],
                    [0.03249975, -0.00460748, 0.05003899, 0.04029131],
                    [0.02185687, -0.10226749, -0.06508022, -0.01267705],
                ],
                [
                    [0.00277598, -0.00513832, -0.059_048_3, 0.00567626],
                    [-0.03149522, -0.195_757_4, 0.03474613, 0.01997269],
                    [-0.10096474, 0.00679589, 0.041_919_7, -0.02464108],
                    [-0.03174751, 0.02963913, -0.02703723, -0.01860938],
                ],
            ]],
            &device,
        );

        output
            .to_data()
            .assert_approx_eq(&expected.to_data(), precision);
    }

    #[test]
    fn conv_transpose2d_full() {
        let record = PyTorchFileRecorder::<FullPrecisionSettings>::default()
            .load("tests/conv_transpose2d/conv_transpose2d.pt".into())
            .expect("Failed to decode state");

        conv_transpose2d(record, 7);
    }
    #[test]
    fn conv_transpose2d_half() {
        let record = PyTorchFileRecorder::<HalfPrecisionSettings>::default()
            .load("tests/conv_transpose2d/conv_transpose2d.pt".into())
            .expect("Failed to decode state");

        conv_transpose2d(record, 4);
    }
}
