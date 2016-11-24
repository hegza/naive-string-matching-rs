#![allow(unused_must_use)]
#![allow(dead_code)]

extern crate ocl;
extern crate time;

extern crate clap;
pub mod file_io;
mod algo;
mod test;

pub use file_io::*;
use algo::measure::Measure;
use clap::{Arg, App};
use std::io::stdout;
use std::io::prelude::*;
use time::Duration;

fn main() {

    let matches = App::new("GPU String Matching")
        .version("0.1")
        .author("Lunnikivi H. <henri.lunnikivi@gmail.com>")
        .about("Counts the instances of a pattern in a text. Example: rust-string-matching cat data/genome/ecoli.txt")
        .arg(Arg::with_name("pattern")
             .value_name("PATTERN")
             .help("The pattern to be matched against the text")
             .required(false)
             .index(1))
        .arg(Arg::with_name("text")
             .value_name("TEXT")
             .help("Imports text from the given file")
             .required(false)
             .index(2))
        .arg(Arg::with_name("output_file")
             .required(false)
             .help("File for outputting results as .csv")
             .long("output")
             .short("o")
             .takes_value(true))
        .arg(Arg::with_name("repeat_count")
            .required(false)
            .help("Number of times to run the test case")
            .long("count")
            .short("c")
            .takes_value(true))
        .get_matches();


    // Read TEXT from a file
    let text_file = matches.value_of("text").unwrap_or("data/genome/ecoli.txt");
    stdout().flush();
    print!("Loading the text of {} to heap... ", text_file);
    let text = read_file( text_file );
    println!("done.");


    // Define the pattern to be searched for
    let pattern_str = matches.value_of("pattern").unwrap_or("cat");
    let pattern: &[u8] = pattern_str.as_bytes();

    // Parse repeat count
    let repeat_count = matches.value_of("repeat_count").unwrap_or("10").parse::<usize>().unwrap();


    println!("Searching for instances of \"{}\" in {}", pattern_str, text_file);

    let cpu_result = elaborate( "Naive SM on the CPU",
        &algo::naive_match_cpu, text.as_bytes(), pattern, repeat_count );

    let gpu_result = elaborate( "Naive SM on the GPU",
        &algo::naive_match_gpu, text.as_bytes(), pattern, repeat_count );


    // Write output
    if let Some(output_filepath) = matches.value_of("output_file") {
        let content = format!("{:?} {:?}\n", cpu_result, gpu_result);

        write_file(output_filepath, &content);
    }

}

fn elaborate<F>(message: &str, test_case: &F, text: &[u8], pattern: &[u8], repeat_count: usize) -> (bool, usize)
    where F: Fn(&[u8], &[u8]) -> usize {

    print!("Computing {} (x{:?})... ", message, repeat_count);
    stdout().flush();
    let total_duration = repeat_testcase( test_case, text, pattern, repeat_count );
    let average_run = total_duration / repeat_count as i32;
    println!("done (averaging {:?} microseconds)", average_run.num_microseconds().unwrap());

    (true, average_run.num_microseconds().unwrap() as usize)

}

fn repeat_testcase<F>(test_case: &F, text: &[u8], pattern: &[u8], repeat_count: usize) -> time::Duration
    where F: Fn(&[u8], &[u8]) -> usize {

    let mut match_counts = vec![0; repeat_count];
    let mut nanos: std::vec::Vec<u32> = vec![0; repeat_count];
    for i in 0..repeat_count {
        let mut measure_cpu = Measure::start();
        match_counts[i] = test_case(text, pattern);
        measure_cpu.stop();

        nanos[i] = measure_cpu.duration().num_nanoseconds().unwrap() as u32;
    }

    // Coalesce
    let mut total_duration = Duration::nanoseconds(0);
    for i in 0..repeat_count {
        total_duration = total_duration + Duration::nanoseconds(nanos[i] as i64);
    }

    return total_duration;

}

/*
fn kmp_gpu_testcase(text: &[u8], pattern: &[u8]) {
    // Compute prefix array
    print!("Computing the prefix array for {:?} (1-thread CPU)... ", pattern);
    stdout().flush();

    let mut measure_prefix_computation = Measure::start();
    let prefix = algo::kmp_compute_prefix_function( pattern );
    measure_prefix_computation.stop();

    println!("done ({:?} micros)", measure_prefix_computation.duration().num_microseconds().unwrap());


    // TODO: Tee multipass ratkaisu, MapReduce vois toimia


    print!("Computing matches on the GPU using KMP... ");
    stdout().flush();

    let mut measure_gpu = Measure::start();
    let mut matches = algo::kmp_match_gpu(text, pattern, &prefix);
    measure_gpu.stop();
    println!("done ({:?} micros).", measure_gpu.duration().num_microseconds().unwrap());

    print!("Deduplicating results vector... ");
    stdout().flush();

    let mut measure_dedup = Measure::start();
    matches.dedup();
    measure_dedup.stop();
    println!("done ({:?} micros)", measure_dedup.duration().num_microseconds().unwrap());


    println!("{} matches found.", matches.len()-1);
}
*/
