# ic_autolab round 33: protocol (generic track)

Frozen 2026-11-04T12:09:43Z before any data of this round. This file is not edited after that; the selection step appends a dated amendment (after train, before holdout), and nothing else is added.

Loop home: `research/curve25519_autolab_20261104` (`ledger.json` names the incumbents; `families.json` the candidate families). Success criterion: AGENTS.md, "Success criterion: iterative improvement of the index-calculus baseline"; tracks: AGENTS.md, "Tracks of index calculus". The gap to matched rho is reported as the distance remaining, never as a gate.

## Question

Does any candidate configuration of the generic track lower the counted end-to-end cost S (GAE / sqrt r, every phase, every solve verified) below the incumbent on the same fresh public targets of the Curve25519-family miniature at each size below, with the order holding across the AGENTS.md section 5a pricing band (w = 0, 0.1, 1 GAE per unpriced native operation)?

## Incumbents and candidates

Factor base and oracle strings are ecbench `ic.pipeline` parameters. Every candidate is a configuration never tried at its size in this loop (ledger trials).

### k = 20 (p = 1048573, r = 131203, log2 r = 17.00, N_a = 32)

- `inc` (baseline): `prime-projected-abscissa:size=2,scan_cap=32` + `wsum:coef=15,free=1` (family `wsum2g-b15`, N = 2); S = 2.575 / 2.691 / 3.741 since round 28.
- `rho-neg` (reference): `rho.negation`, the matched rho.
- `wwalkG-x06-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.6,g=1,grow=2`; exploration: family wwalkG-x06 untried at k = 20, N = 0.0001 x N_a.
- `wwalkGg-x06-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.6,g=1,grow=2`; exploration: family wwalkGg-x06 untried at k = 20, N = 0.0001 x N_a.
- `wwalkG-x07-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.7,g=1,grow=2`; exploration: family wwalkG-x07 untried at k = 20, N = 0.0001 x N_a.
- `wwalkGg-x07-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.7,g=1,grow=2`; exploration: family wwalkGg-x07 untried at k = 20, N = 0.0001 x N_a.
- `wwalkG-x08-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.8,g=1,grow=2`; exploration: family wwalkG-x08 untried at k = 20, N = 0.0001 x N_a.
- `wwalkGg-x08-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.8,g=1,grow=2`; exploration: family wwalkGg-x08 untried at k = 20, N = 0.0001 x N_a.
- `wwalkG-x09-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.9,g=1,grow=2`; exploration: family wwalkG-x09 untried at k = 20, N = 0.0001 x N_a.
- `wwalkGg-x09-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.9,g=1,grow=2`; exploration: family wwalkGg-x09 untried at k = 20, N = 0.0001 x N_a.
- `wwalkG-x10-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=1.0,g=1,grow=2`; exploration: family wwalkG-x10 untried at k = 20, N = 0.0001 x N_a.
- `wwalkGg-x10-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=1.0,g=1,grow=2`; exploration: family wwalkGg-x10 untried at k = 20, N = 0.0001 x N_a.
- `wwalkG-x12-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=1.2,g=1,grow=2`; exploration: family wwalkG-x12 untried at k = 20, N = 0.0001 x N_a.
- `wwalkGg-x12-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=1.2,g=1,grow=2`; exploration: family wwalkGg-x12 untried at k = 20, N = 0.0001 x N_a.
- Train spec `specs/train-k20.json` (sha256 `960aacdcd61fe8ca8168654af2b4dd2be3680faea2301c6db3c65cf36356af80`): `target_seed` 2026143420, 32 public targets x 1 measured round(s), 1 warm-up, timeout 1800 s per run. Holdout `target_seed` 2026143920 (reserved now, spec written at selection).

### k = 24 (p = 16777213, r = 2097349, log2 r = 21.00, N_a = 81)

