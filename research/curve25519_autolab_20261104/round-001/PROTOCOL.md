# ic_autolab round 1: protocol (generic track)

Frozen 2026-11-04T01:43:53Z before any data of this round. This file is not edited after that; the selection step appends a dated amendment (after train, before holdout), and nothing else is added.

Loop home: `research/curve25519_autolab_20261104` (`ledger.json` names the incumbents; `families.json` the candidate families). Success criterion: AGENTS.md, "Success criterion: iterative improvement of the index-calculus baseline"; tracks: AGENTS.md, "Tracks of index calculus". The gap to matched rho is reported as the distance remaining, never as a gate.

## Question

Does any candidate configuration of the generic track lower the counted end-to-end cost S (GAE / sqrt r, every phase, every solve verified) below the incumbent on the same fresh public targets of the Curve25519-family miniature at each size below, with the order holding across the AGENTS.md section 5a pricing band (w = 0, 0.1, 1 GAE per unpriced native operation)?

## Incumbents and candidates

Factor base and oracle strings are ecbench `ic.pipeline` parameters. Every candidate is a configuration never tried at its size in this loop (ledger trials).

### k = 20 (p = 1048573, r = 131203, log2 r = 17.00, N_a = 32)

- `inc` (baseline): `prime-projected-abscissa:size=11,scan_cap=176` + `ksum:m=3` (family `ksum3`, N = 11); S = 8.987 / 9.710 / 16.217 since round 0.
- `rho-neg` (reference): `rho.negation`, the matched rho.
- `ksum3-n15` (candidate): `prime-projected-abscissa:size=15,scan_cap=240` + `ksum:m=3`; incumbent neighbourhood: N = 1.4 x 11.
- `ksum3-n8` (candidate): `prime-projected-abscissa:size=8,scan_cap=128` + `ksum:m=3`; incumbent neighbourhood: N = 0.7 x 11.
- `ksum3-n6` (candidate): `prime-projected-abscissa:size=6,scan_cap=96` + `ksum:m=3`; incumbent neighbourhood: N = 0.5 x 11.
- `ksum3-n22` (candidate): `prime-projected-abscissa:size=22,scan_cap=352` + `ksum:m=3`; incumbent neighbourhood: N = 2 x 11.
- `mitm2-n32` (candidate): `prime-projected-abscissa:size=32,scan_cap=512` + `mitm:negation_folded=1`; exploration: family mitm2 untried at k = 20, N = 1 x N_a.
- `mitm3-n16` (candidate): `prime-projected-abscissa:size=16,scan_cap=256` + `mitm:m=3,negation_folded=1`; exploration: family mitm3 untried at k = 20, N = 0.5 x N_a.
- Train spec `specs/train-k20.json` (sha256 `028bdec8ad447cc26c3d803e36986140be5f5fd7994ff0bf3b1237a44122bb0f`): `target_seed` 2026111420, 16 public targets x 1 measured round(s), 1 warm-up, timeout 1800 s per run. Holdout `target_seed` 2026111920 (reserved now, spec written at selection).

### k = 24 (p = 16777213, r = 2097349, log2 r = 21.00, N_a = 81)

- `inc` (baseline): `prime-projected-abscissa:size=20,scan_cap=320` + `ksum:m=3` (family `ksum3`, N = 20); S = 7.554 / 8.173 / 13.745 since round 0.
- `rho-neg` (reference): `rho.negation`, the matched rho.
- `ksum3-n28` (candidate): `prime-projected-abscissa:size=28,scan_cap=448` + `ksum:m=3`; incumbent neighbourhood: N = 1.4 x 20.
- `ksum3-n14` (candidate): `prime-projected-abscissa:size=14,scan_cap=224` + `ksum:m=3`; incumbent neighbourhood: N = 0.7 x 20.
- `ksum3-n10` (candidate): `prime-projected-abscissa:size=10,scan_cap=160` + `ksum:m=3`; incumbent neighbourhood: N = 0.5 x 20.
- `ksum3-n40` (candidate): `prime-projected-abscissa:size=40,scan_cap=640` + `ksum:m=3`; incumbent neighbourhood: N = 2 x 20.
- `mitm2-n81` (candidate): `prime-projected-abscissa:size=81,scan_cap=1296` + `mitm:negation_folded=1`; exploration: family mitm2 untried at k = 24, N = 1 x N_a.
- `mitm3-n41` (candidate): `prime-projected-abscissa:size=41,scan_cap=656` + `mitm:m=3,negation_folded=1`; exploration: family mitm3 untried at k = 24, N = 0.5 x N_a.
- Train spec `specs/train-k24.json` (sha256 `6b11e83ef1c9c8e8074cdf0abcb7d130d692289f27456c4df66ac417cf465359`): `target_seed` 2026111424, 16 public targets x 1 measured round(s), 1 warm-up, timeout 1800 s per run. Holdout `target_seed` 2026111924 (reserved now, spec written at selection).

## Train (development screen)

Per size: `rho-neg`, `inc` and every candidate on 16 fresh public targets, 1 measured round(s) (an odd count cannot be balanced under either alternating order; ecbench warns, accepted). Audit: `ecbench verify --replay` on every session; a failed audit voids the session. The screen is diagnostic and in sample: `promotion_eligible=false`. A timed-out, failed or unverified arm is reported and is not selectable.

## Selection (disclosed in-sample rule)

Per size, the challenger `sel` is the candidate with the lowest mean S at w = 0 among arms whose every run verified. The best candidate at w = 1 is recorded as the Pareto alternative for the next round's panel. If no candidate has mean S at w = 0 below `inc` on the train targets, the size has no challenger and ends the round as `no_challenger` (no holdout is run).

## Holdout (confirmation)

Per size with a challenger: `rho-neg`, `inc`, `sel`, `inc-aa` (A/A copy of `inc`) and `rho-neg-aa`, on 32 fresh public targets x 2 measured rounds, 1 warm-up, order alternate, the reserved holdout `target_seed`. Audit as in train.

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

Exposures excluded: 23 target seeds already used on this family (ledger exposures); the seeds above are new.

## Amendment: selection (2026-11-04T02:09:34Z, after train, before holdout)

- k = 20: no candidate below `inc` (8.998 / 9.734 / 16.356) at w = 0 on the train targets; no challenger, no holdout.
- k = 24: no candidate below `inc` (8.053 / 8.760 / 15.129) at w = 0 on the train targets; no challenger, no holdout.

Holdout spec sha256: {}
