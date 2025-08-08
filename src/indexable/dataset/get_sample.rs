use std::collections::VecDeque;
use std::sync::Arc;

/// Return a sample from the dataset at a given index.
pub trait GetSample {
    /// Type of one sample of the dataset.
    type Sample: Sized;
    /// Return the dataset sample corresponding to the index.
    fn get_sample(&self, index: usize) -> Self::Sample;
}

impl<T: Clone> GetSample for Vec<T> {
    type Sample = T;
    fn get_sample(&self, index: usize) -> Self::Sample {
        self[index].clone()
    }
}

impl<T: Clone> GetSample for VecDeque<T> {
    type Sample = T;
    fn get_sample(&self, index: usize) -> Self::Sample {
        self[index].clone()
    }
}

impl<T> GetSample for Arc<T>
where
    T: GetSample,
{
    type Sample = T::Sample;

    fn get_sample(&self, index: usize) -> Self::Sample {
        self.as_ref().get_sample(index)
    }
}

// TODO: `GetSample` for Array?
