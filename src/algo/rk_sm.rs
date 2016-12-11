extern crate num;

use algo::rk_sm::num::Integer;

// TODO: move to another file
pub fn match_cpu(text: &[u8], pattern: &[u8]) -> usize {
    let d: i16 = 4;
    let q: i16 = 13; // d*q should be less than word length (64?) and a prime
    let n = text.len();
    let m = pattern.len();
    let h: i16 = d.pow((m-1) as u32).mod_floor( &q );
    let mut p = 0;
    let mut t = 0;

    // Preprocessing
    for i in 1..m+1 {
        p = (d*p + pattern[i - 1] as i16).mod_floor( &q );
        t = (d*t + text[i - 1] as i16).mod_floor( &q );
    }

    let mut match_count = 0;

    // Matching
    for s in 0..n-m+1 {
        if p == t {
            if pattern[0..m] == text[s..s+m] {
                match_count += 1;
            }
        }
        if s < n-m {
            t = ((d * (t - (text[s] as i16 * h)) + text[s + m] as i16)).mod_floor( &(q) );
        }

    }

    match_count
}

/*
fn create_kernel(text: &[u8], pattern: &[u8]) -> (Kernel, Buffer<i32>) {

    let n = text.len();
    let m = pattern.len();

    let kernel_string = super::super::read_file("kernel/naive.c");

    let ocl_pq = ProQue::builder()
        .src( kernel_string )
        .dims([n])
        .build().expect("Build ProQue");

    // Convert input data into on-device buffers
    let text_buffer = Buffer::new(&ocl_pq.queue().clone(), Some(core::MEM_READ_ONLY |
        core::MEM_COPY_HOST_PTR), [n], Some(text)).unwrap();
    let pattern_buffer = Buffer::new(&ocl_pq.queue().clone(), Some(core::MEM_READ_ONLY |
        core::MEM_COPY_HOST_PTR), [m], Some(pattern)).unwrap();


    // Create an empty buffer for results
    let result_buffer: Buffer<i32> = ocl_pq.create_buffer().unwrap();


    let kernel = ocl_pq.create_kernel("naive_sm").unwrap()
        .arg_scl(m as i32)
        .arg_buf(&text_buffer)
        .arg_buf(&pattern_buffer)
        .arg_buf(&result_buffer);

    (kernel, result_buffer)

}

pub fn rk_match_gpu(text: &[u8], pattern: &[u8]) -> usize {

    let (kernel, result_buffer) = create_naive_kernel(text, pattern);

    // Execute the string matching
    kernel.enq().unwrap();

    // Download the results
    let mut results_vector = vec![0i32; text.len()];
    result_buffer.read(&mut results_vector).enq().unwrap();

    results_vector.dedup();
    let match_count = results_vector.len()-1;

    return match_count;

}
*/
