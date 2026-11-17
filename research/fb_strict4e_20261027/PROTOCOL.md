# Protocol: strict-4E factor-base selection vs the default binary-subspace (frozen before any run)

Date 2026-10-27. Code: `src/cryptanalysis/ic_framework/subspace_select.rs`,
option `binary-subspace:dimension=l,select=strict4e,seed=s` in
`BinarySubspaceBase`. Source of the rule: ml-cryptanalysis
`experiments/fb-cost-model-construct` (T4, sieve) — that work measured PDP-stage cost
per relation on toy fields; this is the first complete-DLP test of it in Rust.

## Question
At equal dimension l, does a sieve-built 4E-strict subspace solve a complete,
verified DLP with fewer counted group operations than the polynomial-prefix
`binary-subspace`, on Koblitz a=0 curves with h=4? Distance to the matched rho is
reported, not gated.

## Design
- Curves: Koblitz a=0, n = 13, 19, 23 (h = 4 checked by the unit tests for n=13, 19; n=23
  h is checked by `strict4e_refusal` at run time, a refusal fails the run).
- Cells: l in {3, 4} at each n (the cells of the ml reports; l=4 at n=13 has no strict
  subspace, so that cell exercises the documented fallback).
- Oracle `mitm:m=3,negation_folded=1`, solver defaults, 8 public targets per curve
  (`target_seed` 20261027), measurement seed 20261028, one measured round after one warm-up.
- Arms per cell: `ic-prefix-lL` (baseline), `ic-strict-lL-s{1,2,3,4}` (candidate; four independent
  sieve seeds), and per curve `rho-strong` (reference, `rho.signed_frobenius_strong`) and
  `rho-strong-aa` (control). Arms differ only in the factor-base string.
- Specs: `specs/spec-n{13,19,23}.json`, sha256 in `SPECS.sha256` (written with this file).

## Primary statistic
D = log2( Sum S_prefix / Sum S_strict ) per (cell, sieve seed), from `ecbench compare`
(paired over the same 8 targets; S = charged group-addition equivalents / sqrt(r); positive
means strict4e is cheaper). Pooled estimate: mean of D over 6 cells of (mean over the 4 seeds).
Interval: percentile bootstrap over the 6 cells (B = 10000, fixed seed), script
`pool.sh` in this directory; the per-cell intervals are `ecbench compare`'s own.

## Decision rule (fixed now)
- "improvement": pooled lower end > 0 AND the band verdict (below) holds in the cells claimed.
- "null": pooled interval contains 0. "regression": pooled upper end < 0.
- A per-cell claim needs that cell's `ecbench compare` interval to exclude 0 for at least
  3 of the 4 seeds.

## Accounting and band (AGENTS.md section 5a)
`ecbench band --dir ... --weights 0,0.1,1` prices every arm's `*_uncharged` counters alike.
The strict4e arm emits `strict4e_membership_tests_uncharged` (2^n, rebuilt in every run:
the construction is charged to the solve that uses it, nothing is amortised across targets)
and `strict4e_sieve_lookups_uncharged`. Two arms are ranked only where the order holds at
w = 0, 0.1 and 1. If the order flips, the result is "unresolved at that boundary".
No calibrated IC cost is compared with an uncalibrated rho cost.

## Validation-gate artifacts
`ecbench verify --replay-all` (replay certificate); validator report = independent recompute
of the pooled headline from the saved compare JSON (`validate.sh`); matched rho =
`rho.signed_frobenius_strong` in the same binary and session. `VALIDATION.json` is written only
if all three exist and `scripts/validation_gate.py evidence` says valid; otherwise the missing
piece is named in the report. Nothing is relabelled.

## Known limits, stated before the data
- The cost-model ridge and exact rank3 of the ml work are not ported; `gated-rank` uses a
  structural substitute (see source) and is NOT measured here.
- The sieve omits the original's coordinate-descent polish.
- Construction needs an O(2^n) bitmap, so n <= 26; and the 2^n membership scan is a larger
  native cost than the whole rho run on these toy curves. The band is where this shows.
- Toy sizes only; no claim about ECC2K-130 beyond the ml report's structure-only statement.
