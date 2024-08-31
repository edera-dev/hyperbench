use anyhow::Result;
use crate::{Test, TestParameters};
use nix::time::{ClockId, clock_gettime};
use std::io::{Read, Write};
use os_pipe;

pub struct PipeCopyTest {}

impl Test for PipeCopyTest {
    fn name(&self) -> String {
        "Pipe copy (1GiB)".to_string()
    }

    fn run(&self, _paras: TestParameters) -> Result<f32, ()> {
        let (mut reader, mut writer) = os_pipe::pipe().expect("failed to open pipe");
        let mut buf = vec![0u8; 1024];

        let starttime = clock_gettime(ClockId::CLOCK_REALTIME).expect("realtime clock value not found");
        let start_ns = (starttime.tv_sec() * 1_000_000_000 + starttime.tv_nsec()) / 1000;

        for _ in 0..1_024_768 {
            writer.write_all(&buf).expect("failed to write to pipe");
            reader.read_exact(&mut buf).expect("failed to read from pipe");
        }

        let endtime = clock_gettime(ClockId::CLOCK_REALTIME).expect("realtime clock value not found");
        let end_ns = (endtime.tv_sec() * 1_000_000_000 + endtime.tv_nsec()) / 1000;

        let result = (end_ns - start_ns) as f32;

        Ok(result)
    }
}

