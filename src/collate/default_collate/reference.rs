use super::DefaultCollate;
use crate::collate::Collate;

/// We think it makes no sense to but a bench of reference into a Tensor. That's why if the dataset yield reference a
/// we clone them them.
/// It is useful for having a non-consuming `Iterator` over the `Dataloader`.
impl<T> Collate<&T> for DefaultCollate
where
    T: Clone,
    Self: Collate<T>,
{
    type Output = <Self as Collate<T>>::Output;
    fn collate(&self, batch: Vec<&T>) -> Self::Output {
        DefaultCollate.collate(batch.into_iter().cloned().collect())
    }
}
