use super::super::Collate;
use super::DefaultCollate;

impl<T, const N: usize> Collate<[T; N]> for DefaultCollate
where
    Self: Collate<T>,
    T: Clone,
{
    type Output = Vec<<Self as Collate<T>>::Output>;
    fn collate(&self, batch: Vec<[T; N]>) -> Self::Output {
        let mut collated = Vec::with_capacity(batch.len());
        for i in 0..batch[0].len() {
            let vec: Vec<_> = batch.iter().map(|sample| sample[i].clone()).collect();
            collated.push(self.collate(vec));
        }
        collated
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn vec_of_array() {
        assert_eq!(
            DefaultCollate::default().collate(vec![[1, 2], [3, 4], [5, 6]]),
            vec![array![1, 3, 5], array![2, 4, 6]]
        );
    }
}
