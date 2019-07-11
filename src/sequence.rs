//use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::fmt;

pub trait SequenceCollection {
    type SeqType;
    type CollectionType;
    fn from_fasta(fasta: File) -> Self::CollectionType;
    fn new() -> Self::CollectionType;
    fn seqs(&self) -> &Vec<Box<Self::SeqType>>;
    fn nseqs(&self) -> usize;
}
pub trait Sequence {
    type SeqType;
    fn print_fasta(&self) {}
    fn write_fasta(&self, path: &str) {}
    fn complement(&self) -> Self::SeqType;
    //fn orfs(&self) -> Vec<Vec<String>>;
}

#[derive(Default, Clone)]
pub struct DnaSequence {
    pub header: String,
    pub sequence: Vec<u8>,
}
impl DnaSequence {
    pub fn new(header: String, sequence: Vec<u8>) -> Self {
        let upperseq = sequence
                        .iter()
                        .map(|b| b.to_ascii_uppercase())
                        .collect::<Vec<u8>>();
        DnaSequence { header, sequence: upperseq }
    }
}
impl Sequence for DnaSequence {
    type SeqType = DnaSequence;

    /*
    fn print_fasta(&self) {
        println!(">{}\n{}", self.header, self.sequence.iter().collect());
    }
    fn write_fasta(&self, path: &str) {
        let out = format!(">{}\n{}\n", self.header, self.sequence.iter().collect());
        let bytes = out.as_bytes();
        let mut buffer = File::create(path.to_string()).expect("Couldn't create file.");
        buffer
            .write_all(&bytes)
            .expect("Could not write fasta file.");
    }
    */
    fn complement(&self) -> DnaSequence {
        let pairs = vec![[b'A', b'T'], [b'T', b'A'], [b'C', b'G'], [b'G', b'C'], [b'N', b'N']];
        let complement = self
            .sequence
            .iter()
            .filter_map(|base| {
                pairs
                    .iter()
                    .find(|[l, _c]| l == base)
                    .map(|[_l, c]| *c)
            })
            .collect::<Vec<u8>>();
        DnaSequence::new(self.header.clone(), complement)
    }

    /*
    fn orfs (&self) -> Vec<Vec<String>> {
        let mut all: Vec<Vec<String>> = vec![vec![], vec![], vec![]];
        for p in 0..3 {
            let mut seq: Vec<char> = self.sequence.clone().chars().collect();
            while seq.len() != 0 {
                if seq.len() < 3 {
                    all[p].push(seq.into_iter().collect::<String>());
                    break;
                }
                all[p].push(seq.drain(p..3+p).collect::<String>());
            }
        }
        for i in all.iter() {
        println!("{:?}\n",i);
        }
        all
    }
    */
}

pub struct DnaSequenceVector(pub Vec<Box<DnaSequence>>);
impl SequenceCollection for DnaSequenceVector {
    type SeqType = DnaSequence;
    type CollectionType = DnaSequenceVector;
    fn new() -> Self {
        DnaSequenceVector(Vec::new())
    }
    fn seqs(&self) -> &Vec<Box<DnaSequence>> {
        &self.0
    }
    fn nseqs(&self) -> usize {
        self.0.len()
    }
    fn from_fasta(fasta: File) -> DnaSequenceVector {
        let mut collector: Vec<Box<DnaSequence>> = Vec::new();
        for pair in parse_fasta(fasta).into_iter() {
            collector.push(Box::new(DnaSequence::new(pair.0, pair.1.into_bytes())));
        }
        DnaSequenceVector(collector)
    }
}
/* generalized with parse_fasta and reimplemented above
impl SequenceCollection for DnaSequenceVector {
    type SeqType = DnaSequence;
    fn readfasta(&mut self, fasta: File) {
        let mut buffer = BufReader::new(fasta);
        let mut output: Vec<Box<DnaSequence>> = Vec::new();
        let mut current_sequence = Box::new(DnaSequence::default());
        loop {
            let mut line = String::new();
            let bytes = buffer.read_line(&mut line).unwrap();
            if bytes == 0 {
                output.push(current_sequence.clone());
                break;
            }
            line = line.replace('\n',"");
            if line.starts_with(">") {
                if current_sequence.sequence.len() != 0 {
                    output.push(current_sequence.clone());
                }
                current_sequence.header = line.replace('>',"");
                current_sequence.sequence.clear();
            } else {
                current_sequence.sequence.push_str(&line);
            }
        }
        self.0 = output;
    }
}
*/

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

/*
// Display implementations
impl fmt::Display for DnaSequence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, ">{}\n{}", self.header, self.sequence)
    }
}
*/