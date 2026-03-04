data1 <- read.table("wasmer-cranelift-none.txt", header=FALSE)
data2 <- read.table("wasmer-cranelift-speed.txt", header=FALSE)
data3 <- read.table("wasmer-singlepass.txt", header=FALSE)
data4 <- read.table("wasmtime-cranelift-none.txt", header=FALSE)
data5 <- read.table("wasmtime-cranelift-speed.txt", header=FALSE)
data6 <- read.table("wasmtime-singlepass.txt", header=FALSE)
data7 <- read.table("cosmwasm-singlepass.txt", header=FALSE)

svg("compare.svg", width = 10, height = 10)

plot(data2$V2, data2$V3, xlab="WASM Size (bytes)", ylab="Compilation Time (ns)", main="Compilation Time", col="red", pch=1)

points(data7$V2, data7$V3, col="orange", pch=6)
points(data1$V2, data1$V3, col="black", pch=0)
points(data3$V2, data3$V3, col="blue", pch=2)
points(data4$V2, data4$V3, col="cyan", pch=3)
points(data5$V2, data5$V3, col="magenta", pch=4)
points(data6$V2, data6$V3, col="green", pch=5)


fit <- loess(V3 ~ V2, data = data1)
lines(sort(data1$V2), predict(fit, newdata = data.frame(V2=sort(data1$V2))), col="black", lwd=2)

fit <- loess(V3 ~ V2, data = data2)
lines(sort(data2$V2), predict(fit, newdata = data.frame(V2=sort(data2$V2))), col="red", lwd=2)

fit <- loess(V3 ~ V2, data = data3)
lines(sort(data3$V2), predict(fit, newdata = data.frame(V2=sort(data3$V2))), col="blue", lwd=2)

fit <- loess(V3 ~ V2, data = data4)
lines(sort(data4$V2), predict(fit, newdata = data.frame(V2=sort(data4$V2))), col="cyan", lwd=2)

fit <- loess(V3 ~ V2, data = data5)
lines(sort(data5$V2), predict(fit, newdata = data.frame(V2=sort(data5$V2))), col="magenta", lwd=2)

fit <- loess(V3 ~ V2, data = data6)
lines(sort(data6$V2), predict(fit, newdata = data.frame(V2=sort(data6$V2))), col="green", lwd=2)

fit <- loess(V3 ~ V2, data = data7)
lines(sort(data7$V2), predict(fit, newdata = data.frame(V2=sort(data7$V2))), col="orange", lwd=2)

legend("topleft",
       legend = c(
         "Wasmer Cranelift (no optimizations)",
         "Wasmer Cranelift (optimized for speed)",
         "Wasmer Singlepass (Wasmer 7.0.1)",
         "Wasmtime Cranelift (no optimizations)",
         "Wasmtime Cranelift (optimized for speed)",
         "Wasmtime Singlepass",
         "CosmWasm Singlepass (Wasmer 5.0.6)"
       ),
       col = c("black","red","blue","cyan","magenta","green","orange"),
       pch = c(0,1,2,3,4,5,6),
       lwd = 2,
       cex = 0.8,
       bg = "white")
