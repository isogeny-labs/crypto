# Protocol: charge the isogeny walk into the weak class on the validated JV sieve instances

Frozen before any run of this study (the smoke run p = 1009, seed 17, moves 8 in the scratchpad is disclosed
below and is not part of the data). The validated row `research/jv_sieve_line_vs_rho_20261103` (arm U, against
a matched negation rho) starts on the weak curve. This study measures what it costs to **reach** that weak curve
from a non-weak curve in the same isogeny class, and adds it to the validated cost, instance by instance.

## Instances and arms

- The validated instances: `generate_spec(p, seed)`, p ∈ {1009, 1511, 1823}, seeds 17–24 (24 instances).
- Start curve: `jv_isogeny_walk --e2e` builds a non-weak challenge curve C by M random 2- and 3-isogeny moves
  away from the weak curve, transporting G and Q. M ∈ {8, 32, 128, 512} (96 runs). A larger M puts C further
  into the class. M = 512 is the primary arm, as the closest available proxy for a random curve in the class.
- Then: a breadth-first walk from C to the nearest weak curve (`walk_muls`), the transport of the points
  (`transport_muls`) and the change to the cover's model (`model_muls`). Their sum is **reach**.
- The route on the weak curve runs inside the same process and must solve and verify on C.
- Binary: `examples/jv_isogeny_walk` built from the commit that freezes this protocol (sha256 in
  `runs/binary.sha256`). Environment: `JV_LIN=sparse JV_FB_LINE=32768 JV_FB_LINE_MODE=table`, `--stop 64`.

## Estimand (fixed now)

- Per instance i at M: `reach_S_i = reach_muls_i / (331 sqrt l)`, the unit of the validated S.
- Charged cost: `S_e2e_i = S_U,i + reach_S_i`, where S_U,i is the validated arm-U S of the same (p, seed),
  read from `research/jv_sieve_line_vs_rho_20261103/results/cover/U_<p>.json`.
- Ratio: `S_e2e_i / ref_p(U2)`, with the validated per-prime rho means. Report the geometric mean over the 24
  instances (pooled) and per prime, with the validator's bootstrap, and the §5a band, layer A (reach adds no
  native operations: the walk's hash-set probes are not emitted and stay unpriced, a declared gap).
- Also report reach as a share of S_U, the walk path lengths, and the failures.

## Decision rule (fixed now)

- **"Walk charged, still below rho"** if, at M = 512, the pooled bootstrap upper bound of the charged ratio is
  below 1 at w = 0, 0.1 and 1.
- **"Walk erases the gap"** if the pooled lower bound is above 1 at any w.
- Otherwise, **"not resolved"**.
- A run whose walk is not found within the tool's cap, or whose route fails on C, counts as a failure. Failures
  are reported, and their instance is charged the cap's cost (`walk_muls` as emitted) with no solve. If any
  instance fails at M = 512, the verdict also states the failure count.

## Not measured here

- The **fraction of classes that hold a weak curve.** `research/jv_weak_class_law_20261007` puts it at 0 for
  depth-1 2-volcanoes (half of all classes) and 86–95% for depth ≥ 2, at toy sizes. A random curve in a class
  without a weak curve cannot use this route at any cost. That is a coverage limit, not a cost, and it is quoted,
  not re-measured.
- Whether M random moves reach a uniform curve of the class. M is varied to see if reach saturates.

## Prediction (before the runs)

Walk charged, still below rho: 0.85. Reach share of S_U at M = 512: median below 10% (0.6).
