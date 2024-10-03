mod base;
mod full;
mod metrics;
mod minimal;

pub use base::*;
pub(crate) use full::*;
pub(crate) use metrics::*;

#[cfg(test)]
pub(crate) use minimal::*;

#[cfg(test)]
pub(crate) mod test_utils {
    use crate::metric_test::{
        processor::{Event, EventProcessor, LearnerItem, MinimalEventProcessor},
        Adaptor, LossInput,
    };
    use burn_core::tensor::{backend::Backend, ElementConversion, Tensor};

    impl<B: Backend> Adaptor<LossInput<B>> for f64 {
        fn adapt(&self) -> LossInput<B> {
            let device = B::Device::default();
            LossInput::new(Tensor::from_data([self.elem::<B::FloatElem>()], &device))
        }
    }

    pub(crate) fn process_train(
        processor: &mut MinimalEventProcessor<f64>,
        value: f64,
        epoch: usize,
    ) {
        let dummy_progress = burn_core::data::dataloader::Progress {
            items_processed: 1,
            items_total: 10,
        };
        let num_epochs = 3;
        let dummy_iteration = 1;

        processor.process(Event::ProcessedItem(LearnerItem::new(
            value,
            dummy_progress,
            dummy_iteration,
        )));
    }

    pub(crate) fn end_epoch(processor: &mut MinimalEventProcessor<f64, f64>, epoch: usize) {
        processor.process(Event::EndEpoch(epoch));
    }
}
