#![allow(dead_code)]

use algo::*;
use algo::measure::Measure;

use ocl::{util, core, ProQue, Buffer};

const COEFF: i32 = 2;

fn opencl_test( item_count: usize ) -> TestResult {

    let kernel_string = super::read_file("data/test.c");

    let ocl_pq = ProQue::builder()
        .src( kernel_string )
        .dims([item_count])
        .build().expect("Build ProQue");    


    print!("Creating on-device buffers... ");
    let mut measure_buffering = Measure::start();
    // Create input data
    let input = util::scrambled_vec((0, 10), ocl_pq.dims().to_len());
    // Convert it into an on-device buffer
    let source_buffer = Buffer::new(&ocl_pq.queue().clone(), Some(core::MEM_READ_WRITE |
        core::MEM_COPY_HOST_PTR), ocl_pq.dims().clone(), Some(&input)).unwrap();

    // Create an empty buffer for results
    let result_buffer: Buffer<i32> = ocl_pq.create_buffer().unwrap();
    measure_buffering.stop();

    println!( "{}", measure_buffering.get_message() );


    print!("Creating kernel... ");
    let mut measure_kernel_create = Measure::start();
    let kernel = ocl_pq.create_kernel("test").unwrap()
        .arg_scl(COEFF)
        .arg_buf(&source_buffer)
        .arg_buf(&result_buffer);
    measure_kernel_create.stop();

    println!( "{}", measure_kernel_create.get_message() );


    print!("Computing... ");
    let mut measure_opencl_compute_time = Measure::start();
    kernel.enq().unwrap();
    measure_opencl_compute_time.stop();

    println!( "{}", measure_opencl_compute_time.get_message() );
    

    // Download the results
    let mut results_vector = vec![0i32; item_count];
    result_buffer.read(&mut results_vector).enq().unwrap();

    TestResult{success: true, total_time: measure_opencl_compute_time.duration() + measure_kernel_create.duration() + measure_buffering.duration()}

}

fn cpu_test( item_count: usize ) -> TestResult {

    print!("Reserving memory...");

    let mut measure_ram = Measure::start();

    let rust_source = vec![0; item_count];
    let mut rust_results = vec![0; item_count];
    measure_ram.stop();

    println!( "{}", measure_ram.get_message() );

    print!("Computing...");
    let mut measure_cpu_compute = Measure::start();
    for i in 0..item_count {
        rust_results[i] = rust_source[i] * COEFF;
    }
    measure_cpu_compute.stop();

    println!( "{}", measure_cpu_compute.get_message() );

    TestResult{success: true, total_time: measure_ram.duration() + measure_cpu_compute.duration()}

}


#[test]
fn kmp_is_working() {
    assert!( kmp_match_cpu(String::from("abc"), String::from("abc") ) );
    assert!( kmp_match_cpu(String::from("abcd"), String::from("abc") ) );
    assert!( kmp_match_cpu(String::from("abcd"), String::from("bcd") ) );
    assert!( !kmp_match_cpu(String::from("abc"), String::from("cba") ) );
    assert!( kmp_match_cpu(String::from("jossain kissikadulla käveli kisa kisassa joku outo kissa voi koira"), String::from("kissa")) );
    assert!( !kmp_match_cpu(String::from("koirien etsiminen koirista kis on k-vaikeaa"), String::from("kissa")) );
    assert!( kmp_match_cpu(String::from("ABC ABCDAB ABCDABCDABDE"), String::from("ABCDABD")) );
}

#[test]
fn naive_string_matching_is_working() {
    assert!( naive_string_matching_cpu(String::from("abc"), String::from("abc")) );
    assert!( naive_string_matching_cpu(String::from("abcd"), String::from("abc")) );
    assert!( naive_string_matching_cpu(String::from("abcd"), String::from("bcd")) );
    assert!( !naive_string_matching_cpu(String::from("abc"), String::from("cba")) );
}

