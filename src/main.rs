use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use walkdir::WalkDir;

const WASM_BINARIES_DIR: &str = "neutron_wasm_codes";

fn get_code(path: impl AsRef<Path>) -> Vec<u8> {
    std::fs::read(path).expect("failed to load WASM file")
}

fn print_result(
    is_ok: bool,
    path: impl AsRef<Path>,
    size: usize,
    duration: &Duration,
    error: Option<String>,
) {
    let file = path.as_ref().file_name().unwrap().to_string_lossy();
    if is_ok {
        println!("{:>12} {:>20} {:>20}", file, size, duration.as_nanos());
    } else {
        eprintln!(
            "{:>12} {:>20} {}",
            file,
            size,
            error.unwrap_or("compilation error".to_string())
        );
    }
}

fn compile_using_wasmer<T>(files: T, singlepass: bool, speed: bool)
where
    T: Iterator<Item = PathBuf>,
{
    for file in files {
        let code = get_code(&file);

        let store = if singlepass {
            let compiler = wasmer::sys::Singlepass::default();
            wasmer::Store::new(compiler)
        } else {
            let mut compiler = wasmer::sys::Cranelift::default();
            if speed {
                compiler.opt_level(wasmer::sys::CraneliftOptLevel::Speed);
            } else {
                compiler.opt_level(wasmer::sys::CraneliftOptLevel::None);
            }
            wasmer::Store::new(compiler)
        };

        let start = Instant::now();
        let result = wasmer::Module::new(&store, &code);
        let duration = start.elapsed();

        print_result(
            result.is_ok(),
            file,
            code.len(),
            &duration,
            result.err().map(|e| e.to_string()),
        );
    }
}

fn compile_using_wasmtime<T>(files: T, singlepass: bool, speed: bool)
where
    T: Iterator<Item = PathBuf>,
{
    let mut config = wasmtime::Config::new();

    if singlepass {
        config.strategy(wasmtime::Strategy::Winch);
    } else {
        config.strategy(wasmtime::Strategy::Cranelift);
        if speed {
            config.cranelift_opt_level(wasmtime::OptLevel::Speed);
        } else {
            config.cranelift_opt_level(wasmtime::OptLevel::None);
        }
    }

    config.parallel_compilation(true);

    let engine = wasmtime::Engine::new(&config).expect("failed to instantiate engine");

    for file in files {
        // Load the binary to memory.
        let code = get_code(&file);

        let start = Instant::now();
        let result = engine.precompile_module(&code);
        let duration = start.elapsed();

        print_result(
            result.is_ok(),
            file,
            code.len(),
            &duration,
            result.err().map(|e| e.to_string()),
        );
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
        "wasmer-cranelift-none" => compile_using_wasmer(files.values().cloned(), false, false),
        "wasmer-cranelift-speed" => compile_using_wasmer(files.values().cloned(), false, true),
        "wasmer-singlepass" => compile_using_wasmer(files.values().cloned(), true, false),
        "wasmtime-cranelift-none" => compile_using_wasmtime(files.values().cloned(), false, false),
        "wasmtime-cranelift-speed" => compile_using_wasmtime(files.values().cloned(), false, true),
        "wasmtime-singlepass" => compile_using_wasmtime(files.values().cloned(), true, false),
        _ => eprintln!("error: invalid argument"),
    }
}
