use std::fs::File;
pub mod config;
pub mod coverage_holes;
pub mod sequence;
//use sequence::{Sequence, SequenceCollection};

fn main() {
    let args = config::get_clargs();

    coverage_holes::main(&args[1]);

    /*)
    let fasta = File::open("/home/aimzez/scgid/test_data/mock_MDA_scgid_output/esom/mock.Scerevisiae.MDA.contigs.clip.200.fasta").expect("Error opening file.");

    let seqs = sequence::DnaSequenceVector::from_fasta(fasta);

    println!("{:?}", seqs.seqs()[801].sequence);
    */
}
