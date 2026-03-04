data1 <- read.table("wasmer-cranelift-none.txt", header = FALSE)
data2 <- read.table("wasmer-cranelift-speed.txt", header = FALSE)
data3 <- read.table("wasmer-singlepass.txt", header = FALSE)
data4 <- read.table("wasmtime-cranelift-none.txt", header = FALSE)
data5 <- read.table("wasmtime-cranelift-speed.txt", header = FALSE)
data6 <- read.table("wasmtime-winch.txt", header = FALSE)

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

cat("\nWasmer Cranelift None:\n")
cat("  avg:", sprintf("%10.3f", stats1$avg), "[ms]\n")
cat("  min:", sprintf("%10.3f", stats1$min), "[ms]\n")
cat("  max:", sprintf("%10.3f", stats1$max), "[ms]\n")

cat("\nWasmer Cranelift Speed:\n")
cat("  avg:", sprintf("%10.3f", stats2$avg), "[ms]\n")
cat("  min:", sprintf("%10.3f", stats2$min), "[ms]\n")
cat("  max:", sprintf("%10.3f", stats2$max), "[ms]\n")

cat("\nWasmer Singlepass:\n")
cat("  avg:", sprintf("%10.3f", stats3$avg), "[ms]\n")
cat("  min:", sprintf("%10.3f", stats3$min), "[ms]\n")
cat("  max:", sprintf("%10.3f", stats3$max), "[ms]\n")

cat("\nWasmtime Cranelift None:\n")
cat("  avg:", sprintf("%10.3f", stats4$avg), "[ms]\n")
cat("  min:", sprintf("%10.3f", stats4$min), "[ms]\n")
cat("  max:", sprintf("%10.3f", stats4$max), "[ms]\n")

cat("\nWasmtime Cranelift Speed:\n")
cat("  avg:", sprintf("%10.3f", stats5$avg), "[ms]\n")
cat("  min:", sprintf("%10.3f", stats5$min), "[ms]\n")
cat("  max:", sprintf("%10.3f", stats5$max), "[ms]\n")

cat("\nWasmtime Winch:\n")
cat("  avg:", sprintf("%10.3f", stats6$avg), "[ms]\n")
cat("  min:", sprintf("%10.3f", stats6$min), "[ms]\n")
cat("  max:", sprintf("%10.3f", stats6$max), "[ms]\n")

cat("\n")
