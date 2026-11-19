# ic_autolab round 6: protocol (multibase track)

Frozen 2026-11-19T08:43:46Z before any data of this round. This file is not edited after that; the selection step appends a dated amendment (after train, before holdout), and nothing else is added.

Loop home: `research/prime_generic_autolab_20261118` (`ledger.json` names the incumbents; `families.json` the candidate families). Success criterion: AGENTS.md, "Success criterion: iterative improvement of the index-calculus baseline"; tracks: AGENTS.md, "Tracks of index calculus". The gap to matched rho is reported as the distance remaining, never as a gate.

## Question

Does any candidate configuration of the multibase track lower the counted end-to-end cost S (GAE / sqrt r, every phase, every solve verified) below the incumbent on the same fresh public targets of the registered `prime-generic` curve at each size below, with the order holding across the AGENTS.md section 5a pricing band (w = 0, 0.1, 1 GAE per unpriced native operation)?

## Incumbents and candidates

Factor base and oracle strings are ecbench `ic.pipeline` parameters. Every candidate is a configuration never tried at its size in this loop (ledger trials).

### k = 28 (p = prime_search(28-bit, seed 59297), r = 260192759, log2 r = 27.96, N_a = 402)

- `inc` (baseline): `prime-abscissa:size=3` + `wsum:coef=20,free=1` (family `wsum-b20`, N = 3); S = 3.081 / 3.466 / 6.936 since round 2.
- `rho-neg` (reference): `rho.negation`, the matched rho.
- `wsum-b5-n4` (candidate): `prime-abscissa:size=4` + `wsum:coef=5,free=1`; cross-size transfer: wsum-b5-n3 is the k = 20 incumbent at N / N_a = 0.058.
- `wsum-b28-n3` (candidate): `prime-abscissa:size=3` + `wsum:coef=28,free=1`; exploration: family wsum-b28 untried at k = 28, N = 0.0001 x N_a.
- `wsum-b34-n3` (candidate): `prime-abscissa:size=3` + `wsum:coef=34,free=1`; exploration: family wsum-b34 untried at k = 28, N = 0.0001 x N_a.
- Train spec `specs/train-k28.json` (sha256 `bc7c937ff37ef86e4b12db579acef34a5b13ce52b289b703ccb009dd05ac8a19`): `target_seed` 2026116428, 12 public targets x 1 measured round(s), 1 warm-up, timeout 300 s per run. Holdout `target_seed` 2026116928 (reserved now, spec written at selection).

### k = 32 (p = prime_search(32-bit, seed 59297), r = 3466623553, log2 r = 31.69, N_a = 953)

- `inc` (baseline): `prime-abscissa:size=3` + `wsum:coef=20,free=1` (family `wsum-b20`, N = 3); S = 4.078 / 4.828 / 11.577 since round 2.
- `rho-neg` (reference): `rho.negation`, the matched rho.
- `wsum-b5-n4` (candidate): `prime-abscissa:size=4` + `wsum:coef=5,free=1`; cross-size transfer: wsum-b5-n3 is the k = 20 incumbent at N / N_a = 0.058.
- `wsum-b28-n3` (candidate): `prime-abscissa:size=3` + `wsum:coef=28,free=1`; exploration: family wsum-b28 untried at k = 32, N = 0.0001 x N_a.
- `wsum-b34-n3` (candidate): `prime-abscissa:size=3` + `wsum:coef=34,free=1`; exploration: family wsum-b34 untried at k = 32, N = 0.0001 x N_a.
- Train spec `specs/train-k32.json` (sha256 `8eafbf9b084e73081f006fe289d8d877e743b88f49c9883096a0b563fd9db114`): `target_seed` 2026116432, 12 public targets x 1 measured round(s), 1 warm-up, timeout 300 s per run. Holdout `target_seed` 2026116932 (reserved now, spec written at selection).

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

Exposures excluded: 42 target seeds already used on this family (ledger exposures); the seeds above are new.

## Amendment: selection (2026-11-19T08:44:05Z, after train, before holdout)

- k = 28: no candidate below `inc` (3.144 / 3.542 / 7.122) at w = 0 on the train targets; no challenger, no holdout.
- k = 32: selected `wsum-b28-n3` (`prime-abscissa:size=3` + `wsum:coef=28,free=1`), train mean S = 3.043 / 3.489 / 7.499 against `inc` 4.154 / 4.919 / 11.804; Pareto alternative at w = 1: `wsum-b28-n3`. Holdout spec `specs/holdout-k32.json`, `target_seed` 2026116932.

Holdout spec sha256: {"holdout-k32":"4259db4878f5ba7d188d24a936105c6ddabba6e60921c7e756da3b09cc927324"}
