use std::collections::BTreeMap;
use std::fmt::Write;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use walkdir::WalkDir;

const WASM_BINARIES_DIR: &str = "input";
const RESULTS_DIR: &str = "output";

fn get_code(path: impl AsRef<Path>) -> Vec<u8> {
    std::fs::read(path).expect("failed to load WASM binary")
}

fn write_file(file_name: &str, content: &str) {
    let path = Path::new(RESULTS_DIR).join(file_name);
    std::fs::write(path, content).expect("failed to write result file");
}

fn print_result(
    success: &mut dyn Write,
    failure: &mut dyn Write,
    is_ok: bool,
    path: impl AsRef<Path>,
    size: usize,
    duration: &Duration,
    error: Option<String>,
) {
    let file = path.as_ref().file_name().unwrap().to_string_lossy();
    if is_ok {
        println!("{:>12} {:>20} {:>20}", file, size, duration.as_nanos());
        writeln!(
            success,
            "{:>12} {:>20} {:>20}",
            file,
            size,
            duration.as_nanos()
        )
        .unwrap();
    } else {
        eprintln!(
            "{:>12} {:>20} {}",
            file,
            size,
            error.clone().unwrap_or("compilation error".to_string())
        );
        writeln!(
            failure,
            "{:>12} {:>20} {}",
            file,
            size,
            error.unwrap_or("compilation error".to_string())
        )
        .unwrap();
    }
}

fn use_wasmer<T>(profile: &str, files: T, singlepass: bool, speed: bool)
where
    T: Iterator<Item = PathBuf>,
{
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

    let mut success = String::new();
    let mut failure = String::new();
    for file in files {
        let code = get_code(&file);

        let start = Instant::now();
        let result = wasmer::Module::new(&store, &code);
        let duration = start.elapsed();

        print_result(
            &mut success,
            &mut failure,
            result.is_ok(),
            file,
            code.len(),
            &duration,
            result.err().map(|e| e.to_string()),
        );
    }
    write_file(&format!("{}.txt", profile), &success);
    write_file(&format!("{}-err.txt", profile), &failure);
}

fn use_wasmtime<T>(profile: &str, files: T, singlepass: bool, speed: bool)
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

    let mut success = String::new();
    let mut failure = String::new();
    for file in files {
        // Load the binary to memory.
        let code = get_code(&file);

        let start = Instant::now();
        let result = engine.precompile_module(&code);
        let duration = start.elapsed();

        print_result(
            &mut success,
            &mut failure,
            result.is_ok(),
            file,
            code.len(),
            &duration,
            result.err().map(|e| e.to_string()),
        );
    }
    write_file(&format!("{}.txt", profile), &success);
    write_file(&format!("{}-err.txt", profile), &failure);
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
        p @ "wasmer-cranelift-none" => use_wasmer(p, files.values().cloned(), false, false),
        p @ "wasmer-cranelift-speed" => use_wasmer(p, files.values().cloned(), false, true),
        p @ "wasmer-singlepass" => use_wasmer(p, files.values().cloned(), true, false),
        p @ "wasmtime-cranelift-none" => use_wasmtime(p, files.values().cloned(), false, false),
        p @ "wasmtime-cranelift-speed" => use_wasmtime(p, files.values().cloned(), false, true),
        p @ "wasmtime-singlepass" => use_wasmtime(p, files.values().cloned(), true, false),
        _ => eprintln!("error: invalid argument"),
    }
}
