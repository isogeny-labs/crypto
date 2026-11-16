# ic_autolab round 18: protocol (generic track)

Frozen 2026-11-04T06:46:59Z before any data of this round. This file is not edited after that; the selection step appends a dated amendment (after train, before holdout), and nothing else is added.

Loop home: `research/curve25519_autolab_20261104` (`ledger.json` names the incumbents; `families.json` the candidate families). Success criterion: AGENTS.md, "Success criterion: iterative improvement of the index-calculus baseline"; tracks: AGENTS.md, "Tracks of index calculus". The gap to matched rho is reported as the distance remaining, never as a gate.

## Question

Does any candidate configuration of the generic track lower the counted end-to-end cost S (GAE / sqrt r, every phase, every solve verified) below the incumbent on the same fresh public targets of the Curve25519-family miniature at each size below, with the order holding across the AGENTS.md section 5a pricing band (w = 0, 0.1, 1 GAE per unpriced native operation)?

## Incumbents and candidates

Factor base and oracle strings are ecbench `ic.pipeline` parameters. Every candidate is a configuration never tried at its size in this loop (ledger trials).

### k = 28 (p = 268435157, r = 33557809, log2 r = 25.00, N_a = 203)

- `inc` (baseline): `prime-projected-abscissa:size=18,scan_cap=288` + `ksum:m=4` (family `ksum4`, N = 18); S = 7.445 / 7.910 / 12.100 since round 8.
- `rho-neg` (reference): `rho.negation`, the matched rho.
- `wsum-b4-n4` (candidate): `prime-projected-abscissa:size=4,scan_cap=64` + `wsum:coef=4,free=1`; cross-size transfer: wsum-b4-n4 is the k = 24 incumbent at N / N_a = 0.049.
- `wsum-b5-n4` (candidate): `prime-projected-abscissa:size=4,scan_cap=64` + `wsum:coef=5,free=1`; cross-size transfer: wsum-b5-n3 is the k = 20 incumbent at N / N_a = 0.094.
- `ksum4f-n13` (candidate): `prime-projected-abscissa:size=13,scan_cap=208` + `ksum:m=4,free=1`; alternative ksum4f (best S(0) 6.675 at N = 15): N = 0.85 x 15.
- `wsum-b12-n3` (candidate): `prime-projected-abscissa:size=3,scan_cap=48` + `wsum:coef=12,free=1`; exploration: family wsum-b12 untried at k = 28, N = 0.0001 x N_a.
- `wsum-b20-n3` (candidate): `prime-projected-abscissa:size=3,scan_cap=48` + `wsum:coef=20,free=1`; exploration: family wsum-b20 untried at k = 28, N = 0.0001 x N_a.
- `wsum-b5-n3` (candidate): `prime-projected-abscissa:size=3,scan_cap=48` + `wsum:coef=5,free=1`; exploration: family wsum-b5 untried at k = 28, N = 0.0001 x N_a.
- Train spec `specs/train-k28.json` (sha256 `01e410619998c7466040008bc73249f54decdb98c454c3f17bcc8a659060b163`): `target_seed` 2026128428, 32 public targets x 1 measured round(s), 1 warm-up, timeout 1800 s per run. Holdout `target_seed` 2026128928 (reserved now, spec written at selection).

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

Exposures excluded: 55 target seeds already used on this family (ledger exposures); the seeds above are new.

## Amendment: selection (2026-11-04T06:47:16Z, after train, before holdout)

- k = 28: selected `wsum-b12-n3` (`prime-projected-abscissa:size=3,scan_cap=48` + `wsum:coef=12,free=1`), train mean S = 3.094 / 3.396 / 6.107 against `inc` 7.463 / 7.932 / 12.153; Pareto alternative at w = 1: `wsum-b12-n3`. Holdout spec `specs/holdout-k28.json`, `target_seed` 2026128928.

Holdout spec sha256: {"holdout-k28":"29a1dde7fbf1cc6a6b5a08dbfb37699116dfd0b3f967cbf4904b4129c165e6bc"}
