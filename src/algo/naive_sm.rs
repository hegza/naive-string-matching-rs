use ocl::{core, ProQue, Buffer, Kernel};
use std::i32;

pub fn match_cpu( text: &[u8], pattern: &[u8] ) -> usize {

    // Note: both the book and rust use inclusive lower bound but the book uses inclusive upper
    // bound while rust uses exclusive upper bound for for loops
    let n = text.len();
    let m = pattern.len();

    // Number of instances of pattern in text
    let mut count = 0;
    for s in 0..n-m+1 {
        // Slice from the given position
        let shifted = &text[s..s+m];
        if pattern == shifted {
            // Pattern occurs with shift: shifted
            count += 1;
        }
    }

    return count;

}

pub fn create_kernel_and_results_buffer(text: &[u8], pattern: &[u8]) -> (Kernel, Buffer<i32>) {

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

pub fn match_gpu(text: &[u8], pattern: &[u8], kernel_buffer: Option<(Kernel, Buffer<i32>)>) -> usize {

    let (kernel, result_buffer) = kernel_buffer.unwrap_or(create_kernel_and_results_buffer(text, pattern));

    // Execute the string matching
    kernel.enq().unwrap();

    // Download the results
    let mut results_vector = vec![0i32; text.len()];
    result_buffer.read(&mut results_vector).enq().unwrap();

    let mut match_count = 0;
    for r in results_vector {
        if r == 0 {
            match_count += 1;
        }
    }
    
    return match_count;

}

pub fn match_gpu_full(text: &[u8], pattern: &[u8]) -> usize {
    return match_gpu(text, pattern, None);
}