- `inc` (baseline): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.8,grow=1` (family `wwalkg-x08-grow`, N = 2); S = 2.164 / 2.243 / 2.955 since round 31.
- `rho-neg` (reference): `rho.negation`, the matched rho.
- `wwalkG-x06-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.6,g=1,grow=2`; exploration: family wwalkG-x06 untried at k = 24, N = 0.0001 x N_a.
- `wwalkGg-x06-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.6,g=1,grow=2`; exploration: family wwalkGg-x06 untried at k = 24, N = 0.0001 x N_a.
- `wwalkG-x07-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.7,g=1,grow=2`; exploration: family wwalkG-x07 untried at k = 24, N = 0.0001 x N_a.
- `wwalkGg-x07-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.7,g=1,grow=2`; exploration: family wwalkGg-x07 untried at k = 24, N = 0.0001 x N_a.
- `wwalkG-x08-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.8,g=1,grow=2`; exploration: family wwalkG-x08 untried at k = 24, N = 0.0001 x N_a.
- `wwalkGg-x08-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.8,g=1,grow=2`; exploration: family wwalkGg-x08 untried at k = 24, N = 0.0001 x N_a.
- `wwalkG-x09-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.9,g=1,grow=2`; exploration: family wwalkG-x09 untried at k = 24, N = 0.0001 x N_a.
- `wwalkGg-x09-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.9,g=1,grow=2`; exploration: family wwalkGg-x09 untried at k = 24, N = 0.0001 x N_a.
- `wwalkG-x10-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=1.0,g=1,grow=2`; exploration: family wwalkG-x10 untried at k = 24, N = 0.0001 x N_a.
- `wwalkGg-x10-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=1.0,g=1,grow=2`; exploration: family wwalkGg-x10 untried at k = 24, N = 0.0001 x N_a.
- `wwalkG-x12-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=1.2,g=1,grow=2`; exploration: family wwalkG-x12 untried at k = 24, N = 0.0001 x N_a.
- `wwalkGg-x12-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=1.2,g=1,grow=2`; exploration: family wwalkGg-x12 untried at k = 24, N = 0.0001 x N_a.
- Train spec `specs/train-k24.json` (sha256 `90cb69df90753a6b3c79de3836d3c1cc0cc1fd8a696ee4133c0ee7e24d1a0fb3`): `target_seed` 2026143424, 32 public targets x 1 measured round(s), 1 warm-up, timeout 1800 s per run. Holdout `target_seed` 2026143924 (reserved now, spec written at selection).

### k = 28 (p = 268435157, r = 33557809, log2 r = 25.00, N_a = 203)

- `inc` (baseline): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.8` (family `wwalkg-x08`, N = 2); S = 1.921 / 2.029 / 3.001 since round 31.
- `rho-neg` (reference): `rho.negation`, the matched rho.
- `wwalkG-x06-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.6,g=1,grow=2`; exploration: family wwalkG-x06 untried at k = 28, N = 0.0001 x N_a.
- `wwalkGg-x06-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.6,g=1,grow=2`; exploration: family wwalkGg-x06 untried at k = 28, N = 0.0001 x N_a.
- `wwalkG-x07-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.7,g=1,grow=2`; exploration: family wwalkG-x07 untried at k = 28, N = 0.0001 x N_a.
- `wwalkGg-x07-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.7,g=1,grow=2`; exploration: family wwalkGg-x07 untried at k = 28, N = 0.0001 x N_a.
- `wwalkG-x08-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.8,g=1,grow=2`; exploration: family wwalkG-x08 untried at k = 28, N = 0.0001 x N_a.
- `wwalkGg-x08-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.8,g=1,grow=2`; exploration: family wwalkGg-x08 untried at k = 28, N = 0.0001 x N_a.
- `wwalkG-x09-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.9,g=1,grow=2`; exploration: family wwalkG-x09 untried at k = 28, N = 0.0001 x N_a.
- `wwalkGg-x09-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.9,g=1,grow=2`; exploration: family wwalkGg-x09 untried at k = 28, N = 0.0001 x N_a.
- `wwalkG-x10-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=1.0,g=1,grow=2`; exploration: family wwalkG-x10 untried at k = 28, N = 0.0001 x N_a.
- `wwalkGg-x10-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=1.0,g=1,grow=2`; exploration: family wwalkGg-x10 untried at k = 28, N = 0.0001 x N_a.
- `wwalkG-x12-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=1.2,g=1,grow=2`; exploration: family wwalkG-x12 untried at k = 28, N = 0.0001 x N_a.
- `wwalkGg-x12-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=1.2,g=1,grow=2`; exploration: family wwalkGg-x12 untried at k = 28, N = 0.0001 x N_a.
- Train spec `specs/train-k28.json` (sha256 `5dca8d74fc86a7ce64407243b97ece246e57ca552230b949c7e6d4a43544c21a`): `target_seed` 2026143428, 32 public targets x 1 measured round(s), 1 warm-up, timeout 1800 s per run. Holdout `target_seed` 2026143928 (reserved now, spec written at selection).

