#![allow(unused_must_use)]
#![allow(dead_code)]

extern crate ocl;
extern crate time;
extern crate clap;

mod algo;
mod test;

use algo::measure::Measure;
use std::io::Write;
use std::io::stdout;
use clap::{Arg, App, SubCommand};

fn main() {

    let matches = App::new("String Matching")
        .version("0.1")
        .author("Lunnikivi H. <henri.lunnikivi@gmail.com>")
        .about("Counts the instances of PATTERN in the TEXT. Example: rust-string-matching cat data/genome/ecoli.txt")
        .arg(Arg::with_name("PATTERN")
             .help("The pattern to be matched against the text")
             .required(true)
             .index(1))
        .arg(Arg::with_name("TEXT")
             .value_name("TEXT")
             .help("Imports TEXT from the given file")
             .required(true)
             .index(2)).get_matches();

    

    // Read file
    print!("Loading the text of E. Coli to heap... ");
    let text = algo::read_file( matches.value_of("TEXT").unwrap() );
    println!("done.");


    // Define the pattern to be searched
    let pattern_str = matches.value_of("PATTERN").unwrap();
    let pattern: &[u8] = pattern_str.as_bytes();

    println!("Matching \"{}\" to E. Coli DNA.", pattern_str);


    //kmp_gpu_testcase(text.as_bytes(), pattern);
    naive_cpu_testcase(text.as_bytes(), pattern);
    naive_gpu_testcase(text.as_bytes(), pattern);

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

