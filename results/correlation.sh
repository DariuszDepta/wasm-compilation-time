#!/usr/bin/env zsh

R --vanilla <<EOF
# Read the data (assuming space-separated)
data <- read.table("singlepass.txt", header = FALSE)
# Column 2 = size in bytes, Column 3 = time in nanoseconds
correlation <- cor(data\$V2, data\$V3)
# Print the result
print(correlation)
EOF
