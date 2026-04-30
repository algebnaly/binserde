use binserde::{Decode, Encode};

#[allow(unused)]
#[derive(Encode, Decode)]
enum Color {
    Red,
    Green,
    Blue,
}

fn main() {}
