use std::error::Error;
use stderrlog;

fn main() -> Result<(), Box<dyn Error>> {
    stderrlog::new()
        .module(module_path!())
        .verbosity(3)
        .init()?;
    let pipeline = wapr::Pipeline::new();
    pipeline.run()
}
