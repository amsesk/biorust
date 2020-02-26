use crate::lib::read_lines;
use crate::sequence::{DnaSequenceVector, SequenceCollection};
use std::convert::TryFrom;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn spdbify_fasta_headers(mut seqs: DnaSequenceVector, tag: String) -> DnaSequenceVector {
    for s in seqs.0.iter_mut() {
        s.header = format!("{} OS={}", s.header, tag);
    }
    seqs
}

pub fn spdbify_proteomes<P>(map_path: P, out_fasta: P)
where
    P: AsRef<Path>,
{
    if let Ok(lines) = read_lines(map_path) {
        let parsed_map = TagMap::try_from(lines).unwrap();
        let mut buffer = File::create(out_fasta).expect("Couldn't create file.");
        for m in parsed_map.into_iter() {
            let seqs = DnaSequenceVector::from_fasta(m.file);
            let seqs_renamed = spdbify_fasta_headers(seqs, m.tag);
            for s in seqs_renamed.seqs() {
                let out = format!("{}\n", s);
                let bytes = out.as_bytes();
                buffer
                    .write_all(&bytes)
                    .expect("Could not write to fasta file.");
            }
        }
    }
}

pub struct TagMap(Vec<TagMapping>);

impl TagMap {
    fn empty() -> TagMap {
        TagMap(Vec::<TagMapping>::new())
    }

    fn push(&mut self, tag_mapping: TagMapping) {
        self.0.push(tag_mapping);
    }

    fn into_iter(self) -> std::vec::IntoIter<TagMapping> {
        self.0.into_iter()
    }
}

impl TryFrom<std::io::Lines<std::io::BufReader<std::fs::File>>> for TagMap {
    type Error = String;
    fn try_from(
        lines: std::io::Lines<std::io::BufReader<std::fs::File>>,
    ) -> Result<Self, Self::Error> {
        let mut map = TagMap::empty();
        for (idx, line) in lines.enumerate() {
            let line = line.unwrap();
            let spl: Vec<&str> = line.split("\t").collect();
            if spl.len() != 3 {
                return Err(format!(
                    "Line {} is not composed of three tab-separated values. Found {} values: {}",
                    idx + 1,
                    spl.len(),
                    spl.join(",")
                ));
            }
            if let Ok(f_open) = File::open(&spl[1]) {
                map.push(TagMapping::new(f_open, String::from(spl[0])))
            } else {
                return Err(format!("Unable to open file at `{}`", &spl[0]));
            }
        }
        return Ok(map);
    }
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

/* No longer needed
pub fn read_raw_map<'a, P>(map_path: P) -> (std::vec::IntoIter<File>, std::vec::IntoIter<String>)
where
    P: AsRef<Path>,
{
    let mut files = vec![];
    let mut tags = vec![];

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
*/
