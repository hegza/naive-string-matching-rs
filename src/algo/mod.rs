#![allow(dead_code)]

pub mod measure;
mod naive_sm;
mod rk;
mod kmp;

pub use self::naive_sm::*;
pub use self::rk::*;
pub use self::kmp::*;

use self::measure::Measure;
use time::Duration;

pub struct TestResult {
    pub success: bool,
    pub total_time: Duration,
}

impl Measure {

    pub fn get_message( &self ) -> String {
        format!(" took {:?} micros", self.duration().num_microseconds().unwrap() )
    }

}

