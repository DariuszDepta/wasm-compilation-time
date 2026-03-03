use cosmwasm_vm::internals::{compile, make_compiling_engine};
use cosmwasm_vm::Size;
use std::path::Path;
use std::time::Instant;
use walkdir::WalkDir;

const WASM_BINARIES_DIR: &str = "neutron_wasm_codes";

const DEFAULT_MEMORY_LIMIT: Option<Size> = Some(Size::mebi(16));

fn main() {
    let mut files = vec![];
    let root_dir = Path::new(WASM_BINARIES_DIR).canonicalize().unwrap();
    for entry in WalkDir::new(root_dir).max_depth(1) {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            files.push(entry.path().to_path_buf());
        }
    }
    files.sort();

    for file in files {
        // Load the binary to memory.
        let code = std::fs::read(&file).expect("failed to load WASM file");
        let engine = make_compiling_engine(DEFAULT_MEMORY_LIMIT);

        let start = Instant::now();
        let is_ok = compile(&engine, &code).is_ok();
        let duration = start.elapsed();

        if is_ok {
            println!(
                "{:>12} {:>20} {:>20}",
                file.file_name().unwrap().to_string_lossy(),
                code.len(),
                duration.as_nanos()
            );
        }
    }
}
