# ic_autolab round 32: protocol (generic track)

Frozen 2026-11-04T11:20:39Z before any data of this round. This file is not edited after that; the selection step appends a dated amendment (after train, before holdout), and nothing else is added.

Loop home: `research/curve25519_autolab_20261104` (`ledger.json` names the incumbents; `families.json` the candidate families). Success criterion: AGENTS.md, "Success criterion: iterative improvement of the index-calculus baseline"; tracks: AGENTS.md, "Tracks of index calculus". The gap to matched rho is reported as the distance remaining, never as a gate.

## Question

Does any candidate configuration of the generic track lower the counted end-to-end cost S (GAE / sqrt r, every phase, every solve verified) below the incumbent on the same fresh public targets of the Curve25519-family miniature at each size below, with the order holding across the AGENTS.md section 5a pricing band (w = 0, 0.1, 1 GAE per unpriced native operation)?

## Incumbents and candidates

Factor base and oracle strings are ecbench `ic.pipeline` parameters. Every candidate is a configuration never tried at its size in this loop (ledger trials).

### k = 20 (p = 1048573, r = 131203, log2 r = 17.00, N_a = 32)

- `inc` (baseline): `prime-projected-abscissa:size=2,scan_cap=32` + `wsum:coef=15,free=1` (family `wsum2g-b15`, N = 2); S = 2.575 / 2.691 / 3.741 since round 28.
- `rho-neg` (reference): `rho.negation`, the matched rho.
- `wwalk-x05-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.5`; exploration: family wwalk-x05 untried at k = 20, N = 0.0001 x N_a.
- `wwalkg-x05-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.5`; exploration: family wwalkg-x05 untried at k = 20, N = 0.0001 x N_a.
- `wwalk-x05-grow-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.5,grow=1`; exploration: family wwalk-x05-grow untried at k = 20, N = 0.0001 x N_a.
- `wwalkg-x05-grow-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.5,grow=1`; exploration: family wwalkg-x05-grow untried at k = 20, N = 0.0001 x N_a.
- `wwalk-x06-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.6`; exploration: family wwalk-x06 untried at k = 20, N = 0.0001 x N_a.
- `wwalkg-x06-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.6`; exploration: family wwalkg-x06 untried at k = 20, N = 0.0001 x N_a.
- `wwalk-x06-grow-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.6,grow=1`; exploration: family wwalk-x06-grow untried at k = 20, N = 0.0001 x N_a.
- `wwalkg-x06-grow-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.6,grow=1`; exploration: family wwalkg-x06-grow untried at k = 20, N = 0.0001 x N_a.
- `wwalk-x07-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.7`; exploration: family wwalk-x07 untried at k = 20, N = 0.0001 x N_a.
- `wwalkg-x07-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.7`; exploration: family wwalkg-x07 untried at k = 20, N = 0.0001 x N_a.
- `wwalk-x07-grow-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.7,grow=1`; exploration: family wwalk-x07-grow untried at k = 20, N = 0.0001 x N_a.
- `wwalkg-x07-grow-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.7,grow=1`; exploration: family wwalkg-x07-grow untried at k = 20, N = 0.0001 x N_a.
- `wwalk-x09-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.9`; exploration: family wwalk-x09 untried at k = 20, N = 0.0001 x N_a.
- `wwalkg-x09-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.9`; exploration: family wwalkg-x09 untried at k = 20, N = 0.0001 x N_a.
- `wwalk-x09-grow-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.9,grow=1`; exploration: family wwalk-x09-grow untried at k = 20, N = 0.0001 x N_a.
- `wwalkg-x09-grow-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.9,grow=1`; exploration: family wwalkg-x09-grow untried at k = 20, N = 0.0001 x N_a.
- Train spec `specs/train-k20.json` (sha256 `5861837ccc37d8cf72392b3d5935acd3f95c3d2da255f51e0cf9d0e7959d1ad9`): `target_seed` 2026142420, 32 public targets x 1 measured round(s), 1 warm-up, timeout 1800 s per run. Holdout `target_seed` 2026142920 (reserved now, spec written at selection).

### k = 24 (p = 16777213, r = 2097349, log2 r = 21.00, N_a = 81)

