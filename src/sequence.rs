use std::fs::File;
use std::collections::HashMap;
use std::io::prelude::*;
use std::io::{self, BufReader};

pub trait SequenceCollection {
    type SeqType;
    type CollectionType;
    fn from_fasta(fasta: File) -> Self::CollectionType;
    fn new() -> Self::CollectionType;
    fn seqs(&self) -> &Vec<Box<Self::SeqType>>;
    fn nseqs(&self) -> usize;
}
pub trait Sequence {}

#[derive(Default, Clone)]
pub struct DnaSequence {
    pub header: String,
    pub sequence: String,
}
impl DnaSequence {
    pub fn new(header: String, sequence: String) -> Self {
        DnaSequence {header, sequence}
    }
}
impl Sequence for DnaSequence {}

pub struct DnaSequenceVector (pub Vec<Box<DnaSequence>>);
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
            collector.push( Box::new(DnaSequence::new(pair.0, pair.1)));
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
#[derive(Default,Clone)]
pub struct GenericFastaPair (String, String);
pub fn parse_fasta (fasta: File) -> Vec<GenericFastaPair> {
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
        line = line.replace('\n',"");
        if line.starts_with(">") {
            if pair.1.len() != 0 {
                collector.push(pair.clone());
            }
            pair.0 = line.replace('>',"");
            pair.1.clear();
        } else {
            pair.1.push_str(&line);
        }
    }
    collector
}