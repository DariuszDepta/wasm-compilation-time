#!/usr/bin/env bash

R --vanilla <<EOF
# Read the data (assuming space-separated)
data <- read.table("cranelift.txt", header = FALSE)
# Open PNG device
png("cranelift.png", width=1600, height=1200)
# Plot scatter
plot(data\$V2, data\$V3, xlab="Size (bytes)", ylab="Time (ns)", main="Size vs Time")
# Fit a smooth curve using loess
fit <- loess(V3 ~ V2, data = data)
# Add the curve
lines(sort(data\$V2), predict(fit, newdata = data.frame(V2=sort(data\$V2))), col="blue", lwd=2)
EOF
