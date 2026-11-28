# ic_autolab round 4: protocol (generic track)

Frozen 2026-11-19T08:08:20Z before any data of this round. This file is not edited after that; the selection step appends a dated amendment (after train, before holdout), and nothing else is added.

Loop home: `research/prime_generic_autolab_20261118` (`ledger.json` names the incumbents; `families.json` the candidate families). Success criterion: AGENTS.md, "Success criterion: iterative improvement of the index-calculus baseline"; tracks: AGENTS.md, "Tracks of index calculus". The gap to matched rho is reported as the distance remaining, never as a gate.

## Question

Does any candidate configuration of the generic track lower the counted end-to-end cost S (GAE / sqrt r, every phase, every solve verified) below the incumbent on the same fresh public targets of the registered `prime-generic` curve at each size below, with the order holding across the AGENTS.md section 5a pricing band (w = 0, 0.1, 1 GAE per unpriced native operation)?

## Incumbents and candidates

Factor base and oracle strings are ecbench `ic.pipeline` parameters. Every candidate is a configuration never tried at its size in this loop (ledger trials).

### k = 20 (p = prime_search(20-bit, seed 59297), r = 562427, log2 r = 19.10, N_a = 52)

- `inc` (baseline): `prime-abscissa:size=45` + `mitm:negation_folded=1` (family `mitm2`, N = 45); S = 10.345 / 11.768 / 24.575 since round 0.
- `rho-neg` (reference): `rho.negation`, the matched rho.
- `mitm2-n38` (candidate): `prime-abscissa:size=38` + `mitm:negation_folded=1`; incumbent neighbourhood: N = 0.85 x 45.
- `mitm2-n54` (candidate): `prime-abscissa:size=54` + `mitm:negation_folded=1`; incumbent neighbourhood: N = 1.2 x 45.
- `tw2l-x20-j16-n2` (candidate): `prime-abscissa:size=2` + `wwalk:x=2.0,jumps=16,g=1,grow=2,tw=2,cj=1,lazy=1,seed=run`; exploration: family tw2l-x20-j16 untried at k = 20, N = 0.0001 x N_a.
- `wwalkGg-x09-sr-n2` (candidate): `prime-abscissa:size=2` + `wwalk:x=0.9,g=1,grow=2,seed=run`; exploration: family wwalkGg-x09-sr untried at k = 20, N = 0.0001 x N_a.
- `ksum4f-n8` (candidate): `prime-abscissa:size=8` + `ksum:m=4,free=1`; exploration: family ksum4f untried at k = 20, N = 0.15 x N_a.
- Train spec `specs/train-k20.json` (sha256 `50216dd59ad5e8fb2fd242da40651f9718d51b7cfb5a2e9e37d4ea86814496ea`): `target_seed` 2026114420, 12 public targets x 1 measured round(s), 1 warm-up, timeout 300 s per run. Holdout `target_seed` 2026114920 (reserved now, spec written at selection).

### k = 24 (p = prime_search(24-bit, seed 59297), r = 16482251, log2 r = 23.97, N_a = 160)

- `inc` (baseline): `prime-abscissa:size=160` + `mitm:negation_folded=1` (family `mitm2`, N = 160); S = 13.300 / 15.250 / 32.800 since round 0.
- `rho-neg` (reference): `rho.negation`, the matched rho.
- `mitm2-n136` (candidate): `prime-abscissa:size=136` + `mitm:negation_folded=1`; incumbent neighbourhood: N = 0.85 x 160.
- `mitm2-n192` (candidate): `prime-abscissa:size=192` + `mitm:negation_folded=1`; incumbent neighbourhood: N = 1.2 x 160.
- `tw2l-x20-j16-n2` (candidate): `prime-abscissa:size=2` + `wwalk:x=2.0,jumps=16,g=1,grow=2,tw=2,cj=1,lazy=1,seed=run`; exploration: family tw2l-x20-j16 untried at k = 24, N = 0.0001 x N_a.
- `wwalkGg-x09-sr-n2` (candidate): `prime-abscissa:size=2` + `wwalk:x=0.9,g=1,grow=2,seed=run`; exploration: family wwalkGg-x09-sr untried at k = 24, N = 0.0001 x N_a.
- `ksum4f-n24` (candidate): `prime-abscissa:size=24` + `ksum:m=4,free=1`; exploration: family ksum4f untried at k = 24, N = 0.15 x N_a.
- Train spec `specs/train-k24.json` (sha256 `708348eb247164e9fa6729cec8aa56c25890af14b6ee1526a7378b1797ce12bd`): `target_seed` 2026114424, 12 public targets x 1 measured round(s), 1 warm-up, timeout 300 s per run. Holdout `target_seed` 2026114924 (reserved now, spec written at selection).

### k = 28 (p = prime_search(28-bit, seed 59297), r = 260192759, log2 r = 27.96, N_a = 402)

