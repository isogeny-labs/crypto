# Replication V2: the Joux–Vitse sieve against a strong rho, thresholds fixed before data

**Outcome: the pre-registered acceptance rule A4 failed, so every decision is withheld.** Everything
else in the acceptance list passed. The tables below are descriptive; they are not claims.

**Scope: a weak class only.** Curves with full rational 2-torsion whose isogeny class contains a weak
curve, never prime order; the cover's Jacobian has **genus 3**. Class: measurement of an existing
algorithm against a stronger reference. Protocol with numeric thresholds, committed before any run:
[`PROTOCOL.md`](PROTOCOL.md) (one amendment, below). Numbers: [`TABLES.md`](TABLES.md),
[`summary.json`](summary.json); raw runs: [`raw/`](raw/); report: [`REPORT.pdf`](REPORT.pdf).

## What was run

New instances (seeds 17–32, disjoint from the first study's 1–16) at `p ∈ {251, 503, 1009, 1511, 1823}`
(ℓ = 2^45.8 … 2^63.0), on the corrected producer of PR #2281 (DP table keeps `y`, every `hash_x`
evaluation counted, option validation). The producer's commit and binary hash were recorded before
the first run (`raw/PROVENANCE.json`: commit `d688f3c2b`, `jv_cover` SHA-256 `1e63ff73…da848`). Arms,
samples, prices and the §5a band are the first study's; 16 runs per size for the strong rho (R3) and
the sieve (dense and sparse), a 2×2 rho factorial, a plain-rho control, and A/A repeats at `p = 1009`.

## The acceptance rules

| rule | requirement | result |
|:--|:--|:--|
| A1 | complete seed inventories, every run correct and uncapped | pass |
| A2 | hash-scope completeness per run (2 hash evaluations per step plain, 4 with negation) | pass (the engine emits exactly 2.0000 and 4.0000) |
| A3 | plain rho reproduces the ledger's S ≈ 1.3 (interval contains 1.3, point in [1.0, 1.6]) | pass at all five sizes (points 1.06, 1.33, 1.19, 1.21, 1.28) |
| **A4** | **rho A/A at p = 1009: ratio of means in [0.85, 1.15] and interval contains 1** | **FAIL** |
| A5 | sieve A/A at p = 1009: identical counts and digests | pass (16/16) |
| A6 | dense and sparse FGLM identical descent digests and outcomes | pass (80/80) |
| A7 | strong-rho exponent point in [0.45, 0.55], interval contains 0.5 | pass (0.511 [0.483, 0.540]) |

**What failed.** R0 first run over repeat: 0.792 [0.572, 1.073]; R3: 0.729 [0.516, 1.035]. Both
intervals contain 1; both point estimates are outside [0.85, 1.15]. Under the protocol a failed
acceptance rule withholds D1–D6, so **no K1/K2 decision, no exponent classification, no crossover and
no acceleration claim is made from this study**, and the first study's qualified status is unchanged.

**Why I think the rule was mis-specified, and why I did not change it.** The ±0.15 window has no power
calculation behind it. With a per-run spread of about 50 % (the A/A arms have coefficients of variation
0.39–0.64) and 16 runs, the standard error of a ratio of two arm means is about 0.18, so the window is
0.85 standard errors wide: a sound instrument would fail it about 40 % of the time per arm, about 64 %
for two. The failure is therefore weak evidence of an instrument defect; the intervals contain 1 and no
difference is detected. That is an argument about the rule, not a licence to relax it after seeing the
data: the protocol says a failed rule is reported as failed, and a changed threshold is a new protocol
version. The cheapest honest next step is a V3 with a rule sized to the sample (for example, the
interval of the ratio must contain 1, with an equivalence margin of about two standard errors, or
enough A/A runs, roughly 85 per arm for ±0.15), fixed before a further campaign.

## Amendment (before the final analysis)

A dry run of the analyzer on partial data showed that A1 only inspected arms that had finished runs, so
a scheduled arm with none would have passed. The analyzer now fails A1 for any absent scheduled arm
(`missing_arms`, unit-tested). No threshold, window, sample or rule changed; the producer was not
rebuilt. Logged at the end of `PROTOCOL.md`.

## Descriptive results (not decisions)

Sieve/R3 ratios, paired by instance seed, 95 % bootstrap (below 1 is the sieve cheaper):

<!-- T1:begin -->
| p | S, harness | S, tuned | S⁺, harness | S⁺, tuned |
|--:|--:|--:|--:|--:|
| 251 | 45.8 [5.2, 137] | 110 [13, 332] | 86.4 [11, 249] | 170 [22, 496] |
| 503 | 0.647 [0.45, 0.93] | 1.56 [1.1, 2.2] | 1.78 [1.2, 2.6] | 3.5 [2.3, 5.1] |
| 1009 | 0.0289 [0.023, 0.036] | 0.0695 [0.055, 0.086] | 0.0759 [0.063, 0.093] | 0.149 [0.12, 0.18] |
| 1511 | 0.00738 [0.0055, 0.0099] | 0.0177 [0.013, 0.024] | 0.025 [0.019, 0.032] | 0.0493 [0.038, 0.064] |
| 1823 | 0.00404 [0.0033, 0.0051] | 0.00971 [0.0079, 0.012] | 0.014 [0.011, 0.018] | 0.0275 [0.022, 0.036] |
<!-- T1:end -->

For orientation only: the first study (seeds 1–16, earlier producer) read 0.066 [0.053, 0.083],
0.027 [0.022, 0.036] and 0.015 [0.012, 0.019] for the `S⁺` harness ratio at `p = 1009, 1511, 1823`
against 0.076, 0.025 and 0.014 here; its fitted sieve exponents were −0.122 and 0.052 (post hoc,
`p ≥ 1009`) against −0.152 [−0.208, −0.107] and 0.042 [−0.020, 0.103] here; its strong-rho exponent 0.508
against 0.511. Different seeds and a different producer, so this is a description of agreement, not a
test. At `p = 503` the sieve is below the strong rho in `S` (0.647 [0.447, 0.932]) and above it in
`S⁺` (1.78 [1.18, 2.61]); `p = 251` is far above.

In this tower batching shows no saving in the harness price (R1 against R0, R3 against R2 in
[TABLES.md](TABLES.md) T3), negation lowers `S` (R2 against R0), and sparse FGLM leaves the descent rows
identical (80/80) with the linear stage 2.08–2.20× cheaper and `S` 1.006–1.135× cheaper. These are
descriptions of this sample; the decision rules D5 and D6 were not evaluated.

## Limits

Weak class only; `p ≤ 1823` (ℓ < 2^63). The isogeny walk to reach the class was not re-measured. The
tuned price is a cost model that credits rho only. The sieve's F4 additions are not emitted by the
harness; the band uses a labelled upper bound. Wall time is an observation only. One instance at
`p = 251` (seed 22) needed `m = 11` and took 31 minutes per sieve arm: a heavy tail of the sieve at small
`p`, deterministic and included.
