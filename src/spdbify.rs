use crate::lib::read_lines;
use crate::sequence::{DnaSequenceVector, SequenceCollection};
use std::fs::File;
use std::path::Path;

pub fn spdbify_fasta_headers(seqs: DnaSequenceVector, tag: String) {
    for mut s in seqs.into_seqs() {
        s.header = format!("{} OS={}", s.header, tag);
        println!("{}", s);
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

pub fn read_raw_map<'a, P>(map_path: P) -> (std::vec::IntoIter<File>, std::vec::IntoIter<String>)
where
    P: AsRef<Path>,
{
    let mut files = vec![];
    let mut tags = vec![];

    if let Ok(lines) = read_lines(map_path) {
        for line in lines {
            let res = line.unwrap();
            let spl: Vec<&str> = res.split("\t").collect();
            let f_open = File::open(&spl[1]).expect("Error opening file.");
            files.push(f_open);
            tags.push(String::from(spl[0]));
        }
    }
    (files.into_iter(), tags.into_iter())
}

pub fn build_tag_map<I, T>(files: I, tags: T) -> Vec<TagMapping>
where
    I: Iterator<Item = File>,
    T: Iterator<Item = String>,
{
    let mut map = vec![];
    for (p, t) in files.zip(tags) {
        map.push(TagMapping::new(p, t));
    }

    map
}

pub struct TagMapping {
    file: File,
    tag: String,
}

impl TagMapping {
    fn new(file: File, tag: String) -> TagMapping {
        TagMapping { file, tag }
    }
}
