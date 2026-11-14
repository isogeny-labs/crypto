# L4 results: missing baselines (a) gcd with L_V, (b) FES, (c) polynomial method

Every claim here is `[pcc:unverified]`. Protocol: `PROTOCOL.md`. The frozen body has
sha256 `96b5fcb9717ab3bc1eaa35b7209449f8b1c016e663da450c256cf4608039cff3`. With
Amendment 1 (written before any data) it has sha256
`7fd6f26f5f23a01ae1245564cf3111d4cf70f283e7b7fef3c2587bff925de893`, and that is the
hash in `raw/run-identity.txt`. After Amendment 2 (written after the data; it records
deviations and changes no number) the current file has sha256
`3942baa788dd9b6155a58fb10fe59e29ed9f5285874f036a5d937bab37329aed`.

Run identity (`raw/run-identity.txt`): `ic` sha256 `bd6a9473…2f889479`, crypto HEAD
`632819fb`, `l4bench` sha256 `64b55128…d25db2`, seed 1, 8 targets a cell, 2 repetitions
of each `ic` engine, 120 s budget. Seed-2 top-up: `raw/run-identity-s2.txt`.

## Verdict per sub-item

| item | outcome | basis |
|---|---|---|
| (b) libFES Gray-code exhaustive search | **CLOSED-BY-PRIOR** | `crypto:src/cryptanalysis/mq_fes.rs`, ic_framework `fes-f2`/`fes-f2-wide`; head-to-head with `inherited-f4` on identical seeded targets in `crypto:research/inherited_f4_next_20261112` (Run 1 counted + wall, Run 2 end-to-end S) and `crypto:research/fes_then_batched_20261112`; counted bit operations FES vs Boolean F4 in `crypto:research/blindspots_20261018/03-polynomial-method` |
| (c) polynomial method (Dinur 2021, LPTWY; BKW 2019) | **CLOSED-BY-PRIOR** | `crypto:research/blindspots_20261018/03-polynomial-method`: Dinur's MultParityCount and LPTWY implemented, real calls at N = 12, 16 (S3) and 13 (S4) cross-checked with a counted model on 8,856 calls; real FES and F4 arms. Model crossover with FES at N = 80 (Dinur, working set 2^71.3 bits) and N = 128 (LPTWY), none under a 2^43-bit memory cap. That study's verdict was INCONCLUSIVE because of a sensitivity control. BKW was not a separate arm; its exponent lies between the two. |
| (a) fix m-1 points, gcd(f, L_V mod f) | **implemented by prior work, now measured** (below) | the method is `crypto:src/cryptanalysis/semaev_decomp.rs` (`subspace_poly`, `roots_in_subspace(_quartic)`, the S4 pairs-and-solve). Earlier "root-finding vs F4" head-to-heads used the pair table, not this gcd. A counted, same-instance comparison against `inherited-f4`/`fes-f2` did not exist. |

**(a) under the frozen rule.** The rule is internally inconsistent (Amendment 2), so
both readings are given:

* **Literal IMPROVES clause** ("below I at all three prices at some m = 3 cell"): it is
  met at `13:5:3`, where I/G is 75.5, 27.5 and 5.0 at P1, P4 and P2n. It is also met at
  `19:6:3`, where I hit its budget at ≥1.46e8 native word XORs against G's 2.11e7 at
  P2n, so I/G ≥ 7. This is a **stage** result against the default descent oracle. It
  improves no baseline row: the baselines at n = 41 and 83 already use pair-table and
  root-index oracles.
* **Against the oracle the baseline rows use** (S, the subtract / 2-sum-table counts
  cited from `pdp_root_finder_20261012`): G is above S at every cell and every price
  (G/S at P4 is 7.2 to 14.2 for m = 2 and 305 to 1547 for m = 3). Read this way the
  result is **NEGATIVE**, which is what `l4bench summarize` printed.

There is no improvement of the index-calculus baseline. The gcd-with-L_V oracle is a
real but known constant-factor stage alternative to `inherited-f4`. It is not a new
route.

## Controls

* **Exact-agreement gate.** 0 mismatches. On every one of 136 targets (13 measured
  cells at seed 1 and a seed-2 top-up of 4 of them), G's ordered solution set equals the set of
  every engine that decided the target.
* **Brute force.** On every target with `|V|^m ≤ 2^21`, G equals direct evaluation of
  S_{m+1}, with 0 failures. Before the run, `l4bench selftest` also agreed with brute
  force on 7 configurations (78 targets).
* **Positive control.** 37 satisfiable targets across 9 cells. Cells `29:6`, `29:8`,
  `29:10` and `29:12` had none even after the seed-2 top-up (Amendment 2).
* **Negative control.** The wrong-subspace L_V (`span{z..z^l}`) differs from the
  reference on at least one target in every cell that has a satisfiable target. In
  total it differs on 45 targets.
* **A/A and determinism.** Every G run was repeated, with identical counts and sets on
  all targets. The native `ops` of `ic` are identical across repetitions. The complete
  bitop tally of `inherited-f4` is not always identical: the first repetition can carry
  a one-time tally, so the minimum is used (Amendment 1).

## Numbers (geometric means over the 8 or 16 targets of a cell; `summary/summary.md`)

Units: G is counted in field operations, priced in word ops at P1 (1/1/1), P4
(mul 4, sqr 3, add 1) and P2n (mul 2n, sqr n). An inversion is priced by Itoh-Tsujii.
I and F are native word XORs. I's native count leaves out its matrix builds, which
favours I.

