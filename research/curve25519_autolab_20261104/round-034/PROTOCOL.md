# ic_autolab round 34: protocol (generic track)

Frozen 2026-11-04T12:14:18Z before any data of this round. This file is not edited after that; the selection step appends a dated amendment (after train, before holdout), and nothing else is added.

Loop home: `research/curve25519_autolab_20261104` (`ledger.json` names the incumbents; `families.json` the candidate families). Success criterion: AGENTS.md, "Success criterion: iterative improvement of the index-calculus baseline"; tracks: AGENTS.md, "Tracks of index calculus". The gap to matched rho is reported as the distance remaining, never as a gate.

## Question

Does any candidate configuration of the generic track lower the counted end-to-end cost S (GAE / sqrt r, every phase, every solve verified) below the incumbent on the same fresh public targets of the Curve25519-family miniature at each size below, with the order holding across the AGENTS.md section 5a pricing band (w = 0, 0.1, 1 GAE per unpriced native operation)?

## Incumbents and candidates

Factor base and oracle strings are ecbench `ic.pipeline` parameters. Every candidate is a configuration never tried at its size in this loop (ledger trials).

### k = 20 (p = 1048573, r = 131203, log2 r = 17.00, N_a = 32)

- `inc` (baseline): `prime-projected-abscissa:size=2,scan_cap=32` + `wsum:coef=15,free=1` (family `wsum2g-b15`, N = 2); S = 2.575 / 2.691 / 3.741 since round 28.
- `rho-neg` (reference): `rho.negation`, the matched rho.
- `wwalkG-x03-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.3,g=1,grow=2`; exploration: family wwalkG-x03 untried at k = 20, N = 0.0001 x N_a.
- `wwalkG-x04-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.4,g=1,grow=2`; exploration: family wwalkG-x04 untried at k = 20, N = 0.0001 x N_a.
- `wwalkG-x05-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.5,g=1,grow=2`; exploration: family wwalkG-x05 untried at k = 20, N = 0.0001 x N_a.
- `wwalkG-x06-j8-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.6,jumps=8,g=1,grow=2`; exploration: family wwalkG-x06-j8 untried at k = 20, N = 0.0001 x N_a.
- `wwalkG-x09-j8-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.9,jumps=8,g=1,grow=2`; exploration: family wwalkG-x09-j8 untried at k = 20, N = 0.0001 x N_a.
- `wwalkG-x06-j32-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.6,jumps=32,g=1,grow=2`; exploration: family wwalkG-x06-j32 untried at k = 20, N = 0.0001 x N_a.
- `wwalkG-x09-j32-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.9,jumps=32,g=1,grow=2`; exploration: family wwalkG-x09-j32 untried at k = 20, N = 0.0001 x N_a.
- `wwalkGg-x03-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.3,g=1,grow=2`; exploration: family wwalkGg-x03 untried at k = 20, N = 0.0001 x N_a.
- `wwalkGg-x04-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.4,g=1,grow=2`; exploration: family wwalkGg-x04 untried at k = 20, N = 0.0001 x N_a.
- `wwalkGg-x05-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.5,g=1,grow=2`; exploration: family wwalkGg-x05 untried at k = 20, N = 0.0001 x N_a.
- `wwalkGg-x06-j8-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.6,jumps=8,g=1,grow=2`; exploration: family wwalkGg-x06-j8 untried at k = 20, N = 0.0001 x N_a.
- `wwalkGg-x09-j8-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.9,jumps=8,g=1,grow=2`; exploration: family wwalkGg-x09-j8 untried at k = 20, N = 0.0001 x N_a.
- `wwalkGg-x06-j32-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.6,jumps=32,g=1,grow=2`; exploration: family wwalkGg-x06-j32 untried at k = 20, N = 0.0001 x N_a.
- `wwalkGg-x09-j32-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.9,jumps=32,g=1,grow=2`; exploration: family wwalkGg-x09-j32 untried at k = 20, N = 0.0001 x N_a.
- Train spec `specs/train-k20.json` (sha256 `233a16cf2165c58adb286f664a4a2421c8d6598b90e18453468b5137940ba2fb`): `target_seed` 2026144420, 32 public targets x 1 measured round(s), 1 warm-up, timeout 1800 s per run. Holdout `target_seed` 2026144920 (reserved now, spec written at selection).

### k = 24 (p = 16777213, r = 2097349, log2 r = 21.00, N_a = 81)

