#[cfg(test)]
pub trait ConvertableToVecMatrix<T> {
    fn to_vecs(self) -> Vec<Vec<T>>;
}

#[cfg(test)]
impl<T, const N: usize, const M: usize> ConvertableToVecMatrix<T> for [[T; M]; N] {
    fn to_vecs(self) -> Vec<Vec<T>> {
        self.into_iter()
            .map(|line| Vec::from_iter(line))
            .collect::<Vec<Vec<T>>>()
    }
}
