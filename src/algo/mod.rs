#![allow(dead_code)]

extern crate time;

pub mod measure;
pub mod naive_sm;
pub mod rk_sm;
pub mod kmp;

use self::measure::Measure;
use time::Duration;
use std;
use std::boxed::Box;

impl Measure {

    pub fn get_message( &self ) -> String {
        format!(" took {:?} micros", self.duration().num_microseconds().unwrap() )
    }

}

pub struct StringMatchSuite<'a> {
    pub suite_name: String,
    pub repeat_count: usize,
    pub text: &'a [u8],
    pub patterns: Box<Vec<&'a [u8]>>,
    pub test_func: &'a Fn(&[u8], &[u8]) -> usize,
}

impl<'a> StringMatchSuite<'a> {
    pub fn create_many<F>(suite_name: &str, test_func: &'a F, text: &'a [u8], patterns: Box<Vec<&'a[u8]>>, repeat_count: usize) -> StringMatchSuite<'a>
            where F: Fn(&[u8], &[u8]) -> usize {
        StringMatchSuite {suite_name: String::from(suite_name), repeat_count: repeat_count, text: text, patterns: patterns, test_func: test_func}
    }
    pub fn create<F>(suite_name: &str, test_func: &'a F, text: &'a [u8], pattern: &'a [u8]) -> StringMatchSuite<'a>
            where F: Fn(&[u8], &[u8]) -> usize {
        StringMatchSuite {suite_name: String::from(suite_name), repeat_count: 1, text: text, patterns: Box::new(vec!(pattern)), test_func: test_func}
    }

    pub fn description(&self) -> String {
        format!("Compute {} (x{:?})... ", self.suite_name, self.repeat_count)
    }

    pub fn execute(&self) -> SuiteResult {
        let (total_duration, match_counts) = self.repeat_testcase();
        let average_run = total_duration / self.repeat_count as i32;

        SuiteResult { average_run: average_run, match_counts: match_counts }
    }

    fn repeat_testcase(&self) -> (time::Duration, Box<Vec<usize>>) {
        let mut match_counts = vec![0; self.repeat_count];
        let mut nanos: std::vec::Vec<u32> = vec![0; self.repeat_count];
        for i in 0..self.repeat_count {
            let pattern = self.patterns[i];
            let mut measure_cpu = Measure::start();
            // !!!<Mission Critical>!!!
            match_counts[i] = (self.test_func)(self.text, pattern);
            // !!!</Mission Critical>!!!
            measure_cpu.stop();

            nanos[i] = measure_cpu.duration().num_nanoseconds().unwrap() as u32;
        }

        // Coalesce
        let mut total_duration = Duration::nanoseconds(0);
        for i in 0..self.repeat_count {
            total_duration = total_duration + Duration::nanoseconds(nanos[i] as i64);
        }

        (total_duration, Box::new(match_counts))
    }
}

#[derive(Debug)]
pub struct SuiteResult {
    pub average_run: time::Duration,
    pub match_counts: Box<Vec<usize>>,
}

impl SuiteResult {
    pub fn empty() -> SuiteResult {
        SuiteResult { average_run: time::Duration::nanoseconds(0), match_counts: Box::new(vec!()) }
    }

    pub fn elaborate(&self) -> String {
        format!("done (averaging {:?} microseconds)", self.average_run.num_microseconds().unwrap())
    }
}
