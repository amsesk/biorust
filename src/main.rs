use std::fs::File;
pub mod config;
pub mod coverage_holes;
pub mod lib;
pub mod sequence;
pub mod spdbify;

fn main() {
    let args = config::get_clargs();

    //coverage_holes::main(&args[1]);

    //let fasta = File::open(&args[1]).expect("Error opening file.");

    //let seqs = sequence::DnaSequenceVector::from_fasta(fasta);

    spdbify::spdbify_proteomes(&args[1], &args[2]);

    //println!("{:?}", seqs.seqs()[801].sequence);
}
