#!/usr/bin/env zsh

R --vanilla <<EOF
data <- read.table("a.txt", header = FALSE)
data\$Difference <- data\$V2 - data\$V1
print(data)
EOF
