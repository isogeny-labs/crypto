# Results: the measured price of native operations in genus-4 IC (lead A)

Every number is `[pcc:unverified]`. The protocol, frozen before any measured run, is `PROTOCOL.md`
(`frozen.sha256`). Tables come from `analyze.py` (frozen); `tables.md` is its output and
`analysis.json` its machine-readable form. Raw rows and logs are the `*.jsonl.gz` and `*.log.gz`
files beside this note. Host: a local 4-vCPU cloud VM, timing tier (no PMU), one measuring process
pinned to one core, blocks run back to back with nothing else started.

## Headline

On this host, one counted relation-phase operation costs **0.0026 GAE** and one counted mod-r
linear-algebra operation costs **0.0042–0.0045 GAE**, where 1 GAE is one genus-4 Jacobian group
operation (measured at 2.0 µs). The w = 1 end of the §5a band overcharges native work by a factor of
about 220 to 400. The w = 1 headlines of the validated genus-4 studies are therefore
**conservative** (frozen decision rule). They are not fragile.

| p | t_G (µs per group op) | w_rel [95%] | w_la [95%] | arm | IC wall / ideal negation rho time (R1) |
|---|---|---|---|---|---|
| 16411 | 2.00 (3 rho runs: 1.97–2.01) | 0.0026 [0.0019, 0.0032] | 0.0042 [0.0038, 0.0046] | x (cc 2.0) | **0.0053** [0.0052, 0.0054] |
| 16411 | | | | x\* (cc 1.4) | 0.0056 [0.0056, 0.0057] |
| 32771 | 2.03 (1 rho run) | 0.0025 [0.0018, 0.0031] | 0.0045 [0.0041, 0.0048] | x (cc 2.0) | **0.0039** [0.0038, 0.0039] |
| 32771 | | | | x\* (cc 1.4) | 0.0039 [0.0038, 0.0039] |

- **R1, read plainly.** At p = 16411 the ideal negation-rho bound `sqrt(pi r/4)` times the measured
  2.0 µs per group operation is 477 s. Arm x solves the same logarithm in 2.5 s. At p = 32771 the
  bound is 1926 s and arm x takes 7.4 s. The ideal negation bound is a lower bound on any rho with the
  negation map. It charges nothing for canonicalisation or fruitless cycles, so a real rho would take
  longer.
- **Cross-check (post hoc, not frozen).** Pricing the counts at the measured w (cost = G + w_rel ·
  rel_ops + w_la · la_ops, over `sqrt(pi r/4)`) gives 0.0056 (x) and 0.0060 (x\*) at p = 16411, and
  0.0043 for both at p = 32771. These agree with R1 to within 10%, so the counted operations account
  for the wall time.
- **Where the time goes.** For arm x at p = 16411: group operations about 0.85 s, relation-phase F_p
  work about 0.28 s, linear algebra about 1.5 s. Linear algebra is still the largest share of wall
  time. Each mod-r operation is about 240 times cheaper than a group operation, but there are about
  400 times as many of them.

## What this changes

- **The §5a band is very wide on the IC side.** The real weights sit just above the w = 0 end
  (0.0025–0.0045). At those weights the genus-4 IC rows are 0.4–0.6% of the ideal negation rho bound,
  against 64–100% at w = 1. The w = 0.1 column overstates native cost by about 25×.
- **Tuning flips.** The factor-base constant was tuned at w = 1, where linear algebra dominates, and
  that picked cc 1.4 (x\*). By wall time, x\* is 6% slower than x at p = 16411 and tied at
  p = 32771. Mean wall per configuration (post hoc, both passes, SGE (4, 1.5)):

  | p | cc 1.0 | cc 1.4 | cc 2.0 | cc 2.8 |
  |---|---|---|---|---|
  | 16411 | 3.69 s | 2.68 s | **2.52 s** | 2.73 s |
  | 32771 | 9.79 s | **7.43 s** | 7.44 s | 8.87 s |

  The w = 1 convention was steering tuning toward smaller bases. A tuning rule priced at measured
  weights should replace it for real-time claims. That is a follow-up, not a result of this study.
- **No verdict changes.** Every genus-4 IC row that was below the ideal negation bound at w = 1 stays
  below it at the measured weights, by a much wider margin.

## Gates, predictions and noise

- **G1** pass: 128 IC runs certified and correct, and 4 rho runs certified.
- **G2** pass: the 32 rows for cc 1.4 and 2.0 at SGE (4, 1.5) equal the prime-order study's rows in
  every count field.
- **G3** pass: pass 2 counts equal pass 1 counts row for row.
- **A/A:** median |ln(wall₂ / wall₁)| = 0.023, maximum 0.105, over 64 row pairs.

| # | prediction | outcome |
|---|---|---|
| P1 | t_G(16411) ∈ [1.5, 5] µs | **holds**: 2.00 µs |
| P2 | w_rel, w_la ∈ [0.002, 0.05] at both primes | **holds**: 0.0025–0.0045 |
| P3 | R1 for x\* ∈ [0.003, 0.03] | **holds**: 0.0056 and 0.0039 |
| P4 | A/A median < 0.05 | **holds**: 0.023 |
| P5 | t_G(32771) / t_G(16411) ∈ [0.8, 1.3] | **holds**: 1.016 |

## Caveats

- **This implementation on this host.** The weights describe this lab's Cantor arithmetic and its
  u64/u128 mod-r arithmetic on one shared VM. Optimized genus-4 arithmetic (explicit formulas,
  Montgomery F_p) would lower t_G, and with it the cost of rho and of IC's group operations alike. It
  would raise w_rel and w_la. Sensitivity: if t_G fell tenfold with linear algebra unchanged, w_la
  would be about 0.04, and arm x at p = 16411 would cost about 0.86 s × 0.1 + 0.28 s + 1.5 s ≈ 1.9 s
  against 47.7 s, i.e. still about 0.04 of the bound.
- **The OLS intercept is negative** (−0.15 s at p = 16411). It has no physical meaning. Run-level
  overhead is small, and the intercept absorbs some collinearity between relation and
  linear-algebra counts, which both fall with p and rise with cc in opposite directions. The interval
  for w_la is tight, and the cross-check above does not use the intercept.
- **Thin rho sample at p = 32771** (one run). P5 shows t_G does not depend on p within 2%, so the
  p = 16411 value would give the same reading.
- **Not a new speed claim.** This calibrates the price convention under which the validated genus-4
  results are reported. The post hoc tuning table is exploratory.
- **Host:** local, not isolab-eks (no AWS identity in this session). No S3 upload.

## Sources checked

- `research/genus4_prime_order_20261118`: the curves, the binary and the G2 reference rows.
- `research/genus4_followup_20261118`: the timing-tier precedent and the §5a band convention.
- research-kb is unavailable in this session (CLI not installed), so the repository was searched
  instead.
