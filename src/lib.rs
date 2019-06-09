extern crate ndarray;
extern crate num_traits;

pub fn scalar_product<T>(v: &[T], w: &[T]) -> T
where T: std::ops::Mul<Output = T> + num_traits::Zero + Copy
{
    v.iter()
        .zip(w.iter())
        .map(|(x, y)| (*x) * (*y))
        .fold(T::zero(), |x, y| x + y)
}

#[cfg(test)]
mod tests {
    use crate::scalar_product;

    #[test]
    fn it_works_with_floats() {
        let v: Vec<f32> = vec![10.0, 11.0, 0.8];
        let w: Vec<f32> = vec![11.0, 10.0, 10.0];

        assert!((scalar_product(&v, &w) - 228.0).abs() < std::f32::EPSILON);
    }

    #[test]
    fn it_works_with_ints() {
        let v: Vec<i32> = vec![10, 11, 8];
        let w: Vec<i32> = vec![11, 10, 1];

        assert_eq!(scalar_product(&v, &w), 228)
    }
}