- `inc` (baseline): `prime-abscissa:size=402` + `mitm:negation_folded=1` (family `mitm2`, N = 402); S = 20.640 / 23.749 / 51.724 since round 0.
- `rho-neg` (reference): `rho.negation`, the matched rho.
- `mitm2-n342` (candidate): `prime-abscissa:size=342` + `mitm:negation_folded=1`; incumbent neighbourhood: N = 0.85 x 402.
- `mitm2-n482` (candidate): `prime-abscissa:size=482` + `mitm:negation_folded=1`; incumbent neighbourhood: N = 1.2 x 402.
- `tw2l-x20-j16-n2` (candidate): `prime-abscissa:size=2` + `wwalk:x=2.0,jumps=16,g=1,grow=2,tw=2,cj=1,lazy=1,seed=run`; exploration: family tw2l-x20-j16 untried at k = 28, N = 0.0001 x N_a.
- `wwalkGg-x09-sr-n2` (candidate): `prime-abscissa:size=2` + `wwalk:x=0.9,g=1,grow=2,seed=run`; exploration: family wwalkGg-x09-sr untried at k = 28, N = 0.0001 x N_a.
- `ksum4f-n60` (candidate): `prime-abscissa:size=60` + `ksum:m=4,free=1`; exploration: family ksum4f untried at k = 28, N = 0.15 x N_a.
- Train spec `specs/train-k28.json` (sha256 `e2c0be732c0cbcca2da9a67fc0af6209d20dde5a74cc6cd7e42b0cab3512347d`): `target_seed` 2026114428, 12 public targets x 1 measured round(s), 1 warm-up, timeout 300 s per run. Holdout `target_seed` 2026114928 (reserved now, spec written at selection).

### k = 32 (p = prime_search(32-bit, seed 59297), r = 3466623553, log2 r = 31.69, N_a = 953)

- `inc` (baseline): `prime-abscissa:size=953` + `mitm:negation_folded=1` (family `mitm2`, N = 953); S = 31.270 / 35.981 / 78.379 since round 0.
- `rho-neg` (reference): `rho.negation`, the matched rho.
- `mitm2-n810` (candidate): `prime-abscissa:size=810` + `mitm:negation_folded=1`; incumbent neighbourhood: N = 0.85 x 953.
- `mitm2-n1144` (candidate): `prime-abscissa:size=1144` + `mitm:negation_folded=1`; incumbent neighbourhood: N = 1.2 x 953.
- `tw2l-x20-j16-n2` (candidate): `prime-abscissa:size=2` + `wwalk:x=2.0,jumps=16,g=1,grow=2,tw=2,cj=1,lazy=1,seed=run`; exploration: family tw2l-x20-j16 untried at k = 32, N = 0.0001 x N_a.
- `wwalkGg-x09-sr-n2` (candidate): `prime-abscissa:size=2` + `wwalk:x=0.9,g=1,grow=2,seed=run`; exploration: family wwalkGg-x09-sr untried at k = 32, N = 0.0001 x N_a.
- `ksum4f-n143` (candidate): `prime-abscissa:size=143` + `ksum:m=4,free=1`; exploration: family ksum4f untried at k = 32, N = 0.15 x N_a.
- Train spec `specs/train-k32.json` (sha256 `d09dbf8ed23e0ddb47922f92f37a7a7f648cb25667a5cf172aae43b7e2144091`): `target_seed` 2026114432, 12 public targets x 1 measured round(s), 1 warm-up, timeout 300 s per run. Holdout `target_seed` 2026114932 (reserved now, spec written at selection).

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

Exposures excluded: 27 target seeds already used on this family (ledger exposures); the seeds above are new.

## Amendment: selection (2026-11-19T08:18:51Z, after train, before holdout)

- k = 20: selected `tw2l-x20-j16-n2` (`prime-abscissa:size=2` + `wwalk:x=2.0,jumps=16,g=1,grow=2,tw=2,cj=1,lazy=1,seed=run`), train mean S = 2.158 / 2.344 / 4.015 against `inc` 11.114 / 12.671 / 26.685; Pareto alternative at w = 1: `tw2l-x20-j16-n2`. Holdout spec `specs/holdout-k20.json`, `target_seed` 2026114920.
- k = 24: selected `wwalkGg-x09-sr-n2` (`prime-abscissa:size=2` + `wwalk:x=0.9,g=1,grow=2,seed=run`), train mean S = 1.712 / 1.872 / 3.306 against `inc` 13.492 / 15.476 / 33.333; Pareto alternative at w = 1: `wwalkGg-x09-sr-n2`. Holdout spec `specs/holdout-k24.json`, `target_seed` 2026114924.
- k = 28: selected `tw2l-x20-j16-n2` (`prime-abscissa:size=2` + `wwalk:x=2.0,jumps=16,g=1,grow=2,tw=2,cj=1,lazy=1,seed=run`), train mean S = 1.632 / 1.793 / 3.243 against `inc` 20.483 / 23.560 / 51.249; Pareto alternative at w = 1: `tw2l-x20-j16-n2`. Holdout spec `specs/holdout-k28.json`, `target_seed` 2026114928.
- k = 32: selected `tw2l-x20-j16-n2` (`prime-abscissa:size=2` + `wwalk:x=2.0,jumps=16,g=1,grow=2,tw=2,cj=1,lazy=1,seed=run`), train mean S = 1.694 / 1.863 / 3.381 against `inc` 29.823 / 34.244 / 74.035; Pareto alternative at w = 1: `tw2l-x20-j16-n2`. Holdout spec `specs/holdout-k32.json`, `target_seed` 2026114932.

Holdout spec sha256: {"holdout-k20":"1396391d76e14e9d06e6ebd516d637a7f513d3e8d1dbecaacd407233cc6793dc","holdout-k24":"72e196749271d291450a9062e3f34737932f62db9417c212fbb328a5f9089789","holdout-k28":"c1a9b3e70e0ca22a33e6be3378c5a27c7034f7012c504c19c23ab13470b00bfb","holdout-k32":"bcb2dc58ba3c3361ba2a457b77a39dcdbd2a2b4c577c3583452fcbd127552a47"}
