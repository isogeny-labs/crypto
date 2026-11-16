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
