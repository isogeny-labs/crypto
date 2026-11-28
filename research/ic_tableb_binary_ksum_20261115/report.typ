#set page(paper: "a4", margin: 1.6cm)
#set text(size: 9pt)
#set heading(numbering: "1.")
#show table: set text(size: 7.6pt)

= The k-sum table on Table B's random-binary curves

*Study:* `research/ic_tableb_binary_ksum_20261115/` (PROTOCOL.md registered in commit 77bc214198 before any study data). *Record:* `benchmarks/records/20261115T1728Z-tableb-binary-ksum-l1.json`. *Track:* generic decomposition (bounded by $sqrt(r)$; cannot beat rho or BSGS). *Standing:* counted cost at L0, `[pcc:unverified]` (no pcc claim filed, no second-host replay, no independent validator; the validation gate is not met). Wall time is not claimed.

== Outcome

- *L1 holds for `ksum5` on all three primary rows (n21, n24, n27)* and is not falsified. Against the strictest of the tuned pair-table incumbents (`binary-multiplier` bases at three sizes each, plus Table B's recipe base), run in the same session on the same curve, targets and seeds, `ksum5` costs 0.68 / 0.66 / 0.59x (n21), 0.64 / 0.63 / 0.61x (n24) and 0.46 / 0.44 / 0.39x (n27) at w = 0 / 0.1 / 1 GAE per unpriced native operation. All pooled and per-replicate 95% upper ends are below 0.73 (n21), 0.69 (n24), 0.49 (n27), and each gain exceeds the replicate noise delta (1.07, 1.13, 1.10). `ksum4` agrees (0.65 / 0.65 / 0.63, 0.60 / 0.59 / 0.54, 0.48 / 0.47 / 0.43).
- *Secondary rows do not give the same answer.* On n15 and n18 `ksum5` is not below the strictest incumbent (n15: 1.14 / 1.11 / 0.99, upper ends above 1; n18: 1.04 / 1.02 / 0.92); L1 does not hold for `ksum5` there. `ksum4` is below it on both (0.95 / 0.93 / 0.87 and 0.90 / 0.89 / 0.85) with upper ends below 1 pooled and per replicate; the registered rule also needs the gain to exceed the noise delta, which passes `ksum4` on n18 (0.072 against 0.054) and not on n15 (0.033 against 0.045). Secondary rows were never used to rescue a primary row.
- *Predictions* (set from pilots on the same curves, so not independent): `ksum5` $R_0$ 0.681, 0.636, 0.460 against 0.65, 0.65, 0.45 on n21, n24, n27, and 1.138, 1.038 against about 1.0 on n15, n18: all within the registered 0.25.
- *Distance remaining.* Matched negation rho is ahead at every weight on every row (`ecbench band --ranks`: rho-neg and bsgs-neg below every IC arm). `ksum5` costs 5.6-6.9x rho on the primary rows and `ksum4` 5.3-7.5x; the strictest incumbent costs 5.8, 8.2, 9.1, 13.0x rho on n18 to n27 and 6.0x on n15 (w = 0). Table B's recorded recipe stands at 13.1x, 10.6x, 18.0x, 23.0x, 38.2x rho on n15 to n27. This distance depends on how uncharged native work is priced (the band charges only `table_insert_attempts`).
- *Table B's own recipe base is not the strictest incumbent.* In-session `inc-tb` (the `binary-subspace` base of Table B's dimension) costs 1.4x, 1.9x, 2.2x, 2.1x, 3.4x the best tuned `binary-multiplier` incumbent at w = 0 (n15 to n27), so most of the gain over Table B's recipe is a base-family and size change that this study also measured on the pair table itself; `ksum5` against `inc-tb` alone is 0.32x (n21), 0.31x (n24), 0.13x (n27) at w = 0. The claim is the ratio against the strictest incumbent.
- *Verification.* All 15 sessions (1,728 records each, 25,920 in all) verified at record level, and 19,440 replays (1,296 per session) reproduced with `ecbench verify --replay-all --exit-code` (rc 0 for all 15, 0 problems) with the producing binary on the *same host*, which is a weaker check than a second host class.

