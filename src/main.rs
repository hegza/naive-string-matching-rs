mod algo;

extern crate ocl;
extern crate time;


const DATA_SET_POWER: usize = 18;
const DATA_SET_SIZE: usize = 2 << DATA_SET_POWER;


fn main() {

    println!("# of work items: {:?} (2^{:?})", DATA_SET_SIZE, DATA_SET_POWER);

    println!("{}", "Initiating OpenCL test");
    
    algo::opencl_test(DATA_SET_SIZE);
    
    println!("");

    // TODO: use test::blackbox to avoid compiler optimizations
   
    println!("Initiating CPU test");
   
    algo::cpu_test(DATA_SET_SIZE);

}

