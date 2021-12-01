use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let inputfile = &args[1];
    
    let contents = fs::read_to_string(inputfile).expect("failed to read inputfile");

    let measurements: Vec<i32> = contents.split_whitespace().filter_map(|s| s.parse().ok()).collect();

    let mut last_measurement = -1;
    let mut num_increases = 0;
    let mut index = 0;
    while index < measurements.len()-2 {    
        let current = measurements[index] + measurements[index+1] + measurements[index+2];
	if last_measurement > -1 && current > last_measurement {
            num_increases += 1;
        }
        last_measurement = current;
	index += 1;     
    }

    println!("total increases: {}", num_increases);
}
