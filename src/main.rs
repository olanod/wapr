use std::error::Error;
use stderrlog;
use std::env::args;

fn main() -> Result<(), Box<dyn Error>> {
    stderrlog::new()
        .module(module_path!())
        .verbosity(3)
        .init()?;
    let mut pipeline = wapr::Pipeline::new();
    for arg in args().skip(1) {
        pipeline.add(&arg)?;
    }
    pipeline.run()
}
