use anyhow::Result;
use crate::{Test, TestParameters};

pub struct SieveTest {}

impl Test for SieveTest {
    fn name(&self) -> String {
        "Prime evaluation up to 1000000 items".to_string()
    }

    fn run(&self, paras: TestParameters) -> Result<f32, ()> {
        Ok(0.0)
    }
}