- `inc` (baseline): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.8,grow=1` (family `wwalkg-x08-grow`, N = 2); S = 2.164 / 2.243 / 2.955 since round 31.
- `rho-neg` (reference): `rho.negation`, the matched rho.
- `wwalk-x05-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.5`; exploration: family wwalk-x05 untried at k = 24, N = 0.0001 x N_a.
- `wwalkg-x05-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.5`; exploration: family wwalkg-x05 untried at k = 24, N = 0.0001 x N_a.
- `wwalk-x05-grow-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.5,grow=1`; exploration: family wwalk-x05-grow untried at k = 24, N = 0.0001 x N_a.
- `wwalkg-x05-grow-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.5,grow=1`; exploration: family wwalkg-x05-grow untried at k = 24, N = 0.0001 x N_a.
- `wwalk-x06-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.6`; exploration: family wwalk-x06 untried at k = 24, N = 0.0001 x N_a.
- `wwalkg-x06-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.6`; exploration: family wwalkg-x06 untried at k = 24, N = 0.0001 x N_a.
- `wwalk-x06-grow-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.6,grow=1`; exploration: family wwalk-x06-grow untried at k = 24, N = 0.0001 x N_a.
- `wwalkg-x06-grow-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.6,grow=1`; exploration: family wwalkg-x06-grow untried at k = 24, N = 0.0001 x N_a.
- `wwalk-x07-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.7`; exploration: family wwalk-x07 untried at k = 24, N = 0.0001 x N_a.
- `wwalkg-x07-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.7`; exploration: family wwalkg-x07 untried at k = 24, N = 0.0001 x N_a.
- `wwalk-x07-grow-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.7,grow=1`; exploration: family wwalk-x07-grow untried at k = 24, N = 0.0001 x N_a.
- `wwalkg-x07-grow-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.7,grow=1`; exploration: family wwalkg-x07-grow untried at k = 24, N = 0.0001 x N_a.
- `wwalk-x09-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.9`; exploration: family wwalk-x09 untried at k = 24, N = 0.0001 x N_a.
- `wwalkg-x09-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.9`; exploration: family wwalkg-x09 untried at k = 24, N = 0.0001 x N_a.
- `wwalk-x09-grow-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.9,grow=1`; exploration: family wwalk-x09-grow untried at k = 24, N = 0.0001 x N_a.
- `wwalkg-x09-grow-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.9,grow=1`; exploration: family wwalkg-x09-grow untried at k = 24, N = 0.0001 x N_a.
- Train spec `specs/train-k24.json` (sha256 `f444cc5ba7b1ae268c5bcf729d762dcf1016c1f1bb461c733d288f27453f8c2e`): `target_seed` 2026142424, 32 public targets x 1 measured round(s), 1 warm-up, timeout 1800 s per run. Holdout `target_seed` 2026142924 (reserved now, spec written at selection).

### k = 28 (p = 268435157, r = 33557809, log2 r = 25.00, N_a = 203)

- `inc` (baseline): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.8` (family `wwalkg-x08`, N = 2); S = 1.921 / 2.029 / 3.001 since round 31.
- `rho-neg` (reference): `rho.negation`, the matched rho.
- `wwalk-x05-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.5`; exploration: family wwalk-x05 untried at k = 28, N = 0.0001 x N_a.
- `wwalkg-x05-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.5`; exploration: family wwalkg-x05 untried at k = 28, N = 0.0001 x N_a.
- `wwalk-x05-grow-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.5,grow=1`; exploration: family wwalk-x05-grow untried at k = 28, N = 0.0001 x N_a.
- `wwalkg-x05-grow-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.5,grow=1`; exploration: family wwalkg-x05-grow untried at k = 28, N = 0.0001 x N_a.
- `wwalk-x06-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.6`; exploration: family wwalk-x06 untried at k = 28, N = 0.0001 x N_a.
- `wwalkg-x06-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.6`; exploration: family wwalkg-x06 untried at k = 28, N = 0.0001 x N_a.
- `wwalk-x06-grow-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.6,grow=1`; exploration: family wwalk-x06-grow untried at k = 28, N = 0.0001 x N_a.
- `wwalkg-x06-grow-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.6,grow=1`; exploration: family wwalkg-x06-grow untried at k = 28, N = 0.0001 x N_a.
- `wwalk-x07-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.7`; exploration: family wwalk-x07 untried at k = 28, N = 0.0001 x N_a.
- `wwalkg-x07-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.7`; exploration: family wwalkg-x07 untried at k = 28, N = 0.0001 x N_a.
- `wwalk-x07-grow-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.7,grow=1`; exploration: family wwalk-x07-grow untried at k = 28, N = 0.0001 x N_a.
- `wwalkg-x07-grow-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.7,grow=1`; exploration: family wwalkg-x07-grow untried at k = 28, N = 0.0001 x N_a.
- `wwalk-x09-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.9`; exploration: family wwalk-x09 untried at k = 28, N = 0.0001 x N_a.
- `wwalkg-x09-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.9`; exploration: family wwalkg-x09 untried at k = 28, N = 0.0001 x N_a.
- `wwalk-x09-grow-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.9,grow=1`; exploration: family wwalk-x09-grow untried at k = 28, N = 0.0001 x N_a.
- `wwalkg-x09-grow-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.9,grow=1`; exploration: family wwalkg-x09-grow untried at k = 28, N = 0.0001 x N_a.
- Train spec `specs/train-k28.json` (sha256 `0f2d96dfbe5e61919574c2b91c8d154220c70e299cbfcd24583d9f4286c8b675`): `target_seed` 2026142428, 32 public targets x 1 measured round(s), 1 warm-up, timeout 1800 s per run. Holdout `target_seed` 2026142928 (reserved now, spec written at selection).

