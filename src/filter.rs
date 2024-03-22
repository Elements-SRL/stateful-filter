pub mod order_n;
pub mod first_order;
use std::rc::Rc;
use std::ops::{Mul, SubAssign, AddAssign};

pub trait Filter<T>
where
    T: Default,
    T: Mul<Output = T>,
    T: AddAssign,
    T: SubAssign,
    T: Clone,
{
    fn filt(&mut self, input: &[T]) -> Vec<T>;

    fn get_b_coeffs(&mut self) -> Rc<[T]>;
    // fn get_a_coeffs(&mut self) -> Rc<[T]>;

    fn init(&mut self, initial_status: T);
    // / Get the filter coefficients ordered as b, a.
    // /
    // / # Returns
    // /
    // / The the filter coefficients ordered as b, a.
    // /
    // / # Examples
    // /
    // / ```
    // / use stateful_filter::FilterBuilder;
    // / let a = vec![1.0, -0.50952545];
    // / let b = vec![0.24523728, 0.24523728];
    // / let f = FilterBuilder::from(&a, &b);
    // / let (b, a) = f.get_coeffs();
    // / assert_eq!(result, 8);
    // / ``` 
    // fn get_coeffs(&mut self) -> (Rc<[T]>, Rc<[T]>);
}