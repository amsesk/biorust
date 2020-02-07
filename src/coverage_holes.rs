use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    let mut ranges: Vec<Range> = vec![];

    let mut last_range: Option<Range> = None;

    if let Ok(lines) = read_lines("/home/aimzez/dev/biorust/data/para_ISR_toParsed1_SW1000.tsv") {
        for line in lines {
            if let Ok(ip) = line {
                let spl: Vec<&str> = ip.split('\t').collect();

                let this_scaffold: i32 = spl[0]
                    .parse::<i32>()
                    .expect("Failed to parse scaffold number");

                let range_spl: Vec<i64> = spl[1]
                    .split(':')
                    .map(|val| val.parse::<i64>().expect("Failed to parse range."))
                    .collect::<Vec<i64>>();

                match last_range {
                    Some(r) => {
                        // In either case, we have moved on to a new Range and need to push the last Range before creating a new one going forward
                        // In the first case, we have switched scaffolds,
                        // In the second case, we are no longer in the same sliding window
                        //println!("{}, {}", r.stop, range_spl[1]);
                        if r.scaffold != this_scaffold || range_spl[1] - 1 != r.stop {
                            ranges.push(r);
                            last_range =
                                Some(Range::new(this_scaffold, range_spl[0], range_spl[1]));
                        } else {
                            last_range = Some(Range::new(this_scaffold, r.start, r.stop + 1));
                        }
                    }

                    None => {
                        last_range = Some(Range::new(this_scaffold, range_spl[0], range_spl[1]))
                    }
                }
            }
        }
    }

    for r in ranges.iter() {
        println!("{}", r);
    }
}

#[derive(Copy, Clone)]
struct Range {
    scaffold: i32,
    start: i64,
    stop: i64,
}
impl Range {
    fn new(scaffold: i32, start: i64, stop: i64) -> Range {
        Range {
            scaffold,
            start,
            stop,
        }
    }

    fn increment_stop(&mut self) {
        self.stop += 1;
    }
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{},{}", self.scaffold, self.start, self.stop)
    }
}
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    Ok(io::BufReader::new(file).lines())
}
