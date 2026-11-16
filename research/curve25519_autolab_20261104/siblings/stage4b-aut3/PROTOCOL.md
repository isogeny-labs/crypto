# SIBLING-POINTS Stage 4b: the aut=3 panel re-banded with its canonicalisations priced

Status: frozen before any data of this stage (2026-11-12). Every number below is an expectation or
a prior measurement, labelled `[pcc:unverified]`.

## Why

Stage 4 (`../stage4-aut3/`, PR #3883) was validated by an independent validator with verdict
pass_with_errata. Its erratum E1: the `aut=3` canonicalisations were counted in a native map that
the §5a band does not read, while `rho.cubic`'s identical rotations are listed as
`canonicalisations_uncharged` and charged at w.

The engine on `main` now emits them as `aut_canonicalisations_uncharged`. So `cost.unpriced`
lists them, the band charges them at w, and bitop/v1 prices them at 2 field multiplications and
2 word compares.

Stage 4b re-runs Stage 4's design with that engine, so the band is measured directly rather than
re-priced afterwards. Its purpose is to let the validation gate close or open on a clean record.

## Design

Identical to `../stage4-aut3/PROTOCOL.md`, with three exceptions:
- **Fresh seeds and curves.** The measurement seed, target seed and curve seeds are 35201 + 10·bits
  (+ i). Five prime-order `prime_orbit_shape` j0_b7 curves per size, at p = 34, 37 and 40 bits.
- **Engine.** `ecbench` is built from this branch's commit that adds the native analysis tools
  (below), `4e2ea6c1be`, on top of `main` at `60ed08cf38`, which contains `a6a9750f53`. The engine (`wwalk`,
  rho) is `main`'s; only `compare` and `trend` change.
- **Analysis.** Native tools only (AGENTS.md "Implementation language: no Python"):
  - `ecbench compare --save` at w = 0, 0.1 and 1;
  - `ecbench band --ranks`;
  - `ecbench verify --replay 8`;
  - a shell script that only calls these;
  - bit-op ratios from `ecbench compare --unit bitop/v1`, which pairs each run's recorded
    `work.bitops_per_sqrt_r`. It replaces Stage 4's `stats_bitops.py`; on Stage 4's p 40 session
    it reproduces sib8-p512-aut3 / tw2cj-lazy 0.348 [0.328, 0.369].
  - the slope in r from `ecbench trend` (fit `free_r`; it now accepts prime-field slugs) over the
    saved w = 0 comparisons of the three sizes. It replaces `scaling_fit.py`. Its per-curve
    weighted fit differs from that script's workload bootstrap, so its interval is not directly
    comparable with Stage 4's.

Arms, per size: `rho-neg` (reference), `rho-cubic`, `rho-neg-aa` (A/A control), `tw2cj-lazy`,
`tw2cj-lazy-aut3`, `sib8-p512`, `sib8-p512-aut3`. There are 6 public targets per curve, 6 rounds
plus 1 warmup, alternating order, isolation L0, and a 900 s timeout. The panel runs on this
session's host.

## Pre-registered expectations [pcc:unverified]

From Stage 4 and the validator's corrected band:
- **J1.** sib8-p512-aut3 / rho-cubic at w = 0 lies in [0.80, 1.00] at every size.
- **J2.** sib8-p512-aut3 / rho-cubic lies in [1.10, 1.40] at w = 0.1 and in [2.4, 3.1] at w = 1, at
  every size. rho-cubic is lower at w = 0.1 and w = 1, so the pooled band order is unresolved at
  every size.
- **J3.** At w = 1, tw2cj-lazy-aut3 / tw2cj-lazy and sib8-p512-aut3 / sib8-p512 each lie in
  [0.70, 0.95]. At w = 0, each lies in [0.50, 0.66].
- **J4.** Every `aut=3` record lists `aut_canonicalisations_uncharged` in `cost.unpriced`, and its
  count equals the record's `table_insert_attempts`.

## Admission

Each item is checked per spec. A failure voids that spec; it is reported, not re-run under the same
name.
- Every record verified.
- `verify --replay 8` reproduces.
- The A/A ratio is 1.

## Not claimed

- No leaderboard or `benchmarks/README.md` row until a validator re-issues the gate.
- No claim about secp256k1 itself.
