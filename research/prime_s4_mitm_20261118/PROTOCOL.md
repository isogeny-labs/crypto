# Prime-field S₄ 3-decomposition: pair-sum meet-in-the-middle

**Status:** preregistered 2026-11-18, before the first measured run.
**Lane:** prime regime, `decomposition` / `relation_yield` (wall-clock,
exploratory). Public synthetic known-answer fixtures only.

## Question

The recorded S₃-anchored oracle visits `2B²` candidates per random draw at
cost `c` each and yields `≈ 2B³/p` relations, so a log costs `≈ p·c`
independent of `B`. Replacing the inner pair sweep by a hash probe should
change the *shape*, not just `c`:

- build a table of all `B(B+1)/2` pair sums, keyed by abscissa (once);
- per draw, for each of `B` anchors and both signs, probe `R ∓ F_k` in the
  table (`2B` probes).

A relation then costs `≈ p/B²` probes; `B` relations cost `≈ B² + p/B`, and
the linear algebra adds `B³`. The prediction is a measured **crossover**: at
small `p` the table build makes MITM slower than the anchored sweep; from
some rung on it is faster, and the ratio grows roughly like `p`.

This is still not subexponential. It removes a factor `B²` from the relation
stage; it does not give a decomposition whose per-target cost is sublinear in
`B`, which is what subexponential prime-field IC requires (see
`research/prime_s4_s3_sweep_20261117/REPORT.md` §5).

## Design (frozen before measuring)

- Oracle: `pair_sum_mitm` (`ec_index_calculus_dlp_s4_mitm_staged`).
- Relation family: identical to the S₄ three-summand family; every relation
  is re-added in the group by the shared `try_finalise_s4`.
- Fixtures: `a3-bench-{16,20,24,28}bit-p256class`, and the 28-bit
  CryptoPro-B rung.
- Policy: `fb = 120`, `+8` extra relations, 128 rows, `max_trials = 1e6`,
  `max_relation_attempts = 64`; one seed per rung for the first pass
  (`20261005`), then seeds `20261006`, `20261007` on any rung that passes.
- Controls: paired `s3` runs on the identical fixture and seed (the recorded
  ladder), and the driver-level equality test
  `mitm_driver_agrees_with_s3_driver`.
- Extra point: one 28-bit run at `fb = 256` (closer to `p^{1/4} = 2^7`) to
  show the policy dependence; reported separately, never pooled.

## Decision rule

- Report the crossover rung if any; report every wall, including rungs where
  MITM loses.
- No exponent fit is claimed from three or four rungs; a fitted slope is
  reported only as a description, with the raw table beside it.
- Any rung that fails to recover `[x]G = Q`, match the sidecar, or agree with
  the paired rho control is reported as a failure, not dropped.

## Claim boundary

Public synthetic fixtures only; no deployed curve; no key recovery; no
`vs_rho` claim (IC remains slower than rho at every measured rung); no
asymptotic or subexponential claim.
