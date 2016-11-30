#![allow(dead_code)]

extern crate time;

pub mod measure;
mod naive_sm;
mod rk;
mod kmp;

pub use self::naive_sm::*;
pub use self::rk::*;
pub use self::kmp::*;

use self::measure::Measure;
use time::Duration;
use std;

impl Measure {

    pub fn get_message( &self ) -> String {
        format!(" took {:?} micros", self.duration().num_microseconds().unwrap() )
    }

}

pub struct StringMatchSuite<'a> {
    pub suite_name: String,
    pub repeat_count: usize,
    pub text: &'a [u8],
    pub pattern: &'a [u8],
    pub test_func: &'a Fn(&[u8], &[u8]) -> usize,
}

impl<'a> StringMatchSuite<'a> {
    pub fn create<F>(suite_name: &str, test_func: &'a F, text: &'a [u8], pattern: &'a [u8], repeat_count: usize) -> StringMatchSuite<'a>
            where F: Fn(&[u8], &[u8]) -> usize {
        StringMatchSuite {suite_name: String::from(suite_name), repeat_count: repeat_count, text: text, pattern: pattern, test_func: test_func}
    }

    pub fn description(&self) -> String {
        format!("Compute {} (x{:?})... ", self.suite_name, self.repeat_count)
    }

    pub fn execute(&self) -> SuiteResult {
        let total_duration = self.repeat_testcase();
        let average_run = total_duration / self.repeat_count as i32;

        SuiteResult { average_run: average_run }
    }

    fn repeat_testcase(&self) -> time::Duration {
        let mut match_counts = vec![0; self.repeat_count];
        let mut nanos: std::vec::Vec<u32> = vec![0; self.repeat_count];
        for i in 0..self.repeat_count {
            let mut measure_cpu = Measure::start();
            // !!!<Mission Critical>!!!
            match_counts[i] = (self.test_func)(self.text, self.pattern);
            // !!!</Mission Critical>!!!
            measure_cpu.stop();

            nanos[i] = measure_cpu.duration().num_nanoseconds().unwrap() as u32;
        }

        // Coalesce
        let mut total_duration = Duration::nanoseconds(0);
        for i in 0..self.repeat_count {
            total_duration = total_duration + Duration::nanoseconds(nanos[i] as i64);
        }

        // Make sure that the results are all matching
        let a_match_count = match_counts[0];
        for i in 1..self.repeat_count {
            if a_match_count != match_counts[i] {
                panic!("Not all runs returned the same amount of matches");
            }
        }

        total_duration
    }
}

#[derive(Debug)]
pub struct SuiteResult {
    pub average_run: time::Duration,
}

impl SuiteResult {
    pub fn elaborate(&self) -> String {
        format!("done (averaging {:?} microseconds)", self.average_run.num_microseconds().unwrap())
    }
}
