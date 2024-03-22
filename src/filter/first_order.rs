use std::rc::Rc;
use std::ops::{Add, AddAssign, Mul, SubAssign, Sub};

use super::Filter;

pub struct FirstOrderIirFilter<T> {
    a: T,
    b: (T, T),
    x_prev: T,
    y_prev: T,
}

impl <T: Default> FirstOrderIirFilter<T> {
    pub fn new(a: T, b: (T, T)) -> Self {
        let x_prev = T::default();
        let y_prev = T::default();
        Self {a, b , x_prev, y_prev}
    }

    fn update(&mut self, x_prev: T, y_prev: T) {
        self.x_prev = x_prev;
        self.y_prev = y_prev;
    }
}

impl <T> Filter<T> for FirstOrderIirFilter<T> 
where
    T: Default,
    T: Mul<Output = T>,
    T: AddAssign,
    T: SubAssign,
    T: Clone,
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Copy,
{
    fn filt(&mut self, input: &[T]) -> Vec<T> {
        let mut output_signal = Vec::with_capacity(input.len());
        for &x in input {
            let output = (self.b.0 * x ) + (self.b.1 * self.x_prev) - (self.a * self.y_prev);
            output_signal.push(output);
            self.update(x, output)
        }
        output_signal
    }
    // fn get_coeffs(&mut self) -> (Rc<[T]>, Rc<[T]>) {
    //     (Rc::from_iter([self.b.0, self.b.1]), Rc::from_iter([1.0, self.a]))
    // }
    fn get_b_coeffs(&mut self) -> Rc<[T]> {
        Rc::from_iter([self.b.0, self.b.1])
    }
    // fn get_a_coeffs(&mut self) -> Rc<[T]> {
    //     Rc::from_iter([1.into(), self.a])
    // }
    fn init(&mut self, initial_status: T) {
        self.update(initial_status, initial_status)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn truncate_vec(vals: Vec<f64>) -> Vec<f64> {
        vals.into_iter()
        .map(|v| truncate_decimal_digits(v, 5))
        .collect()
    }
    fn truncate_decimal_digits(number: f64, decimal_digits: u32) -> f64 {
        let multiplier = 10_f64.powi(decimal_digits as i32);
        (number * multiplier).trunc() / multiplier
    }

    #[test]
    fn correctness_test() {
        // Example coefficients and input data
        let coef_a =  -0.50952545;
        let coeffs_b = (0.24523728, 0.24523728);
        let mut f = FirstOrderIirFilter::new(coef_a, coeffs_b);
        let input_data1 = (0..5).into_iter().map(|f| f as f64).collect::<Vec<_>>();
        let input_data2 = (5..10).into_iter().map(|f| f as f64).collect::<Vec<_>>();
        let input_data3 = (10..15).into_iter().map(|f| f as f64).collect::<Vec<_>>();
        let real_res_from_py: Vec<_> =  truncate_vec(vec![0.0, 0.24523728,  0.86066646,  1.66471784,  2.56487703, 
                                                3.5140056, 4.48808531,  5.47487826,  6.46814894,  7.46472017,  
                                                8.46297313, 9.46208297, 10.46162941, 11.46139831 ,12.46128056]);
        let results: Vec<_> = truncate_vec([f.filt(&input_data1), f.filt(&input_data2), f.filt(&input_data3)].concat());
        assert_eq!(real_res_from_py, results);
    }

    #[test]
    fn coeffs_test() {
        let coef_a =  -0.50952545;
        let coeffs_b = (0.24523728, 0.24523728);
        let mut f = FirstOrderIirFilter::new(coef_a, coeffs_b);
        // assert_eq!(f.get_coeffs(), (Rc::from_iter([coeffs_b.0, coeffs_b.1]), Rc::from_iter([1.0, coef_a])))
    }

}
