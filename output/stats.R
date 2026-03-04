data1 <- read.table("wasmer-cranelift-none.txt", header = FALSE)
data2 <- read.table("wasmer-cranelift-speed.txt", header = FALSE)
data3 <- read.table("wasmer-singlepass.txt", header = FALSE)
data4 <- read.table("wasmtime-cranelift-none.txt", header = FALSE)
data5 <- read.table("wasmtime-cranelift-speed.txt", header = FALSE)
data6 <- read.table("wasmtime-singlepass.txt", header = FALSE)
data7 <- read.table("cosmwasm-singlepass.txt", header = FALSE)

convert <- function(x) x / 1e6

compute_stats <- function(data) {
  v <- data$V3
  list(
    avg = convert(mean(v)),
    min = convert(min(v)),
    max = convert(max(v))
  )
}

stats1 <- compute_stats(data1)
stats2 <- compute_stats(data2)
stats3 <- compute_stats(data3)
stats4 <- compute_stats(data4)
stats5 <- compute_stats(data5)
stats6 <- compute_stats(data6)
stats7 <- compute_stats(data7)

cat("Wasmer Cranelift (no optimizations):\n")
cat("  avg:", sprintf("%10.3f", stats1$avg), "[ms]\n")
cat("  min:", sprintf("%10.3f", stats1$min), "[ms]\n")
cat("  max:", sprintf("%10.3f", stats1$max), "[ms]\n")

cat("\nWasmer Cranelift (optimized for speed):\n")
cat("  avg:", sprintf("%10.3f", stats2$avg), "[ms]\n")
cat("  min:", sprintf("%10.3f", stats2$min), "[ms]\n")
cat("  max:", sprintf("%10.3f", stats2$max), "[ms]\n")

cat("\nWasmer Singlepass (Wasmer 7.0.1):\n")
cat("  avg:", sprintf("%10.3f", stats3$avg), "[ms]\n")
cat("  min:", sprintf("%10.3f", stats3$min), "[ms]\n")
cat("  max:", sprintf("%10.3f", stats3$max), "[ms]\n")

cat("\nWasmtime Cranelift (no optimizations):\n")
cat("  avg:", sprintf("%10.3f", stats4$avg), "[ms]\n")
cat("  min:", sprintf("%10.3f", stats4$min), "[ms]\n")
cat("  max:", sprintf("%10.3f", stats4$max), "[ms]\n")

cat("\nWasmtime Cranelift (optimized for speed):\n")
cat("  avg:", sprintf("%10.3f", stats5$avg), "[ms]\n")
cat("  min:", sprintf("%10.3f", stats5$min), "[ms]\n")
cat("  max:", sprintf("%10.3f", stats5$max), "[ms]\n")

cat("\nWasmtime Singlepass (Winch 42.0.1):\n")
cat("  avg:", sprintf("%10.3f", stats6$avg), "[ms]\n")
cat("  min:", sprintf("%10.3f", stats6$min), "[ms]\n")
cat("  max:", sprintf("%10.3f", stats6$max), "[ms]\n")

cat("\nCosmwasm Singlepass (Wasmer 5.0.1):\n")
cat("  avg:", sprintf("%10.3f", stats7$avg), "[ms]\n")
cat("  min:", sprintf("%10.3f", stats7$min), "[ms]\n")
cat("  max:", sprintf("%10.3f", stats7$max), "[ms]\n")

svg("stats.svg", width=10, height=10)
par(mar=c(11,4,4,2))
boxplot(
  convert(data1$V3),
  convert(data2$V3),
  convert(data3$V3),
  convert(data4$V3),
  convert(data5$V3),
  convert(data6$V3),
  convert(data7$V3),
  names = c(
    "Wasmer Cranelift\n(no optimizations)",
    "Wasmer Cranelift\n(optimized for speed)",
    "Wasmer Singlepass\n(Wasmer 7.0.1)",
    "Wasmtime Cranelift\n(no optimizations)",
    "Wasmtime Cranelift\n(optimized for speed)",
    "Wasmtime Singlepass\n(Winch 42.0.1)",
    "CosmWasm Singlepass\n(Wasmer 5.0.6)"
  ),
  pch = 1,
  las = 2,
  col = "orange",
  medlwd = 2,
  ylab = "Compilation Time (ms)",
  main = "Compilation Time Distribution"
)
