use log::{debug, info};
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug, Default)]
pub struct Pipeline {
    bins: HashMap<String, Bin>,
}

pub type Bin = String;

impl Pipeline {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn run(self: &Self) -> Result<(), Box<dyn Error>> {
        info!("Pipeline started, running {} bins", self.bins.len());
        self.bins.iter().for_each(|(k, _)| debug!("Running {}", k));
        Ok(())
    }
}
