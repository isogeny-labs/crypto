# Protocol: three engineering levers on the generator-orbit IC arm at K1 n29 (2026-11-18)

Frozen before any measured data (hashes in `REGISTRATION.sha256`, commit named in `registered-at.txt`). Class:
**engineering lever on the validated generator-orbit IC arm** (`research/ecbench_generator_orbit_20261022`), T0 scope.
Every number is `[pcc:unverified]` until a validator signs it.

## 0. What was seen before freezing (disclosed)

- The committed A0 session of `research/koblitz_known_log_columns_20261118` (K1 a=1 n=29, seed 20261118402, 144 measured
  records per arm) was profiled from its raw records. Mean per solve of `gen-c1`: total 93.64 GAE, factor base 0.99,
  pair table (oracle set-up) 32.07 (30 additions, 29 canonicalisations, 29 table inserts), relations 40.01 (37.0 trials),
  linear algebra 0.04, verify 20.52 (one double-and-add). Native counts the band prices at `w`: factor base 377 unpriced
  operations (hash probes 174, point negations 87, factor-point inserts 58, curve-equation checks 30, orbit-key squarings
  28) plus 29 calibrated Frobenius maps; set-up 29 + 29; relations 37 + 37. Band S (w = 0 / 0.1 / 1): gen-c1 0.425 /
  0.689 / 3.065, rho-v5 0.337 / 0.366 / 0.623, rho-strong 4.525 / 4.634 / 5.614. At w = 1 about 60% of the arm's priced
  cost is factor-base bookkeeping.
- Code reading showed that the orbit-layout oracle (`mitm-frobenius-layout-counted`) names base points by index and never
  reads the base's hash index (`point_index`, `x_index`); that its pair table forms `P + X` and `P - X` with two inversions
  and also forms `P + (-P)`; and that the recovery check is a double-and-add although the base already stores all 58
  signed Frobenius images of `G`.
- Unit test `generator_orbit_lean_sib_tau_levers_keep_the_solve_and_remove_work` ran the pipeline with and without the
  levers on n23, n29 and n37 (planted logs, 2 seeds each) and asserted only pass/fail properties (same recovered log, same
  trials, lower total). No ratio, mean or interval was looked at. No ecbench session of any candidate arm exists before
  this freeze.

## 1. Levers (each one parameter, default off; absent or 0 is the incumbent bit for bit)

1. **`lean_layout=1`** on `compact-orbit-scan` (needs `generator_orbit=1` and `generator_multiples = columns`, so every
   column is a known seed's orbit). Same points, columns, coefficients and negation map (`neg_index[i] = i ^ 1` by the
   layout). Skipped: the hash index of `FactorBase::from_column_map` (58 inserts, 174 probes, 58 negative-key negations at
   n29), the on-curve check of each Frobenius image (the seed `G` is still checked; the Frobenius maps the curve to
   itself), and with one column the orbit's minimum key (it only detects a repeated column). The base is refused by every
   oracle but the orbit-layout one. Expected: ~347 fewer unpriced native operations per solve; no GAE change at w = 0.