| n | l | m | G mul | G P4 | I native | F native | I/G P1 | I/G P4 | I/G P2n | F/G P1 | F/G P4 | F/G P2n |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 11 | 6 | 2 | 1,281 | 9.20e3 | 2.20e4 | 8.45e3 | 6.48 | 2.40 | 0.55 | 2.49 | 0.92 | 0.21 |
| 13 | 7 | 2 | 2,813 | 2.02e4 | 5.24e4 | 3.32e4 | 7.00 | 2.60 | 0.51 | 4.43 | 1.64 | 0.32 |
| 19 | 10 | 2 | 28,675 | 2.09e5 | 5.21e5 | 2.10e6 | 6.69 | 2.49 | 0.34 | 26.9 | 10.0 | 1.36 |
| 29 | 15 | 2 | 1,245,099 | 9.31e6 | 2.05e7 | 2.15e9 | 5.90 | 2.20 | 0.20 | 618 | 231 | 20.7 |
| 29 | 6 | 2 | 1,283 | 9.12e3 | 1.63e2 | 8.88e3 | 0.048 | 0.018 | 0.002 | 2.63 | 0.97 | 0.09 |
| 29 | 8 | 2 | 6,149 | 4.42e4 | 4.60e2 | 1.32e5 | 0.028 | 0.010 | 0.001 | 8.04 | 2.99 | 0.27 |
| 29 | 10 | 2 | 28,679 | 2.09e5 | 7.39e4 (336 to 2.6e5) | 2.10e6 | 0.95 | 0.35 | 0.03 | 26.9 | 10.0 | 0.90 |
| 29 | 12 | 2 | 131,072 | 9.67e5 | 8.02e5 | 3.36e7 | 2.23 | 0.83 | 0.07 | 93.1 | 34.7 | 3.10 |
| 29 | 14 | 2 | 589,790 | 4.39e6 | 5.31e6 | 5.37e8 | 3.24 | 1.21 | 0.11 | 328 | 122 | 10.9 |
| 11 | 4 | 3 | 26,006 | 1.39e5 | 6.35e5 | — | 12.6 | 4.56 | 0.98 | — | — | — |
| 13 | 4 | 3 | 26,013 | 1.39e5 | 6.18e5 | — | 12.3 | 4.44 | 0.81 | — | — | — |
| 13 | 5 | 3 | 113,051 | 6.13e5 | 1.68e7 | — | 75.5 | 27.5 | 5.02 | — | — | — |
| 19 | 6 | 3 | 486,101 | 2.67e6 | ≥1.46e8 (budget, 8/8) | — | ≥149 | ≥55 | ≥7.0 | — | — | — |

The complete `bitop/v1` count for I, which includes matrix builds and hash probes, is
2.5 to 9.6 times its native count (31 to 43 times on the overdetermined cells `29:6` and `29:8`). On it, G is below I at all three prices at `11:6`,
`13:7`, `19:10` and every finished m = 3 cell. At `29:15` it is not (I/G at P2n is
0.79). Full table: `summary/summary.md`, "Complete counts".

### Crossovers [pcc:unverified]

* **G vs F (m = 2).** In the descent regime (n' = ceil(n/2)), G is below F at P1 from
  l = 6, at P4 from l = 7 (n = 13), and at all three prices from l = 10 (n = 19). In
  the n = 29 l-sweep it is below F at all prices from l = 12. Above that the
  advantage grows like 2^l / l: F/G at P4 is 231 at l = 15.
* **G vs I (m = 2).** In the regime cells, I/G at P4 stays between 2.2 and 2.6 from
  l = 6 to 15. The log2 slope per unit of l is 1.109 for G and 1.096 for I between
  l = 6 and 15. The two have the same exponent, so the difference is a constant, and
  its sign flips inside the pricing band (above 1 at P1 and P4, below 1 at P2n).
  Off the regime, in overdetermined cells (n = 29, l ≤ 10), I is up to 1000× cheaper:
  F4 closes the system at low degree. In the l-sweep at n = 29, I grows by 1.62 bits
  per unit of l between l = 10 and 15, so I/G at P4 rises from 0.35 to 2.20.
* **G vs I (m = 3).** At l = 4 the order is unresolved (P2n 0.98 and 0.81). At l = 5
  and 6, G is below I at every price. G costs about 2^{2l} pairs times about 110 mul;
  I grows faster than that on these cells.
* **Cost formula.** At m = 2, G is about 2^l (2l + 8) multiplications per target
  (measured 2,813 at l = 7 and 1,245,099 at l = 15, so 2^15 × 38). At m = 3 it is
  about 2^{2l} × 110 mul at l = 4 to 6. The general cost is |V|^{m-1} × (gcd cost),
  where the gcd cost is about l × (one Frobenius step modulo a degree-2^{m-2} f).
* **G vs S.** S (pair table / subtract, cited) is O(1) group-law multiplications per
  candidate. G is 7 to 14× above S at m = 2. At m = 3, S is a 2-sum table with |V|
  probes per target against G's |V|^2 pairs, so G is 300 to 1500× above it.

## Gap to rho

This is a decomposition-stage study and has no end-to-end S. The §5a GAE band and the
distance to rho do not apply. The matched-rho gaps of the arms' pipelines are in the
cited end-to-end studies, for example `fes_then_batched_20261112`, where every
descent row is 35× to 3,567× rho.

## Files

`PROTOCOL.md`, `RESULTS.md`, `REPORT.md`, `REPORT.pdf`, `SUMMARY.md`, `results.json`,
`cost_vs_unknowns.svg`, `run.sh`, `src/l4bench.rs`, `summary/` (summary.md,
summary.json, svg), and `raw/` (`ic-*.json` and `g-*.jsonl` per cell and seed, logs,
run identities, `SHA256SUMS`).
