use anyhow::Result;
use nix::time::{ClockId, clock_gettime};

const STEPS_N: usize = 16 * 1024 * 1024;
const SCRATCH_N: usize = 64 * STEPS_N;

pub fn measure_l1d() -> Result<usize, ()> {
    let mut buffer = vec![251u8; SCRATCH_N];
    let mut counter = 4;
    let mut deduced_cache_size = counter;
    let mut previous_delta = 0;
    let mut previous_diff = 1_000_000;

    loop {
        if counter > 256 {
            break;
        }

        let stride = counter;
        let starttime = clock_gettime(ClockId::CLOCK_REALTIME).expect("realtime clock value not found");
        let start_ns = (starttime.tv_sec() * 1_000_000_000 + starttime.tv_nsec()) / 1000;
        let mut pos = 0;

        loop {
            buffer[(pos * (stride * 2)) & (STEPS_N - 1)] = buffer[(pos * (stride * 2)) & (STEPS_N - 1)].wrapping_add(1);

            pos += 1;
            if pos > STEPS_N {
                break;
            }
        }

        let endtime = clock_gettime(ClockId::CLOCK_REALTIME).expect("realtime clock value not found");
        let end_ns = (endtime.tv_sec() * 1_000_000_000 + endtime.tv_nsec()) / 1000;
        let delta = end_ns - start_ns;

        if previous_delta > 0 {
            let diff = delta - previous_delta;
            if diff < previous_diff {
                deduced_cache_size = stride;
            }
            previous_diff = diff;
        }

        counter <<= 1;
        previous_delta = delta;
    }

    Ok(deduced_cache_size / 2)
}
