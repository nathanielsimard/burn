pub mod add {
    include!(concat!(env!("OUT_DIR"), "/model/add.rs"));
}

#[cfg(test)]
mod tests {
    use super::*;

    use burn::tensor::{Data, Tensor};

    type Backend = burn_ndarray::NdArrayBackend<f32>;

    #[test]
    fn add() {
        // The model contains add two tensors together and add a constant

        // Initialize the model with weights (loaded from the exported file)
        let model: add::Model<Backend> = add::Model::default();

        // Run the model
        let input = Tensor::<Backend, 4>::from_floats([[[[1., 2., 3., 4.]]]]);
        let output = model.forward(input);
        let expected = Data::from([[[[7., 8., 9., 10.]]]]);

        assert_eq!(output.to_data(), expected);
    }
}
