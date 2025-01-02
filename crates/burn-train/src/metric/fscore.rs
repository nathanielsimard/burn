use super::{
    classification::{ClassReduction, ClassificationMetricConfig, DecisionRule},
    confusion_stats::{ConfusionStats, ConfusionStatsInput},
    state::{FormatOptions, NumericMetricState},
    Metric, MetricEntry, MetricMetadata, Numeric,
};
use burn_core::{
    prelude::{Backend, Tensor},
    tensor::cast::ToElement,
};
use core::marker::PhantomData;
use std::num::NonZeroUsize;

///The FScore Metric
#[derive(Default)]
pub struct FScoreMetric<B: Backend> {
    state: NumericMetricState,
    _b: PhantomData<B>,
    config: ClassificationMetricConfig,
    beta: f64,
}

impl<B: Backend> FScoreMetric<B> {
    /// FScore metric for binary classification.
    ///
    /// # Arguments
    ///
    /// * `threshold` - The threshold to transform a probability into a binary prediction.
    #[allow(dead_code)]
    pub fn binary(beta: f64, threshold: f64) -> Self {
        Self {
            config: ClassificationMetricConfig {
                decision_rule: DecisionRule::Threshold(threshold),
                // binary classification results are the same independently of class_reduction
                ..Default::default()
            },
            beta,
            ..Default::default()
        }
    }

    /// FScore metric for multiclass classification.
    ///
    /// # Arguments
    ///
    /// * `top_k` - The number of highest predictions considered to find the correct label (typically `1`).
    #[allow(dead_code)]
    pub fn multiclass(beta: f64, top_k: usize, class_reduction: ClassReduction) -> Self {
        Self {
            config: ClassificationMetricConfig {
                decision_rule: DecisionRule::TopK(
                    NonZeroUsize::new(top_k).expect("top_k must be non-zero"),
                ),
                class_reduction,
            },
            beta,
            ..Default::default()
        }
    }

    /// FScore metric for multi-label classification.
    ///
    /// # Arguments
    ///
    /// * `threshold` - The threshold to transform a probability into a binary prediction.
    #[allow(dead_code)]
    pub fn multilabel(beta: f64, threshold: f64, class_reduction: ClassReduction) -> Self {
        Self {
            config: ClassificationMetricConfig {
                decision_rule: DecisionRule::Threshold(threshold),
                class_reduction,
            },
            beta,
            ..Default::default()
        }
    }

    fn class_average(&self, mut aggregated_metric: Tensor<B, 1>) -> f64 {
        use ClassReduction::{Macro, Micro};
        let avg_tensor = match self.config.class_reduction {
            Micro => aggregated_metric,
            Macro => {
                if aggregated_metric.contains_nan().any().into_scalar() {
                    let nan_mask = aggregated_metric.is_nan();
                    aggregated_metric = aggregated_metric
                        .clone()
                        .select(0, nan_mask.bool_not().argwhere().squeeze(1))
                }
                aggregated_metric.mean()
            }
        };
        avg_tensor.into_scalar().to_f64()
    }
}

impl<B: Backend> Metric for FScoreMetric<B> {
    const NAME: &'static str = "FScore";
    type Input = ConfusionStatsInput<B>;

    fn update(&mut self, input: &Self::Input, _metadata: &MetricMetadata) -> MetricEntry {
        let [sample_size, _] = input.predictions.dims();

        let cf_stats = ConfusionStats::new(input, &self.config);
        let scaled_true_positive = cf_stats.clone().true_positive() * (1.0 + self.beta.powi(2));
        let metric = self.class_average(
            scaled_true_positive.clone()
                / (scaled_true_positive
                    + cf_stats.clone().false_negative() * self.beta.powi(2)
                    + cf_stats.false_positive()),
        );

        self.state.update(
            100.0 * metric,
            sample_size,
            FormatOptions::new(Self::NAME).unit("%").precision(2),
        )
    }

    fn clear(&mut self) {
        self.state.reset()
    }
}