- `inc` (baseline): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.9,g=1,grow=2` (family `wwalkGg-x09`, N = 2); S = 1.673 / 1.718 / 2.117 since round 33.
- `rho-neg` (reference): `rho.negation`, the matched rho.
- `wwalkG-x03-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.3,g=1,grow=2`; exploration: family wwalkG-x03 untried at k = 24, N = 0.0001 x N_a.
- `wwalkG-x04-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.4,g=1,grow=2`; exploration: family wwalkG-x04 untried at k = 24, N = 0.0001 x N_a.
- `wwalkG-x05-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.5,g=1,grow=2`; exploration: family wwalkG-x05 untried at k = 24, N = 0.0001 x N_a.
- `wwalkG-x06-j8-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.6,jumps=8,g=1,grow=2`; exploration: family wwalkG-x06-j8 untried at k = 24, N = 0.0001 x N_a.
- `wwalkG-x09-j8-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.9,jumps=8,g=1,grow=2`; exploration: family wwalkG-x09-j8 untried at k = 24, N = 0.0001 x N_a.
- `wwalkG-x06-j32-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.6,jumps=32,g=1,grow=2`; exploration: family wwalkG-x06-j32 untried at k = 24, N = 0.0001 x N_a.
- `wwalkG-x09-j32-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.9,jumps=32,g=1,grow=2`; exploration: family wwalkG-x09-j32 untried at k = 24, N = 0.0001 x N_a.
- `wwalkGg-x03-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.3,g=1,grow=2`; exploration: family wwalkGg-x03 untried at k = 24, N = 0.0001 x N_a.
- `wwalkGg-x04-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.4,g=1,grow=2`; exploration: family wwalkGg-x04 untried at k = 24, N = 0.0001 x N_a.
- `wwalkGg-x05-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.5,g=1,grow=2`; exploration: family wwalkGg-x05 untried at k = 24, N = 0.0001 x N_a.
- `wwalkGg-x06-j8-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.6,jumps=8,g=1,grow=2`; exploration: family wwalkGg-x06-j8 untried at k = 24, N = 0.0001 x N_a.
- `wwalkGg-x09-j8-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.9,jumps=8,g=1,grow=2`; exploration: family wwalkGg-x09-j8 untried at k = 24, N = 0.0001 x N_a.
- `wwalkGg-x06-j32-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.6,jumps=32,g=1,grow=2`; exploration: family wwalkGg-x06-j32 untried at k = 24, N = 0.0001 x N_a.
- `wwalkGg-x09-j32-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.9,jumps=32,g=1,grow=2`; exploration: family wwalkGg-x09-j32 untried at k = 24, N = 0.0001 x N_a.
- Train spec `specs/train-k24.json` (sha256 `fceeceb06fc54b8114e728dd0b4b487a6896a2b777f512280b7c8fe9b6b0549b`): `target_seed` 2026144424, 32 public targets x 1 measured round(s), 1 warm-up, timeout 1800 s per run. Holdout `target_seed` 2026144924 (reserved now, spec written at selection).

### k = 28 (p = 268435157, r = 33557809, log2 r = 25.00, N_a = 203)

- `inc` (baseline): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.9,g=1,grow=2` (family `wwalkGg-x09`, N = 2); S = 1.572 / 1.611 / 1.961 since round 33.
- `rho-neg` (reference): `rho.negation`, the matched rho.
- `wwalkG-x03-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.3,g=1,grow=2`; exploration: family wwalkG-x03 untried at k = 28, N = 0.0001 x N_a.
- `wwalkG-x04-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.4,g=1,grow=2`; exploration: family wwalkG-x04 untried at k = 28, N = 0.0001 x N_a.
- `wwalkG-x05-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.5,g=1,grow=2`; exploration: family wwalkG-x05 untried at k = 28, N = 0.0001 x N_a.
- `wwalkG-x06-j8-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.6,jumps=8,g=1,grow=2`; exploration: family wwalkG-x06-j8 untried at k = 28, N = 0.0001 x N_a.
- `wwalkG-x09-j8-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.9,jumps=8,g=1,grow=2`; exploration: family wwalkG-x09-j8 untried at k = 28, N = 0.0001 x N_a.
- `wwalkG-x06-j32-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.6,jumps=32,g=1,grow=2`; exploration: family wwalkG-x06-j32 untried at k = 28, N = 0.0001 x N_a.
- `wwalkG-x09-j32-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.9,jumps=32,g=1,grow=2`; exploration: family wwalkG-x09-j32 untried at k = 28, N = 0.0001 x N_a.
- `wwalkGg-x03-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.3,g=1,grow=2`; exploration: family wwalkGg-x03 untried at k = 28, N = 0.0001 x N_a.
- `wwalkGg-x04-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.4,g=1,grow=2`; exploration: family wwalkGg-x04 untried at k = 28, N = 0.0001 x N_a.
- `wwalkGg-x05-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.5,g=1,grow=2`; exploration: family wwalkGg-x05 untried at k = 28, N = 0.0001 x N_a.
- `wwalkGg-x06-j8-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.6,jumps=8,g=1,grow=2`; exploration: family wwalkGg-x06-j8 untried at k = 28, N = 0.0001 x N_a.
- `wwalkGg-x09-j8-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.9,jumps=8,g=1,grow=2`; exploration: family wwalkGg-x09-j8 untried at k = 28, N = 0.0001 x N_a.
- `wwalkGg-x06-j32-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.6,jumps=32,g=1,grow=2`; exploration: family wwalkGg-x06-j32 untried at k = 28, N = 0.0001 x N_a.
- `wwalkGg-x09-j32-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.9,jumps=32,g=1,grow=2`; exploration: family wwalkGg-x09-j32 untried at k = 28, N = 0.0001 x N_a.
- Train spec `specs/train-k28.json` (sha256 `bd230cc687cfcb4008a78c707ca59b9c1f2c849b8c48f7f2e011fb30fbd47f02`): `target_seed` 2026144428, 32 public targets x 1 measured round(s), 1 warm-up, timeout 1800 s per run. Holdout `target_seed` 2026144928 (reserved now, spec written at selection).

