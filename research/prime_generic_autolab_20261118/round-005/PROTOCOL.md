# ic_autolab round 5: protocol (generic track)

Frozen 2026-11-19T08:20:20Z before any data of this round. This file is not edited after that; the selection step appends a dated amendment (after train, before holdout), and nothing else is added.

Loop home: `research/prime_generic_autolab_20261118` (`ledger.json` names the incumbents; `families.json` the candidate families). Success criterion: AGENTS.md, "Success criterion: iterative improvement of the index-calculus baseline"; tracks: AGENTS.md, "Tracks of index calculus". The gap to matched rho is reported as the distance remaining, never as a gate.

## Question

Does any candidate configuration of the generic track lower the counted end-to-end cost S (GAE / sqrt r, every phase, every solve verified) below the incumbent on the same fresh public targets of the registered `prime-generic` curve at each size below, with the order holding across the AGENTS.md section 5a pricing band (w = 0, 0.1, 1 GAE per unpriced native operation)?

## Incumbents and candidates

Factor base and oracle strings are ecbench `ic.pipeline` parameters. Every candidate is a configuration never tried at its size in this loop (ledger trials).

### k = 20 (p = prime_search(20-bit, seed 59297), r = 562427, log2 r = 19.10, N_a = 52)

- `inc` (baseline): `prime-abscissa:size=2` + `wwalk:x=2.0,jumps=16,g=1,grow=2,tw=2,cj=1,lazy=1,seed=run` (family `tw2l-x20-j16`, N = 2); S = 2.123 / 2.305 / 3.943 since round 4.
- `rho-neg` (reference): `rho.negation`, the matched rho.
- `wsum-b8-n3` (candidate): `prime-abscissa:size=3` + `wsum:coef=8,free=1`; exploration: family wsum-b8 untried at k = 20, N = 0.0001 x N_a.
- `wsum-b12-n3` (candidate): `prime-abscissa:size=3` + `wsum:coef=12,free=1`; exploration: family wsum-b12 untried at k = 20, N = 0.0001 x N_a.
- `tw2l-x15-j16-n2` (candidate): `prime-abscissa:size=2` + `wwalk:x=1.5,jumps=16,g=1,grow=2,tw=2,cj=1,lazy=1,seed=run`; exploration: family tw2l-x15-j16 untried at k = 20, N = 0.0001 x N_a.
- Train spec `specs/train-k20.json` (sha256 `af8f666765ab87479e22589de92381a307a9699c2599ccc23967381050e58674`): `target_seed` 2026115420, 12 public targets x 1 measured round(s), 1 warm-up, timeout 300 s per run. Holdout `target_seed` 2026115920 (reserved now, spec written at selection).

### k = 24 (p = prime_search(24-bit, seed 59297), r = 16482251, log2 r = 23.97, N_a = 160)

- `inc` (baseline): `prime-abscissa:size=2` + `wwalk:x=0.9,g=1,grow=2,seed=run` (family `wwalkGg-x09-sr`, N = 2); S = 1.946 / 2.128 / 3.771 since round 4.
- `rho-neg` (reference): `rho.negation`, the matched rho.
- `wsum-b8-n3` (candidate): `prime-abscissa:size=3` + `wsum:coef=8,free=1`; exploration: family wsum-b8 untried at k = 24, N = 0.0001 x N_a.
- `wsum-b12-n3` (candidate): `prime-abscissa:size=3` + `wsum:coef=12,free=1`; exploration: family wsum-b12 untried at k = 24, N = 0.0001 x N_a.
- `tw2l-x15-j16-n2` (candidate): `prime-abscissa:size=2` + `wwalk:x=1.5,jumps=16,g=1,grow=2,tw=2,cj=1,lazy=1,seed=run`; exploration: family tw2l-x15-j16 untried at k = 24, N = 0.0001 x N_a.
- Train spec `specs/train-k24.json` (sha256 `7d00e747c3831e329ac1c7893c8ee34f9d7e1224bdf38c832e3f26ae21c592d8`): `target_seed` 2026115424, 12 public targets x 1 measured round(s), 1 warm-up, timeout 300 s per run. Holdout `target_seed` 2026115924 (reserved now, spec written at selection).

### k = 28 (p = prime_search(28-bit, seed 59297), r = 260192759, log2 r = 27.96, N_a = 402)

