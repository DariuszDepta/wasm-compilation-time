#!/usr/bin/env zsh

R --vanilla <<EOF
# Read two files
data1 <- read.table("singlepass.txt", header=FALSE)
data2 <- read.table("cranelift.txt", header=FALSE)

plot(data2\$V2, data2\$V3, xlab="Size (bytes)", ylab="Time (ns)", main="Size vs Time", col="cyan", pch=16)
points(data1\$V2, data1\$V3, col="blue", pch=17)

fit <- loess(V3 ~ V2, data = data1)
lines(sort(data1\$V2), predict(fit, newdata = data.frame(V2=sort(data1\$V2))), col="green", lwd=2)

fit <- loess(V3 ~ V2, data = data2)
lines(sort(data2\$V2), predict(fit, newdata = data.frame(V2=sort(data2\$V2))), col="red", lwd=2)

EOF
