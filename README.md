# advent2025

This is my adventure for [advent of code 2025](https://adventofcode.com/2025/).

## [Day 01](https://adventofcode.com/2025/day/1)
The first part was easy, just run the list of rotations, and check wether we're equal to zero or not.

For the second part, I first made a quick and dirty solution by checking each individual **click**, this works but I wasn't really satisfied with the method.

The second solution takes a more mathematical approach. We first count a zero for each full rotation using `off % DRUMM`, then we check if the rest of the rotation passes (or reaches) zero. To do this we just check the bounds for the left or right case.

## [Day 02](https://adventofcode.com/2025/day/2)
Again, a bit of analysis reveals that if an id has `2k` digit, then it is invalid if and only if it is divisible by `10^k + 1`, that is a two digit number is divisible by 11, a 4 digit number by 101 and so on.

For the second part, it was a bit more complicated, the criteria is that if an id has length `l = p * q` with `p >= 2` then it's an invalid id if it's divisible by `A`, with `A` being a concatenation of `p` repetitions of the following motif, `0..01`, with `q` zeros.
