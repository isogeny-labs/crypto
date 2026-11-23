# ic_autolab round 2: protocol (multibase track)

Frozen 2026-11-19T07:33:20Z before any data of this round. This file is not edited after that; the selection step appends a dated amendment (after train, before holdout), and nothing else is added.

Loop home: `research/prime_generic_autolab_20261118` (`ledger.json` names the incumbents; `families.json` the candidate families). Success criterion: AGENTS.md, "Success criterion: iterative improvement of the index-calculus baseline"; tracks: AGENTS.md, "Tracks of index calculus". The gap to matched rho is reported as the distance remaining, never as a gate.

## Question

Does any candidate configuration of the multibase track lower the counted end-to-end cost S (GAE / sqrt r, every phase, every solve verified) below the incumbent on the same fresh public targets of the registered `prime-generic` curve at each size below, with the order holding across the AGENTS.md section 5a pricing band (w = 0, 0.1, 1 GAE per unpriced native operation)?

## Incumbents and candidates

Factor base and oracle strings are ecbench `ic.pipeline` parameters. Every candidate is a configuration never tried at its size in this loop (ledger trials).

### k = 20 (p = prime_search(20-bit, seed 59297), r = 562427, log2 r = 19.10, N_a = 52)

- `inc` (baseline): `prime-abscissa:size=3` + `wsum:coef=5,free=1` (family `wsum-b5`, N = 3); S = 4.479 / 5.010 / 9.787 since round 1.
- `rho-neg` (reference): `rho.negation`, the matched rho.
- `wsum-b5-n4` (candidate): `prime-abscissa:size=4` + `wsum:coef=5,free=1`; incumbent neighbourhood: N = 1.2 x 3.
- `mitm2-n56` (candidate): `prime-abscissa:size=56` + `mitm:negation_folded=1`; cross-size transfer: mitm2 is incumbent at 3 sizes; ln N fitted on log2 r (slope 0.210) and extrapolated.
- `wsum-b8-n3` (candidate): `prime-abscissa:size=3` + `wsum:coef=8,free=1`; exploration: family wsum-b8 untried at k = 20, N = 0.0001 x N_a.
- `wsum-b12-n3` (candidate): `prime-abscissa:size=3` + `wsum:coef=12,free=1`; exploration: family wsum-b12 untried at k = 20, N = 0.0001 x N_a.
- `wsum-b20-n3` (candidate): `prime-abscissa:size=3` + `wsum:coef=20,free=1`; exploration: family wsum-b20 untried at k = 20, N = 0.0001 x N_a.
- Train spec `specs/train-k20.json` (sha256 `c2e7a0056321ba6582ac95316011e3d00bbea96b167790c215b9d0a061bf996e`): `target_seed` 2026112420, 12 public targets x 1 measured round(s), 1 warm-up, timeout 300 s per run. Holdout `target_seed` 2026112920 (reserved now, spec written at selection).

### k = 24 (p = prime_search(24-bit, seed 59297), r = 16482251, log2 r = 23.97, N_a = 160)

- `inc` (baseline): `prime-abscissa:size=160` + `mitm:negation_folded=1` (family `mitm2`, N = 160); S = 13.300 / 15.250 / 32.800 since round 0.
- `rho-neg` (reference): `rho.negation`, the matched rho.
- `mitm2-n224` (candidate): `prime-abscissa:size=224` + `mitm:negation_folded=1`; incumbent neighbourhood: N = 1.4 x 160.
- `mitm2-n112` (candidate): `prime-abscissa:size=112` + `mitm:negation_folded=1`; incumbent neighbourhood: N = 0.7 x 160.
- `wsum-b8-n3` (candidate): `prime-abscissa:size=3` + `wsum:coef=8,free=1`; exploration: family wsum-b8 untried at k = 24, N = 0.0001 x N_a.
- `wsum-b12-n3` (candidate): `prime-abscissa:size=3` + `wsum:coef=12,free=1`; exploration: family wsum-b12 untried at k = 24, N = 0.0001 x N_a.
- `wsum-b20-n3` (candidate): `prime-abscissa:size=3` + `wsum:coef=20,free=1`; exploration: family wsum-b20 untried at k = 24, N = 0.0001 x N_a.
- Train spec `specs/train-k24.json` (sha256 `e4cf15c3d87261aabed62b42f1f1a30dc5467638cfe6a55d8b3412dfdaca61bd`): `target_seed` 2026112424, 12 public targets x 1 measured round(s), 1 warm-up, timeout 300 s per run. Holdout `target_seed` 2026112924 (reserved now, spec written at selection).

