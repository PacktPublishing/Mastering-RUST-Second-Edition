// complex/src/lib.rs

use std::ops::Add;
use std::convert::Into;

#[derive(Default, Debug, PartialEq, Copy, Clone)]
struct Complex<T> {
    // Real part
    re: T,
    // Complex part
    im: T
}

impl<T> Complex<T> {
    fn new(re: T, im: T) -> Self {
        Complex { re, im }
    }
}

impl<T: Add<T, Output=T>> Add for Complex<T> { 
    type Output = Complex<T>; 
    fn add(self, rhs: Complex<T>) -> Self::Output { 
        Complex { re: self.re + rhs.re, im: self.im + rhs.im } 
    } 
}

impl<T> Into<(T, T)> for Complex<T> { 
    fn into(self) -> (T, T) { 
        (self.re, self.im)
    } 
}

#[cfg(test)]
mod tests {
    use Complex;
    #[test]
    fn complex_basics() {
        let first = Complex::new(3,5);
        let second: Complex<i32> = Complex::default();
    }
    #[test]
    fn complex_addition() {
        let a = Complex::new(1,-2);
        let b = Complex::default();
        let res = a + b;
        assert_eq!(res, a);
    }
    #[test]
    fn complex_into() {
        let a = Complex::new(2345,456);
        let (a, b) = a.into();
        assert_eq!(a, 2345);
        assert_eq!(b, 456);
    }
}
