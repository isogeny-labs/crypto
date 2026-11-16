# ic_autolab round 27: protocol (generic track)

Frozen 2026-11-04T08:32:31Z before any data of this round. This file is not edited after that; the selection step appends a dated amendment (after train, before holdout), and nothing else is added.

Loop home: `research/curve25519_autolab_20261104` (`ledger.json` names the incumbents; `families.json` the candidate families). Success criterion: AGENTS.md, "Success criterion: iterative improvement of the index-calculus baseline"; tracks: AGENTS.md, "Tracks of index calculus". The gap to matched rho is reported as the distance remaining, never as a gate.

## Question

Does any candidate configuration of the generic track lower the counted end-to-end cost S (GAE / sqrt r, every phase, every solve verified) below the incumbent on the same fresh public targets of the Curve25519-family miniature at each size below, with the order holding across the AGENTS.md section 5a pricing band (w = 0, 0.1, 1 GAE per unpriced native operation)?

## Incumbents and candidates

Factor base and oracle strings are ecbench `ic.pipeline` parameters. Every candidate is a configuration never tried at its size in this loop (ledger trials).

### k = 24 (p = 16777213, r = 2097349, log2 r = 21.00, N_a = 81)

- `inc` (baseline): `prime-projected-abscissa:size=2,scan_cap=32` + `wsum:coef=30,free=1` (family `wsum2-b30`, N = 2); S = 3.318 / 3.573 / 5.863 since round 24.
- `rho-neg` (reference): `rho.negation`, the matched rho.
- `wsum2-b160-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wsum:coef=160,free=1`; exploration: family wsum2-b160 untried at k = 24, N = 0.0001 x N_a.
- `wsum2g-b20-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wsum:coef=20,free=1`; exploration: family wsum2g-b20 untried at k = 24, N = 0.0001 x N_a.
- `wsum2g-b30-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wsum:coef=30,free=1`; exploration: family wsum2g-b30 untried at k = 24, N = 0.0001 x N_a.
- `wsum2g-b45-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wsum:coef=45,free=1`; exploration: family wsum2g-b45 untried at k = 24, N = 0.0001 x N_a.
- `wsum2g-b60-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wsum:coef=60,free=1`; exploration: family wsum2g-b60 untried at k = 24, N = 0.0001 x N_a.
- `wsum2g-b90-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wsum:coef=90,free=1`; exploration: family wsum2g-b90 untried at k = 24, N = 0.0001 x N_a.
- Train spec `specs/train-k24.json` (sha256 `d5ef83901da217dc6494fd0f92fb0ce2561245b4860cd3d2188839f0fbb299d1`): `target_seed` 2026137424, 32 public targets x 1 measured round(s), 1 warm-up, timeout 1800 s per run. Holdout `target_seed` 2026137924 (reserved now, spec written at selection).

### k = 28 (p = 268435157, r = 33557809, log2 r = 25.00, N_a = 203)

- `inc` (baseline): `prime-projected-abscissa:size=2,scan_cap=32` + `wsum:coef=60,free=1` (family `wsum2-b60`, N = 2); S = 2.576 / 2.793 / 4.746 since round 23.
- `rho-neg` (reference): `rho.negation`, the matched rho.
- `wsum2-b160-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wsum:coef=160,free=1`; exploration: family wsum2-b160 untried at k = 28, N = 0.0001 x N_a.
- `wsum2g-b20-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wsum:coef=20,free=1`; exploration: family wsum2g-b20 untried at k = 28, N = 0.0001 x N_a.
- `wsum2g-b30-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wsum:coef=30,free=1`; exploration: family wsum2g-b30 untried at k = 28, N = 0.0001 x N_a.
- `wsum2g-b45-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wsum:coef=45,free=1`; exploration: family wsum2g-b45 untried at k = 28, N = 0.0001 x N_a.
- `wsum2g-b60-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wsum:coef=60,free=1`; exploration: family wsum2g-b60 untried at k = 28, N = 0.0001 x N_a.
- `wsum2g-b90-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wsum:coef=90,free=1`; exploration: family wsum2g-b90 untried at k = 28, N = 0.0001 x N_a.
- Train spec `specs/train-k28.json` (sha256 `9ec4523e0cb83fd3bff41fba55765840544f429ba520fabc633236297226f862`): `target_seed` 2026137428, 32 public targets x 1 measured round(s), 1 warm-up, timeout 1800 s per run. Holdout `target_seed` 2026137928 (reserved now, spec written at selection).

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

Exposures excluded: 73 target seeds already used on this family (ledger exposures); the seeds above are new.

## Amendment: selection (2026-11-04T08:33:00Z, after train, before holdout)

- k = 24: selected `wsum2g-b30-n2` (`prime-projected-abscissa:size=2,scan_cap=32` + `wsum:coef=30,free=1`), train mean S = 2.672 / 2.808 / 4.031 against `inc` 3.085 / 3.296 / 5.201; Pareto alternative at w = 1: `wsum2g-b45-n2`. Holdout spec `specs/holdout-k24.json`, `target_seed` 2026137924.
- k = 28: selected `wsum2g-b60-n2` (`prime-projected-abscissa:size=2,scan_cap=32` + `wsum:coef=60,free=1`), train mean S = 2.274 / 2.374 / 3.273 against `inc` 2.644 / 2.874 / 4.946; Pareto alternative at w = 1: `wsum2g-b60-n2`. Holdout spec `specs/holdout-k28.json`, `target_seed` 2026137928.

Holdout spec sha256: {"holdout-k24":"bd869e322d7dee3f2370c7b460dfa4c600714b23fd3e455f56ffe6f30b4608ea","holdout-k28":"31737008489867ca6b47e791b06f1c5026bdce38015d54497721ef267cf86a6e"}