### k = 28 (p = prime_search(28-bit, seed 59297), r = 260192759, log2 r = 27.96, N_a = 402)

- `inc` (baseline): `prime-abscissa:size=342` + `mitm:negation_folded=1` (family `mitm2`, N = 342); S = 19.309 / 22.426 / 50.475 since round 1.
- `rho-neg` (reference): `rho.negation`, the matched rho.
- `mitm2-n291` (candidate): `prime-abscissa:size=291` + `mitm:negation_folded=1`; incumbent neighbourhood: N = 0.85 x 342.
- `mitm2-n410` (candidate): `prime-abscissa:size=410` + `mitm:negation_folded=1`; incumbent neighbourhood: N = 1.2 x 342.
- `wsum-b8-n3` (candidate): `prime-abscissa:size=3` + `wsum:coef=8,free=1`; exploration: family wsum-b8 untried at k = 28, N = 0.0001 x N_a.
- `wsum-b12-n3` (candidate): `prime-abscissa:size=3` + `wsum:coef=12,free=1`; exploration: family wsum-b12 untried at k = 28, N = 0.0001 x N_a.
- `wsum-b20-n3` (candidate): `prime-abscissa:size=3` + `wsum:coef=20,free=1`; exploration: family wsum-b20 untried at k = 28, N = 0.0001 x N_a.
- Train spec `specs/train-k28.json` (sha256 `8a1bf5ef718d3af55b9700da577c85a5e3b2d10182c49415bb3fe121254aaae0`): `target_seed` 2026112428, 12 public targets x 1 measured round(s), 1 warm-up, timeout 300 s per run. Holdout `target_seed` 2026112928 (reserved now, spec written at selection).

### k = 32 (p = prime_search(32-bit, seed 59297), r = 3466623553, log2 r = 31.69, N_a = 953)

- `inc` (baseline): `prime-abscissa:size=810` + `mitm:negation_folded=1` (family `mitm2`, N = 810); S = 29.239 / 33.970 / 76.548 since round 1.
- `rho-neg` (reference): `rho.negation`, the matched rho.
- `mitm2-n689` (candidate): `prime-abscissa:size=689` + `mitm:negation_folded=1`; incumbent neighbourhood: N = 0.85 x 810.
- `mitm2-n972` (candidate): `prime-abscissa:size=972` + `mitm:negation_folded=1`; incumbent neighbourhood: N = 1.2 x 810.
- `wsum-b8-n3` (candidate): `prime-abscissa:size=3` + `wsum:coef=8,free=1`; exploration: family wsum-b8 untried at k = 32, N = 0.0001 x N_a.
- `wsum-b12-n3` (candidate): `prime-abscissa:size=3` + `wsum:coef=12,free=1`; exploration: family wsum-b12 untried at k = 32, N = 0.0001 x N_a.
- `wsum-b20-n3` (candidate): `prime-abscissa:size=3` + `wsum:coef=20,free=1`; exploration: family wsum-b20 untried at k = 32, N = 0.0001 x N_a.
- Train spec `specs/train-k32.json` (sha256 `1a480a2792063cc65dd82534b6e8b113cc6bbd13ea95dd912138f1bb256a974c`): `target_seed` 2026112432, 12 public targets x 1 measured round(s), 1 warm-up, timeout 300 s per run. Holdout `target_seed` 2026112932 (reserved now, spec written at selection).

## Train (development screen)

Per size: `rho-neg`, `inc` and every candidate on 12 fresh public targets, 1 measured round(s) (an odd count cannot be balanced under either alternating order; ecbench warns, accepted). Audit: `ecbench verify --replay` on every session; a failed audit voids the session. The screen is diagnostic and in sample: `promotion_eligible=false`. A timed-out, failed or unverified arm is reported and is not selectable.