### k = 32 (p = 4294967197, r = 536884639, log2 r = 29.00, N_a = 512)

- `inc` (baseline): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=1.0` (family `wwalk-x10`, N = 2); S = 2.110 / 2.316 / 4.174 since round 31.
- `rho-neg` (reference): `rho.negation`, the matched rho.
- `wwalkG-x06-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.6,g=1,grow=2`; exploration: family wwalkG-x06 untried at k = 32, N = 0.0001 x N_a.
- `wwalkGg-x06-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.6,g=1,grow=2`; exploration: family wwalkGg-x06 untried at k = 32, N = 0.0001 x N_a.
- `wwalkG-x07-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.7,g=1,grow=2`; exploration: family wwalkG-x07 untried at k = 32, N = 0.0001 x N_a.
- `wwalkGg-x07-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.7,g=1,grow=2`; exploration: family wwalkGg-x07 untried at k = 32, N = 0.0001 x N_a.
- `wwalkG-x08-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.8,g=1,grow=2`; exploration: family wwalkG-x08 untried at k = 32, N = 0.0001 x N_a.
- `wwalkGg-x08-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.8,g=1,grow=2`; exploration: family wwalkGg-x08 untried at k = 32, N = 0.0001 x N_a.
- `wwalkG-x09-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.9,g=1,grow=2`; exploration: family wwalkG-x09 untried at k = 32, N = 0.0001 x N_a.
- `wwalkGg-x09-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.9,g=1,grow=2`; exploration: family wwalkGg-x09 untried at k = 32, N = 0.0001 x N_a.
- `wwalkG-x10-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=1.0,g=1,grow=2`; exploration: family wwalkG-x10 untried at k = 32, N = 0.0001 x N_a.
- `wwalkGg-x10-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=1.0,g=1,grow=2`; exploration: family wwalkGg-x10 untried at k = 32, N = 0.0001 x N_a.
- `wwalkG-x12-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=1.2,g=1,grow=2`; exploration: family wwalkG-x12 untried at k = 32, N = 0.0001 x N_a.
- `wwalkGg-x12-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=1.2,g=1,grow=2`; exploration: family wwalkGg-x12 untried at k = 32, N = 0.0001 x N_a.
- Train spec `specs/train-k32.json` (sha256 `d5e319ada51e040409b43006a71823c6a7e59983b1d36168b5bac8a726b82ccd`): `target_seed` 2026143432, 32 public targets x 1 measured round(s), 1 warm-up, timeout 1800 s per run. Holdout `target_seed` 2026143932 (reserved now, spec written at selection).

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

Exposures excluded: 101 target seeds already used on this family (ledger exposures); the seeds above are new.

## Amendment: selection (2026-11-04T12:11:05Z, after train, before holdout)

- k = 20: no candidate below `inc` (2.541 / 2.654 / 3.673) at w = 0 on the train targets; no challenger, no holdout.
- k = 24: selected `wwalkGg-x09-n2` (`prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.9,g=1,grow=2`), train mean S = 1.685 / 1.730 / 2.140 against `inc` 2.327 / 2.422 / 3.281; Pareto alternative at w = 1: `wwalkGg-x09-n2`. Holdout spec `specs/holdout-k24.json`, `target_seed` 2026143924.
- k = 28: selected `wwalkGg-x09-n2` (`prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.9,g=1,grow=2`), train mean S = 1.644 / 1.690 / 2.106 against `inc` 2.287 / 2.431 / 3.732; Pareto alternative at w = 1: `wwalkGg-x09-n2`. Holdout spec `specs/holdout-k28.json`, `target_seed` 2026143928.
- k = 32: selected `wwalkG-x06-n2` (`prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.6,g=1,grow=2`), train mean S = 1.336 / 1.409 / 2.071 against `inc` 1.983 / 2.164 / 3.794; Pareto alternative at w = 1: `wwalkGg-x06-n2`. Holdout spec `specs/holdout-k32.json`, `target_seed` 2026143932.

Holdout spec sha256: {"holdout-k24":"eaa6e01a15267db316c1e5183a39b8f495a25fcadd7dea3473de9087c0a99a23","holdout-k28":"b27a7024ee9edb111b8eeb44b8666f95eb96603f117e4bd219978fedd3406c7a","holdout-k32":"37dda3e9acbc9cc3d12f69e2a9ccec8b16d13de7c6243aacad645ddea30bbfce"}
