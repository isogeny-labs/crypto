# R101 independent verifier first attempt

The source-bound producer and its fresh replay passed. The first separately
committed Python GF(2) verifier exited 1 before any certificate output
because this host's Python `int` lacks `bit_count`. The verifier now counts
set bits with `bin(q).count("1")`, preserving the same polynomial and length
checks. The failed traceback was observed at the initial verifier command;
the corrected verifier was committed before its fresh run.
