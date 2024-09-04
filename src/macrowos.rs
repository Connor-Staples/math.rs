macro_rules! com { //sub macro of commutativity, implements commutativity for one primitive
    ($struct:ident, $t:ty) => {
        impl Mul<$struct <$t> > for $t {
            type Output = $struct<$t>;
            fn mul(self, rhs: $struct<$t>) -> $struct<$t> {
                rhs * self
            }
        }
    };
}


macro_rules! Commutativity  { //A macro where given an already defined struct(T) * T, it will implement T * struct(T)
   
    ($struct:ident) => {
        
        macrowos::com!($struct, f64);
        macrowos::com!($struct, f32);
        macrowos::com!($struct, i64);
        macrowos::com!($struct, i32);
        macrowos::com!($struct, i16);
        macrowos::com!($struct, i8);
        macrowos::com!($struct, u64);
        macrowos::com!($struct, u32);
        macrowos::com!($struct, u16);
        macrowos::com!($struct, u8);
    };
}

pub(crate) use Commutativity;

pub(crate) use com;
