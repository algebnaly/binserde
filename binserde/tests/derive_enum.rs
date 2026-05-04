use binserde::{Decode, Encode};

#[allow(unused)]
#[derive(Encode, Decode)]
enum Color {
    Red,
    Green,
    Blue,
}

#[allow(unused)]
#[derive(Encode, Decode)]
enum WithUnitCatchAll {
    A,
    B,
    #[binserde(catch_all)]
    Unknown,
}

#[allow(unused)]
#[derive(Encode, Decode)]
#[repr(u32)]
enum WithNewtypeCatchAll {
    A,
    B,
    #[binserde(catch_all)]
    Unknown(u32),
}

#[allow(unused)]
#[derive(Encode, Decode)]
#[repr(u32)]
enum WithTupleCatchAll {
    A,
    B,
    #[binserde(catch_all)]
    Unknown(u32, String),
}

fn main() {}
