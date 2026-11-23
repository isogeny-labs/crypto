# ic_autolab round 3: protocol (multibase track)

Frozen 2026-11-19T07:35:17Z before any data of this round. This file is not edited after that; the selection step appends a dated amendment (after train, before holdout), and nothing else is added.

Loop home: `research/prime_generic_autolab_20261118` (`ledger.json` names the incumbents; `families.json` the candidate families). Success criterion: AGENTS.md, "Success criterion: iterative improvement of the index-calculus baseline"; tracks: AGENTS.md, "Tracks of index calculus". The gap to matched rho is reported as the distance remaining, never as a gate.

## Question

Does any candidate configuration of the multibase track lower the counted end-to-end cost S (GAE / sqrt r, every phase, every solve verified) below the incumbent on the same fresh public targets of the registered `prime-generic` curve at each size below, with the order holding across the AGENTS.md section 5a pricing band (w = 0, 0.1, 1 GAE per unpriced native operation)?

## Incumbents and candidates

Factor base and oracle strings are ecbench `ic.pipeline` parameters. Every candidate is a configuration never tried at its size in this loop (ledger trials).

### k = 20 (p = prime_search(20-bit, seed 59297), r = 562427, log2 r = 19.10, N_a = 52)

- `inc` (baseline): `prime-abscissa:size=3` + `wsum:coef=5,free=1` (family `wsum-b5`, N = 3); S = 4.479 / 5.010 / 9.787 since round 1.
- `rho-neg` (reference): `rho.negation`, the matched rho.
- `wsum-b20-n4` (candidate): `prime-abscissa:size=4` + `wsum:coef=20,free=1`; cross-size transfer neighbourhood: N = 1.2 x 3 (wsum-b20 from k = 28).
- `wsum-b8-n4` (candidate): `prime-abscissa:size=4` + `wsum:coef=8,free=1`; alternative wsum-b8 (best S(0) 5.146 at N = 3): N = 1.2 x 3.
- `mitmblocked-n52` (candidate): `prime-abscissa:size=52` + `mitm-prime-blocked:m=2`; exploration: family mitmblocked untried at k = 20, N = 1 x N_a.
- `mitmbatch-n52` (candidate): `prime-abscissa:size=52` + `mitm-prime-batch:m=2`; exploration: family mitmbatch untried at k = 20, N = 1 x N_a.
- `ksum5f-n8` (candidate): `prime-abscissa:size=8` + `ksum:m=5,free=1`; exploration: family ksum5f untried at k = 20, N = 0.08 x N_a.
- Train spec `specs/train-k20.json` (sha256 `d6dce23b36b65438aac9f393f4b348801181c334f681fa7e5a9af0d7be28eae8`): `target_seed` 2026113420, 12 public targets x 1 measured round(s), 1 warm-up, timeout 300 s per run. Holdout `target_seed` 2026113920 (reserved now, spec written at selection).

### k = 24 (p = prime_search(24-bit, seed 59297), r = 16482251, log2 r = 23.97, N_a = 160)

- `inc` (baseline): `prime-abscissa:size=3` + `wsum:coef=12,free=1` (family `wsum-b12`, N = 3); S = 3.060 / 3.422 / 6.678 since round 2.
- `rho-neg` (reference): `rho.negation`, the matched rho.
- `wsum-b12-n4` (candidate): `prime-abscissa:size=4` + `wsum:coef=12,free=1`; incumbent neighbourhood: N = 1.2 x 3.
- `wsum-b20-n4` (candidate): `prime-abscissa:size=4` + `wsum:coef=20,free=1`; cross-size transfer neighbourhood: N = 1.2 x 3 (wsum-b20 from k = 28).
- `mitmblocked-n160` (candidate): `prime-abscissa:size=160` + `mitm-prime-blocked:m=2`; exploration: family mitmblocked untried at k = 24, N = 1 x N_a.
- `mitmbatch-n160` (candidate): `prime-abscissa:size=160` + `mitm-prime-batch:m=2`; exploration: family mitmbatch untried at k = 24, N = 1 x N_a.
- `ksum5f-n13` (candidate): `prime-abscissa:size=13` + `ksum:m=5,free=1`; exploration: family ksum5f untried at k = 24, N = 0.08 x N_a.
- Train spec `specs/train-k24.json` (sha256 `b4fd3f863356e261b921fb160f5b364b959a26bc36a1555f116693b842282859`): `target_seed` 2026113424, 12 public targets x 1 measured round(s), 1 warm-up, timeout 300 s per run. Holdout `target_seed` 2026113924 (reserved now, spec written at selection).