2. **`sib=1`** on `mitm-frobenius-layout-counted`. The pair table keys abscissae only; on a binary curve
   `-X = (x_X, x_X + y_X)` changes the slope of `P + X` by `x_X / (x_P + x_X)`, so `x(P + X)` and `x(P - X)` share one
   inversion (one multiplication and one squaring more). Each such pair is charged one group addition plus one
   `sibling_second_points` (the repository's convention for `wwalk`/`ksum` siblings, priced at `w` by the band), and the
   pair `P + (-P)`, the identity by the layout, is not formed. Keys, shifts, entries, insert order and every probe are
   unchanged. Expected at n29: 30 -> 15 additions in the set-up, +14 sibling points.
3. **`tau_verify=1`** on `mitm-frobenius-layout-counted` (column 0 must be `G`'s orbit; checked). The recovery check
   forms `[d]G` itself as `sum u_i tau^i(G)` over the tau-adic NAF of the integer `d` (Solinas 2000, Alg. 1), reading
   `tau^(i mod n)(G)` and its negative from the stored orbit: one charged addition per non-zero digit after the first, no
   doubling, no Frobenius map, then the full-point comparison with `Q`. Soundness uses only the ring identity
   `tau^2 - mu tau + 2 = 0` (Frobenius on every point of a Koblitz curve, `mu = (-1)^(1-a)`) and `tau^n = 1` on
   `E(F_2^n)`; no eigenvalue, logarithm or group order enters it, so a wrong `lambda` would still fail the check. The
   recoding loop's steps are reported as `tau_naf_recode_steps_uncharged` (priced at `w` by the band; ~31-33 per solve).
   Expected: verify ~20.5 -> ~10 GAE.

None of the three changes the walk, the table or the relation: the candidate does the same trials on the same targets
and recovers the same `d` (checked per record, section 3). What goes is performed work: field inversions, hash-map
operations and group operations. Phase wall times are reported beside the counts (descriptive, L0).

Named so nobody mistakes it: `gen-c1` is, as the gen-orbit README and the known-log study say, a Frobenius- and
negation-folded baby-step giant-step, and stays one. The levers lower its constant; the asymptotic ceiling
(`~2 sqrt(r/2n)` against `sqrt(pi r/4n)` for folded rho) is unchanged. T0 only; no claim on exponents or on IC in general.

Model prediction (from section 0, not a measurement): lv-all / gen-c1 ~0.71 / ~0.61 / ~0.49 at w = 0 / 0.1 / 1.

## 2. Arms (one ecbench session, K1 a=1 n=29, 48 public targets, target seed 20261118404, 3 rounds, 1 warm-up, alternating)

| arm | factor base extra | oracle extra | role |
|:--|:--|:--|:--|
| rho-strong | `rho.signed_frobenius_strong` | | reference (leaderboard's current) |
| rho-v5 | `rho.signed_frobenius_v5` | | reference (strongest measured) |
| gen-c1, gen-c1-aa | `compact-orbit-scan:columns=1,raw_x_cap=1e8,generator_orbit=1` + `mitm-frobenius-layout-counted:m=2`, `stride-generator` | | incumbent; A/A |
| gen-c1-off | `,lean_layout=0` | `,sib=0,tau_verify=0` | default-off control |
| **lv-all** | `,lean_layout=1` | `,sib=1,tau_verify=1` | **primary candidate** |
| lv-lean / lv-sib / lv-tau | one lever each | | attribution (secondary) |

Spec: `make_specs.py` -> `specs/n29.json`. Run: `run.sh` (cpu lock, `--cpus none --allow-busy`, L0) with `ECBENCH` pinned
to a binary built from code commit 33dccd579d (the commit before this registration; `src/` unchanged by it) in a clean scratch worktree (own `CARGO_TARGET_DIR`, `CARGO_BUILD_JOBS=1`),
its sha256 in `logs/binary.sha256` and checked with `strings` for the new parameter names. Analysis: `analyze.py`.

## 3. Pricing, statistics, decision

`ecbench verify --replay-all`; `ecbench band --ranks` at w = 0, 0.1, 1 (table work priced, the default);
`ecbench compare --weight w --resamples 4000 --seed 20261002`, paired by target, cluster bootstrap 95%.

- **P1 (primary, single test):** lv-all / gen-c1. Improvement iff the 95% upper end is below 1 at all three weights;
  improvement at some weights only is reported as such, per weight.
- **P2 (gap, never a gate):** lv-all / rho-strong and lv-all / rho-v5 at the three weights, beside gen-c1's same ratios
  (before), with the band order.
- **Controls (void the session if they fail):** gen-c1-off equals gen-c1 on every record (cost, phases' counts and native
  maps, counters, outcome, seed; wall time excluded); A/A identical; every solve verified with `matches_target`; every
  candidate does the same `targets_tried` as gen-c1 on every (round, target). Cross-binary: the pinned binary replays the
  A0 n29 session of `koblitz_known_log_columns_20261118` identically (incumbent unchanged since its validation).
- **Secondary (descriptive):** lv-lean, lv-sib, lv-tau / gen-c1 at the three weights (attribution; lv-lean is 1.000 at
  w = 0 by construction); phase split; native counts; phase wall times.

## 4. Deviations declared in advance

- No EKS/Iso Lab and no S3 (no AWS in this container): local L0 run, raw session gzipped in the scratchpad with its
  SHA-256 in `summary.json`; compact per-record table in git.
- No conductor and no `research-kb` server in this container; the novelty of the levers is not claimed (siblings and
  tau-adic NAFs are textbook; the repository already uses siblings for `wwalk`/`ksum` and a stored ladder for rho v5).
- CPU budget <= 3 CPU-hours (expected well under one).
- `rho.signed_frobenius_v5` leaves its per-step jump-index selection untallied (koblitz_rho_reference_20261115 section 1);
  the v5 distance is slightly optimistic for rho at w > 0.
- Single curve (n29), as tasked; n23/n37 are an open thread (sib and tau_verify apply to their sp-c*-d5 arms too).
