extern crate ocl;
extern crate time;

use std::{option, mem};
use ocl::{util, core, ProQue, Buffer};
use time::{PreciseTime, Duration};

const DATA_SET_POWER: usize = 18;
const DATA_SET_SIZE: usize = 2 << DATA_SET_POWER;
const COEFF: i32 = 2;

static KERNEL_SRC: &'static str = r#"
    __kernel void test(
                __private int const coeff,
                __global int const* const input,
                __global int* const result)
    {
        uint const idx = get_global_id(0);
        result[idx] = input[idx] * coeff;
    }
"#;

fn main() {

    println!("# of work items: {:?} (2^{:?})", DATA_SET_SIZE, DATA_SET_POWER);


    println!("{}", "Initiating OpenCL test");

    let ocl_pq = ProQue::builder()
        .src(KERNEL_SRC)
        .dims([DATA_SET_SIZE])
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
    let mut results_vector = vec![0i32; DATA_SET_SIZE];
    result_buffer.read(&mut results_vector).enq().unwrap();

    println!("");

    // TODO: use test::blackbox to avoid compiler optimizations
   
    println!("Initiating CPU test");

    print!("Reserving memory...");
    let ram_timer_begin = PreciseTime::now();
    let rust_source: [i32;DATA_SET_SIZE] = [0;DATA_SET_SIZE];
    let mut rust_results: [i32;DATA_SET_SIZE] = [0;DATA_SET_SIZE];
    let ram_timer_end = PreciseTime::now();
    println!(" took {:?} micros", ram_timer_begin.to( ram_timer_end ).num_microseconds().unwrap());

    print!("Computing...");
    let cpu_compute_time_begin = PreciseTime::now();
    for i in 0..DATA_SET_SIZE {
        rust_results[i] = rust_source[i] * COEFF;
    }
    let cpu_compute_time_end = PreciseTime::now();
    println!(" took {:?} micros", cpu_compute_time_begin.to( cpu_compute_time_end ).num_microseconds().unwrap() );
    

    /*
    let mut i = 0;
    for result in results_vector {
        println!("Result (idx:{}): {} -> {}", i, input[i], result);
        i += 1;
    }
    */
    
    //println!()


    
}