### k = 28 (p = prime_search(28-bit, seed 59297), r = 260192759, log2 r = 27.96, N_a = 402)

- `inc` (baseline): `prime-abscissa:size=3` + `wsum:coef=20,free=1` (family `wsum-b20`, N = 3); S = 3.081 / 3.466 / 6.936 since round 2.
- `rho-neg` (reference): `rho.negation`, the matched rho.
- `wsum-b20-n4` (candidate): `prime-abscissa:size=4` + `wsum:coef=20,free=1`; incumbent neighbourhood: N = 1.2 x 3.
- `wsum-b12-n4` (candidate): `prime-abscissa:size=4` + `wsum:coef=12,free=1`; cross-size transfer: wsum-b12-n3 is the k = 24 incumbent at N / N_a = 0.019.
- `mitmblocked-n402` (candidate): `prime-abscissa:size=402` + `mitm-prime-blocked:m=2`; exploration: family mitmblocked untried at k = 28, N = 1 x N_a.
- `mitmbatch-n402` (candidate): `prime-abscissa:size=402` + `mitm-prime-batch:m=2`; exploration: family mitmbatch untried at k = 28, N = 1 x N_a.
- `ksum5f-n32` (candidate): `prime-abscissa:size=32` + `ksum:m=5,free=1`; exploration: family ksum5f untried at k = 28, N = 0.08 x N_a.
- Train spec `specs/train-k28.json` (sha256 `f5cdece74a6620a5230dd8fc547679d431c590130380e00ea752321b7c42f1c4`): `target_seed` 2026113428, 12 public targets x 1 measured round(s), 1 warm-up, timeout 300 s per run. Holdout `target_seed` 2026113928 (reserved now, spec written at selection).

### k = 32 (p = prime_search(32-bit, seed 59297), r = 3466623553, log2 r = 31.69, N_a = 953)

- `inc` (baseline): `prime-abscissa:size=3` + `wsum:coef=20,free=1` (family `wsum-b20`, N = 3); S = 4.078 / 4.828 / 11.577 since round 2.
- `rho-neg` (reference): `rho.negation`, the matched rho.
- `wsum-b20-n4` (candidate): `prime-abscissa:size=4` + `wsum:coef=20,free=1`; incumbent neighbourhood: N = 1.2 x 3.
- `wsum-b12-n4` (candidate): `prime-abscissa:size=4` + `wsum:coef=12,free=1`; cross-size transfer: wsum-b12-n3 is the k = 24 incumbent at N / N_a = 0.019.
- `mitmblocked-n953` (candidate): `prime-abscissa:size=953` + `mitm-prime-blocked:m=2`; exploration: family mitmblocked untried at k = 32, N = 1 x N_a.
- `mitmbatch-n953` (candidate): `prime-abscissa:size=953` + `mitm-prime-batch:m=2`; exploration: family mitmbatch untried at k = 32, N = 1 x N_a.
- `ksum5f-n76` (candidate): `prime-abscissa:size=76` + `ksum:m=5,free=1`; exploration: family ksum5f untried at k = 32, N = 0.08 x N_a.
- Train spec `specs/train-k32.json` (sha256 `142ff1830d89dd27012c70121d55025219a1c3efde5135461a16d96af0e7e563`): `target_seed` 2026113432, 12 public targets x 1 measured round(s), 1 warm-up, timeout 300 s per run. Holdout `target_seed` 2026113932 (reserved now, spec written at selection).

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

Exposures excluded: 23 target seeds already used on this family (ledger exposures); the seeds above are new.

## Amendment: selection (2026-11-19T08:08:19Z, after train, before holdout)

- k = 20: no candidate below `inc` (4.877 / 5.479 / 10.895) at w = 0 on the train targets; no challenger, no holdout.
- k = 24: no candidate below `inc` (3.059 / 3.420 / 6.663) at w = 0 on the train targets; no challenger, no holdout.
- k = 28: no candidate below `inc` (3.273 / 3.696 / 7.509) at w = 0 on the train targets; no challenger, no holdout.
- k = 32: no candidate below `inc` (3.686 / 4.358 / 10.403) at w = 0 on the train targets; no challenger, no holdout.

Holdout spec sha256: {}
