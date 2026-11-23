# ic_autolab round 2: protocol (generic track)

Frozen 2026-11-04T02:11:15Z before any data of this round. This file is not edited after that; the selection step appends a dated amendment (after train, before holdout), and nothing else is added.

Loop home: `research/curve25519_autolab_20261104` (`ledger.json` names the incumbents; `families.json` the candidate families). Success criterion: AGENTS.md, "Success criterion: iterative improvement of the index-calculus baseline"; tracks: AGENTS.md, "Tracks of index calculus". The gap to matched rho is reported as the distance remaining, never as a gate.

## Question

Does any candidate configuration of the generic track lower the counted end-to-end cost S (GAE / sqrt r, every phase, every solve verified) below the incumbent on the same fresh public targets of the Curve25519-family miniature at each size below, with the order holding across the AGENTS.md section 5a pricing band (w = 0, 0.1, 1 GAE per unpriced native operation)?

## Incumbents and candidates

Factor base and oracle strings are ecbench `ic.pipeline` parameters. Every candidate is a configuration never tried at its size in this loop (ledger trials).

### k = 20 (p = 1048573, r = 131203, log2 r = 17.00, N_a = 32)

- `inc` (baseline): `prime-projected-abscissa:size=11,scan_cap=176` + `ksum:m=3` (family `ksum3`, N = 11); S = 8.987 / 9.710 / 16.217 since round 0.
- `rho-neg` (reference): `rho.negation`, the matched rho.
- `mitm2-n45` (candidate): `prime-projected-abscissa:size=45,scan_cap=720` + `mitm:negation_folded=1`; alternative mitm2 (best S(0) 11.170 at N = 32): N = 1.4 x 32.
- `mitm2-n22` (candidate): `prime-projected-abscissa:size=22,scan_cap=352` + `mitm:negation_folded=1`; alternative mitm2 (best S(0) 11.170 at N = 32): N = 0.7 x 32.
- `ksum4-n5` (candidate): `prime-projected-abscissa:size=5,scan_cap=80` + `ksum:m=4`; exploration: family ksum4 untried at k = 20, N = 0.15 x N_a.
- `ksum5-n4` (candidate): `prime-projected-abscissa:size=4,scan_cap=64` + `ksum:m=5`; exploration: family ksum5 untried at k = 20, N = 0.08 x N_a.
- Train spec `specs/train-k20.json` (sha256 `215bfad4496c644d8a261737f4fb93293bdb872bdb5bd7698563135fe28d258a`): `target_seed` 2026112420, 16 public targets x 1 measured round(s), 1 warm-up, timeout 1800 s per run. Holdout `target_seed` 2026112920 (reserved now, spec written at selection).

### k = 24 (p = 16777213, r = 2097349, log2 r = 21.00, N_a = 81)

- `inc` (baseline): `prime-projected-abscissa:size=20,scan_cap=320` + `ksum:m=3` (family `ksum3`, N = 20); S = 7.554 / 8.173 / 13.745 since round 0.
- `rho-neg` (reference): `rho.negation`, the matched rho.
- `ksum4-n12` (candidate): `prime-projected-abscissa:size=12,scan_cap=192` + `ksum:m=4`; exploration: family ksum4 untried at k = 24, N = 0.15 x N_a.
- `ksum5-n6` (candidate): `prime-projected-abscissa:size=6,scan_cap=96` + `ksum:m=5`; exploration: family ksum5 untried at k = 24, N = 0.08 x N_a.
- Train spec `specs/train-k24.json` (sha256 `215c402769733d550a83382e18758ef9a44345572e3563988393bbead751e0bb`): `target_seed` 2026112424, 16 public targets x 1 measured round(s), 1 warm-up, timeout 1800 s per run. Holdout `target_seed` 2026112924 (reserved now, spec written at selection).

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

Exposures excluded: 25 target seeds already used on this family (ledger exposures); the seeds above are new.

## Amendment: selection (2026-11-04T02:32:01Z, after train, before holdout)

- k = 20: no candidate below `inc` (8.679 / 9.366 / 15.550) at w = 0 on the train targets; no challenger, no holdout.
- k = 24: selected `ksum4-n12` (`prime-projected-abscissa:size=12,scan_cap=192` + `ksum:m=4`), train mean S = 6.814 / 7.309 / 11.769 against `inc` 7.848 / 8.521 / 14.572; Pareto alternative at w = 1: `ksum4-n12`. Holdout spec `specs/holdout-k24.json`, `target_seed` 2026112924.

Holdout spec sha256: {"holdout-k24":"8cac8f91ebf047cd5d788812398f62b38a1fd9b058c4a8f967fa1c55d0349476"}
