use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let inputfile = &args[1];
    
    let contents = fs::read_to_string(inputfile).expect("failed to read inputfile");

    let measurements: Vec<&str> = contents.split('\n').collect();

    let mut last_measurement = -1;
    let mut num_increases = 0;
    for measurement in measurements {
        if measurement.len() > 0 {
	    let current: i32 = measurement.parse().unwrap();
	    if last_measurement > 0 {
	        if current > last_measurement {
	            num_increases += 1;
                }
	    }
            
            last_measurement = current;
        }
    }

    println!("total increases: {}", num_increases);
}