## Selection (disclosed in-sample rule)

Per size, the challenger `sel` is the candidate with the lowest mean S at w = 0 among arms whose every run verified. The best candidate at w = 1 is recorded as the Pareto alternative for the next round's panel. If no candidate has mean S at w = 0 below `inc` on the train targets, the size has no challenger and ends the round as `no_challenger` (no holdout is run).

## Holdout (confirmation)

Per size with a challenger: `rho-neg`, `inc`, `sel`, `inc-aa` (A/A copy of `inc`) and `rho-neg-aa`, on 24 fresh public targets x 2 measured rounds, 1 warm-up, order alternate, the reserved holdout `target_seed`. Audit as in train.

## Decision rule

Per size, from the holdout session:

- **improvement**: `ecbench compare --a inc --b sel` (paired, linearised-t interval over workloads) has `ops.status = ok`, is admissible, and its 95% interval lies below 1 at w = 0; AND `ecbench band --ranks` orders `sel < inc` on that curve (the order holds at w = 0, 0.1 and 1 alike). A/A: `inc-aa / inc` must contain 1 (deterministic counts give exactly 1).
- **regression**: the interval lies above 1 at w = 0, or the band orders `inc < sel`.
- **null**: anything else (interval spans 1, or the band is unresolved).
- Relabelling (a headline count falls while total cost rises) is caught by the band; accounting changes are not improvements. Distance to rho is `mean S(sel) / mean S(rho-neg)` at each weight, reported as the distance remaining.

## Ratchet

An improvement makes `sel` the incumbent of its track and size in `ledger.json`, with the paired ratio and interval as its delta, the session id, spec sha256 and audit receipt sha256 as its evidence, and `since_round` = this round. Every train and holdout arm joins the trial history (so it is never proposed again at that size); every target seed joins the exposures. A null or regression leaves the incumbent in place and is recorded as such.

## Route and scope

Route: `local`. Counted GAE, deterministic and host-independent; no wall-time claim. Isolation level earned is recorded per session and does not change the counts. This is a development loop: no round reaches the IC leaderboard or `benchmarks/README.md` without the validation gate (replay certificate on a second host class, validator report, matched rho with the band; `docs/VALIDATION_GATE.md`).

Exposures excluded: 16 target seeds already used on this family (ledger exposures); the seeds above are new.

## Amendment: selection (2026-11-19T07:34:13Z, after train, before holdout)

- k = 20: no candidate below `inc` (4.028 / 4.493 / 8.676) at w = 0 on the train targets; no challenger, no holdout.
- k = 24: selected `wsum-b12-n3` (`prime-abscissa:size=3` + `wsum:coef=12,free=1`), train mean S = 3.548 / 4.006 / 8.124 against `inc` 13.615 / 15.627 / 33.734; Pareto alternative at w = 1: `wsum-b12-n3`. Holdout spec `specs/holdout-k24.json`, `target_seed` 2026112924.
- k = 28: selected `wsum-b20-n3` (`prime-abscissa:size=3` + `wsum:coef=20,free=1`), train mean S = 2.944 / 3.302 / 6.525 against `inc` 19.796 / 23.012 / 51.951; Pareto alternative at w = 1: `wsum-b20-n3`. Holdout spec `specs/holdout-k28.json`, `target_seed` 2026112928.
- k = 32: selected `wsum-b20-n3` (`prime-abscissa:size=3` + `wsum:coef=20,free=1`), train mean S = 4.496 / 5.330 / 12.830 against `inc` 27.526 / 31.914 / 71.402; Pareto alternative at w = 1: `wsum-b20-n3`. Holdout spec `specs/holdout-k32.json`, `target_seed` 2026112932.

Holdout spec sha256: {"holdout-k24":"620ee1f1ba60496f01a93e9db62db6ac5c493c1ec2ac5d8a403fcbd01d76bf04","holdout-k28":"dc347eb91fac3c1c8e5045d1f3ac81cb825a4a63343cb81dd1fe59c1b357014e","holdout-k32":"1f155ab48f1bc1d5ad5e6640ea35ae4a44adb3e849f2a608ce4ce0f93e8db2ce"}
