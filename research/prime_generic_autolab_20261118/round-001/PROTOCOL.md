# ic_autolab round 1: protocol (multibase track)

Frozen 2026-11-19T07:18:34Z before any data of this round. This file is not edited after that; the selection step appends a dated amendment (after train, before holdout), and nothing else is added.

Loop home: `research/prime_generic_autolab_20261118` (`ledger.json` names the incumbents; `families.json` the candidate families). Success criterion: AGENTS.md, "Success criterion: iterative improvement of the index-calculus baseline"; tracks: AGENTS.md, "Tracks of index calculus". The gap to matched rho is reported as the distance remaining, never as a gate.

## Question

Does any candidate configuration of the multibase track lower the counted end-to-end cost S (GAE / sqrt r, every phase, every solve verified) below the incumbent on the same fresh public targets of the registered `prime-generic` curve at each size below, with the order holding across the AGENTS.md section 5a pricing band (w = 0, 0.1, 1 GAE per unpriced native operation)?

## Incumbents and candidates

Factor base and oracle strings are ecbench `ic.pipeline` parameters. Every candidate is a configuration never tried at its size in this loop (ledger trials).

### k = 20 (p = prime_search(20-bit, seed 59297), r = 562427, log2 r = 19.10, N_a = 52)

- `inc` (baseline): `prime-abscissa:size=45` + `mitm:negation_folded=1` (family `mitm2`, N = 45); S = 10.345 / 11.768 / 24.575 since round 0.
- `rho-neg` (reference): `rho.negation`, the matched rho.
- `mitm2-n38` (candidate): `prime-abscissa:size=38` + `mitm:negation_folded=1`; incumbent neighbourhood: N = 0.85 x 45.
- `mitm2-n54` (candidate): `prime-abscissa:size=54` + `mitm:negation_folded=1`; incumbent neighbourhood: N = 1.2 x 45.
- `ksum3f-n18` (candidate): `prime-abscissa:size=18` + `ksum:m=3,free=1`; exploration: family ksum3f untried at k = 20, N = 0.35 x N_a.
- `ksum4f-n8` (candidate): `prime-abscissa:size=8` + `ksum:m=4,free=1`; exploration: family ksum4f untried at k = 20, N = 0.15 x N_a.
- `wsum-b5-n3` (candidate): `prime-abscissa:size=3` + `wsum:coef=5,free=1`; exploration: family wsum-b5 untried at k = 20, N = 0.0001 x N_a.
- Train spec `specs/train-k20.json` (sha256 `111d468f637cd7559c445c117da228eefdb94952e6810266b58cee2af41ed813`): `target_seed` 2026111420, 12 public targets x 1 measured round(s), 1 warm-up, timeout 300 s per run. Holdout `target_seed` 2026111920 (reserved now, spec written at selection).

### k = 24 (p = prime_search(24-bit, seed 59297), r = 16482251, log2 r = 23.97, N_a = 160)

- `inc` (baseline): `prime-abscissa:size=160` + `mitm:negation_folded=1` (family `mitm2`, N = 160); S = 13.300 / 15.250 / 32.800 since round 0.
- `rho-neg` (reference): `rho.negation`, the matched rho.
- `mitm2-n136` (candidate): `prime-abscissa:size=136` + `mitm:negation_folded=1`; incumbent neighbourhood: N = 0.85 x 160.
- `mitm2-n192` (candidate): `prime-abscissa:size=192` + `mitm:negation_folded=1`; incumbent neighbourhood: N = 1.2 x 160.
- `ksum3f-n56` (candidate): `prime-abscissa:size=56` + `ksum:m=3,free=1`; exploration: family ksum3f untried at k = 24, N = 0.35 x N_a.
- `ksum4f-n24` (candidate): `prime-abscissa:size=24` + `ksum:m=4,free=1`; exploration: family ksum4f untried at k = 24, N = 0.15 x N_a.
- `wsum-b5-n3` (candidate): `prime-abscissa:size=3` + `wsum:coef=5,free=1`; exploration: family wsum-b5 untried at k = 24, N = 0.0001 x N_a.
- Train spec `specs/train-k24.json` (sha256 `96e80b42697ebc5621d72c4c6b8fe21f5e3299c90dd86d91285020148481f1dc`): `target_seed` 2026111424, 12 public targets x 1 measured round(s), 1 warm-up, timeout 300 s per run. Holdout `target_seed` 2026111924 (reserved now, spec written at selection).

### k = 28 (p = prime_search(28-bit, seed 59297), r = 260192759, log2 r = 27.96, N_a = 402)

