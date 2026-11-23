# Solver execution amendment, version 2

The first 1,930-cell attempt used the human-oriented polynomial printer for
external input. Its Unicode operators are invalid CAS input; all external
outputs in that attempt are rejected. Preserve the complete attempt and its
native records. No external speed or equivalence result follows from it.

Repeat exactly the V1 cells, seeds, encodings, resources, time limits and
acceptance criteria in a fresh directory. Serialize every polynomial with ASCII
operators and explicit positive coefficients, with a round-trip test before
execution. Keep native basis records compact without changing their contents.
Disable child core dumps; retain ordinary stdout, stderr and exit status.
No time or performance result is admitted across the two attempts.
