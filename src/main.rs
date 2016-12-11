#![allow(unused_must_use)]

extern crate ocl;
extern crate time;
extern crate clap;
extern crate rand;

pub mod file_io;
mod algo;
mod test;

pub use file_io::*;
use std::io::stdout;
use clap::{Arg, App};
use std::io::prelude::*;
use rand::Rng;

enum RepeatType {
    Single,
    Repeat(usize),
    Sweep {repeat_count: usize, sweep_lower: usize, sweep_upper: usize},
}

fn main() {

    let params = App::new("GPU String Matching")
        .version("0.1")
        .author("Lunnikivi H. <henri.lunnikivi@gmail.com>")
        .about("Counts the instances of a pattern in a text. Example: rust-string-matching cat data/genome/ecoli.txt")
        .arg(Arg::with_name("text")
             .value_name("TEXT")
             .help("Imports text from the given file")
             .required(false)
             .index(1))
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
        .arg(Arg::with_name("pattern")
            .required(false)
            .help("The pattern")
            .long("pattern")
            .short("p")
            .takes_value(true))
        .arg(Arg::with_name("generate_pattern")
            .required(false)
            .help("Randomly generate the pattern with N characters")
            .long("generate")
            .short("g")
            .takes_value(true))
        .arg(Arg::with_name("sweep")
            .required(false)
            .help("Repeat test case for each count of characters less than the # of chars in pattern and more than sweep")
            .long("sweep")
            .short("s")
            .takes_value(true))
        .get_matches();

    let chars = vec!['c', 'a', 't'];

    // Read TEXT from a file
    let text_file = params.value_of("text").unwrap_or("data/genome/ecoli.txt");
    print!("Loading the text of {} to stack... ", text_file);
    stdout().flush();
    let file_contents = read_file( text_file );
    let text: &[u8] = file_contents.as_bytes();
    println!("done.");

    // Parse pattern
    let pattern_str = params.value_of("pattern").unwrap_or("cat");

    // Parse generate pattern_str
    let generate_pattern = params.value_of("generate_pattern");
    let pattern_len: usize = if let Some(generate_pattern) = generate_pattern {
        generate_pattern.parse::<usize>().unwrap_or(0)
    }
    else {
        pattern_str.chars().count()
    };

    // Parse repeat count
    let repeat_count_str = params.value_of("repeat_count").unwrap_or("1");
    let repeat_count = repeat_count_str.parse::<usize>().unwrap();

    let repeat;
    // Parse repeat or sweep
    if let Some(sweep) = params.value_of("sweep") {
        let sweep_lower = sweep.parse::<usize>().unwrap();
        let sweep_upper = pattern_len;
        if sweep_lower > sweep_upper {
            panic!("Sweep lower bound cannot be higher than upper bound");
        }
        repeat = RepeatType::Sweep {repeat_count: repeat_count, sweep_lower: sweep_lower, sweep_upper: sweep_upper};
    }
    else if repeat_count != 1 {
        repeat = RepeatType::Repeat(repeat_count);
    }
    else {
        repeat = RepeatType::Single;
    }

    match repeat {
        RepeatType::Single => {
            println!("Performing a single experiment");
        },
        RepeatType::Repeat(repeat_count) => {
            println!("Performing {} experiments", repeat_count);
        },
        RepeatType::Sweep {repeat_count, sweep_lower, sweep_upper} => {
            println!("Sweeping from {} to {}, each character count is repeated {} times", sweep_lower, sweep_upper, repeat_count);
        },
    }

    let mut output_content = String::from("");

    match repeat {
        RepeatType::Single => {
            let pattern: String = match generate_pattern {
                None => String::from(pattern_str),
                Some(_) => {
                    let mut p: String = String::from("");
                    for _ in 0..pattern_len {
                        p.push( rand::thread_rng().choose(&chars).unwrap().clone() );
                    }
                    p
                }
            };
            println!("Searching for pattern: {} in {}", pattern, text_file);
            let results = single_case(&pattern.as_bytes(), &text);
            output_content = format!("{:?}, {:?}\n",
                    results.0.average_run.num_microseconds().unwrap(),
                    results.1.average_run.num_microseconds().unwrap());
        },
        RepeatType::Repeat(repeat_count) => {
            let mut patterns: Vec<String> = vec![String::from("");repeat_count];
            match generate_pattern {
                None => {
                    for idx in 0..repeat_count {
                        patterns[idx] = String::from(pattern_str);
                    }
                },
                Some(_) => {
                    for idx in 0..repeat_count {
                        for _ in 0..pattern_len {
                            patterns[idx].push( rand::thread_rng().choose(&chars).unwrap().clone() );
                        }
                    }
                },
            }

            println!("Searching for patterns: {:?} in {}", patterns, text_file);
            let results = repeat_cases(repeat_count, patterns.iter().map(|x|x.as_bytes()).collect(), &text);
            output_content = format!("{:?}, {:?}\n",
                    results.0.average_run.num_microseconds().unwrap(),
                    results.1.average_run.num_microseconds().unwrap());
        },
        RepeatType::Sweep {repeat_count, sweep_lower, sweep_upper} => {
            // eg. repeat_count = 2, sweep_lower = 3, sweep_upper = 5, [["cat", "tac"], ["atca", "ttac"], ["atcct", "ttata"]]
            let mut patterns_by_sweep: Vec<Vec<String>> = vec![vec![String::from("");repeat_count];sweep_upper-sweep_lower + 1];
            for sweep in sweep_lower..sweep_upper+1 {
                let pattern_len = sweep;
                let patterns = &mut patterns_by_sweep[sweep-sweep_lower];
                for idx in 0..repeat_count {
                    for _ in 0..pattern_len {
                        patterns[idx].push( rand::thread_rng().choose(&chars).unwrap().clone() );
                    }
                }
            };

            println!("Searching for patterns: {:?} in {}", patterns_by_sweep, text_file);
            let results = sweep_cases(repeat_count, sweep_lower, sweep_upper, patterns_by_sweep.iter().map(|v|v.iter().map(|x|x.as_bytes()).collect()).collect(), text);
            output_content.push_str( "letters, CPU, GPU\n" );
            for idx in 0..sweep_upper-sweep_lower+1 {
                let line: String = format!( "{}, {:?}, {:?}\n",
                    idx+1,
                    results[idx].0.average_run.num_microseconds().unwrap(),
                    results[idx].1.average_run.num_microseconds().unwrap()).to_owned();
                output_content.push_str( &line );
            }
        },
    }

    // Write output
    if let Some(output_filepath) = params.value_of("output_file") {
        write_file(output_filepath, &output_content);
    }

}

