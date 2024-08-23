use anyhow::Result;
use crate::{Test, TestParameters};
use nix::time::{ClockId, clock_gettime};
use sha_crypt::sha512_check;

pub struct ShaCryptVerifyTest {}

const ANSWER: &str = "hyperbench testing string";
const VERIFY_HASH: &str = "$6$azxnqvtzf$6MJezWlizwqxFCgKah/5W8WVia4sa2OvRPc.STTJjBOABkry2qCUYnp3GLAZMpm2Okyhykoq6mxz8sGe4iTJI1";

impl Test for ShaCryptVerifyTest {
    fn name(&self) -> String {
        "SHA512-crypt password verification".to_string()
    }

    fn run(&self, paras: TestParameters) -> Result<f32, ()> {
        let starttime = clock_gettime(ClockId::CLOCK_REALTIME).expect("realtime clock value not found");
        let start_ns = (starttime.tv_sec() * 1000000000 + starttime.tv_nsec()) / 1000;

        for _ in 0..1000 {
            let _ = sha512_check(ANSWER, VERIFY_HASH);
        }

        let endtime = clock_gettime(ClockId::CLOCK_REALTIME).expect("realtime clock value not found");
        let end_ns = (endtime.tv_sec() * 1000000000 + endtime.tv_nsec()) / 1000;

        let result = (end_ns - start_ns) as f32;

        Ok(result)
    }
}

