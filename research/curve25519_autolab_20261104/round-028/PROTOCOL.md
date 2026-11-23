# ic_autolab round 28: protocol (generic track)

Frozen 2026-11-04T08:36:02Z before any data of this round. This file is not edited after that; the selection step appends a dated amendment (after train, before holdout), and nothing else is added.

Loop home: `research/curve25519_autolab_20261104` (`ledger.json` names the incumbents; `families.json` the candidate families). Success criterion: AGENTS.md, "Success criterion: iterative improvement of the index-calculus baseline"; tracks: AGENTS.md, "Tracks of index calculus". The gap to matched rho is reported as the distance remaining, never as a gate.

## Question

Does any candidate configuration of the generic track lower the counted end-to-end cost S (GAE / sqrt r, every phase, every solve verified) below the incumbent on the same fresh public targets of the Curve25519-family miniature at each size below, with the order holding across the AGENTS.md section 5a pricing band (w = 0, 0.1, 1 GAE per unpriced native operation)?

## Incumbents and candidates

Factor base and oracle strings are ecbench `ic.pipeline` parameters. Every candidate is a configuration never tried at its size in this loop (ledger trials).

### k = 20 (p = 1048573, r = 131203, log2 r = 17.00, N_a = 32)

- `inc` (baseline): `prime-projected-abscissa:size=2,scan_cap=32` + `wsum:coef=15,free=1` (family `wsum2-b15`, N = 2); S = 4.860 / 5.083 / 7.092 since round 25.
- `rho-neg` (reference): `rho.negation`, the matched rho.
- `wsum2g-b30-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wsum:coef=30,free=1`; cross-size transfer: wsum2g-b30-n2 is the k = 24 incumbent at N / N_a = 0.025.
- `wsum2g-b60-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wsum:coef=60,free=1`; cross-size transfer: wsum2g-b60-n2 is the k = 28 incumbent at N / N_a = 0.010.
- `wsum2g-b120-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wsum:coef=120,free=1`; exploration: family wsum2g-b120 untried at k = 20, N = 0.0001 x N_a.
- `wsum2g-b10-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wsum:coef=10,free=1`; exploration: family wsum2g-b10 untried at k = 20, N = 0.0001 x N_a.
- `wsum2g-b15-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wsum:coef=15,free=1`; exploration: family wsum2g-b15 untried at k = 20, N = 0.0001 x N_a.
- `wsum2g-b160-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wsum:coef=160,free=1`; exploration: family wsum2g-b160 untried at k = 20, N = 0.0001 x N_a.
- `wsum2g-b180-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wsum:coef=180,free=1`; exploration: family wsum2g-b180 untried at k = 20, N = 0.0001 x N_a.
- `wsum2-b45-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wsum:coef=45,free=1`; exploration: family wsum2-b45 untried at k = 20, N = 0.0001 x N_a.
- Train spec `specs/train-k20.json` (sha256 `1ed843cb5176d977189882b2b7a9c4f574ccd60ba1a2e60b2eff45b650f75395`): `target_seed` 2026138420, 32 public targets x 1 measured round(s), 1 warm-up, timeout 1800 s per run. Holdout `target_seed` 2026138920 (reserved now, spec written at selection).

### k = 32 (p = 4294967197, r = 536884639, log2 r = 29.00, N_a = 512)

- `inc` (baseline): `prime-projected-abscissa:size=2,scan_cap=32` + `wsum:coef=120,free=1` (family `wsum2-b120`, N = 2); S = 2.519 / 2.759 / 4.915 since round 22.
- `rho-neg` (reference): `rho.negation`, the matched rho.
- `wsum2g-b60-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wsum:coef=60,free=1`; cross-size transfer: wsum2g-b60-n2 is the k = 28 incumbent at N / N_a = 0.010.
- `wsum2g-b30-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wsum:coef=30,free=1`; cross-size transfer: wsum2g-b30-n2 is the k = 24 incumbent at N / N_a = 0.025.
- `wsum2g-b120-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wsum:coef=120,free=1`; exploration: family wsum2g-b120 untried at k = 32, N = 0.0001 x N_a.
- `wsum2g-b10-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wsum:coef=10,free=1`; exploration: family wsum2g-b10 untried at k = 32, N = 0.0001 x N_a.
- `wsum2g-b15-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wsum:coef=15,free=1`; exploration: family wsum2g-b15 untried at k = 32, N = 0.0001 x N_a.
- `wsum2g-b160-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wsum:coef=160,free=1`; exploration: family wsum2g-b160 untried at k = 32, N = 0.0001 x N_a.
- `wsum2g-b180-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wsum:coef=180,free=1`; exploration: family wsum2g-b180 untried at k = 32, N = 0.0001 x N_a.
- `wsum2-b20-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wsum:coef=20,free=1`; exploration: family wsum2-b20 untried at k = 32, N = 0.0001 x N_a.
- Train spec `specs/train-k32.json` (sha256 `3e8bfc6580263b03097616c6cf19f444431cbad685e4f49fa51ca11ac98dee6e`): `target_seed` 2026138432, 32 public targets x 1 measured round(s), 1 warm-up, timeout 1800 s per run. Holdout `target_seed` 2026138932 (reserved now, spec written at selection).

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

Exposures excluded: 77 target seeds already used on this family (ledger exposures); the seeds above are new.

## Amendment: selection (2026-11-04T08:38:21Z, after train, before holdout)

- k = 20: selected `wsum2g-b15-n2` (`prime-projected-abscissa:size=2,scan_cap=32` + `wsum:coef=15,free=1`), train mean S = 2.553 / 2.667 / 3.698 against `inc` 5.009 / 5.263 / 7.552; Pareto alternative at w = 1: `wsum2g-b15-n2`. Holdout spec `specs/holdout-k20.json`, `target_seed` 2026138920.
- k = 32: no candidate below `inc` (2.561 / 2.809 / 5.041) at w = 0 on the train targets; no challenger, no holdout.

Holdout spec sha256: {"holdout-k20":"31d58ece51b9f5adce749866dc1cf1c79dab8fe17509eec999e62b3cae0a5242"}
