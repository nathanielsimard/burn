use burn::optim::decay::WeightDecayConfig;
use text_generation::{training::ExperimentConfig, DbPediaDataset};

#[cfg(feature = "f16")]
type Elem = burn::tensor::f16;
#[cfg(not(feature = "f16"))]
type Elem = burn::tensor::f16;

type Backend = burn::backend::Autodiff<burn::backend::CudaJit<Elem>>;

fn main() {
    let config = ExperimentConfig::new(
        burn::nn::transformer::TransformerEncoderConfig::new(384, 1536, 12, 6)
            .with_norm_first(true),
        burn::optim::AdamConfig::new().with_weight_decay(Some(WeightDecayConfig::new(1.0e-6))),
    );

    text_generation::training::train::<Backend, DbPediaDataset>(
        Default::default(),
        DbPediaDataset::train(),
        DbPediaDataset::test(),
        config,
        "/tmp/text-generation",
    );
}
