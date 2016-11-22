#![allow(dead_code)]

pub mod measure;
mod naive_sm;

pub use self::naive_sm::*;
use self::measure::Measure;
use ocl::{core, ProQue, Buffer, Kernel};
use std::io::prelude::*;
use std::fs::File;
use time::Duration;
use std::i32;

pub struct TestResult {
    pub success: bool,
    pub total_time: Duration,
}

pub fn read_file(source_file: &str) -> String {
    
    let mut kernel_file = match File::open( source_file ) {
        Err(why) => panic!("couldn't open file: {}", why),
        Ok(file) => file,
    };
    let mut kernel_string = String::new();
    kernel_file.read_to_string(&mut kernel_string).unwrap();

    kernel_string

}

impl Measure {

    pub fn get_message( &self ) -> String {
        format!(" took {:?} micros", self.duration().num_microseconds().unwrap() )
    }

}

// TODO: move to another file
/*
pub fn rk_match_cpu(text: &[u8], pattern: &[u8]) -> i32 {
    let d = 4;
    let q = 13; // d*q should be less than word length (64?) and a prime
    let n = text.len();
    let m = pattern.len();
    let h = d.pow(m-1) % q;
    let mut p = 0;
    let mut t = 0;
    
    // Preprocessing
    for i in 1..m+1 {
        p = (d*p + pattern[i]) % q;
        t = (d*t + text[i]) % q;
    }
    // Matching
    for s in 0..n-m+1 {
        if p == t {
            if pattern[1..m] == text[s..s+m] {
                return s;
            }
        }
        if s < n-m {
            t = (d(t-text[s]*h)+text[s+m]) % q;
        }

    }
}
*/

// TODO: move to another file
pub fn kmp_compute_prefix_function(pattern: &[u8]) -> Vec<i32> {

    let m = pattern.len();
    let mut prefix: Vec<i32> = vec![0; m];

    let mut k = 0;
    for q in 1..m-1 { // skip index 0
        while k > 0 && (pattern[k] != pattern[q]) {
            k = prefix[k-1] as usize;
        }
        if pattern[k] == pattern[q] {
            k += 1;
        }
        prefix[q+1] = k as i32;
    }

    return prefix;

}

pub fn kmp_match_cpu(text: &[u8], pattern: &[u8], prefix: &Vec<i32>, offset: usize) -> usize {
    let n = text.len();
    let m = pattern.len();

    let mut q = 0;
    for i in offset..n+1 {
        while q > 0 && pattern[q] != text[i] {
            q = prefix[q] as usize;
        }
        if pattern[q] == text[i] {
            q += 1;
        }
        if q == m {
            let shift = (i+1)-m;
            return shift;
        }
    }

    return text.len();
}

fn create_kmp_kernel(text: &[u8], pattern: &[u8], prefix: &Vec<i32>) -> (Kernel, Buffer<i32>) {
    
    let n = text.len();
    let m = pattern.len();

    let kernel_string = read_file("data/kmp.c");

    let ocl_pq = ProQue::builder()
        .src( kernel_string )
        .dims([n])
        .build().expect("Build ProQue");    

    // Convert input data into on-device buffers
    let text_buffer = Buffer::new(&ocl_pq.queue().clone(), Some(core::MEM_READ_WRITE | 
        core::MEM_COPY_HOST_PTR), [n], Some(text)).unwrap();
    let pattern_buffer = Buffer::new(&ocl_pq.queue().clone(), Some(core::MEM_READ_WRITE | 
        core::MEM_COPY_HOST_PTR), [m], Some(pattern)).unwrap();
    let prefix_buffer = Buffer::new(&ocl_pq.queue().clone(), Some(core::MEM_READ_WRITE | 
        core::MEM_COPY_HOST_PTR), [prefix.len()], Some(prefix)).unwrap();

    
    // Create an empty buffer for results
    let result_buffer: Buffer<i32> = ocl_pq.create_buffer().unwrap();


    let kernel = ocl_pq.create_kernel("kmp").unwrap()
        .arg_scl(n as i32)
        .arg_scl(m as i32)
        .arg_buf(&text_buffer)
        .arg_buf(&pattern_buffer)
        .arg_buf(&prefix_buffer)
        .arg_buf(&result_buffer);

    (kernel, result_buffer)

}

pub fn kmp_match_gpu(text: &[u8], pattern: &[u8], prefix: &Vec<i32>) -> Vec<i32> {

    let (kernel, result_buffer) = create_kmp_kernel(text, pattern, prefix);

    kernel.enq().unwrap();

    // Download the results
    let mut results_vector = vec![0i32; text.len()];
    result_buffer.read(&mut results_vector).enq().unwrap();

    results_vector
    //TestResult{success: true, total_time: measure_opencl_compute_time.duration() + measure_kernel_create.duration() + measure_buffering.duration()}
    
}

macro_rules! kmp_match_cpu {
    ($a: expr, $b: expr) => { let prefix = kmp_compute_prefix_function($b); kmp_match_cpu($a, $b, prefix, 0) };
}

