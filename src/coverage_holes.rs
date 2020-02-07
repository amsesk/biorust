use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main(sw_tsv_path: &str) {
    if let Ok(lines) = read_lines(sw_tsv_path) {
        match find_bounds_from_sliding_windows(lines) {
            Ok(ranges) => {
                for r in ranges.iter() {
                    println!("{},{},{}", r.scaffold, r.start, r.stop);
                }
            }

            Err(e) => println!("{}", e),
        }
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

    fn incr_stop(mut self) -> Self {
        self.stop += 1;
        self
    }
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{},{}", self.scaffold, self.start, self.stop)
    }
}

fn find_bounds_from_sliding_windows(
    lines: io::Lines<io::BufReader<File>>,
) -> Result<Vec<Range>, Box<dyn Error>> {
    let mut ranges: Vec<Range> = vec![];

    let mut last_range: Option<Range> = None;

    for line in lines {
        if let Ok(ip) = line {
            let spl: Vec<&str> = ip.split('\t').collect();

            let this_scaffold: i32 = spl[0].parse::<i32>()?;

            let range_spl: Vec<i64> = spl[1]
                .split(':')
                .map(|val| val.parse::<i64>().expect("Error parsing range pair."))
                .collect::<Vec<i64>>();

            last_range = match last_range {
                Some(r) => {
                    // In either case, we have moved on to a new Range and need to push the last Range before creating a new one going forward
                    // In the first case, we have switched scaffolds,
                    // In the second case, we are no longer in the same sliding window
                    if r.scaffold != this_scaffold || range_spl[1] - 1 != r.stop {
                        ranges.push(r);
                        Some(Range::new(this_scaffold, range_spl[0], range_spl[1]))
                    } else {
                        //Some(Range::new(this_scaffold, r.start, r.stop + 1))
                        Some(r.incr_stop())
                    }
                }
                None => Some(Range::new(this_scaffold, range_spl[0], range_spl[1])),
            }
        }
    }

    Ok(ranges)
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