- `inc` (baseline): `prime-abscissa:size=2` + `wwalk:x=2.0,jumps=16,g=1,grow=2,tw=2,cj=1,lazy=1,seed=run` (family `tw2l-x20-j16`, N = 2); S = 1.726 / 1.897 / 3.432 since round 4.
- `rho-neg` (reference): `rho.negation`, the matched rho.
- `wsum-b8-n3` (candidate): `prime-abscissa:size=3` + `wsum:coef=8,free=1`; exploration: family wsum-b8 untried at k = 28, N = 0.0001 x N_a.
- `wsum-b12-n3` (candidate): `prime-abscissa:size=3` + `wsum:coef=12,free=1`; exploration: family wsum-b12 untried at k = 28, N = 0.0001 x N_a.
- `tw2l-x15-j16-n2` (candidate): `prime-abscissa:size=2` + `wwalk:x=1.5,jumps=16,g=1,grow=2,tw=2,cj=1,lazy=1,seed=run`; exploration: family tw2l-x15-j16 untried at k = 28, N = 0.0001 x N_a.
- Train spec `specs/train-k28.json` (sha256 `09a77be3df69cab74b4ab170313358da5415b0dfa95d337d903fbcdd8576e2a7`): `target_seed` 2026115428, 12 public targets x 1 measured round(s), 1 warm-up, timeout 300 s per run. Holdout `target_seed` 2026115928 (reserved now, spec written at selection).

### k = 32 (p = prime_search(32-bit, seed 59297), r = 3466623553, log2 r = 31.69, N_a = 953)

- `inc` (baseline): `prime-abscissa:size=2` + `wwalk:x=2.0,jumps=16,g=1,grow=2,tw=2,cj=1,lazy=1,seed=run` (family `tw2l-x20-j16`, N = 2); S = 1.833 / 2.015 / 3.659 since round 4.
- `rho-neg` (reference): `rho.negation`, the matched rho.
- `wsum-b8-n3` (candidate): `prime-abscissa:size=3` + `wsum:coef=8,free=1`; exploration: family wsum-b8 untried at k = 32, N = 0.0001 x N_a.
- `wsum-b12-n3` (candidate): `prime-abscissa:size=3` + `wsum:coef=12,free=1`; exploration: family wsum-b12 untried at k = 32, N = 0.0001 x N_a.
- `tw2l-x15-j16-n2` (candidate): `prime-abscissa:size=2` + `wwalk:x=1.5,jumps=16,g=1,grow=2,tw=2,cj=1,lazy=1,seed=run`; exploration: family tw2l-x15-j16 untried at k = 32, N = 0.0001 x N_a.
- Train spec `specs/train-k32.json` (sha256 `bd20cf595e16b449b90e57338f848eedce34ca122d592ef1319cf9f28f4e8ee2`): `target_seed` 2026115432, 12 public targets x 1 measured round(s), 1 warm-up, timeout 300 s per run. Holdout `target_seed` 2026115932 (reserved now, spec written at selection).

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

Exposures excluded: 35 target seeds already used on this family (ledger exposures); the seeds above are new.

## Amendment: selection (2026-11-19T08:20:57Z, after train, before holdout)

- k = 20: selected `tw2l-x15-j16-n2` (`prime-abscissa:size=2` + `wwalk:x=1.5,jumps=16,g=1,grow=2,tw=2,cj=1,lazy=1,seed=run`), train mean S = 1.915 / 2.077 / 3.534 against `inc` 2.055 / 2.231 / 3.811; Pareto alternative at w = 1: `tw2l-x15-j16-n2`. Holdout spec `specs/holdout-k20.json`, `target_seed` 2026115920.
- k = 24: selected `tw2l-x15-j16-n2` (`prime-abscissa:size=2` + `wwalk:x=1.5,jumps=16,g=1,grow=2,tw=2,cj=1,lazy=1,seed=run`), train mean S = 1.775 / 1.945 / 3.480 against `inc` 2.081 / 2.276 / 4.038; Pareto alternative at w = 1: `tw2l-x15-j16-n2`. Holdout spec `specs/holdout-k24.json`, `target_seed` 2026115924.
- k = 28: no candidate below `inc` (1.652 / 1.815 / 3.282) at w = 0 on the train targets; no challenger, no holdout.
- k = 32: selected `tw2l-x15-j16-n2` (`prime-abscissa:size=2` + `wwalk:x=1.5,jumps=16,g=1,grow=2,tw=2,cj=1,lazy=1,seed=run`), train mean S = 1.856 / 2.041 / 3.706 against `inc` 1.868 / 2.054 / 3.730; Pareto alternative at w = 1: `tw2l-x15-j16-n2`. Holdout spec `specs/holdout-k32.json`, `target_seed` 2026115932.

Holdout spec sha256: {"holdout-k20":"f049a07d51f3844b1234e0afb2ea53ac3725d0e491bb26540d8c8cbc64bd7aad","holdout-k24":"8b468758006f60624a09dfc70f974d267d59a9496bd81fc5797ed8de57d286f9","holdout-k32":"0564798c247ad6543b325cfef60bf91c608db0160fe95a07ec4e0bb4bbbccc61"}
