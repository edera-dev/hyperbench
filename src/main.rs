pub mod l1d;
pub mod pipecopy;
pub mod shacrypt;
pub mod sieve;
pub mod socketpaircopy;

use anyhow::Result;
use self::l1d::measure_l1d;
use self::pipecopy::PipeCopyTest;
use self::shacrypt::ShaCryptVerifyTest;
use self::sieve::SieveTest;
use self::socketpaircopy::SocketPairCopyTest;

fn banner() {
    println!("HyperBench version {}.  Copyright 2024 Edera Inc.\n", env!("CARGO_PKG_VERSION"));
}

fn cacheline_size() -> Result<usize, ()> {
    let mut l1d = 0;

    // We measure L1d timings three times to ensure the L1d cache is warm.
    for _ in 0..2 {
        let current_l1d = match measure_l1d() {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        if current_l1d > l1d {
            l1d = current_l1d;
        }
    }

    Ok(l1d)
}

#[derive(Copy, Clone, Debug)]
pub struct TestParameters {
    pub l1d: usize,
}

pub trait Test {
    fn name(&self) -> String;
    fn run(&self, paras: TestParameters) -> Result<f32, ()>;
}

fn main() {
    banner();

    println!("Warming cache and deducing optimal cache-line size, please wait.");

    let l1d = cacheline_size().expect("unable to determine cacheline size");

    println!("Apparent L1d cache-line size: {} bytes\n", l1d);

    let testparas = TestParameters { l1d };
    let tests: Vec<Box<dyn Test>> = vec![
        Box::new(ShaCryptVerifyTest {}),
        Box::new(SieveTest {}),
        Box::new(PipeCopyTest {}),
        Box::new(SocketPairCopyTest {}),
    ];
    let mut totalscore: f32 = 0.0;
    let mut runtests = 0;

    for test in tests {
        print!("Test '{}': ", test.name());
        let result = test.run(testparas).expect("failed to run test");
        println!("{}", result);
        totalscore += result;
        runtests += 1;
    }

    println!("\nFinal score (lower is better): {}", totalscore / (runtests as f32));
}
