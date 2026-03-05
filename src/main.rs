use std::collections::BTreeMap;
use std::fmt::Write;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use walkdir::WalkDir;

const WASM_BINARIES_DIR: &str = "input";
const RESULTS_DIR: &str = "output";

pub fn get_code(path: impl AsRef<Path>) -> Vec<u8> {
  std::fs::read(path).expect("failed to load WASM binary")
}

pub fn write_file(file_name: &str, content: &str) {
  let path = Path::new(RESULTS_DIR).join(file_name);
  std::fs::write(path, content).expect("failed to write result file");
}

fn print_result(success: &mut dyn Write, failure: &mut dyn Write, max_key_len: usize, path: impl AsRef<Path>, size: usize, duration: &Duration, error: Option<String>) {
  let file = path.as_ref().file_name().unwrap().to_string_lossy();
  if error.is_none() {
    println!("{:>max_key_len$} {:>20} {:>20}", file, size, duration.as_nanos());
    writeln!(success, "{:>max_key_len$} {:>20} {:>20}", file, size, duration.as_nanos()).unwrap();
  } else {
    eprintln!("{:>max_key_len$} {:>20} {}", file, size, error.clone().unwrap_or("compilation error".to_string()));
    writeln!(failure, "{:>max_key_len$} {:>20} {}", file, size, error.unwrap_or("compilation error".to_string())).unwrap();
  }
}

pub fn measure_time<T, F>(profile: &str, max_key_len: usize, files: T, fun: F)
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

    print_result(&mut success, &mut failure, max_key_len, file, code.len(), &duration, result);
  }
  write_file(&format!("{}.txt", profile), &success);
  write_file(&format!("{}-err.txt", profile), &failure);
}

#[cfg(feature = "wasmer")]
fn use_wasmer<T>(profile: &str, max_key_len: usize, files: T, singlepass: bool, speed: bool)
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

  measure_time(profile, max_key_len, files, |code| wasmer::Module::new(&store, code).err().map(|e| e.to_string()));
}

#[cfg(feature = "wasmtime")]
fn use_wasmtime<T>(profile: &str, max_key_len: usize, files: T, singlepass: bool, speed: bool)
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

  measure_time(profile, max_key_len, files, |code| engine.precompile_module(code).err().map(|e| e.to_string()));
}

#[cfg(feature = "cosmwasm")]
fn use_cosmwasm<T>(profile: &str, max_key_len: usize, files: T)
where
  T: Iterator<Item = PathBuf>,
{
  const DEFAULT_MEMORY_LIMIT: Option<cosmwasm_vm::Size> = Some(cosmwasm_vm::Size::mebi(16));

  measure_time(profile, max_key_len, files, |code| {
    let engine = cosmwasm_vm::internals::make_compiling_engine(DEFAULT_MEMORY_LIMIT);
    cosmwasm_vm::internals::compile(&engine, code).err().map(|e| e.to_string())
  });
}

fn main() {
  let args = std::env::args().skip(1).collect::<Vec<String>>();

  let mut files = BTreeMap::new();
  let root_dir = Path::new(WASM_BINARIES_DIR).canonicalize().unwrap();
  let mut max_key_len = 0;
  for entry in WalkDir::new(root_dir).max_depth(1) {
    let entry = entry.unwrap();
    if entry.file_type().is_file() {
      let path = entry.path().to_path_buf();
      let key = path.file_name().unwrap().to_string_lossy().to_string();
      if key.len() > max_key_len {
        max_key_len = key.len();
      }
      files.insert(key, path);
    }
  }

  if args.len() == 1 {
    match args[0].as_str() {
      #[cfg(feature = "wasmer")]
      p @ "wasmer-cranelift-none" => use_wasmer(p, max_key_len, files.values().cloned(), false, false),
      #[cfg(feature = "wasmer")]
      p @ "wasmer-cranelift-speed" => use_wasmer(p, max_key_len, files.values().cloned(), false, true),
      #[cfg(feature = "wasmer")]
      p @ "wasmer-singlepass" => use_wasmer(p, max_key_len, files.values().cloned(), true, false),
      #[cfg(feature = "wasmtime")]
      p @ "wasmtime-cranelift-none" => use_wasmtime(p, max_key_len, files.values().cloned(), false, false),
      #[cfg(feature = "wasmtime")]
      p @ "wasmtime-cranelift-speed" => use_wasmtime(p, max_key_len, files.values().cloned(), false, true),
      #[cfg(feature = "wasmtime")]
      p @ "wasmtime-singlepass" => use_wasmtime(p, max_key_len, files.values().cloned(), true, false),
      #[cfg(feature = "cosmwasm")]
      p @ "cosmwasm-singlepass" => use_cosmwasm(p, max_key_len, files.values().cloned()),
      _ => eprintln!("error: invalid argument"),
    }
  } else {
    eprintln!("error: no arguments provided");
  }
}
