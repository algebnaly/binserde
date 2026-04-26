pub trait Decode: Sized {
    fn decode<D: Decoder>(decoder: D) -> Result<Self, D::Error>;
}

pub trait Decoder {
    type Error;
    fn decode_unit(&mut self) -> Result<(), Self::Error>;
    fn decode_bool(&mut self) -> Result<bool, Self::Error>;
    fn decode_u8(&mut self) -> Result<u8, Self::Error>;
    fn decode_u16(&mut self) -> Result<u16, Self::Error>;
    fn decode_u32(&mut self) -> Result<u32, Self::Error>;
    fn decode_u64(&mut self) -> Result<u64, Self::Error>;
    fn decode_u128(&mut self) -> Result<u128, Self::Error> {
        unimplemented!()
    }
    fn decode_i8(&mut self) -> Result<i8, Self::Error>;
    fn decode_i16(&mut self) -> Result<i16, Self::Error>;
    fn decode_i32(&mut self) -> Result<i32, Self::Error>;
    fn decode_i64(&mut self) -> Result<i64, Self::Error>;
    fn decode_i128(&mut self) -> Result<i128, Self::Error> {
        unimplemented!()
    }

    fn decode_f32(&mut self) -> Result<f32, Self::Error>;
    fn decode_f64(&mut self) -> Result<f64, Self::Error>;

    fn decode_bytes(&mut self) -> Result<Vec<u8>, Self::Error>;
    fn decode_string(&mut self) -> Result<String, Self::Error>;
    fn decode_byte_array(&mut self, len: usize) -> Result<Vec<u8>, Self::Error>;
}

impl<'a, D: Decoder> Decoder for &'a mut D {
    type Error = D::Error;

    fn decode_unit(&mut self) -> Result<(), Self::Error> {
        (**self).decode_unit()
    }

    fn decode_bool(&mut self) -> Result<bool, Self::Error> {
        (**self).decode_bool()
    }

    fn decode_u8(&mut self) -> Result<u8, Self::Error> {
        (**self).decode_u8()
    }

    fn decode_u16(&mut self) -> Result<u16, Self::Error> {
        (**self).decode_u16()
    }

    fn decode_u32(&mut self) -> Result<u32, Self::Error> {
        (**self).decode_u32()
    }

    fn decode_u64(&mut self) -> Result<u64, Self::Error> {
        (**self).decode_u64()
    }

    fn decode_u128(&mut self) -> Result<u128, Self::Error> {
        (**self).decode_u128()
    }

    fn decode_i8(&mut self) -> Result<i8, Self::Error> {
        (**self).decode_i8()
    }

    fn decode_i16(&mut self) -> Result<i16, Self::Error> {
        (**self).decode_i16()
    }

    fn decode_i32(&mut self) -> Result<i32, Self::Error> {
        (**self).decode_i32()
    }

    fn decode_i64(&mut self) -> Result<i64, Self::Error> {
        (**self).decode_i64()
    }

    fn decode_i128(&mut self) -> Result<i128, Self::Error> {
        (**self).decode_i128()
    }

    fn decode_f32(&mut self) -> Result<f32, Self::Error> {
        (**self).decode_f32()
    }

    fn decode_f64(&mut self) -> Result<f64, Self::Error> {
        (**self).decode_f64()
    }

    fn decode_bytes(&mut self) -> Result<Vec<u8>, Self::Error> {
        (**self).decode_bytes()
    }

    fn decode_string(&mut self) -> Result<String, Self::Error> {
        (**self).decode_string()
    }

    fn decode_byte_array(&mut self, len: usize) -> Result<Vec<u8>, Self::Error> {
        (**self).decode_byte_array(len)
    }
}

