# Complete-model external XOR-SAT semantic control

Freeze the exact Gray `all_x` XOR-DIMACS file and complete native assignment
named in `audit.json` before either solver call. The original export has
118,684 variables, 548,445 CNF clauses, and 6,059 XOR rows. Construct two
derived inputs by copying every original row unchanged, incrementing the
DIMACS header by 118,684 clauses, and appending one unit clause per model bit.
The first input fixes the independently audited native assignment. The second
flips only variable 1 in those appended units; the original export itself is
unchanged. Hash and retain both complete derived inputs in local scratch.

Run the pinned CryptoMiniSat 5.14.7 binary, one thread, with 5,000 conflicts,
120 seconds of solver wall limit, a 150-second outer timeout, and no printed
assignment. Record command, binary and input hashes, exit code, parsed status,
wall time, peak RSS, stdout/stderr hashes, and raw file-tree hash. The expected
positive result is SAT (exit 10); the negative control is UNSAT (exit 20),
because variable 1 is already constrained by the original all-x instance.
Anything else is censored or a producer failure. Do not infer natural relation
yield or a factor-base ranking from this planted-witness control. Charge both
full 150-second envelopes even if the calls stop sooner, and keep validation
cost separate.

This control tests the external solver's interpretation of the actual exported
XOR syntax. It does not assume that the independent checker and CryptoMiniSat
share a convention merely because the native assignment passes the checker.
