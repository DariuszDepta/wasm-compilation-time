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

fn print_result(success: &mut dyn Write, failure: &mut dyn Write, path: impl AsRef<Path>, size: usize, duration: &Duration, error: Option<String>) {
  let file = path.as_ref().file_name().unwrap().to_string_lossy();
  if error.is_none() {
    println!("{:>12} {:>20} {:>20}", file, size, duration.as_nanos());
    writeln!(success, "{:>12} {:>20} {:>20}", file, size, duration.as_nanos()).unwrap();
  } else {
    eprintln!("{:>12} {:>20} {}", file, size, error.clone().unwrap_or("compilation error".to_string()));
    writeln!(failure, "{:>12} {:>20} {}", file, size, error.unwrap_or("compilation error".to_string())).unwrap();
  }
}

fn measure_time<T, F>(profile: &str, files: T, fun: F)
where
  T: Iterator<Item = PathBuf>,
  F: Fn(&[u8]) -> Option<String>,
{
  let mut success = String::new();
  let mut failure = String::new();
  for file in files {
    let code = get_code(&file);

    let start = Instant::now();
    let result = fun(&code);
    let duration = start.elapsed();

    print_result(&mut success, &mut failure, file, code.len(), &duration, result);
  }
  write_file(&format!("{}.txt", profile), &success);
  write_file(&format!("{}-err.txt", profile), &failure);
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

  measure_time(profile, files, |code| wasmer::Module::new(&store, code).err().map(|e| e.to_string()));
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

  measure_time(profile, files, |code| engine.precompile_module(code).err().map(|e| e.to_string()));
}

fn use_cosmwasm<T>(profile: &str, files: T)
where
  T: Iterator<Item = PathBuf>,
{
  const DEFAULT_MEMORY_LIMIT: Option<cosmwasm_vm::Size> = Some(cosmwasm_vm::Size::mebi(16));

  measure_time(profile, files, |code| {
    let engine = cosmwasm_vm::internals::make_compiling_engine(DEFAULT_MEMORY_LIMIT);
    cosmwasm_vm::internals::compile(&engine, code).err().map(|e| e.to_string())
  });
}

fn main() {
  let args = std::env::args().skip(1).collect::<Vec<String>>();

  let mut files = BTreeMap::new();
  let root_dir = Path::new(WASM_BINARIES_DIR).canonicalize().unwrap();
  for entry in WalkDir::new(root_dir).max_depth(1) {
    let entry = entry.unwrap();
    if entry.file_type().is_file() {
      let path = entry.path().to_path_buf();
      let key = path.file_name().unwrap().to_string_lossy().to_string();
      files.insert(key, path);
    }
  }

  if args.len() == 1 {
    match args[0].as_str() {
      p @ "wasmer-cranelift-none" => use_wasmer(p, files.values().cloned(), false, false),
      p @ "wasmer-cranelift-speed" => use_wasmer(p, files.values().cloned(), false, true),
      p @ "wasmer-singlepass" => use_wasmer(p, files.values().cloned(), true, false),
      p @ "wasmtime-cranelift-none" => use_wasmtime(p, files.values().cloned(), false, false),
      p @ "wasmtime-cranelift-speed" => use_wasmtime(p, files.values().cloned(), false, true),
      p @ "wasmtime-singlepass" => use_wasmtime(p, files.values().cloned(), true, false),
      p @ "cosmwasm-singlepass" => use_cosmwasm(p, files.values().cloned()),
      "all" => {
        use_wasmer("wasmer-cranelift-none", files.values().cloned(), false, false);
        use_wasmer("wasmer-cranelift-speed", files.values().cloned(), false, true);
        use_wasmer("wasmer-singlepass", files.values().cloned(), true, false);
        use_wasmtime("wasmtime-cranelift-none", files.values().cloned(), false, false);
        use_wasmtime("wasmtime-cranelift-speed", files.values().cloned(), false, true);
        use_wasmtime("wasmtime-singlepass", files.values().cloned(), true, false);
        use_cosmwasm("cosmwasm-singlepass", files.values().cloned());
      }
      _ => eprintln!("error: invalid argument"),
    }
  } else {
    eprintln!("error: no arguments provided");
  }
}