#table(columns: 11, align: (left, right, left, right, right, right, right, right, left, left, right),
  [*curve*], [*w*], [*strictest inc.*], [*S inc.*], [*S inc-tb*], [*S rho*], [*S k4*], [*S k5*], [*ksum5 / inc. [95%]*], [*ksum4 / inc. [95%]*], [*k5 / rho*],
  [n15 (sec.)], [0], [inc-n18], [11.84], [16.27], [1.966], [11.19], [13.47], [*1.138* [1.120, 1.155]], [0.945 [0.929, 0.962]], [6.85],
  [], [0.1], [inc-n18], [12.75], [17.71], [2.106], [11.89], [14.19], [*1.113* [1.095, 1.131]], [0.933 [0.916, 0.950]], [6.74],
  [], [1], [inc-n18], [20.92], [30.65], [3.365], [18.17], [20.64], [*0.987* [0.967, 1.007]], [0.869 [0.849, 0.888]], [6.13],
  [n18 (sec.)], [0], [inc-n21], [10.85], [20.82], [1.877], [9.79], [11.26], [*1.038* [1.019, 1.057]], [0.902 [0.883, 0.922]], [6.00],
  [], [0.1], [inc-n21], [11.77], [23.42], [2.023], [10.52], [11.98], [*1.018* [0.998, 1.037]], [0.894 [0.874, 0.914]], [5.92],
  [], [1], [inc-n21], [20.12], [46.75], [3.333], [17.12], [18.50], [*0.920* [0.898, 0.942]], [0.851 [0.828, 0.875]], [5.55],
  [n21], [0], [inc-n62], [10.28], [22.15], [1.261], [6.73], [7.01], [*0.681* [0.668, 0.695]], [0.654 [0.638, 0.670]], [5.56],
  [], [0.1], [inc-n62], [11.67], [26.13], [1.371], [7.57], [7.73], [*0.662* [0.649, 0.677]], [0.649 [0.632, 0.665]], [5.64],
  [], [1], [inc-n62], [24.18], [61.88], [2.359], [15.16], [14.27], [*0.590* [0.575, 0.605]], [0.627 [0.609, 0.645]], [6.05],
  [n24], [0], [inc-n67], [10.99], [22.60], [1.211], [6.58], [6.99], [*0.636* [0.621, 0.652]], [0.598 [0.587, 0.610]], [5.77],
  [], [0.1], [inc-n80], [12.52], [26.55], [1.320], [7.33], [7.87], [*0.629* [0.614, 0.644]], [0.586 [0.576, 0.595]], [5.96],
  [], [1], [inc-n80], [26.14], [62.07], [2.303], [14.14], [15.81], [*0.605* [0.588, 0.622]], [0.541 [0.531, 0.552]], [6.86],
  [n27], [0], [inc-n150], [13.95], [47.91], [1.069], [6.70], [6.41], [*0.460* [0.452, 0.468]], [0.481 [0.473, 0.488]], [6.00],
  [], [0.1], [inc-n150], [16.16], [57.04], [1.168], [7.58], [7.17], [*0.444* [0.435, 0.452]], [0.469 [0.461, 0.477]], [6.14],
  [], [1], [inc-n178], [36.05], [139.22], [2.059], [15.46], [13.95], [*0.387* [0.379, 0.395]], [0.429 [0.421, 0.437]], [6.78],
)

S in ecbench counted GAE per $sqrt(r)$, native work priced at w GAE per operation (AGENTS.md §5a), mean over 3 sessions (144 targets x 3 rounds = 432 paired runs per arm and curve). Ratios are paired total costs of the candidate over the incumbent cheapest at that weight, pooled over the 3 sessions; intervals from `ecbench compare` (4,000 resamples, seed 20261002; the larger upper end of the linearised-t and bootstrap intervals is the tested one). All numbers: `results.json`, `results.md`, `summary.json`.

#figure(image("l1-ratios.svg", width: 100%),
  caption: [Candidate over strictest incumbent per row and weight (square: pooled; bar: 95% interval; circle: the three replicate sessions, each against its own strictest incumbent) for `ksum5` and `ksum4`, and S over matched rho at w = 0.])

== Noise and the registered rule

