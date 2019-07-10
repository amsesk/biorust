use std::fs::File;
pub mod sequence;
use sequence::SequenceCollection;

fn main() {
    let fasta = File::open("/Users/kevinamses/Documents/Flux_Downlaods/zoopagalean.genomes/draft_genomes/concensus/acaulopage_concensus.new.fasta").expect("Error opening file.");

    let seqs = sequence::DnaSequenceVector::from_fasta(fasta);

    println!("{}", seqs.nseqs());
}
