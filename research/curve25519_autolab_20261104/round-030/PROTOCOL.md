# ic_autolab round 30: protocol (generic track)

Frozen 2026-11-04T10:00:06Z before any data of this round. This file is not edited after that; the selection step appends a dated amendment (after train, before holdout), and nothing else is added.

Loop home: `research/curve25519_autolab_20261104` (`ledger.json` names the incumbents; `families.json` the candidate families). Success criterion: AGENTS.md, "Success criterion: iterative improvement of the index-calculus baseline"; tracks: AGENTS.md, "Tracks of index calculus". The gap to matched rho is reported as the distance remaining, never as a gate.

## Question

Does any candidate configuration of the generic track lower the counted end-to-end cost S (GAE / sqrt r, every phase, every solve verified) below the incumbent on the same fresh public targets of the Curve25519-family miniature at each size below, with the order holding across the AGENTS.md section 5a pricing band (w = 0, 0.1, 1 GAE per unpriced native operation)?

## Incumbents and candidates

Factor base and oracle strings are ecbench `ic.pipeline` parameters. Every candidate is a configuration never tried at its size in this loop (ledger trials).

### k = 24 (p = 16777213, r = 2097349, log2 r = 21.00, N_a = 81)

- `inc` (baseline): `prime-projected-abscissa:size=2,scan_cap=32` + `wsum:coef=30,free=1` (family `wsum2g-b30`, N = 2); S = 2.584 / 2.711 / 3.855 since round 27.
- `rho-neg` (reference): `rho.negation`, the matched rho.
- `wsum2r-b30-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wsum:coef=30,free=1`; exploration: family wsum2r-b30 untried at k = 24, N = 0.0001 x N_a.
- `wsum2r-b45-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wsum:coef=45,free=1`; exploration: family wsum2r-b45 untried at k = 24, N = 0.0001 x N_a.
- `wsum2r-b60-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wsum:coef=60,free=1`; exploration: family wsum2r-b60 untried at k = 24, N = 0.0001 x N_a.
- `wsum2r-b90-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wsum:coef=90,free=1`; exploration: family wsum2r-b90 untried at k = 24, N = 0.0001 x N_a.
- `wsum2r-b120-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wsum:coef=120,free=1`; exploration: family wsum2r-b120 untried at k = 24, N = 0.0001 x N_a.
- `wsum2r-b160-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wsum:coef=160,free=1`; exploration: family wsum2r-b160 untried at k = 24, N = 0.0001 x N_a.
- Train spec `specs/train-k24.json` (sha256 `e1b515624b1cb49cb3b641017b5547109f3461dc598acabf92b8458a2fba19e5`): `target_seed` 2026140424, 32 public targets x 1 measured round(s), 1 warm-up, timeout 1800 s per run. Holdout `target_seed` 2026140924 (reserved now, spec written at selection).

### k = 28 (p = 268435157, r = 33557809, log2 r = 25.00, N_a = 203)

- `inc` (baseline): `prime-projected-abscissa:size=2,scan_cap=32` + `wsum:coef=60,free=1` (family `wsum2g-b60`, N = 2); S = 2.373 / 2.483 / 3.472 since round 27.
- `rho-neg` (reference): `rho.negation`, the matched rho.
- `wsum2r-b30-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wsum:coef=30,free=1`; exploration: family wsum2r-b30 untried at k = 28, N = 0.0001 x N_a.
- `wsum2r-b45-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wsum:coef=45,free=1`; exploration: family wsum2r-b45 untried at k = 28, N = 0.0001 x N_a.
- `wsum2r-b60-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wsum:coef=60,free=1`; exploration: family wsum2r-b60 untried at k = 28, N = 0.0001 x N_a.
- `wsum2r-b90-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wsum:coef=90,free=1`; exploration: family wsum2r-b90 untried at k = 28, N = 0.0001 x N_a.
- `wsum2r-b120-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wsum:coef=120,free=1`; exploration: family wsum2r-b120 untried at k = 28, N = 0.0001 x N_a.
- `wsum2r-b160-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wsum:coef=160,free=1`; exploration: family wsum2r-b160 untried at k = 28, N = 0.0001 x N_a.
- Train spec `specs/train-k28.json` (sha256 `8a2e08262d2328ddbdbe0446ad3142e6e4ab508531569c64e30f6a0e011246b5`): `target_seed` 2026140428, 32 public targets x 1 measured round(s), 1 warm-up, timeout 1800 s per run. Holdout `target_seed` 2026140928 (reserved now, spec written at selection).

### k = 32 (p = 4294967197, r = 536884639, log2 r = 29.00, N_a = 512)

- `inc` (baseline): `prime-projected-abscissa:size=2,scan_cap=32` + `wsum:coef=120,free=1` (family `wsum2-b120`, N = 2); S = 2.519 / 2.759 / 4.915 since round 22.
- `rho-neg` (reference): `rho.negation`, the matched rho.
- `wsum2r-b30-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wsum:coef=30,free=1`; exploration: family wsum2r-b30 untried at k = 32, N = 0.0001 x N_a.
- `wsum2r-b45-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wsum:coef=45,free=1`; exploration: family wsum2r-b45 untried at k = 32, N = 0.0001 x N_a.
- `wsum2r-b60-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wsum:coef=60,free=1`; exploration: family wsum2r-b60 untried at k = 32, N = 0.0001 x N_a.
- `wsum2r-b90-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wsum:coef=90,free=1`; exploration: family wsum2r-b90 untried at k = 32, N = 0.0001 x N_a.
- `wsum2r-b120-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wsum:coef=120,free=1`; exploration: family wsum2r-b120 untried at k = 32, N = 0.0001 x N_a.
- `wsum2r-b160-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wsum:coef=160,free=1`; exploration: family wsum2r-b160 untried at k = 32, N = 0.0001 x N_a.
- Train spec `specs/train-k32.json` (sha256 `b524b018d74c0f713d8431d7cbbac65d1c9837ed6a55e84596607c2b9b40e131`): `target_seed` 2026140432, 32 public targets x 1 measured round(s), 1 warm-up, timeout 1800 s per run. Holdout `target_seed` 2026140932 (reserved now, spec written at selection).

## Train (development screen)

Per size: `rho-neg`, `inc` and every candidate on 32 fresh public targets, 1 measured round(s) (an odd count cannot be balanced under either alternating order; ecbench warns, accepted). Audit: `ecbench verify --replay` on every session; a failed audit voids the session. The screen is diagnostic and in sample: `promotion_eligible=false`. A timed-out, failed or unverified arm is reported and is not selectable.

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

Exposures excluded: 83 target seeds already used on this family (ledger exposures); the seeds above are new.

## Amendment: selection (2026-11-04T10:01:24Z, after train, before holdout)

- k = 24: no candidate below `inc` (2.738 / 2.881 / 4.165) at w = 0 on the train targets; no challenger, no holdout.
- k = 28: no candidate below `inc` (2.277 / 2.377 / 3.279) at w = 0 on the train targets; no challenger, no holdout.
- k = 32: no candidate below `inc` (2.513 / 2.752 / 4.899) at w = 0 on the train targets; no challenger, no holdout.

Holdout spec sha256: {}
