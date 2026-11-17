# Twist obstruction for summation-polynomial relations

2026-10-26. The results report is [`REPORT.pdf`](REPORT.pdf), with source
[`REPORT.typ`](REPORT.typ). The run report is
[`reports/run-20261026T1456Z/RUN_REPORT.pdf`](reports/run-20261026T1456Z/RUN_REPORT.pdf).
The protocol, frozen before the run at `ff3bf9d48`, is
[`PROTOCOL.md`](PROTOCOL.md).

**Question.** A curve and its quadratic twist share x-coordinates, so they
share summation polynomials. Can twist points supply index-calculus
relations for the DLP on E?

**Answer.** No. Frobenius acts as +1 on E(F_q) and as −1 on the embedded
twist, so in R = A + B the twist part satisfies 2B = 0. This study checks
the argument and its consequences exactly, on toy fields in the ECC2K-130
model: y² + xy = x³ + ax² + b over F_{2^n} with n odd, twist a ↦ a+1.

| check (exact computation) | result |
|:--|:--|
| census: P ∈ E(F_q), Q ∈ φ(E′(F_q)) with Q ∉ E[2], over n = 7, 11, 13 and 15 curves | 0 of 356,530,036 sums are rational |
| decompositions, m = 2 and 3, n = 11–19, 4954 targets | 0 violations; the off-parity sub-sum is always O or T |
| off-parity solutions vs the set predicted from the target and V alone (m = 3) | equal on 1154 of 1154 targets |
| x-solution sets on E_a and on the twist E_{a+1} | identical on 4954 of 4954 targets |
| point-enumerated sets vs explicit S₃ / S₄ roots | equal on 3800 of 3800 / 105 of 105 targets |
| positive controls C1 (x_R ∈ V) and C2 (planted T-pairs) | 18,042 and 95 predicted solutions, all found |

The heuristic reference assumes lift types are independent of the solution
condition. It predicts that 74–89 % of solutions would contain a
twist-lifting coordinate. Observed:
- m = 2: 0 % (95 % upper bounds ≤ 0.16 %);
- m = 3: 0 % at n = 17, 19, and 4–6 % at n = 11, 13. That mass is entirely
  the predicted degenerate family (x_R, x, x), which arises because x_R
  happens to fall in V.

**Consequences.**
- Twist points add no relations to a factor base.
- Every F_q-root of S_{m+1} is a valid relation on E, so a rationality check
  discards nothing usable.
- The predicted-set comparison is a correctness invariant that any
  decomposition pipeline can be tested against.

No ECDLP cost was measured, so there is no leaderboard, benchmark or rho
row. Scope and limits are in REPORT §4.

## Layout and reproduction

```sh
cargo run --release -- full reports/run-<UTC>/         # harness, std-only Rust (~270 s)
(cd summarize && cargo run --release -- ../reports/run-<UTC>/)   # summary.json
python plot.py reports/run-<UTC>/                       # figures (layout only)
```

- `src/main.rs`: harness. It does F_{2^n} and F_{q²} arithmetic, point
  arithmetic, the census, the decompositions, the classifier and the
  predicted-set builder.
- `summarize/`: aggregation and Clopper–Pearson intervals. It checks its
  CP intervals against known values at start-up.
- `reports/run-20261026T1456Z/`: `rows.jsonl` (raw, one row per census
  curve or target), `summary.json`, `run.json`, `host.txt`, `stderr.log`,
  the figures (SVG and PNG) and the run report.
