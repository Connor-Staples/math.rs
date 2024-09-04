use std::{fmt::Display, ops::Div};

use num::traits::int::PrimInt;


pub fn integer_sqrt<T: PrimInt + Default + Display>(num: T) -> (T, bool) {

    //default is 0 for all integers if i am aware
    if num < T::default()  {

    }
    //saves from doing this every time we want a 2
    let two = T::from(2).unwrap();
    
    let mut last_iteration = T::default();
    let mut  current_n = num / two;

    while last_iteration != current_n {
        println!("{}", current_n);
        last_iteration = current_n;

        current_n = (current_n + (num / current_n))/two;

        
    }
    

    let mut proot = false;

    if (current_n * current_n) == num {
        proot = true;
    }


    (current_n, proot)
}