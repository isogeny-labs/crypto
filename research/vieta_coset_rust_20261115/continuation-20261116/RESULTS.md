# Matched Rust continuation results

**Producer observations and hand arguments remain [pcc:unverified].**
The original completed study is unchanged. This continuation adds explicit
Karatsuba and Karatsuba+Newton backends and tests them on matched inputs.

| Conditioned input | m | Schoolbook multiplication median | Karatsuba median | Paired ratio |
|---|---:|---:|---:|---:|
| 62-bit field fixture | 8192 | 628,385,305 | 300,898,741 | 2.0884 |
| 62-bit field fixture | 16384 | 2,509,465,297 | 1,168,899,497 | 2.1469 |
| Standard P224 curve | 512 | 2,922,285 | 1,908,185 | 1.5314 |
| Standard P224 curve | 2048 | 41,002,731 | 22,039,567 | 1.8604 |

Three retained matched targets per population/configuration; these are native
descriptive medians and paired schoolbook/Karatsuba multiplication ratios, not
statistical confidence intervals or timing ratios. Random-rational inputs stay
separate. All primitive work inside inversion/square-root/output checks is
included. Additions are also recorded. Hashing/allocation/sorting are not priced
by this metric. This is a generic u128/BigUint reference, not a fastest-backend
comparison. Batched lookup still has much smaller query counts.

**Newton division adds no observed effect here:** all successful study queries
have zero Newton divisions, and its counts equal Karatsuba. Its independent
polynomial identity tests exercise long-quotient division, but these Euclidean
chains use the short-quotient fallback. Half-GCD is the next polynomial target.

The default constructor remains schoolbook. Opt in explicitly:

```rust
let arithmetic = Arithmetic::new(field, limits)?
    .with_polynomials(PolynomialBackend::Karatsuba);
```

## Scope, flow and checks

- Primary: four configurations × six arms × (two warmups + six retained)
  = **192 calls, all successful**. m16384 extends the prior largest word-field
  coset. The old field fixtures are reused to isolate the backend change;
  word62 has no asserted prime subgroup order.
- Supplemental P224/m8192: six arms × two retained targets, no warmups
  = **12 calls: four lookup successes, eight polynomial 30-second deadlines**.
  This distinct shorter-budget epoch is not pooled with the earlier 120-second
  study or primary comparisons. No missing calls.
- Separate preflight: 12 successful calls, excluded from comparisons.
- Frozen native analysis reports exact multiplication-count A/A and unaffected
  lookup ratios of1 on all completed retained pairs.
- Separate-process wide scalar replay matched **196 successful calls** and
  retained eight failures. Its exit1 reflects those recorded failures, not a
  set mismatch. Independent signed standing remains pending.
- Seven unit groups and four CLI/replay tests, formatting and clippy pass.
  Root focused release rebuild reached its explicit180-second preparation
  limit while compiling; no full-root-suite pass is asserted.

The source/build and analysis were published before calls at source
`9814b4bb5ca5949685be0f99bc992f6e1d0cc8bd`. Immutable executable copies, source
hashes, distinct pre-call/publication receipts, raw outputs and the frozen native
summary are under [run](run). Original hash checks passed immediately after
calls and before post-call helper changes. An artifact bot moved compressed
binaries to manifests; their original Git checkpoint still retains the bytes.

The unmodified isolated runner reports 930.815920002 seconds on CPU4,
0.94 seconds other-process CPU, 19,816KiB peak RSS and no detected contention.
Missing PSI/unmovable host threads mean **all clocks are below-L2 diagnostics,
wall_valid=false**. No builds/tests/heavy analysis ran during calls; lightweight
Git checkpoints retained completed panels. No cloud spending. Explicit
local/Git-only authorization persists. No timing or external-fastest claim.

All three canonical exports pass file verification with zero findings:
[primary](../export-poly-main.json), [supplemental](../export-poly-supplemental.json),
[preflight](../export-poly-preflight.json). Initial metadata drafts used the
wrong directory for study-relative paths; the failed checker receipt is kept,
and only the unpinned exports were regenerated at the study root. Frozen
protocols and raw data were never rewritten. Post-call additions are a metadata
exporter host-path option and the separate exact-example checker.

Scalar complete-set verification and shared descriptor/base setup are recorded
outside the query intervals. Reported ratios are component query counts, not
cold end-to-end costs. The record is censored because of supplemental limits;
no protected benchmark-index or IC-leaderboard row is promoted.

## Rational-map construction and conditional upper bound

[SPLIT_TOWER_BOUND.md](SPLIT_TOWER_BOUND.md) extends the hand argument to fully
split rational towers N_i/D_i with coprime polynomials, deg N_i=q_i and
deg D_i≤q_i−1. It proposes

    d_F ≤ 2 sum(q_i−1)+2,

for two membership chains plus the exact quadratic Vieta core. Binary towers
give 2 log2(M)+2. The telescoping certificate explicitly carries denominator
factors. Full splitting/separability and finite-fiber assumptions are required;
construction/search costs remain unmeasured. Independent/formal review is
pending. The original PCC-pinned binary proof is unchanged.

The same note gives a homogeneous, division-free/interpolation-free evaluator
in the quadratic algebra and retains the separate repeated-abscissa branch.
This is a construction, not a claim that the measured Kummer qualifier already
implements a general rational/isogeny tower. The Rust exact-example checker
expands a full F17 rational fiber tree and its coordinate identities, plus two
ordered relations on a prime-order toy curve. Its receipt is
[split-tower-certificate.json](split-tower-certificate.json). It does not
formally establish the general bound or supply a large-prime isogeny benchmark.

## Review and reproduction

[Four-page report](report.pdf), [frozen main protocol](protocol-main.json),
[supplemental protocol](protocol-supplemental.json), [native summary](run/summary.json)
and [replay receipt](run/replay.json). Plot rendering reads precomputed Rust
values only; no Python solver or statistical analyzer is used.

Build/check the current source with the standalone manifest. For the measured
instrument, reconstruct source9814b4bb5c or the pinned copied executable;
`run/pre-call-main.json` retains the exact isolated command. The current native
`vieta-poly-compare summarize FILE...` implements the frozen descriptive analysis.
`vieta-split-tower-check` emits only exact symbolic-example data, no timings.
Reproducing benchmark calls requires a new prospective protocol, not altering
this frozen epoch.

Sources checked: repository isogeny_eval/poly.rs, zero_dim_la.rs,
pkm_smooth_tower/prime_fast; official FLINT Newton material; PKM §3.3 and
Huang–Kosters–Yang–Yeo definitions2.1/2.5. KB unavailable (MCP absent, wrapper127);
primary/internal fallback, no novelty claim. Current work remains on PR4410.
