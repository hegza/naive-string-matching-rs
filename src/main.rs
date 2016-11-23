#![allow(unused_must_use)]
#![allow(dead_code)]

extern crate ocl;
extern crate time;
extern crate clap;

mod algo;
mod test;

use std::error::Error;
use algo::measure::Measure;
use std::io::stdout;
use std::io::prelude::*;
use std::fs::File;
use clap::{Arg, App};
use std::path::Path;

fn main() {

    let matches = App::new("String Matching")
        .version("0.1")
        .author("Lunnikivi H. <henri.lunnikivi@gmail.com>")
        .about("Counts the instances of pattern in the text. Example: rust-string-matching cat data/genome/ecoli.txt")
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
        .get_matches();
    

    // Read TEXT from a file
    let text_file = matches.value_of("text").unwrap_or("data/genome/ecoli.txt");
    print!("Loading the text of {} to heap... ", text_file);
    let text = read_file( text_file );
    println!("done.");


    // Define the pattern to be searched for
    let pattern_str = matches.value_of("text").unwrap_or("cat");
    let pattern: &[u8] = pattern_str.as_bytes();

    
    println!("Searching for instances of \"{}\" in {}", pattern_str, text_file);

    naive_cpu_testcase(text.as_bytes(), pattern);
    naive_gpu_testcase(text.as_bytes(), pattern);


    // Write output
    if let Some(output_filepath) = matches.value_of("output_file") {
        write_file(output_filepath, "lorem ipsum dolor sit amet\n");
    }

}

pub fn read_file(source_file: &str) -> String {
    
    let mut file = match File::open( source_file ) {
        Err(why) => panic!("couldn't open file: {}", why),
        Ok(file) => file,
    };
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    content

}

fn write_file(filepath: &str, content: &str) {

    let path = Path::new(filepath);
    let display = path.display();

    let mut file = match File::create( &path ) {
        Err(why) => panic!("couldn't create {}: {}",
                           display,
                           why.description()),
        Ok(file) => file,
    };

    match file.write_all(content.as_bytes()) {
        Err(why) => {
            panic!("couldn't write to {}: {}", display,why.description())
        },
        Ok(_) => println!("successfully wrote to {}", display),
    }

}

fn naive_cpu_testcase(text: &[u8], pattern: &[u8]) {

    print!("Computing matches on the CPU using Naïve SM... ");
    stdout().flush();

    let mut measure_cpu = Measure::start();
    let matches = algo::naive_match_cpu(text, pattern);
    measure_cpu.stop();
    println!("done ({:?} micros).", measure_cpu.duration().num_microseconds().unwrap());

    println!("{} matches found.", matches);

}

fn naive_gpu_testcase(text: &[u8], pattern: &[u8]) {

    print!("Computing matches on the GPU using Naïve SM... ");
    stdout().flush();

    let mut measure_gpu = Measure::start();
    let mut matches = algo::naive_match_gpu(text, pattern);
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


    /*
    {
        println!("<CPU>");
        
        println!("Matching pattern {:?} to the text of E. Coli", &pattern);

        print!("Computing prefix function for {:?}...", &pattern);
        println!(" {:?}", &prefix);

        let mut matchcount = 0;
        let mut match_offset = 0;

        while match_offset != text.chars().count() {
            match_offset = algo::kmp_match_cpu( text.as_bytes(), pattern, &prefix, match_offset );
            println!("a match found at: {}", match_offset);
            match_offset += 1;
            matchcount += 1;
        }

        println!("Matches found: {}", matchcount);
        println!("<CPU>");
    }
    */

