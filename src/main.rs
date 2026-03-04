use cosmwasm_vm::internals::{compile, make_compiling_engine};
use cosmwasm_vm::Size;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::time::Instant;
use walkdir::WalkDir;

const WASM_BINARIES_DIR: &str = "neutron_wasm_codes";

const DEFAULT_MEMORY_LIMIT: Option<Size> = Some(Size::mebi(16));

fn get_code(path: impl AsRef<Path>) -> Vec<u8> {
    std::fs::read(path).expect("failed to load WASM file")
}

fn compile_using_cosmwasm<T>(files: T)
where
    T: Iterator<Item = PathBuf>,
{
    for file in files {
        let code = get_code(&file);
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

fn compile_using_winch<T>(files: T)
where
    T: Iterator<Item = PathBuf>,
{
    let mut config = wasmtime::Config::new();
    config.strategy(wasmtime::Strategy::Winch);
    let engine =
        wasmtime::Engine::new(&config).expect("failed to instantiate the compilation engine");

    for file in files {
        // Load the binary to memory.
        let code = get_code(&file);

        let start = Instant::now();
        let result = engine.precompile_module(&code);
        let duration = start.elapsed();

        if result.is_ok() {
            println!(
                "{:>12} {:>20} {:>20}",
                file.file_name().unwrap().to_string_lossy(),
                code.len(),
                duration.as_nanos()
            );
        }
    }
}

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<String>>();

    let mut files = BTreeMap::new();
    let root_dir = Path::new(WASM_BINARIES_DIR).canonicalize().unwrap();
    for entry in WalkDir::new(root_dir).max_depth(1) {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            let path = entry.path().to_path_buf();
            let key = path
                .file_name()
                .unwrap()
                .to_string_lossy()
                .trim_end_matches(".wasm")
                .parse::<usize>()
                .unwrap();
            files.insert(key, path);
        }
    }

    match args[0].as_str() {
        "cosmwasm-vm" => compile_using_cosmwasm(files.values().cloned()),
        "winch" => compile_using_winch(files.values().cloned()),
        _ => println!("error: invalid argument\n  [possible values: cosmwasm, winch]"),
    }
}
