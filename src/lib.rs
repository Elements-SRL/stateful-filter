pub mod filter;
use filter::{first_order::FirstOrderIirFilter, order_n::GenericIirFilter, Filter};
use std::ops::{Add, AddAssign, Mul, SubAssign, Sub};

pub struct FilterBuilder {}

impl FilterBuilder {
    pub fn from<T>(a: &[T], b:&[T]) -> Box<dyn Filter<T>>
    where
    T: Default,
    T: Mul<Output = T>,
    T: AddAssign,
    T: SubAssign,
    T: Clone,
    T: Copy,
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: 'static,
    {
        match (a.len(), b.len()) {
            (2, 2) => Box::new(FirstOrderIirFilter::new(a[1], (b[0], b[1]))),
            (a_len, b_len) if a_len == b_len && a_len > 1 => Box::new(GenericIirFilter::new(a, b)),
            _ => todo!()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use crate::FilterBuilder;

    #[test]
    fn correctness_test() {
        // Example coefficients and input data
        let coeffs_a = vec![1.0, -2.36951301,  2.31398841, -1.05466541,  0.18737949];
        let coeffs_b = vec![0.00482434, 0.01929737, 0.02894606, 0.01929737, 0.00482434];
        let input_data1 = (0..10).into_iter().map(|f| f as f64).collect::<Vec<_>>();
        let input_data2 = (10..20).into_iter().map(|f| f as f64).collect::<Vec<_>>();
        let mut f = FilterBuilder::from(&coeffs_a, &coeffs_b);
        println!("{:?}", [f.filt(&input_data1), f.filt(&input_data2)].concat());
    }

    #[test]
    fn correctness_test_f32() {
        // Example coefficients and input data
        let coeffs_a: Vec<f32> = vec![1.0, -2.36951301,  2.31398841, -1.05466541,  0.18737949];
        let coeffs_b = vec![0.00482434, 0.01929737, 0.02894606, 0.01929737, 0.00482434];
        let input_data1 = (0..10).into_iter().map(|f| f as f32).collect::<Vec<_>>();
        let input_data2 = (10..20).into_iter().map(|f| f as f32).collect::<Vec<_>>();
        let mut f = FilterBuilder::from(&coeffs_a, &coeffs_b);
        println!("{:?}", [f.filt(&input_data1), f.filt(&input_data2)].concat());
    }

    #[test]
    fn first_order_iir_filter() {
        // Example coefficients and input data
        let coeffs_a = vec![1.0,-0.50952545];
        let coeffs_b = vec![0.24523728, 0.24523728];
        let input_data = (0..1_000_000).into_iter().map(|f| f as f64).collect::<Vec<_>>();
        let mut f = FilterBuilder::from(&coeffs_a, &coeffs_b);
        let start = Instant::now();
        for _ in 0..40 {
            f.filt(&input_data);
            f.filt(&input_data);
        }
        println!("{:?}", start.elapsed());
    }

    #[test]
    fn first_order_iir_filter_f32() {
        // Example coefficients and input data
        let coeffs_a = vec![1.0,-0.50952545];
        let coeffs_b = vec![0.24523728, 0.24523728];
        let input_data = (0..1_000_000).into_iter().map(|f| f as f32).collect::<Vec<_>>();
        let mut f = FilterBuilder::from(&coeffs_a, &coeffs_b);
        let start = Instant::now();
        for _ in 0..40 {
            f.filt(&input_data);
            f.filt(&input_data);
        }
        println!("{:?}", start.elapsed());
    }

    #[test]
    fn third_order_iir_filter() {
        // Example coefficients and input data
        let third_order_a = vec![1.0, -2.36951301, 2.31398841, -1.05466541, 0.18737949];
        let third_order_b = vec![0.00482434, 0.01929737, 0.02894606, 0.01929737, 0.00482434];
        let input_data = (0..1_000_000).into_iter().map(|f| f as f64).collect::<Vec<_>>();
        let mut f = FilterBuilder::from(&third_order_a, &third_order_b);
        let start = Instant::now();
        for _ in 0..40 {
            f.filt(&input_data);
            f.filt(&input_data);
        }
        println!("{:?}", start.elapsed());
    }

    #[test]
    fn fifth_order_iir_filter() {
        // Example coefficients and input data
        let fifth_order_a = vec![1.0, -2.36951301, 2.31398841, -1.05466541, 0.18737949];
        let fifth_order_b = vec![0.00482434, 0.01929737, 0.02894606, 0.01929737, 0.00482434];
        let input_data = (0..1_000_000).into_iter().map(|f| f as f64).collect::<Vec<_>>();
        let mut f = FilterBuilder::from(&fifth_order_a, &fifth_order_b);
        let start = Instant::now();
        for _ in 0..40 {
            f.filt(&input_data);
            f.filt(&input_data);
        }
        println!("{:?}", start.elapsed());
    }
}
