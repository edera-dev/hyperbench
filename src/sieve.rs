use anyhow::Result;
use crate::{Test, TestParameters};
use nix::time::{ClockId, clock_gettime};

pub struct SieveTest {}

fn sieve(n: usize) {
    let mut sieve_array = vec![true; n+1];

    sieve_array[0] = false;
    sieve_array[1] = false;

    for i in (4..n+1).step_by(2) {
        sieve_array[i] = false;
    }

    let mut i = 3;
    while i*i <= n+1 {
        if sieve_array[i] {
            for j in (i*i..n+1).step_by(2*i) {
                sieve_array[j] = false;
            }
        }

        i += 2;
    }
}

impl Test for SieveTest {
    fn name(&self) -> String {
        "Integer math (prime solving up to 10000000, 20 rounds)".to_string()
    }

    fn run(&self, paras: TestParameters) -> Result<f32, ()> {
        let starttime = clock_gettime(ClockId::CLOCK_REALTIME).expect("realtime clock value not found");
        let start_ns = (starttime.tv_sec() * 1000000000 + starttime.tv_nsec()) / 1000;

        for _ in 0..20 {
            sieve(10000000);
        }

        let endtime = clock_gettime(ClockId::CLOCK_REALTIME).expect("realtime clock value not found");
        let end_ns = (endtime.tv_sec() * 1000000000 + endtime.tv_nsec()) / 1000;

        let result = (end_ns - start_ns) as f32;

        Ok(result)
    }
}

