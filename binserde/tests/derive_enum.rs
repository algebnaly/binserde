use binserde::{Decode, Encode};

#[derive(Encode, Decode)]
enum Color {
    Red,
    Green,
    Blue,
}

fn main() {}
