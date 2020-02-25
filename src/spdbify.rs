use crate::lib::read_lines;
use crate::sequence::{DnaSequenceVector, SequenceCollection};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn spdbify_fasta_headers(seqs: DnaSequenceVector, tag: &str) {
    for s in seqs.seqs() {
        println!("{} OS={}", s.header, tag)
    }
}

pub fn spdbify_proteomes<P>(map: P)
where
    P: AsRef<Path>,
{
    let (f, t) = read_raw_map(map);
    let parsed_map = build_tag_map(f, t);
    for m in parsed_map.into_iter() {
        let seqs = DnaSequenceVector::from_fasta(m.file);
        spdbify_fasta_headers(seqs, m.tag)
    }
}

pub fn read_raw_map<'a, P>(map_path: P) -> (std::vec::IntoIter<File>, std::vec::IntoIter<&'a str>)
where
    P: AsRef<Path>,
{
    let mut files = vec![];
    let mut tags = vec![];

    if let Ok(lines) = read_lines(map_path) {
        for line in lines {
            let spl: Vec<&str> = line.unwrap().split("\t").collect();
            let f_open = File::open(&spl[1]).expect("Error opening file.");
            files.push(f_open);
            tags.push(spl[0]);
        }
    }
    (files.into_iter(), tags.into_iter())
}

pub fn build_tag_map<'a, I, T>(files: I, tags: T) -> Vec<TagMapping<'a>>
where
    I: Iterator<Item = File>,
    T: Iterator<Item = &'a str>,
{
    let mut map = vec![];
    for (p, t) in files.zip(tags) {
        map.push(TagMapping::new(p, t));
    }

    map
}

pub struct TagMapping<'a> {
    file: File,
    tag: &'a str,
}

impl TagMapping<'_> {
    fn new(file: File, tag: &str) -> TagMapping {
        TagMapping { file, tag }
    }
}
