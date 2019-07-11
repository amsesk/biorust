use std::fs::File;
pub mod sequence;
use sequence::{Sequence,SequenceCollection};

fn main() {
    let fasta = File::open("/home/aimzez/scgid/test_data/mock_MDA_scgid_output/esom/mock.Scerevisiae.MDA.contigs.clip.200.fasta").expect("Error opening file.");

    let seqs = sequence::DnaSequenceVector::from_fasta(fasta);

    println!("{:?}", seqs.seqs()[801].sequence);
}
