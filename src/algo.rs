
use time::{PreciseTime};
use ocl::{util, core, ProQue, Buffer};
use std::io::prelude::*;
use std::fs::File;

const COEFF: i32 = 2;

fn read_kernel(source_file: &'static str) -> String {
    
    let mut kernel_file = match File::open( source_file ) {
        Err(why) => panic!("couldn't open file: {}", why),
        Ok(file) => file,
    };
    let mut kernel_string = String::new();
    kernel_file.read_to_string(&mut kernel_string).unwrap();

    kernel_string

}

pub fn opencl_test( item_count: usize ) {

    let kernel_string = read_kernel("data/test.c");

    let ocl_pq = ProQue::builder()
        .src( kernel_string )
        .dims([item_count])
        .build().expect("Build ProQue");    


    print!("Creating on-device buffers... ");
    let buffer_timer_begin = PreciseTime::now();
    // Create input data
    let input = util::scrambled_vec((0, 10), ocl_pq.dims().to_len());
    // Convert it into an on-device buffer
    let source_buffer = Buffer::new(&ocl_pq.queue().clone(), Some(core::MEM_READ_WRITE |
        core::MEM_COPY_HOST_PTR), ocl_pq.dims().clone(), Some(&input)).unwrap();

    // Create an empty buffer for results
    let result_buffer: Buffer<i32> = ocl_pq.create_buffer().unwrap();
    let buffer_timer_end = PreciseTime::now();
    println!(" took {:?} micros", buffer_timer_begin.to(buffer_timer_end).num_microseconds().unwrap());

    print!("Creating kernel... ");
    let kernel_creation_timer_begin = PreciseTime::now();
    let kernel = ocl_pq.create_kernel("test").unwrap()
        .arg_scl(COEFF)
        .arg_buf(&source_buffer)
        .arg_buf(&result_buffer);
    let kernel_creation_timer_end = PreciseTime::now(); 
    println!(" took {:?} micros", kernel_creation_timer_begin.to(kernel_creation_timer_end).num_microseconds().unwrap());
    
    print!("Computing... ");
    let opencl_compute_time_begin = PreciseTime::now();
    kernel.enq().unwrap();
    let opencl_compute_time_end = PreciseTime::now();
    println!(" took {:?} micros", opencl_compute_time_begin.to(opencl_compute_time_end).num_microseconds().unwrap());

    
    // Download the results
    let mut results_vector = vec![0i32; item_count];
    result_buffer.read(&mut results_vector).enq().unwrap();

}

pub fn cpu_test( item_count: usize ) {

    print!("Reserving memory...");
    let ram_timer_begin = PreciseTime::now();

    let rust_source = vec![0; item_count];
    let mut rust_results = vec![0; item_count];
    let ram_timer_end = PreciseTime::now();
    println!(" took {:?} micros", ram_timer_begin.to( ram_timer_end ).num_microseconds().unwrap());

    print!("Computing...");
    let cpu_compute_time_begin = PreciseTime::now();
    for i in 0..item_count {
        rust_results[i] = rust_source[i] * COEFF;
    }
    let cpu_compute_time_end = PreciseTime::now();
    println!(" took {:?} micros", cpu_compute_time_begin.to( cpu_compute_time_end ).num_microseconds().unwrap() );

}

