use log::{debug, info};
use std::collections::HashMap;
use std::error::Error;
use std::{fs, io};
use wasmtime::{Engine, HostRef, Instance, Module, Store};
use wasmtime_wasi::create_wasi_instance;

#[derive(Debug, Default)]
pub struct Pipeline {
    bins: HashMap<String, Bin>,
}

pub type Bin = Vec<u8>;

impl Pipeline {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add(self: &mut Self, bin: &str) -> io::Result<&mut Self> {
        let wasm = fs::read(format!("{}.wasm", bin))?;
        self.bins.insert(bin.to_owned(), wasm);
        Ok(self)
    }

    pub fn run(self: &Self) -> Result<(), Box<dyn Error>> {
        if self.bins.len() == 0 {
            debug!("Nothing to run, bye!");
            return Ok(());
        }
        let engine = HostRef::new(Engine::default());
        let store = HostRef::new(Store::new(&engine));
        info!("Pipeline started, running {} bins", self.bins.len());
        let wasi = create_wasi_instance(&store, &[], &[], &[])?;

        for (name, wasm) in self.bins.iter() {
            let module = HostRef::new(Module::new(&store, &wasm)?);
            let imports = module
                .borrow()
                .imports()
                .iter()
                .map(|i| {
                    let field_name = i.name().as_str();
                    if let Some(export) = wasi.find_export_by_name(field_name) {
                        Ok(export.clone())
                    } else {
                        Err("Import error".into())
                    }
                })
                .collect::<Result<Vec<_>, String>>()?;
            debug!("Running {}", name);
            let _instance = HostRef::new(Instance::new(&store, &module, &imports)?);
        }

        info!("Pipeline completed!");
        Ok(())
    }
}
