# End-to-end toy index calculus vs Pollard rho

Measures the whole pipeline, not one step, to give the distance to rho. Both
methods are counted in point additions (group ops) on the same prime-order
curves. Exploratory runs, not bench.record runs (no S3 credentials in the
session). All numbers are [pcc:unverified].

`ic_mt.py p seed n1,n2,...`:
- Curve: random y^2 = x^3 + Ax + B over F_p with prime order r (brute-force point count, seeded).
- Index calculus: the factor base is the n points with smallest x. Building the pair table x(F_i +- F_j) costs about n^2 adds. Targets come from an r-adding random walk (1 add each). Each target is decomposed as R = +-F_i +-F_j +-F_k: about 2n adds per target, plus one add per hit to recover the sign. The run collects n + 5 relations, then does Gaussian elimination mod r (not counted in group ops). The recovered log is asserted equal to the true w.
- Rho: r-adding walk with 20 multipliers and Brent cycle detection, mean of 5 runs, checked against w. No negation map.
- Outputs: `runs/`, plot `ic_vs_rho.{png,svg,pdf}` (`plot.py`).

Mean over 3 curves per p; IC is at its best n of the grid:

| p | rho ops | IC best total ops (n) | IC / rho | sqrt p | p^(2/3) |
|---|---|---|---|---|---|
| 10007 | 163 | 1288 (30) | 7.9 | 100 | 464 |
| 100003 | 501 | 5008 (60) | 10.0 | 316 | 2154 |
| 1000003 | 1752 | 22647 (120) | 12.9 | 1000 | 10000 |

- IC cost grows like about p^0.63 and rho like about p^0.51 over this range, so the gap widens with p (7.9x, 10.0x, 12.9x) [pcc:unverified].
- Every IC and rho log recovered was correct (asserted).
- This is the expected outcome for a decomposition step that is generic group arithmetic: the total is about p^(2/3), and no tuning of n changes the exponent. Speeding up the decomposition step further (as in `sym_m4/`) cannot close the gap. Only a decomposition whose per-target cost grows more slowly than n, which no encoding here has shown, could [pcc:unverified].
- Not counted: linear algebra (n^3 field ops) and the one-time field cost of building the factor base. Counting them would only widen the gap.