- `inc` (baseline): `prime-abscissa:size=402` + `mitm:negation_folded=1` (family `mitm2`, N = 402); S = 20.640 / 23.749 / 51.724 since round 0.
- `rho-neg` (reference): `rho.negation`, the matched rho.
- `mitm2-n342` (candidate): `prime-abscissa:size=342` + `mitm:negation_folded=1`; incumbent neighbourhood: N = 0.85 x 402.
- `mitm2-n482` (candidate): `prime-abscissa:size=482` + `mitm:negation_folded=1`; incumbent neighbourhood: N = 1.2 x 402.
- `ksum3f-n141` (candidate): `prime-abscissa:size=141` + `ksum:m=3,free=1`; exploration: family ksum3f untried at k = 28, N = 0.35 x N_a.
- `ksum4f-n60` (candidate): `prime-abscissa:size=60` + `ksum:m=4,free=1`; exploration: family ksum4f untried at k = 28, N = 0.15 x N_a.
- `wsum-b5-n3` (candidate): `prime-abscissa:size=3` + `wsum:coef=5,free=1`; exploration: family wsum-b5 untried at k = 28, N = 0.0001 x N_a.
- Train spec `specs/train-k28.json` (sha256 `599d908c0c3109a9decd8e02d11ac74de3b54c1bc95b9cd083f86d6cb28fa726`): `target_seed` 2026111428, 12 public targets x 1 measured round(s), 1 warm-up, timeout 300 s per run. Holdout `target_seed` 2026111928 (reserved now, spec written at selection).

### k = 32 (p = prime_search(32-bit, seed 59297), r = 3466623553, log2 r = 31.69, N_a = 953)

- `inc` (baseline): `prime-abscissa:size=953` + `mitm:negation_folded=1` (family `mitm2`, N = 953); S = 31.270 / 35.981 / 78.379 since round 0.
- `rho-neg` (reference): `rho.negation`, the matched rho.
- `mitm2-n810` (candidate): `prime-abscissa:size=810` + `mitm:negation_folded=1`; incumbent neighbourhood: N = 0.85 x 953.
- `mitm2-n1144` (candidate): `prime-abscissa:size=1144` + `mitm:negation_folded=1`; incumbent neighbourhood: N = 1.2 x 953.
- `ksum3f-n334` (candidate): `prime-abscissa:size=334` + `ksum:m=3,free=1`; exploration: family ksum3f untried at k = 32, N = 0.35 x N_a.
- `ksum4f-n143` (candidate): `prime-abscissa:size=143` + `ksum:m=4,free=1`; exploration: family ksum4f untried at k = 32, N = 0.15 x N_a.
- `wsum-b5-n3` (candidate): `prime-abscissa:size=3` + `wsum:coef=5,free=1`; exploration: family wsum-b5 untried at k = 32, N = 0.0001 x N_a.
- Train spec `specs/train-k32.json` (sha256 `084906faa3c6d10c5a999e639cd4e2c7b5c506ffd6f65da3c1ad38e90a131304`): `target_seed` 2026111432, 12 public targets x 1 measured round(s), 1 warm-up, timeout 300 s per run. Holdout `target_seed` 2026111932 (reserved now, spec written at selection).

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

Exposures excluded: 8 target seeds already used on this family (ledger exposures); the seeds above are new.

## Amendment: selection (2026-11-19T07:31:51Z, after train, before holdout)

- k = 20: selected `wsum-b5-n3` (`prime-abscissa:size=3` + `wsum:coef=5,free=1`), train mean S = 4.258 / 4.747 / 9.148 against `inc` 9.047 / 10.254 / 21.115; Pareto alternative at w = 1: `wsum-b5-n3`. Holdout spec `specs/holdout-k20.json`, `target_seed` 2026111920.
- k = 24: selected `wsum-b5-n3` (`prime-abscissa:size=3` + `wsum:coef=5,free=1`), train mean S = 12.445 / 14.808 / 36.069 against `inc` 14.363 / 16.523 / 35.961; Pareto alternative at w = 1: `mitm2-n136`. Holdout spec `specs/holdout-k24.json`, `target_seed` 2026111924.
- k = 28: selected `mitm2-n342` (`prime-abscissa:size=342` + `mitm:negation_folded=1`), train mean S = 19.898 / 23.134 / 52.251 against `inc` 20.541 / 23.629 / 51.426; Pareto alternative at w = 1: `mitm2-n342`. Holdout spec `specs/holdout-k28.json`, `target_seed` 2026111928.
- k = 32: selected `mitm2-n810` (`prime-abscissa:size=810` + `mitm:negation_folded=1`), train mean S = 29.485 / 34.265 / 77.286 against `inc` 30.228 / 34.729 / 75.241; Pareto alternative at w = 1: `mitm2-n810`. Holdout spec `specs/holdout-k32.json`, `target_seed` 2026111932.

Holdout spec sha256: {"holdout-k20":"dc5ec642df2ed831b593024cf0fd2471cf1befa3059ea45710acf7d5efc48ada","holdout-k24":"b0b40e32b91c402d32cb9a30c4cd7bd0583681c29791fda6bd356f27844d5f51","holdout-k28":"d9f9c0f0f745e248f7a233cfb9e18c4866b243f6e0796c5477cc82748525444d","holdout-k32":"c3383270e1b1839ce34aae5f47c333b23e4082773743e3a773b0cf5f690ded44"}
