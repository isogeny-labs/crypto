# Linear algebra amendment

The frozen first s=7 run reached rank 1,093 with 2,186 accepted relations
from 269,998 attempts. Its `Matrix.solve_right` call on a Sage sparse matrix
entered generic square-matrix echelonization. It was interrupted after about
16 minutes of wall time (exit 130); rank was full but no solution or descent
receipt had been produced. The first run and its stdout/stderr are retained.
`s7_attack_v2.sage`
changes only the post-rank workflow: it writes a source-bound relation
checkpoint before linear algebra and uses a dense finite-field matrix for
the solve. It retains the same source curve, target, factor base, relation
walk, caps, transfer and full-cost accounting. A complete result still
requires verified factor-base logs, descent, and the standalone matched rho.

The second run reproduced the full-rank result, then exited 1 before writing
its relation checkpoint because Sage's preparser left padded coordinate zeros
as Sage Integers, which Python JSON cannot serialize. Its stdout and traceback
are retained. `s7_attack_v3.sage` changes only JSON conversion: padded zeros
are Python integers, a factor-base JSON preflight runs before relation
collection, and both receipts use `default=int`. It retains the same fixed
instance and mathematical algorithm.

The first matched-rho source combined negation with the order-seven
Frobenius quotient. It hit its cap on the first target. A diagnostic trace
showed repeated short cycles with exactly identical `(a,b)` walk coefficients
at every collision, so none could recover a scalar. This is the known
fruitless-cycle failure mode of automorphism-quotiented rho, not evidence
that the target is hard. `s7_rho_v2.sage` preserves the saved source target
and order-seven Frobenius optimization, removes negation canonicalization,
and writes one immutable receipt per verified trial. The failed baseline
stdout/stderr and the diagnostic trace are preserved.
