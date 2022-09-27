use std::marker::PhantomData;

use crate::{
    collate::DefaultCollate,
    sampler::{BatchSampler, RandomSampler, Sampler, SequentialSampler},
    DataLoader, Dataset,
};

/// Basic builder for creating dataloader.
#[must_use]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash, Ord)]
pub struct DataLoaderBuilder<D, S = SequentialSampler, C = DefaultCollate>
where
    D: Dataset,
    S: Sampler,
    C: Collate<D::Sample>,
{
    /// The dataset from which the loader will yield the data.
    dataset: D,
    /// The sampler used to gather elements of the batch together.
    batch_sampler: BatchSampler<S>,
    /// Used to collate the data together.
    collate_fn: C,
}

impl<D> DataLoaderBuilder<D, SequentialSampler, DefaultCollate>
where
    D: Dataset,
    DefaultCollate: Collate<D::Sample>,
{
    /// Create a new [`DataLoaderBuilder`], with default fields.
    /// By default the [`DataLoaderBuilder`] is sequential and have a `batch_size` of one.
    pub fn new(dataset: D) -> Self {
        let dataset_len = dataset.len();
        Self {
            dataset,
            batch_sampler: BatchSampler {
                sampler: SequentialSampler::new(dataset_len),
                batch_size: 1,
                drop_last: false,
            },
            collate_fn: DefaultCollate,
        }
    }
}
use crate::collate::Collate;

impl<D, S, C> DataLoaderBuilder<D, S, C>
where
    D: Dataset,
    S: Sampler,
    C: Collate<D::Sample>,
{
    /// Use a random sampler.
    pub fn shuffle(self) -> DataLoaderBuilder<D, RandomSampler, C> {
        self.sampler::<RandomSampler>()
    }
    /// Set the number of elements in a batch.
    pub fn batch_size(mut self, batch_size: usize) -> Self {
        self.batch_sampler.batch_size = batch_size;
        self
    }

    /// Drop the lasts element if they don't feat into a batch. For instance if a dataset have 13
    /// samples and a `batch_size` of 5, the last 3 samples will be droped.
    pub fn drop_last(mut self) -> Self {
        self.batch_sampler.drop_last = true;
        self
    }

    /// Set a custom collate function.
    pub fn collate_fn<CF>(self, collate_fn: CF) -> DataLoaderBuilder<D, S, CF>
    where
        CF: Collate<D::Sample>,
    {
        DataLoaderBuilder {
            dataset: self.dataset,

            batch_sampler: self.batch_sampler,
            collate_fn,
        }
    }

    /// Set a custom [`Sampler`].
    pub fn sampler<SA>(self) -> DataLoaderBuilder<D, SA, C>
    where
        SA: Sampler,
    {
        let sampler: SA = SA::new(self.dataset.len());
        DataLoaderBuilder {
            dataset: self.dataset,
            batch_sampler: BatchSampler {
                sampler,
                batch_size: self.batch_sampler.batch_size,
                drop_last: self.batch_sampler.drop_last,
            },

            collate_fn: self.collate_fn,
        }
    }
    /// Create a [`Dataloader`] from a [`DataLoaderBuilder`].
    pub fn build(self) -> DataLoader<D, S, C> {
        DataLoader {
            dataset: self.dataset,
            batch_sampler: self.batch_sampler,
            phantom: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collate::NoOpCollate;

    #[test]
    fn api() {
        let _loader = DataLoaderBuilder::new(vec![1, 2, 3, 4]).build();
        let _loader = DataLoaderBuilder::new(vec![1, 2, 3, 4]).shuffle().build();

        let _loader = DataLoaderBuilder::new(vec![1, 2, 3, 4])
            .batch_size(2)
            .build();

        let _loader = DataLoaderBuilder::new(vec![1, 2, 3, 4])
            .batch_size(2)
            .drop_last()
            .build();

        let _loader = DataLoaderBuilder::new(vec![1, 2, 3, 4])
            .batch_size(2)
            .drop_last()
            .collate_fn(NoOpCollate)
            .build();

        let _loader = DataLoaderBuilder::new(vec![1, 2, 3, 4])
            .batch_size(2)
            .drop_last()
            .sampler::<RandomSampler>()
            .build();

        let _loader = DataLoaderBuilder::new(vec![1, 2, 3, 4])
            .batch_size(2)
            .drop_last()
            .sampler::<RandomSampler>()
            .collate_fn(NoOpCollate)
            .build();

        // TODO: checker la syntax des builder dans la STL, voir s'il utilise "with_", des verbe, etc..

        let _loader = DataLoaderBuilder::new(vec![1, 2, 3, 4])
            .shuffle()
            .batch_size(2)
            .drop_last()
            .collate_fn(NoOpCollate)
            .build();
    }
}