#table(columns: 10, align: (left, right, right, right, right, right, right, left, right, left),
  [*curve*], [*inc-tb*], [*rho-neg*], [*ksum5*], [*ksum4*], [*delta*], [*k5 max upper pooled / replicate*], [*k5 L1*], [*k4 max upper pooled / replicate*], [*k4 L1*],
  [n15], [1.045], [1.035], [1.005], [1.008], [1.045], [1.160 / 1.188], [no], [0.967 / 0.992], [no],
  [n18], [1.054], [1.020], [1.029], [1.015], [1.054], [1.061 / 1.094], [no], [0.928 / 0.950], [yes],
  [n21], [1.013], [1.074], [1.006], [1.032], [1.074], [0.699 / 0.724], [yes], [0.674 / 0.704], [yes],
  [n24], [1.017], [1.127], [1.026], [1.009], [1.127], [0.657 / 0.685], [yes], [0.614 / 0.632], [yes],
  [n27], [1.070], [1.098], [1.009], [1.014], [1.098], [0.470 / 0.484], [yes], [0.491 / 0.502], [yes],
)

Replicate spread = max/min of one arm's mean S (w = 0) across the three sessions (same arm, same curve, different seeds); delta is the largest of the four. `rho-neg` and `rho-neg-aa` share seeds within a session and have identical counts, so they carry no noise information. The `rho-neg` spread (up to 1.13 on n24) is the largest term and is the rho reference's own target-sample variance, not the candidates'.

== Against Table B's recorded rows

#table(columns: 6, align: (left, right, right, right, right, right),
  [*curve*], [*Table B S*], [*in-session inc-tb S (w = 0)*], [*Table B S / rho S*], [*inc-tb / rho (session)*], [*inc-tb / best tuned incumbent*],
  [n15], [26.31], [16.27], [13.1], [8.3], [1.37],
  [n18], [21.30], [20.82], [10.6], [11.1], [1.92],
  [n21], [24.39], [22.15], [18.0], [17.6], [2.15],
  [n24], [27.24], [22.60], [23.0], [18.7], [2.06],
  [n27], [36.93], [47.91], [38.2], [44.8], [3.44],
)

Table B values come from another harness and pricing pins and are not divided into this session's S. The in-session `inc-tb` is 38% below Table B's S on n15, within 2-17% below it on n18, n21 and n24, and 30% above it on n27 (47.9 against 36.9): the `binary-subspace` base is not an exact copy of the Table B recipe (the recipe is the "family optimum" subset of the subspace, walked and balanced), so `inc-tb` is the nearest `ecbench` arm to the recipe, not a reproduction of it.

== Method

- *Curves.* Round-5 ledger `binary_explicit` instances (modulus $2^n + sum 2^t$ from `polynomial_low_terms`), resolved by `ecbench plan` to Table B's slugs. *Arms.* `rho-neg`, `rho-neg-aa`, `bsgs-neg`; `inc-tb` and three `inc-n<N>` (`mitm:m=2,negation_folded=1`; `binary-subspace` / `binary-multiplier:mult=1`); `ksum4`, `ksum5` (`ksum:m=k`, `binary-multiplier`). Sizes frozen from two disclosed sizing pilots before registration (`pilot/`); the minimum-S choice carries a winner's-curse advantage of a few percent for both incumbents and candidates.
- *Replicates.* 3 sessions per curve, distinct master and target seeds, 48 targets, 3 rounds, 1 warm-up, order alternate, L0, `--cpus none`, run one session at a time on this container (4 vCPU Intel Xeon, shared), counted cost only.
- *Binary.* `target/release/ecbench`, sha256 `bc96a44a...e08f8d` (`BINARY.sha256`), built with `cargo build --release --locked --bin ecbench` from commit 2476332d22 (tree clean).
- *Strictest incumbent.* Chosen per weight on the same data, which can only favour the incumbent (pooled test), and per replicate on that replicate's own data.

== Limits

Counted cost on five binary curves of 14-24.4 bits (n15, n18 secondary); generic decomposition, so the method's limit is BSGS and rho stays ahead. The incumbent family (`binary-multiplier`) is not Table B's family, and Table B's recipe base was not re-tuned for the pair table beyond what the three sizes cover; a differently tuned pair-table incumbent could narrow the gap. Native-work pricing hazards of the prime twin (uncharged counted work in rho) apply here too. No second-host replay, no independent validator, no `VALIDATION.json`: nothing here may enter the leaderboard or `benchmarks/README.md`. S3 upload of raw data was not possible from this container (raw data is in Git, gzip).
