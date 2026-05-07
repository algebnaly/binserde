use crate::{Decode, Decoder, SeqDecoder, TupleDecoder};

impl Decode for () {
    fn decode<D: Decoder>(decoder: D) -> Result<Self, D::Error> {
        decoder.decode_unit()
    }
}

impl Decode for bool {
    fn decode<D: Decoder>(decoder: D) -> Result<Self, D::Error> {
        decoder.decode_bool()
    }
}
impl Decode for u8 {
    fn decode<D: Decoder>(decoder: D) -> Result<Self, D::Error> {
        decoder.decode_u8()
    }
}
impl Decode for u16 {
    fn decode<D: Decoder>(decoder: D) -> Result<Self, D::Error> {
        decoder.decode_u16()
    }
}
impl Decode for u32 {
    fn decode<D: Decoder>(decoder: D) -> Result<Self, D::Error> {
        decoder.decode_u32()
    }
}
impl Decode for u64 {
    fn decode<D: Decoder>(decoder: D) -> Result<Self, D::Error> {
        decoder.decode_u64()
    }
}
impl Decode for u128 {
    fn decode<D: Decoder>(decoder: D) -> Result<Self, D::Error> {
        decoder.decode_u128()
    }
}
impl Decode for i8 {
    fn decode<D: Decoder>(decoder: D) -> Result<Self, D::Error> {
        decoder.decode_i8()
    }
}
impl Decode for i16 {
    fn decode<D: Decoder>(decoder: D) -> Result<Self, D::Error> {
        decoder.decode_i16()
    }
}
impl Decode for i32 {
    fn decode<D: Decoder>(decoder: D) -> Result<Self, D::Error> {
        decoder.decode_i32()
    }
}
impl Decode for i64 {
    fn decode<D: Decoder>(decoder: D) -> Result<Self, D::Error> {
        decoder.decode_i64()
    }
}
impl Decode for i128 {
    fn decode<D: Decoder>(decoder: D) -> Result<Self, D::Error> {
        decoder.decode_i128()
    }
}
impl Decode for f32 {
    fn decode<D: Decoder>(decoder: D) -> Result<Self, D::Error> {
        decoder.decode_f32()
    }
}
impl Decode for f64 {
    fn decode<D: Decoder>(decoder: D) -> Result<Self, D::Error> {
        decoder.decode_f64()
    }
}
impl Decode for String {
    fn decode<D: Decoder>(decoder: D) -> Result<Self, D::Error> {
        decoder.decode_string()
    }
}

impl<T: Decode> Decode for Vec<T>
where
    T: Decode,
{
    fn decode<D: Decoder>(decoder: D) -> Result<Self, D::Error> {
        let mut seq_decoder = decoder.decode_seq()?;
        let len = seq_decoder.decode_len()?;
        let mut buf = Vec::new();
        for _ in 0..len {
            buf.push(seq_decoder.decode_element()?);
        }
        Ok(buf)
    }
}

impl<T: Decode> Decode for Option<T> {
    fn decode<D: Decoder>(decoder: D) -> Result<Self, D::Error> {
        decoder.decode_option()
    }
}

impl<T: Decode> Decode for Box<T> {
    fn decode<D: Decoder>(decoder: D) -> Result<Self, D::Error> {
        let t = T::decode(decoder)?;
        Ok(Box::new(t))
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

macro_rules! tuple_decode {
    ($($T:ident),+) => {
        impl<$($T: Decode),+> Decode for ($($T,)+) {
            #[allow(non_snake_case)]
            fn decode<D: Decoder>(decoder: D) -> Result<Self, D::Error> {
                let mut tuple_decoder = decoder.decode_tuple(count_idents!($($T),+))?;
                let result = ($(tuple_decoder.decode_element::<$T>()?,)+);
                tuple_decoder.end()?;
                Ok(result)
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
