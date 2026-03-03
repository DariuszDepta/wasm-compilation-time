#!/usr/bin/env zsh

R --vanilla <<EOF
data1 <- read.table("singlepass.txt", header = FALSE)
data2 <- read.table("cranelift.txt", header = FALSE)

avg1 <- (mean(data1\$V3) / 1000000000)
min1 <- (min(data1\$V3) / 1000000000)
max1 <- (max(data1\$V3) / 1000000000)

avg2 <- (mean(data2\$V3) / 1000000000)
min2 <- (min(data2\$V3) / 1000000000)
max2 <- (max(data2\$V3) / 1000000000)


cat("Average for Singlepass:", avg1, "[s]\n")
cat("Min for Singlepass:", min1, "[s]\n")
cat("Max for Singlepass:", max1, "[s]\n")

cat("Average for Cranelift:", avg2, "[s]\n")
cat("Min for Cranelift:", min2, "[s]\n")
cat("Max for Cranelift:", max2, "[s]\n")

EOF
