use crate::metric::{
    classification::ClassificationInput, AccuracyInput, Adaptor, HammingScoreInput, LossInput,
};
use burn_core::tensor::backend::Backend;
use burn_core::tensor::{Int, Tensor};

/// Simple classification output adapted for multiple metrics.
#[derive(new)]
pub struct ClassificationOutput<B: Backend> {
    /// The loss.
    pub loss: Tensor<B, 1>,

    /// The output.
    pub output: Tensor<B, 2>,

    /// The targets.
    pub targets: Tensor<B, 1, Int>,
}

impl<B: Backend> Adaptor<AccuracyInput<B>> for ClassificationOutput<B> {
    fn adapt(&self) -> AccuracyInput<B> {
        AccuracyInput::new(self.output.clone(), self.targets.clone())
    }
}

impl<B: Backend> Adaptor<LossInput<B>> for ClassificationOutput<B> {
    fn adapt(&self) -> LossInput<B> {
        LossInput::new(self.loss.clone())
    }
}

impl<B: Backend> Adaptor<ClassificationInput<B>> for ClassificationOutput<B> {
    fn adapt(&self) -> ClassificationInput<B> {
        let [_, num_classes] = self.output.dims();
        if num_classes > 1 {
            ClassificationInput::new(
                self.output.clone(),
                self.targets.clone().one_hot(num_classes).bool(),
            )
        } else {
            ClassificationInput::new(
                self.output.clone(),
                self.targets.clone().unsqueeze_dim(1).bool(),
            )
        }
    }
}

/// Multi-label classification output adapted for multiple metrics.
#[derive(new)]
pub struct MultiLabelClassificationOutput<B: Backend> {
    /// The loss.
    pub loss: Tensor<B, 1>,

    /// The output.
    pub output: Tensor<B, 2>,

    /// The targets.
    pub targets: Tensor<B, 2, Int>,
}

impl<B: Backend> Adaptor<HammingScoreInput<B>> for MultiLabelClassificationOutput<B> {
    fn adapt(&self) -> HammingScoreInput<B> {
        HammingScoreInput::new(self.output.clone(), self.targets.clone())
    }
}

impl<B: Backend> Adaptor<LossInput<B>> for MultiLabelClassificationOutput<B> {
    fn adapt(&self) -> LossInput<B> {
        LossInput::new(self.loss.clone())
    }
}

impl<B: Backend> Adaptor<ClassificationInput<B>> for MultiLabelClassificationOutput<B> {
    fn adapt(&self) -> ClassificationInput<B> {
        ClassificationInput::new(self.output.clone(), self.targets.clone().bool())
    }
}
