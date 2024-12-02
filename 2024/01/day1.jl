#Day 1: Historian Hysteria
# Take an input file of two lists and determine the total distance between thw two lists and their similiarity score
# Author: John Ellis
# Date: 2024-12-01

#Declare usage 
using Printf, DelimitedFiles, StatsBase


# Read file and parse data
#File saved as input.txt
filename = ARGS[1]
lines = readlines(filename)
# separated by space
data = [parse.(Int, split(line)) for line in lines]

# Convert to matrix with Transpose
data = hcat(data...)'  

# Extract left and right columns and sort
left = sort(data[:, 1])
right = sort(data[:, 2])

# Compute differences
diffs = abs.(left .- right)
println("Solution 1: ", sum(diffs))

# Compute scores
counts = countmap(right)
scores = [x * get(counts, x, 0) for x in left]
println("Solution 2: ", sum(scores))
