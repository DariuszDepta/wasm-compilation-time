# Measurement of the compilation time

 
- Wasmer 7.0.1
- Wasmtime 42.0.1

| Compile time | Wasmer Singlepass | Wasmer Cranelift | Wasmtime Winch | Wasmtime Cranelift |
|--------------|------------------:|-----------------:|---------------:|-------------------:|
| avg          |        0.181 [ms] |       1.764 [ms] |     0.181 [ms] |         1.764 [ms] |
| min          |        0.007 [ms] |       0.049 [ms] |     0.007 [ms] |         0.049 [ms] |
| max          |        1.219 [ms] |      11.913 [ms] |     1.219 [ms] |        11.913 [ms] |
                    
> [!CAUTION]
> Version 7.0.1 of the Wasmer Singlepass compiler also reports error when compiling
> smart contracts on macOS, exactly like currently used version 5.0.6, e.g:  
> 
> ```
> Error compiling Wasm: Could not compile: Compilation error:
> Assembler failed finalization with: ImpossibleRelocation(Dynamic(DynamicLabel(0)))
> ```
