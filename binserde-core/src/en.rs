pub trait Encode {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error>;
}

pub trait Encoder {
    type Error;
    fn encode_unit(&mut self) -> Result<(), Self::Error>;
    fn encode_bool(&mut self, value: bool) -> Result<(), Self::Error>;
    fn encode_u8(&mut self, value: u8) -> Result<(), Self::Error>;
    fn encode_u16(&mut self, value: u16) -> Result<(), Self::Error>;
    fn encode_u32(&mut self, value: u32) -> Result<(), Self::Error>;
    fn encode_u64(&mut self, value: u64) -> Result<(), Self::Error>;
    fn encode_u128(&mut self, value: u128) -> Result<(), Self::Error>;
    fn encode_i8(&mut self, value: i8) -> Result<(), Self::Error>;
    fn encode_i16(&mut self, value: i16) -> Result<(), Self::Error>;
    fn encode_i32(&mut self, value: i32) -> Result<(), Self::Error>;
    fn encode_i64(&mut self, value: i64) -> Result<(), Self::Error>;
    fn encode_i128(&mut self, value: i128) -> Result<(), Self::Error>;
    fn encode_f32(&mut self, value: f32) -> Result<(), Self::Error>;
    fn encode_f64(&mut self, value: f64) -> Result<(), Self::Error>;
    fn encode_bytes(&mut self, value: &[u8]) -> Result<(), Self::Error>;
    fn encode_string(&mut self, value: &str) -> Result<(), Self::Error>;

    // this might be useful for encoding byte arrays, e.g. [u8; 32],
    // in this case, length header is not needed, because
    // the length is known as part of the type
    fn encode_byte_array<const N: usize>(&mut self, value: &[u8; N]) -> Result<(), Self::Error>;
}

impl Encode for () {
    fn encode<E: Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
        e.encode_unit()
    }
}
impl Encode for bool {
    fn encode<E: Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
        e.encode_bool(*self)
    }
}
impl Encode for u8 {
    fn encode<E: Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
        e.encode_u8(*self)
    }
}
impl Encode for u16 {
    fn encode<E: Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
        e.encode_u16(*self)
    }
}
impl Encode for u32 {
    fn encode<E: Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
        e.encode_u32(*self)
    }
}
impl Encode for u64 {
    fn encode<E: Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
        e.encode_u64(*self)
    }
}
impl Encode for u128 {
    fn encode<E: Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
        e.encode_u128(*self)
    }
}
impl Encode for i8 {
    fn encode<E: Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
        e.encode_i8(*self)
    }
}
impl Encode for i16 {
    fn encode<E: Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
        e.encode_i16(*self)
    }
}
impl Encode for i32 {
    fn encode<E: Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
        e.encode_i32(*self)
    }
}
impl Encode for i64 {
    fn encode<E: Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
        e.encode_i64(*self)
    }
}
impl Encode for i128 {
    fn encode<E: Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
        e.encode_i128(*self)
    }
}
impl Encode for f32 {
    fn encode<E: Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
        e.encode_f32(*self)
    }
}
impl Encode for f64 {
    fn encode<E: Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
        e.encode_f64(*self)
    }
}
impl Encode for str {
    fn encode<E: Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
        e.encode_string(self)
    }
}
impl Encode for String {
    fn encode<E: Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
        e.encode_string(self)
    }
}
impl Encode for Vec<u8> {
    fn encode<E: Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
        e.encode_bytes(self)
    }
}
impl<const N: usize> Encode for [u8; N] {
    fn encode<E: Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
        e.encode_byte_array(self)
    }
}
impl<T: Encode> Encode for Option<T> {
    fn encode<E: Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
        match self {
            Some(v) => {
                e.encode_bool(true)?;
                v.encode(e)
            }
            None => e.encode_bool(false),
        }
    }
}

macro_rules! tuple_encode {
    ($($T:ident),+) => {
        impl<$($T: Encode),+> Encode for ($($T,)+) {
            #[allow(non_snake_case)]
            fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
                let ($($T,)+) = self;// declare variable names as type names
                // here we have two $T, the first is Type, and the second is variable name
                $(<$T as Encode>::encode($T, encoder)?;)+
                Ok(())
            }
        }
    };
}

tuple_encode!(T1);
tuple_encode!(T1, T2);
tuple_encode!(T1, T2, T3);
tuple_encode!(T1, T2, T3, T4);
tuple_encode!(T1, T2, T3, T4, T5);
tuple_encode!(T1, T2, T3, T4, T5, T6);
tuple_encode!(T1, T2, T3, T4, T5, T6, T7);
tuple_encode!(T1, T2, T3, T4, T5, T6, T7, T8);
tuple_encode!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
tuple_encode!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
tuple_encode!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
tuple_encode!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
tuple_encode!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
tuple_encode!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
tuple_encode!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15
);
tuple_encode!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16
);
