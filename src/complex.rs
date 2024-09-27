use std::ops::{Add, Mul, Sub};
use std::fmt::Display;
use std::cmp::PartialOrd;
use std::fmt;


use num::Num;


use crate::macrowos;




#[derive(Copy)]
pub struct Complex<T> where T: Num {
    pub real: T,
    pub img: T,
}

impl<T: Num> Complex<T> {
    pub fn new(real_part: T, imaginary_part: T) -> Complex<T> {
        Complex {
            real: real_part,
            img: imaginary_part,
        }
    }

    
}

impl Complex<f32> {
    pub fn magnitude(self) -> f32 {
        ((self.real * self.real) + (self.img * self.img)).sqrt()
    }
}

impl Complex<f64> {
    pub fn magnitude(self) -> f64 {
        ((self.real * self.real) + (self.img * self.img)).sqrt()
    }
}
//implements clone
impl<T: Copy + Num> Clone for Complex<T> {
    fn clone(&self) -> Complex<T>{
        Complex {
            real: self.real,
            img: self.img,
        }
    }
}
//addition
impl<T: Copy + Add + Num> Add for Complex<T> where T: Add<Output = T>{
    type Output = Complex<T>;
    fn add(self, other: Complex<T> ) -> Complex<T> {
        Complex {
            real: self.real + other.real,
            img: self.img + other.img,
        }
    }
}
//subtraction
impl<T: Copy + Sub + Num> Sub for Complex<T> where T: Sub<Output = T>{
    type Output = Complex<T>;
    fn sub(self, other: Complex<T> ) -> Complex<T> {
        Complex {
            real: self.real - other.real,
            img: self.img - other.img,
        }
    }
}

//multiply by non-complex scalar :
impl<T: Copy + Num> Mul<T> for Complex<T> where T: Mul<Output = T> {
    type Output = Complex<T>;
    fn mul(self, other: T) -> Complex<T> {
        Complex {
            real: self.real * other,
            img: self.img * other,
        }
    }
    
}

macrowos::Commutativity!(Complex);


//multiplication of 2 complex numbers
impl <T: Copy + Num> Mul<Complex<T>> for Complex<T> where T: Sub<Output = T> + Add<Output = T> + Mul<Output = T> {
    type Output = Complex<T>;
    fn mul(self, other: Complex<T>) -> Complex<T> {
        Complex {
            real: (self.real * other.real) - (self.img * other.img),
            img: (self.real * other.img) + (other.real * self.img),
        }
    }
}

//display implementation
impl<T: Display + PartialOrd + Default + Num> fmt::Display for Complex<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.img >= T::default() {
            write!(f, "{} + {}i", self.real, self.img)
        } else {
            write!(f, "({} {}i)", self.real, self.img)
        }
    }
}

