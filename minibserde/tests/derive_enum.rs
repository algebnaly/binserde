use minibserde::{Decode, Encode};

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
    #[minibserde(catch_all)]
    Unknown,
}

#[allow(unused)]
#[derive(Encode, Decode)]
#[repr(u32)]
enum WithNewtypeCatchAll {
    A,
    B,
    #[minibserde(catch_all)]
    Unknown(u32),
}

#[allow(unused)]
#[derive(Encode, Decode)]
#[repr(u32)]
enum WithTupleCatchAll {
    A,
    B,
    #[minibserde(catch_all)]
    Unknown(u32, String),
}

fn main() {}