fn single_case(pattern: &[u8], text: &[u8]) -> (algo::SuiteResult, algo::SuiteResult) {

    let cpu_fn = algo::naive_sm::match_cpu;
    let gpu_fn = algo::naive_sm::match_gpu_full;
    let cpu_test_case = algo::StringMatchSuite::create(
        "Naive SM on the CPU",
        &cpu_fn,
        text, pattern);

    let gpu_test_case = algo::StringMatchSuite::create(
        "Naive SM on the GPU",
        &gpu_fn,
        text, pattern);

    // Run test cases
    print!("{}", cpu_test_case.description());
    stdout().flush();
    let cpu_result = cpu_test_case.execute();
    println!("{}", cpu_result.elaborate());

    print!("{}", gpu_test_case.description());
    stdout().flush();
    let gpu_result = gpu_test_case.execute();
    println!("{}", gpu_result.elaborate());

    println!("Match counts: {}, {}",
        cpu_result.match_counts[0], gpu_result.match_counts[0]);

    (cpu_result, gpu_result)
}

fn repeat_cases(repeat_count: usize, patterns: Vec<&[u8]>, text: &[u8])
        -> (algo::SuiteResult, algo::SuiteResult) {

    let cpu_fn = algo::naive_sm::match_cpu;
    let gpu_fn = algo::naive_sm::match_gpu_full;

    let cpu_test_case;
    let gpu_test_case;

    cpu_test_case = algo::StringMatchSuite::create_many(
        "Naive SM on the CPU",
        &cpu_fn,
        text, Box::new(patterns.clone()), repeat_count);

    gpu_test_case = algo::StringMatchSuite::create_many(
        "Naive SM on the GPU",
        &gpu_fn,
        text, Box::new(patterns.clone()), repeat_count);


    // Run test cases
    print!("{}", cpu_test_case.description());
    stdout().flush();
    let cpu_result = cpu_test_case.execute();
    println!("{}", cpu_result.elaborate());

    print!("{}", gpu_test_case.description());
    stdout().flush();
    let gpu_result = gpu_test_case.execute();
    println!("{}", gpu_result.elaborate());

    (cpu_result, gpu_result)
}

fn sweep_cases(repeat_count: usize, sweep_lower: usize, sweep_upper: usize,
        patterns: Vec<Vec<&[u8]>>, text: &[u8])
        -> Vec<(algo::SuiteResult, algo::SuiteResult)> {
    let mut results: Vec<(algo::SuiteResult, algo::SuiteResult)> = vec!();

    let cpu_fn = algo::naive_sm::match_cpu;
    let gpu_fn = algo::naive_sm::match_gpu_full;

    for idx in 0..sweep_upper-sweep_lower+1 {

        let cpu_test_case;
        let gpu_test_case;

        cpu_test_case = algo::StringMatchSuite::create_many(
            "Naive SM on the CPU",
            &cpu_fn,
            text, Box::new(patterns[idx].clone()), repeat_count);

        gpu_test_case = algo::StringMatchSuite::create_many(
            "Naive SM on the GPU",
            &gpu_fn,
            text, Box::new(patterns[idx].clone()), repeat_count);


        // Run test cases
        print!("{}", cpu_test_case.description());
        stdout().flush();
        let cpu_result = cpu_test_case.execute();
        println!("{}", cpu_result.elaborate());

        print!("{}", gpu_test_case.description());
        stdout().flush();
        let gpu_result = gpu_test_case.execute();
        println!("{}", gpu_result.elaborate());

        results.push( (cpu_result, gpu_result) );
    }

    results
}