### k = 32 (p = 4294967197, r = 536884639, log2 r = 29.00, N_a = 512)

- `inc` (baseline): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.6,g=1,grow=2` (family `wwalkG-x06`, N = 2); S = 1.515 / 1.625 / 2.608 since round 33.
- `rho-neg` (reference): `rho.negation`, the matched rho.
- `wwalkG-x03-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.3,g=1,grow=2`; exploration: family wwalkG-x03 untried at k = 32, N = 0.0001 x N_a.
- `wwalkG-x04-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.4,g=1,grow=2`; exploration: family wwalkG-x04 untried at k = 32, N = 0.0001 x N_a.
- `wwalkG-x05-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.5,g=1,grow=2`; exploration: family wwalkG-x05 untried at k = 32, N = 0.0001 x N_a.
- `wwalkG-x06-j8-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.6,jumps=8,g=1,grow=2`; exploration: family wwalkG-x06-j8 untried at k = 32, N = 0.0001 x N_a.
- `wwalkG-x09-j8-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.9,jumps=8,g=1,grow=2`; exploration: family wwalkG-x09-j8 untried at k = 32, N = 0.0001 x N_a.
- `wwalkG-x06-j32-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.6,jumps=32,g=1,grow=2`; exploration: family wwalkG-x06-j32 untried at k = 32, N = 0.0001 x N_a.
- `wwalkG-x09-j32-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.9,jumps=32,g=1,grow=2`; exploration: family wwalkG-x09-j32 untried at k = 32, N = 0.0001 x N_a.
- `wwalkGg-x03-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.3,g=1,grow=2`; exploration: family wwalkGg-x03 untried at k = 32, N = 0.0001 x N_a.
- `wwalkGg-x04-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.4,g=1,grow=2`; exploration: family wwalkGg-x04 untried at k = 32, N = 0.0001 x N_a.
- `wwalkGg-x05-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.5,g=1,grow=2`; exploration: family wwalkGg-x05 untried at k = 32, N = 0.0001 x N_a.
- `wwalkGg-x06-j8-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.6,jumps=8,g=1,grow=2`; exploration: family wwalkGg-x06-j8 untried at k = 32, N = 0.0001 x N_a.
- `wwalkGg-x09-j8-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.9,jumps=8,g=1,grow=2`; exploration: family wwalkGg-x09-j8 untried at k = 32, N = 0.0001 x N_a.
- `wwalkGg-x06-j32-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.6,jumps=32,g=1,grow=2`; exploration: family wwalkGg-x06-j32 untried at k = 32, N = 0.0001 x N_a.
- `wwalkGg-x09-j32-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.9,jumps=32,g=1,grow=2`; exploration: family wwalkGg-x09-j32 untried at k = 32, N = 0.0001 x N_a.
- Train spec `specs/train-k32.json` (sha256 `5f4f9ffb13c9f2786b8a2432b046beb6ffefd82c0e9dd349f210af622d4b0101`): `target_seed` 2026144432, 32 public targets x 1 measured round(s), 1 warm-up, timeout 1800 s per run. Holdout `target_seed` 2026144932 (reserved now, spec written at selection).

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

Exposures excluded: 108 target seeds already used on this family (ledger exposures); the seeds above are new.
