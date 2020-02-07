//use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

pub trait SequenceCollection {
    type SeqType;
    type CollectionType;
    fn from_fasta(fasta: File) -> Self::CollectionType;
    fn new() -> Self::CollectionType;
    fn seqs(&self) -> &Vec<Self::SeqType>;
    fn nseqs(&self) -> usize;
}
pub trait Sequence {
    type SeqType;
    fn write_fasta(&self, path: &str) {}
    fn complement_bytes(&self) -> Self::SeqType;
    fn complement_rusty(&self) -> Self::SeqType;
    fn reverse(&self) -> Self::SeqType;
    fn revcomp(&self) -> Self::SeqType;
    fn orfs(&self) -> ();
}
pub enum Nucleotide {
    A,
    T,
    C,
    G,
    U,
    N,
}
impl Nucleotide {
    fn complement(self) -> Nucleotide {
        match self {
            Nucleotide::A => Nucleotide::T,
            Nucleotide::T => Nucleotide::A,
            Nucleotide::G => Nucleotide::C,
            Nucleotide::C => Nucleotide::G,
            Nucleotide::U => Nucleotide::A,
            Nucleotide::N => Nucleotide::N,
        }
    }
}
impl Into<u8> for Nucleotide {
    fn into(self) -> u8 {
        match self {
            Nucleotide::A => b'A',
            Nucleotide::T => b'T',
            Nucleotide::G => b'G',
            Nucleotide::C => b'C',
            Nucleotide::U => b'U',
            Nucleotide::N => b'N',
        }
    }
}
impl TryFrom<u8> for Nucleotide {
    type Error = &'static str;
    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            b'A' | b'a' => Ok(Nucleotide::A),
            b'T' | b't' => Ok(Nucleotide::T),
            b'G' | b'g' => Ok(Nucleotide::G),
            b'C' | b'c' => Ok(Nucleotide::C),
            b'U' | b'u' => Ok(Nucleotide::U),
            b'N' | b'n' => Ok(Nucleotide::U),
            _ => Err("Invalide nucleotide base in sequence."),
        }
    }
}

#[derive(Default, Clone)]
pub struct DnaSequence {
    pub header: String,
    pub sequence: Vec<u8>,
}
impl DnaSequence {
    pub fn new(header: String, sequence: Vec<u8>) -> Self {
        DnaSequence { header, sequence }
    }
}
impl Sequence for DnaSequence {
    type SeqType = DnaSequence;

    fn write_fasta(&self, path: &str) {
        let out = format!("{}", self);
        let bytes = out.as_bytes();
        let mut buffer = File::create(path.to_string()).expect("Couldn't create file.");
        buffer
            .write_all(&bytes)
            .expect("Could not write fasta file.");
    }
    fn complement_bytes(&self) -> DnaSequence {
        let pairs = vec![
            [b'A', b'T'],
            [b'T', b'A'],
            [b'C', b'G'],
            [b'G', b'C'],
            [b'N', b'N'],
        ];
        let complement = self
            .sequence
            .iter()
            .filter_map(|base| pairs.iter().find(|[l, _c]| l == base).map(|[_l, c]| *c))
            .collect::<Vec<u8>>();
        DnaSequence::new(self.header.clone(), complement)
    }
    fn complement_rusty(&self) -> DnaSequence {
        let complement = self
            .sequence
            .iter()
            .map(|byte| Nucleotide::try_from(*byte).expect("Invalide sequence letter."))
            .map(|nucl| nucl.complement().into())
            .collect::<Vec<u8>>();
        DnaSequence::new(self.header.clone(), complement)
    }
    fn reverse(&self) -> DnaSequence {
        DnaSequence::new(
            self.header.clone(),
            self.sequence
                .iter()
                .rev()
                .map(|byte| *byte)
                .collect::<Vec<u8>>(),
        )
    }
    fn revcomp(&self) -> DnaSequence {
        self.reverse().complement_rusty()
    }
    fn orfs(&self) -> () {
        let seq = self.sequence.clone();
        let test = seq
            .chunks(3)
            .map(|c| {
                c.iter()
                    .map(|b| Nucleotide::try_from(*b).expect("bad byte"))
                    .map(|nucl| format!("{}", nucl))
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<Vec<String>>>();
        println!("{:?}", test);
    }
}
#[derive(Clone)]
pub struct DnaSequenceVector(Vec<DnaSequence>);
impl SequenceCollection for DnaSequenceVector {
    type SeqType = DnaSequence;
    type CollectionType = DnaSequenceVector;
    fn new() -> Self {
        DnaSequenceVector(Vec::new())
    }
    fn seqs(&self) -> &Vec<DnaSequence> {
        &self.0
    }
    fn nseqs(&self) -> usize {
        self.0.len()
    }
    fn from_fasta(fasta: File) -> DnaSequenceVector {
        let mut collector: Vec<DnaSequence> = Vec::new();
        let _pushall = parse_fasta(fasta).into_iter().for_each(|p| {
            collector.push(DnaSequence::new(p.0, p.1.into_bytes().to_ascii_uppercase()))
        });
        DnaSequenceVector(collector)
    }
}
// Generic fasta parser that doesn't care what kind of sequence is being read
#[derive(Default, Clone)]
pub struct GenericFastaPair(String, String);
pub fn parse_fasta(fasta: File) -> Vec<GenericFastaPair> {
    let mut buffer = BufReader::new(fasta);
    let mut collector: Vec<GenericFastaPair> = Vec::new();
    let mut pair = GenericFastaPair::default();
    loop {
        let mut line = String::new();
        let bytes = buffer.read_line(&mut line).unwrap();
        if bytes == 0 {
            collector.push(pair.clone());
            break;
        }
        line = line.replace('\n', "");
        if line.starts_with(">") {
            if pair.1.len() != 0 {
                collector.push(pair.clone());
            }
            pair.0 = line.replace('>', "");
            pair.1.clear();
        } else {
            pair.1.push_str(&line);
        }
    }
    collector
}

// Display implementations
impl fmt::Display for Nucleotide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let strrep = match self {
            Nucleotide::A => "A",
            Nucleotide::T => "T",
            Nucleotide::G => "G",
            Nucleotide::C => "C",
            Nucleotide::U => "U",
            Nucleotide::N => "N",
        };
        write!(f, "{}", strrep)
    }
}
impl fmt::Display for DnaSequence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sequence_string = self
            .sequence
            .iter()
            .map(|byte| Nucleotide::try_from(*byte).expect("Invalid sequence character"))
            .map(|n| format!("{}", n))
            .collect::<String>();
        write!(f, ">{}\n{}", self.header, sequence_string)
    }
}
impl fmt::Debug for DnaSequence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sequence_string = self
            .sequence
            .iter()
            .map(|byte| Nucleotide::try_from(*byte).expect("Invalid sequence character"))
            .map(|n| format!("{}", n))
            .collect::<String>();
        write!(f, ">{}\n{}", self.header, sequence_string)
    }
}