### k = 32 (p = 4294967197, r = 536884639, log2 r = 29.00, N_a = 512)

- `inc` (baseline): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=1.0` (family `wwalk-x10`, N = 2); S = 2.110 / 2.316 / 4.174 since round 31.
- `rho-neg` (reference): `rho.negation`, the matched rho.
- `wwalk-x05-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.5`; exploration: family wwalk-x05 untried at k = 32, N = 0.0001 x N_a.
- `wwalkg-x05-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.5`; exploration: family wwalkg-x05 untried at k = 32, N = 0.0001 x N_a.
- `wwalk-x05-grow-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.5,grow=1`; exploration: family wwalk-x05-grow untried at k = 32, N = 0.0001 x N_a.
- `wwalkg-x05-grow-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.5,grow=1`; exploration: family wwalkg-x05-grow untried at k = 32, N = 0.0001 x N_a.
- `wwalk-x06-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.6`; exploration: family wwalk-x06 untried at k = 32, N = 0.0001 x N_a.
- `wwalkg-x06-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.6`; exploration: family wwalkg-x06 untried at k = 32, N = 0.0001 x N_a.
- `wwalk-x06-grow-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.6,grow=1`; exploration: family wwalk-x06-grow untried at k = 32, N = 0.0001 x N_a.
- `wwalkg-x06-grow-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.6,grow=1`; exploration: family wwalkg-x06-grow untried at k = 32, N = 0.0001 x N_a.
- `wwalk-x07-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.7`; exploration: family wwalk-x07 untried at k = 32, N = 0.0001 x N_a.
- `wwalkg-x07-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.7`; exploration: family wwalkg-x07 untried at k = 32, N = 0.0001 x N_a.
- `wwalk-x07-grow-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.7,grow=1`; exploration: family wwalk-x07-grow untried at k = 32, N = 0.0001 x N_a.
- `wwalkg-x07-grow-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.7,grow=1`; exploration: family wwalkg-x07-grow untried at k = 32, N = 0.0001 x N_a.
- `wwalk-x09-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.9`; exploration: family wwalk-x09 untried at k = 32, N = 0.0001 x N_a.
- `wwalkg-x09-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.9`; exploration: family wwalkg-x09 untried at k = 32, N = 0.0001 x N_a.
- `wwalk-x09-grow-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.9,grow=1`; exploration: family wwalk-x09-grow untried at k = 32, N = 0.0001 x N_a.
- `wwalkg-x09-grow-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.9,grow=1`; exploration: family wwalkg-x09-grow untried at k = 32, N = 0.0001 x N_a.
- Train spec `specs/train-k32.json` (sha256 `ea6e76749ac6c0c17353209df759bd52d4f79998589c667df049d7fc70337318`): `target_seed` 2026142432, 32 public targets x 1 measured round(s), 1 warm-up, timeout 1800 s per run. Holdout `target_seed` 2026142932 (reserved now, spec written at selection).

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

Exposures excluded: 94 target seeds already used on this family (ledger exposures); the seeds above are new.