impl Decode for () {
    fn decode<D: Decoder>(mut decoder: D) -> Result<Self, D::Error> {
        decoder.decode_unit()
    }
}
impl Decode for bool {
    fn decode<D: Decoder>(mut decoder: D) -> Result<Self, D::Error> {
        decoder.decode_bool()
    }
}
impl Decode for u8 {
    fn decode<D: Decoder>(mut decoder: D) -> Result<Self, D::Error> {
        decoder.decode_u8()
    }
}
impl Decode for u16 {
    fn decode<D: Decoder>(mut decoder: D) -> Result<Self, D::Error> {
        decoder.decode_u16()
    }
}
impl Decode for u32 {
    fn decode<D: Decoder>(mut decoder: D) -> Result<Self, D::Error> {
        decoder.decode_u32()
    }
}
impl Decode for u64 {
    fn decode<D: Decoder>(mut decoder: D) -> Result<Self, D::Error> {
        decoder.decode_u64()
    }
}
impl Decode for u128 {
    fn decode<D: Decoder>(mut decoder: D) -> Result<Self, D::Error> {
        decoder.decode_u128()
    }
}
impl Decode for i8 {
    fn decode<D: Decoder>(mut decoder: D) -> Result<Self, D::Error> {
        decoder.decode_i8()
    }
}
impl Decode for i16 {
    fn decode<D: Decoder>(mut decoder: D) -> Result<Self, D::Error> {
        decoder.decode_i16()
    }
}
impl Decode for i32 {
    fn decode<D: Decoder>(mut decoder: D) -> Result<Self, D::Error> {
        decoder.decode_i32()
    }
}
impl Decode for i64 {
    fn decode<D: Decoder>(mut decoder: D) -> Result<Self, D::Error> {
        decoder.decode_i64()
    }
}
impl Decode for i128 {
    fn decode<D: Decoder>(mut decoder: D) -> Result<Self, D::Error> {
        decoder.decode_i128()
    }
}
impl Decode for f32 {
    fn decode<D: Decoder>(mut decoder: D) -> Result<Self, D::Error> {
        decoder.decode_f32()
    }
}
impl Decode for f64 {
    fn decode<D: Decoder>(mut decoder: D) -> Result<Self, D::Error> {
        decoder.decode_f64()
    }
}
impl Decode for String {
    fn decode<D: Decoder>(mut decoder: D) -> Result<Self, D::Error> {
        decoder.decode_string()
    }
}
impl Decode for Vec<u8> {
    fn decode<D: Decoder>(mut decoder: D) -> Result<Self, D::Error> {
        decoder.decode_bytes()
    }
}
impl<const N: usize> Decode for [u8; N] {
    fn decode<D: Decoder>(mut decoder: D) -> Result<Self, D::Error> {
        let vec = decoder.decode_byte_array(N)?;
        let mut arr = [0u8; N];
        arr.copy_from_slice(&vec);
        Ok(arr)
    }
}
impl<T: Decode> Decode for Option<T> {
    fn decode<D: Decoder>(mut decoder: D) -> Result<Self, D::Error> {
        if decoder.decode_bool()? {
            T::decode(decoder).map(Some)
        } else {
            Ok(None)
        }
    }
}

macro_rules! tuple_decode {
    ($($T:ident),+) => {
        impl<$($T: Decode),+> Decode for ($($T,)+) {
            #[allow(non_snake_case)]
            fn decode<D: Decoder>(mut decoder: D) -> Result<Self, D::Error> {
                Ok(($($T::decode(&mut decoder)?,)+))
            }
        }
    };
}

tuple_decode!(T1);
tuple_decode!(T1, T2);
tuple_decode!(T1, T2, T3);
tuple_decode!(T1, T2, T3, T4);
tuple_decode!(T1, T2, T3, T4, T5);
tuple_decode!(T1, T2, T3, T4, T5, T6);
tuple_decode!(T1, T2, T3, T4, T5, T6, T7);
tuple_decode!(T1, T2, T3, T4, T5, T6, T7, T8);
tuple_decode!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
tuple_decode!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
tuple_decode!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
tuple_decode!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
tuple_decode!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
tuple_decode!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
tuple_decode!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15
);
tuple_decode!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16
);
