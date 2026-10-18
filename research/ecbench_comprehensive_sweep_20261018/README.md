# Comprehensive generic sweep, 2026-10-18

Three sealed `ecbench` sessions that measure every registered generic ECDLP
method, and one index-calculus configuration, across sizes, so the cost
exponent `α` (cost ∝ `r^α`) and the constant above the generic floor can be
read per method. Companion of
[`../ecdlp_exponent_assessment_20261018/REPORT.md`](../ecdlp_exponent_assessment_20261018/REPORT.md).
Counted group operations only; a measurement, not a performance or speedup
claim. No row's `S` on any leaderboard changes.

## What ran

| session | spec | sizes | arms | runs | status |
|:--|:--|:--|:--|--:|:--|
| `ECBS1ha345d9b7f6b2` ([`sessions/prime`](sessions/prime/), [report](reports/prime/report.md), [PDF](reports/prime/report.pdf)) | [`spec-prime.json`](spec-prime.json) `33639f18…` | prime-order curves over 16, 18, …, 32-bit primes (`prime_search` seed 59297), `log₂ r` 15.7 … 31.7 | `rho.plain`, `rho.negation` (reference), `bsgs.textbook`, `bsgs.interleaved`, `bsgs.negation`, `kangaroo.vow`, A/A control | 378 measured + 126 warm-up, 378/378 verified | complete, 378/378 replays identical |
| `ECBS1h88c58e3e8c89` ([`sessions/koblitz`](sessions/koblitz/), [report](reports/koblitz/report.md), [PDF](reports/koblitz/report.pdf)) | [`spec-koblitz.json`](spec-koblitz.json) `6cec44af…` | `K_1 / F_{2^n}`, `n ∈ {17, 19, 23, 29, 43, 47}` (the degrees ≤ 47 whose `K_1` has a usable prime-order subgroup in the repository's constructor), `log₂ r` 15.4 … 36.6 | `rho.signed_frobenius_strong`, `rho.negation` (reference), `bsgs.negation`, `kangaroo.vow`, A/A control | 180 measured + 60 warm-up, 180/180 verified | complete, 180/180 replays identical |
| `ECBS1hec2206c6baf4` ([`sessions/prime-ic`](sessions/prime-ic/), [report](reports/prime-ic/report.md), [PDF](reports/prime-ic/report.pdf)) | [`spec-prime-ic.json`](spec-prime-ic.json) `3977ff40…` | the 16 … 26-bit curves of the prime sweep | `ic.pipeline` (`prime-abscissa:size=32`, `mitm-prime-batch:m=2`, incremental Gauss, walked targets), `rho.negation` (reference), `rho.frozen_reference`, A/A control | 144 measured + 48 warm-up, 144/144 verified | complete, 144/144 replays identical |

Every workload is one independent single-target instance (2 per curve,
target seed 20261002), 3 interleaved rounds per arm. Specs were written by
`ecbench sweep` (this PR), run by `ecbench run --allow-busy` (the host was
shared with a concurrent C build, so every record is isolation level L0 and
no wall-clock figure here is admitted), reported by `ecbench report`, and
audited by `ecbench verify --replay-all` ([`sessions/*.audit.json`](sessions/)).
Binary `4b05088148c4…` at commit `f793f1fe5` with the session directories
themselves untracked (recorded as a dirty tree). Host: Intel Xeon @ 2.10 GHz
(family 6 model 207), 4 vCPU KVM, environment class `ECBENV2he1389885ccd0`.

## Fitted exponents

OLS of `log₂(mean gae)` on `log₂ r` over the per-size means, slope `α`
with its residual standard error (`reports/*/scaling_fit.csv`; the
`ecbench bound fit` cluster bootstrap is the admitted interval for a bound
record, this is the report's descriptive fit). The A/A control repeats the
reference method with the same seeds and is counted identically, so it is
omitted.

| session | arm | `α` ± se (all sizes) | `α` ± se (upper sizes only) | `C` (gae = C·r^α) | R² |
|:--|:--|--:|--:|--:|--:|
| prime | `rho.plain` | 0.474 ± 0.013 | 0.519 ± 0.041 (4 sizes ≥ 2^24) | 2.16 | 0.995 |
| prime | `rho.negation` | 0.447 ± 0.012 | 0.416 ± 0.023 | 2.97 | 0.995 |
| prime | `bsgs.textbook` | 0.499 ± 0.014 | 0.541 ± 0.051 | 1.46 | 0.994 |
| prime | `bsgs.interleaved` | 0.501 ± 0.023 | 0.450 ± 0.045 | 1.37 | 0.986 |
| prime | `bsgs.negation` | 0.498 ± 0.022 | 0.563 ± 0.081 | 0.96 | 0.987 |
| prime | `kangaroo.vow` | 0.487 ± 0.032 | 0.554 ± 0.157 | 2.37 | 0.971 |
| koblitz | `rho.signed_frobenius_strong` | 0.248 ± 0.015 | 0.294 ± 0.002 (3 sizes ≥ 2^22) | 58.0 | 0.986 |
| koblitz | `rho.negation` | 0.430 ± 0.029 | 0.444 ± 0.110 | 3.91 | 0.982 |
| koblitz | `bsgs.negation` | 0.511 ± 0.015 | 0.539 ± 0.029 | 0.83 | 0.997 |
| koblitz | `kangaroo.vow` | 0.562 ± 0.021 | 0.540 ± 0.028 | 0.63 | 0.994 |
| prime-ic | `rho.negation` | 0.440 ± 0.029 | – | 3.27 | 0.983 |
| prime-ic | `rho.frozen_reference` | 0.404 ± 0.037 | – | 12.5 | 0.967 |
| prime-ic | `ic.pipeline` (|F| = 32, MITM) | 0.757 ± 0.051 | – | 0.48 | 0.982 |

Reading, kept separate from the numbers:

- **Every generic method on prime-field curves fits `α` within two standard
  errors of `1/2`** except `rho.negation`, whose `0.447 ± 0.012` reflects its
  fixed start-up (two scalar multiplications per walk and the jump-table
  build) being a larger share of the count at `2^16` than at `2^32`: the
  per-size `S` falls from 1.91 to 0.91 across the range (table below) while
  the floor `√(π/4) = 0.886` is constant. A slope below `1/2` from a
  shrinking constant is the artefact `docs/bounds/README.md` warns of, not an
  exponent move; the upper-size refit (`0.416 ± 0.023` on four sizes) is
  too short to settle it either way.
- **The Koblitz `rho.signed_frobenius_strong` fit of `0.248` is the same
  artefact, much larger.** That method advances 32 lanes in lockstep and
  pays their start-up before any step, so at `r ≈ 2^15 … 2^18` the count is
  dominated by set-up (`S` = 4.0 – 4.5, 20× the floor) and at `n = 43, 47`
  it is at or below the floor (`S` = 0.209 and 0.107 against floors 0.135
  and 0.129, six runs each; the medium-tier frontier record over 36 runs
  puts it at 1.089 [0.927, 1.261] × floor). Fitting a line through a
  set-up-dominated regime and a step-dominated regime gives a slope that
  describes neither. The frontier's `0.451 [0.399, 0.512]` on `2^32 … 2^47`
  is the admitted exponent for this method; these sizes are below its range
  and this sweep adds no exponent evidence for it.
- **The index-calculus arm fits `α = 0.757 ± 0.051`, above `1/2`,** as the
  assessment's §1 reads the frontier's toy-tier rows (`0.841 [0.500, 1.024]`
  on the same configuration): with `|F|` held at 32 the decomposition work
  grows like `r / |F|`, so a fixed-base IC row cannot fit below `1/2`. Its
  `S` against `rho.negation` on the same instances is 5.7× at `2^16` and
  43× at `2^25` (table below); the gap widens with `r`, which is the
  distance remaining, not a verdict on index calculus with a base that
  scales.
- **`rho.frozen_reference` fits `0.404 ± 0.037`** for the same start-up
  reason (`S` 5.2 → 2.2 across six sizes); it is the historical "before"
  mark, retained for comparison only.

## Mean S per size

`S = gae / √r`, mean over the arm's 6 verified runs on the curve with the
95 % bootstrap interval of the mean (`reports/*/s_by_arm.csv`). Floor
`√(π / 2A)`: 0.886 on prime curves (`A = 2`), `√(π / 4n)` on `K_1` (`A = 2n`).

Prime sweep:

| bits / log₂ r | `rho.plain` | `rho.negation` | `bsgs.textbook` | `bsgs.interleaved` | `bsgs.negation` | `kangaroo.vow` |
|:--|--:|--:|--:|--:|--:|--:|
| 16 / 15.7 | 2.07 [1.40, 2.70] | 1.91 [1.48, 2.39] | 1.64 [1.47, 1.82] | 1.84 [1.80, 1.88] | 1.14 [0.96, 1.32] | 1.77 [1.70, 1.84] |
| 18 / 17.7 | 1.44 [1.02, 1.88] | 1.69 [1.21, 2.40] | 1.31 [1.14, 1.49] | 1.38 [1.22, 1.55] | 0.82 [0.64, 0.99] | 1.94 [1.03, 2.84] |
| 20 / 19.1 | 1.44 [1.01, 1.86] | 1.32 [0.96, 1.70] | 1.44 [1.32, 1.55] | 0.85 [0.62, 1.08] | 0.94 [0.82, 1.05] | 1.95 [1.66, 2.23] |
| 22 / 21.2 | 1.43 [0.81, 2.02] | 1.10 [0.92, 1.32] | 1.28 [1.24, 1.31] | 1.26 [0.85, 1.66] | 0.78 [0.74, 0.81] | 2.67 [2.51, 2.83] |
| 24 / 24.0 | 1.22 [0.76, 1.64] | 1.16 [0.68, 1.65] | 1.82 [1.74, 1.90] | 1.64 [1.47, 1.80] | 1.32 [1.24, 1.40] | 2.65 [2.30, 3.00] |
| 26 / 25.3 | 1.33 [1.00, 1.76] | 1.37 [0.63, 2.14] | 1.42 [1.39, 1.46] | 1.54 [1.53, 1.55] | 0.92 [0.89, 0.96] | 1.07 [0.78, 1.36] |
| 28 / 28.0 | 1.13 [0.61, 1.71] | 1.05 [0.76, 1.33] | 1.14 [1.08, 1.21] | 1.70 [1.59, 1.81] | 0.64 [0.58, 0.71] | 3.00 [2.81, 3.18] |
| 30 / 29.9 | 1.48 [1.01, 2.05] | 1.08 [0.62, 1.55] | 1.41 [1.33, 1.48] | 1.20 [1.10, 1.30] | 0.91 [0.83, 0.98] | 1.48 [1.10, 1.86] |
| 32 / 31.7 | 1.35 [0.95, 1.85] | 0.91 [0.62, 1.20] | 1.67 [1.46, 1.88] | 1.34 [0.92, 1.76] | 1.17 [0.96, 1.38] | 1.62 [1.23, 2.01] |

Expected `S` from the method's own model (`ecbench` [d4]): `rho.plain`
1.25, `rho.negation` 0.89, `bsgs.textbook` 1.5, `bsgs.interleaved` 1.33,
`bsgs.negation` 1.0, `kangaroo.vow` 2.0. The rho arms sit 1.0 – 2.2× above
their expectation at the small sizes and within it at `2^28 … 2^32`; the
BSGS arms are at or below theirs at every size (a deterministic table, so
the per-target variation is the target's position, not the walk's); the
kangaroo arm's six-run means swing 1.1 – 3.0 around its 2.0.

Koblitz sweep (`K_1`):

| n / log₂ r | floor | `rho.signed_frobenius_strong` | `rho.negation` | `bsgs.negation` | `kangaroo.vow` |
|:--|--:|--:|--:|--:|--:|
| 17 / 16.0 | 0.215 | 4.02 [3.85, 4.20] | 1.76 [1.40, 2.12] | 0.96 [0.80, 1.13] | 1.61 [1.42, 1.80] |
| 19 / 18.0 | 0.203 | 2.41 [2.22, 2.60] | 1.72 [1.37, 2.16] | 0.84 [0.72, 0.97] | 1.54 [0.95, 2.13] |
| 23 / 22.0 | 0.185 | 0.87 [0.81, 0.94] | 1.49 [1.17, 1.74] | 0.85 [0.67, 1.02] | 1.81 [1.11, 2.52] |
| 29 / 15.4 | 0.165 | 4.54 [4.41, 4.70] | 1.97 [1.50, 2.52] | 1.17 [1.12, 1.23] | 0.79 [0.59, 0.99] |
| 43 / 32.1 | 0.135 | 0.21 [0.15, 0.27] | 0.43 [0.20, 0.71] | 0.89 [0.63, 1.15] | 2.97 [2.77, 3.17] |
| 47 / 36.6 | 0.129 | 0.11 [0.05, 0.16] | 1.08 [0.86, 1.27] | 1.35 [1.29, 1.41] | 2.55 [2.32, 2.78] |

Index calculus against rho (same instances):

| bits / log₂ r | `rho.negation` | `rho.frozen_reference` | `ic.pipeline` | IC / `rho.negation` |
|:--|--:|--:|--:|--:|
| 16 / 15.7 | 1.91 [1.48, 2.39] | 5.24 [4.40, 6.15] | 10.9 [10.2, 11.7] | 5.7× |
| 18 / 17.7 | 1.69 [1.21, 2.40] | 3.60 [2.66, 4.63] | 10.3 [9.1, 12.0] | 6.1× |
| 20 / 19.1 | 1.32 [0.96, 1.70] | 2.49 [1.77, 3.27] | 10.5 [8.5, 12.2] | 8.0× |
| 22 / 21.2 | 1.10 [0.92, 1.32] | 3.49 [1.80, 5.90] | 17.7 [13.6, 21.9] | 16.2× |
| 24 / 24.0 | 1.16 [0.68, 1.65] | 2.81 [1.71, 4.24] | 30.3 [24.8, 36.0] | 26.2× |
| 26 / 25.3 | 1.37 [0.63, 2.14] | 2.17 [1.23, 3.32] | 59.0 [35.8, 80.2] | 43.0× |

The IC arm's phases (`reports/prime-ic/phases.svg`) are relations-dominated
at every size; factor-base construction and oracle set-up are below 2 % of
the count from `2^18` up.

## Classification under the success criterion

Measurement and tooling. No improvement over any baseline is claimed; no
`S` moves; the exponent fits above are descriptive and none excludes `1/2`
for a generic method on its valid range. Every ratio between arms is in
counted operations with native work unpriced (the §5a price band is not
applied because no cross-method ranking is asserted).

## Files

- `spec-*.json`: the frozen specs, written by `ecbench sweep` (the Koblitz
  spec was regenerated once before any run, after `ecbench plan` rejected
  `n = 13, 31, 37` for `K_1`; the probe of every degree is in the PR).
- `sessions/<name>/`: `spec.json`, `plan.json`, `host.json`, `session.json`,
  `records.jsonl` (every planned execution, warm-ups included), `exec/`;
  `sessions/<name>.run.log`, `.verify.log`, `.audit.json`.
- `reports/<name>/`: `report.md`, `report.typ`, `report.pdf`, `report.json`,
  the graphs `s_by_arm`, `s_distribution`, `scaling` (and `phases` for the
  IC session) as SVG, PNG (150 dpi) and the CSV each is drawn from,
  `scaling_fit.csv`, `raw_records.csv`.
- Browsing copies of the PDFs and graphs:
  [`benchmarks/20261018-ecbench-comprehensive-sweep/`](../../benchmarks/20261018-ecbench-comprehensive-sweep/).

Reproduce: `cargo build --release --bin ecbench`, then for each spec
`ecbench run --spec <spec> --out <dir>`, `ecbench verify --dir <dir> --replay-all`,
`ecbench report --dir <dir> --out <out>`; compile `report.typ` with Typst.
