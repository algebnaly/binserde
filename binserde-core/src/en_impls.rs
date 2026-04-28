use crate::{Encode, Encoder, TupleEncoder};

impl Encode for () {
    fn encode<E: Encoder>(&self, encoder: E) -> Result<(), E::Error> {
        encoder.encode_unit()
    }
}

impl Encode for bool {
    fn encode<E: Encoder>(&self, e: E) -> Result<(), E::Error> {
        e.encode_bool(*self)
    }
}
impl Encode for u8 {
    fn encode<E: Encoder>(&self, e: E) -> Result<(), E::Error> {
        e.encode_u8(*self)
    }
}
impl Encode for u16 {
    fn encode<E: Encoder>(&self, e: E) -> Result<(), E::Error> {
        e.encode_u16(*self)
    }
}
impl Encode for u32 {
    fn encode<E: Encoder>(&self, e: E) -> Result<(), E::Error> {
        e.encode_u32(*self)
    }
}
impl Encode for u64 {
    fn encode<E: Encoder>(&self, e: E) -> Result<(), E::Error> {
        e.encode_u64(*self)
    }
}
impl Encode for u128 {
    fn encode<E: Encoder>(&self, e: E) -> Result<(), E::Error> {
        e.encode_u128(*self)
    }
}
impl Encode for i8 {
    fn encode<E: Encoder>(&self, e: E) -> Result<(), E::Error> {
        e.encode_i8(*self)
    }
}
impl Encode for i16 {
    fn encode<E: Encoder>(&self, e: E) -> Result<(), E::Error> {
        e.encode_i16(*self)
    }
}
impl Encode for i32 {
    fn encode<E: Encoder>(&self, e: E) -> Result<(), E::Error> {
        e.encode_i32(*self)
    }
}
impl Encode for i64 {
    fn encode<E: Encoder>(&self, e: E) -> Result<(), E::Error> {
        e.encode_i64(*self)
    }
}
impl Encode for i128 {
    fn encode<E: Encoder>(&self, e: E) -> Result<(), E::Error> {
        e.encode_i128(*self)
    }
}
impl Encode for f32 {
    fn encode<E: Encoder>(&self, e: E) -> Result<(), E::Error> {
        e.encode_f32(*self)
    }
}
impl Encode for f64 {
    fn encode<E: Encoder>(&self, e: E) -> Result<(), E::Error> {
        e.encode_f64(*self)
    }
}
impl Encode for str {
    fn encode<E: Encoder>(&self, e: E) -> Result<(), E::Error> {
        e.encode_string(self)
    }
}

impl Encode for String {
    fn encode<E: Encoder>(&self, e: E) -> Result<(), E::Error> {
        e.encode_string(self)
    }
}

impl<const N: usize> Encode for [u8; N] {
    fn encode<E: Encoder>(&self, e: E) -> Result<(), E::Error> {
        e.encode_byte_array(self)
    }
}

impl<T: Encode> Encode for Vec<T>
where
    T: Encode,
{
    fn encode<E: Encoder>(&self, e: E) -> Result<(), E::Error> {
        todo!()
    }
}

impl<T: Encode> Encode for Option<T> {
    fn encode<E: Encoder>(&self, e: E) -> Result<(), E::Error> {
        match self {
            Some(v) => {
                Ok(()) // TODO: fixes this
            }
            None => Ok(()),
        }
    }
}

macro_rules! replace_expr {
    ($_t:tt $sub:expr) => {
        $sub
    };
}

macro_rules! count_idents {
    ($($idents:ident),*) => { 0 $(+ replace_expr!($idents 1))* };
}

macro_rules! tuple_encode {
    ($($T:ident),+) => {
        impl<$($T: Encode),+> Encode for ($($T,)+) {
            #[allow(non_snake_case)]
            fn encode<E: Encoder>(&self, encoder: E) -> Result<(), E::Error> {
                let ($($T,)+) = self;
                let mut tuple_encoder = encoder.encode_tuple(count_idents!($($T),+))?;
                $(tuple_encoder.encode_element($T)?;)*
                tuple_encoder.end()
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
