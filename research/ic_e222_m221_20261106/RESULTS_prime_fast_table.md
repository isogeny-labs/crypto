# prime_fast: table-lookup 2-decomposition vs ic_solve (frozen evaluation)

Status: [pcc:unverified] producer's own analysis; not independently replayed; no pcc standing.

Protocol: `examples/prime_fast_table_protocol.rs`; constants frozen on tune curves (20/22 bits) and committed at 08:34:38Z (commit 7714e25932, verified on the remote before the run): fb = round(c p^(1/3)), c = s2 2.0, s3 0.5, tb 1.5, extra relations 6. Evaluation: 5 fresh curves (21 bits res 3, 22 res 1, 23 res 3, 24 res 1, 24 res 3), 40 targets each, 600 rows, 600 verified (k G = Q), single commit and binary hash in all rows. Time = solver total_ms (relations + elimination + verify).

Arms: s2 = `ic_solve` summands 2 (sweep), s3 = `ic_solve` summands 3 (pair table), tb = new table-lookup 2-decomposition mode.

Median total_ms over all rows: s2 101.9, s3 5.70, tb 8.21.

Paired ratios (median over targets within each instance, then across the 5 instances; the interval is the range over the 5 instances because five instances cannot support a bootstrap):

| ratio | pooled | range over instances |
|---|---|---|
| tb / s3 | 1.37 | 1.25 to 1.46 |
| tb / s2 | 0.084 | 0.073 to 0.116 |
| s3 / s2 | 0.057 | 0.052 to 0.095 |

Finding (negative): the table-lookup mode is 1.25x to 1.46x **slower** than `ic_solve` with 3-term decompositions in every one of the 5 instances, as the tune data predicted (3.66 ms vs 3.23 ms). It is 9x to 14x faster than the 2-term sweep, which is the arm the CLI-stack work compared against. So the table idea, which gave 6x to 7x on the BigUint stack, does not improve the strongest implemented solver. Not an improvement; no leaderboard or benchmark row.

Consequence: the real baseline is s3 (about 5.7 ms median at 21 to 24 bits). Gaps to rho and BSGS on the CLI stack do not carry over; they have to be re-measured against s3 and `rho_parallel`. Limits: 5 instances, toy sizes (up to 24 bits), p = 1 and 3 mod 4 both present, no S3 checkpoint, ecdb, isolab or pcc possible in this container.
