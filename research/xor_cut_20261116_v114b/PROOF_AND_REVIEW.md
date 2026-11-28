# Changed Boolean backend contract

The graph and68-input model map are unchanged. The experiment changes to an
explicit SAT tactic with XOR/AIG cut extraction enabled at delay0; ANF is
explicitly false. Primary versioned Z3sources identify these settings and
defaultfalsevalues. The ANF source includes math/grobner/pdd_solver.h, so that
different route is not presented as a non-Groebner replacement here.

Native cut simplification must preserve every Boolean model. Independent tiny
tests enumerate every input point and all native models through UNSAT, compare
complete root sets, exercise unused inputs/constants and read back global
settings while checking the SAT tactic's supported descriptors. This does not
independently certify a hard native UNSAT or establish performance.

The hard worker keeps the exact same cold graph/hash/original-bit/full-support
checks and45second total budget as the preceding default-SMT diagnostic.
Configuration and engine selection are both changed; any observed difference
cannot be attributed to just one flag. No output is accepted as a complete
original339-variable root set. One core4096MiB90second supervisor,78/84second
finalization boundaries. Known SAT cut simplification is not algorithmic novelty.
