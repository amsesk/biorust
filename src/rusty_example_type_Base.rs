// Provided by farnbams @ Rust Programming Language forum

use std::convert::TryFrom;

#[derive(Debug, Clone, Copy)]
enum Base {
    A,
    T,
    G,
    C,
}

impl Base {
    fn complementary(self) -> Self {
        match self {
            Base::A => Base::T,
            Base::T => Base::A,
            Base::G => Base::C,
            Base::C => Base::G,
        }
    }
}

impl TryFrom<u8> for Base {
    type Error = &'static str; // Better us an Error type

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            b'A' | b'a' => Ok(Base::A),
            b'T' | b't' => Ok(Base::T),
            b'G' | b'g' => Ok(Base::G),
            b'C' | b'c' => Ok(Base::C),
            _ => Err("Not a character identifying a base!"),
        }
    }
}

impl Into<u8> for Base {
    fn into(self) -> u8 {
        match self {
            Base::A => b'A',
            Base::T => b'T',
            Base::G => b'G',
            Base::C => b'C',
        }
    }
}

fn main() {
    let dna = b"ATCGGGCAT";
    let complementary: Vec<u8> = dna
        .iter()
        .map(|byte| Base::try_from(*byte).expect("Should in this example not happen"))
        .map(|base| base.complementary().into())
        .collect();
    assert_eq!(complementary, b"TAGCCCGTA");
}