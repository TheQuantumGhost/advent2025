# advent2025

This is my adventure for [advent of code 2025](https://adventofcode.com/2025/).

## [Day 01](https://adventofcode.com/2025/day/1)
The first part was easy, just run the list of rotations, and check wether we're equal to zero or not.

For the second part, I first made a quick and dirty solution by checking each individual **click**, this works but I wasn't really satisfied with the method.
The second solution takes a more mathematical approach. We first count a zero for each full rotation using `off % DRUMM`, then we check if the rest of the rotation passes (or reaches) zero. To do this we just check the bounds for the left or right case.
