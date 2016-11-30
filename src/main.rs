#![allow(unused_must_use)]

extern crate ocl;
extern crate time;
extern crate clap;

pub mod file_io;
mod algo;
mod test;

pub use file_io::*;
use std::io::stdout;
use clap::{Arg, App};
use std::io::prelude::*;
use std::io::{self};

fn main() {

    let params = App::new("GPU String Matching")
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
    let text_file = params.value_of("text").unwrap_or("data/genome/ecoli.txt");
    print!("Loading the text of {} to heap... ", text_file);
    stdout().flush();
    let text = read_file( text_file );
    println!("done.");


    // Define the pattern to be searched for
    let pattern_str = params.value_of("pattern").unwrap_or("cat");
    let pattern: &[u8] = pattern_str.as_bytes();

    // Parse repeat count
    let repeat_count = params.value_of("repeat_count").unwrap_or("10").parse::<usize>().unwrap();

    println!("Searching for instances of \"{}\" in {}", pattern_str, text_file);


    // Initialize test cases
    let cpu_fn = algo::naive_match_cpu;
    let gpu_fn = algo::naive_match_gpu;

    let cpu_test_case = algo::StringMatchSuite::create(
        "Naive SM on the CPU",
        &cpu_fn,
        text.as_bytes(), pattern,
        repeat_count);

    let gpu_test_case = algo::StringMatchSuite::create(
        "Naive SM on the GPU",
        &gpu_fn,
        text.as_bytes(), pattern,
        repeat_count);


    // Run test cases
    print!("{}", cpu_test_case.description());
    stdout().flush();
    let cpu_result = cpu_test_case.execute();
    println!("{}", cpu_result.elaborate());

    print!("{}", gpu_test_case.description());
    stdout().flush();
    let gpu_result = gpu_test_case.execute();
    println!("{}", gpu_result.elaborate());

    // Write output
    if let Some(output_filepath) = params.value_of("output_file") {
        let content = format!(
            "cpu_average_exec_time, gpu_average_exec_time\n{:?}, {:?}\n",
            cpu_result, gpu_result);
        write_file(output_filepath, &content);
    }

}
