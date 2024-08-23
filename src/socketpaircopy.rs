use anyhow::Result;
use crate::{Test, TestParameters};
use nix::time::{ClockId, clock_gettime};
use socketpair;
use std::io::{Read, Write};

pub struct SocketPairCopyTest {}

impl Test for SocketPairCopyTest {
    fn name(&self) -> String {
        "Socketpair copy (1GiB)".to_string()
    }

    fn run(&self, paras: TestParameters) -> Result<f32, ()> {
        let (mut reader, mut writer) = socketpair::socketpair_stream().expect("failed to open socketpair");
        let mut buf = vec![0u8; 1024];

        let starttime = clock_gettime(ClockId::CLOCK_REALTIME).expect("realtime clock value not found");
        let start_ns = (starttime.tv_sec() * 1000000000 + starttime.tv_nsec()) / 1000;

        for _ in 0..1024768 {
            writer.write_all(&buf).expect("failed to write to socket");
            reader.read_exact(&mut buf).expect("failed to read from socket");
        }

        let endtime = clock_gettime(ClockId::CLOCK_REALTIME).expect("realtime clock value not found");
        let end_ns = (endtime.tv_sec() * 1000000000 + endtime.tv_nsec()) / 1000;

        let result = (end_ns - start_ns) as f32;

        Ok(result)
    }
}