impl<B: Backend> Numeric for FScoreMetric<B> {
    fn value(&self) -> f64 {
        self.state.value()
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ClassReduction::{self, *},
        FScoreMetric, Metric, MetricMetadata, Numeric,
    };
    use crate::tests::{dummy_classification_input, ClassificationType, THRESHOLD};
    use burn_core::tensor::TensorData;
    use rstest::rstest;

    #[rstest]
    #[case::binary_b1(1.0, THRESHOLD, 0.5)]
    #[case::binary_b2(2.0, THRESHOLD, 0.5)]
    fn test_binary_fscore(#[case] beta: f64, #[case] threshold: f64, #[case] expected: f64) {
        let input = dummy_classification_input(&ClassificationType::Binary).into();
        let mut metric = FScoreMetric::binary(beta, threshold);
        let _entry = metric.update(&input, &MetricMetadata::fake());
        TensorData::from([metric.value()])
            .assert_approx_eq(&TensorData::from([expected * 100.0]), 3)
    }

    #[rstest]
    #[case::multiclass_b1_micro_k1(1.0, Micro, 1, 3.0/5.0)]
    #[case::multiclass_b1_micro_k2(1.0, Micro, 2, 2.0/(5.0/4.0 + 10.0/4.0))]
    #[case::multiclass_b1_macro_k1(1.0, Macro, 1, (0.5 + 2.0/(1.0 + 2.0) + 2.0/(2.0 + 1.0))/3.0)]
    #[case::multiclass_b1_macro_k2(1.0, Macro, 2, (2.0/(1.0 + 2.0) + 2.0/(1.0 + 4.0) + 0.5)/3.0)]
    #[case::multiclass_b2_micro_k1(2.0, Micro, 1, 3.0/5.0)]
    #[case::multiclass_b2_micro_k2(2.0, Micro, 2, 5.0*4.0/(4.0*5.0 + 10.0))]
    #[case::multiclass_b2_macro_k1(2.0, Macro, 1, (0.5 + 5.0/(4.0 + 2.0) + 5.0/(8.0 + 1.0))/3.0)]
    #[case::multiclass_b2_macro_k2(2.0, Macro, 2, (5.0/(4.0 + 2.0) + 5.0/(4.0 + 4.0) + 0.5)/3.0)]
    fn test_multiclass_fscore(
        #[case] beta: f64,
        #[case] class_reduction: ClassReduction,
        #[case] top_k: usize,
        #[case] expected: f64,
    ) {
        let input = dummy_classification_input(&ClassificationType::Multiclass).into();
        let mut metric = FScoreMetric::multiclass(beta, top_k, class_reduction);
        let _entry = metric.update(&input, &MetricMetadata::fake());
        TensorData::from([metric.value()])
            .assert_approx_eq(&TensorData::from([expected * 100.0]), 3)
    }

    #[rstest]
    #[case::multilabel_micro(1.0, Micro, THRESHOLD, 2.0/(9.0/5.0 + 8.0/5.0))]
    #[case::multilabel_macro(1.0, Macro, THRESHOLD, (2.0/(2.0 + 3.0/2.0) + 2.0/(1.0 + 3.0/2.0) + 2.0/(3.0+2.0))/3.0)]
    #[case::multilabel_micro(2.0, Micro, THRESHOLD, 5.0/(4.0*9.0/5.0 + 8.0/5.0))]
    #[case::multilabel_macro(2.0, Macro, THRESHOLD, (5.0/(8.0 + 3.0/2.0) + 5.0/(4.0 + 3.0/2.0) + 5.0/(12.0+2.0))/3.0)]
    fn test_multilabel_fscore(
        #[case] beta: f64,
        #[case] class_reduction: ClassReduction,
        #[case] threshold: f64,
        #[case] expected: f64,
    ) {
        let input = dummy_classification_input(&ClassificationType::Multilabel).into();
        let mut metric = FScoreMetric::multilabel(beta, threshold, class_reduction);
        let _entry = metric.update(&input, &MetricMetadata::fake());
        TensorData::from([metric.value()])
            .assert_approx_eq(&TensorData::from([expected * 100.0]), 3)
    }
}
