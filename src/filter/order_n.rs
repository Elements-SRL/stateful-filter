use std::{collections::VecDeque, rc::Rc};
use std::ops::{Mul, SubAssign, AddAssign};

use super::Filter;

pub struct GenericIirFilter<T>
{
    a: Rc<[T]>,
    b: Rc<[T]>,
    xs: VecDeque<T>,
    ys: VecDeque<T>,
}

impl <T> GenericIirFilter<T>
where
    T: Default,
    T: Clone,
    T: Copy,
{
    pub fn new(a: &[T], b: &[T]) -> Self {
        let xs = VecDeque::from(vec![T::default(); b.len()]);
        let ys = VecDeque::from(vec![T::default(); a.len()-1]);
        let a = Rc::from_iter(a[1..].iter().copied());
        let b = Rc::from_iter(b.iter().copied());
        GenericIirFilter { a, b, xs, ys }
    }
}

impl <T> Filter<T> for GenericIirFilter<T>
where
    T: Default,
    T: Mul<Output = T>,
    T: AddAssign,
    T: SubAssign,
    T: Clone,
    T: Copy,
    T: Mul,
 {
    fn filt(&mut self, input: &[T]) -> Vec<T> {
        let coeffs_a = &self.a;
        let coeffs_b = &self.b;
        let buffer_x = &mut self.xs;
        let buffer_y = &mut self.ys;
        let mut output_signal = Vec::with_capacity(input.len());
        let a_len = coeffs_a.len();
        for &x in input {
            // Update buffer_x with new input
            buffer_x.push_front(x);
            buffer_x.pop_back();
            // Calculate output based on buffer_x and buffer_y
            let mut output = T::default();
            for i in 0..a_len {
                output += coeffs_b[i] * buffer_x[i];
                output -= coeffs_a[i] * buffer_y[i];
            }
            output += coeffs_b[a_len] * buffer_x[a_len];
            // Update buffer_y with new output
            buffer_y.push_front(output);
            buffer_y.pop_back();
            // Store the output sample
            output_signal.push(output);
        }
        output_signal
    }

    // fn get_coeffs(&mut self) -> (Rc<[T]>, Rc<[T]>) {
    //     (Rc::clone(&self.b), Rc::clone(&self.a))
    // }
    
    fn get_b_coeffs(&mut self) -> Rc<[T]> {
        Rc::clone(&self.b)
    }

    // fn get_a_coeffs(&mut self) -> Rc<[T]> {
    //     Rc::clone(&self.a)
    // }

    fn init(&mut self, initial_status: T) {
        self.xs = VecDeque::from(vec![initial_status; self.xs.len()]);
        self.ys = VecDeque::from(vec![initial_status; self.ys.len()]);
    }
}
