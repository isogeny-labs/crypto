# ic_autolab round 6: protocol (generic track)

Frozen 2026-11-04T04:21:45Z before any data of this round. This file is not edited after that; the selection step appends a dated amendment (after train, before holdout), and nothing else is added.

Loop home: `research/curve25519_autolab_20261104` (`ledger.json` names the incumbents; `families.json` the candidate families). Success criterion: AGENTS.md, "Success criterion: iterative improvement of the index-calculus baseline"; tracks: AGENTS.md, "Tracks of index calculus". The gap to matched rho is reported as the distance remaining, never as a gate.

## Question

Does any candidate configuration of the generic track lower the counted end-to-end cost S (GAE / sqrt r, every phase, every solve verified) below the incumbent on the same fresh public targets of the Curve25519-family miniature at each size below, with the order holding across the AGENTS.md section 5a pricing band (w = 0, 0.1, 1 GAE per unpriced native operation)?

## Incumbents and candidates

Factor base and oracle strings are ecbench `ic.pipeline` parameters. Every candidate is a configuration never tried at its size in this loop (ledger trials).

### k = 28 (p = 268435157, r = 33557809, log2 r = 25.00, N_a = 203)

- `inc` (baseline): `prime-projected-abscissa:size=21,scan_cap=336` + `ksum:m=4` (family `ksum4`, N = 21); S = 10.939 / 11.226 / 13.809 since round 4.
- `rho-neg` (reference): `rho.negation`, the matched rho.
- `ksum4-n29` (candidate): `prime-projected-abscissa:size=29,scan_cap=464` + `ksum:m=4`; incumbent neighbourhood: N = 1.4 x 21.
- `ksum4-n15` (candidate): `prime-projected-abscissa:size=15,scan_cap=240` + `ksum:m=4`; incumbent neighbourhood: N = 0.7 x 21.
- `ksum4-n11` (candidate): `prime-projected-abscissa:size=11,scan_cap=176` + `ksum:m=4`; incumbent neighbourhood: N = 0.5 x 21.
- `ksum3-n70` (candidate): `prime-projected-abscissa:size=70,scan_cap=1120` + `ksum:m=3`; cross-size transfer: ksum3-n11 is the k = 20 incumbent at N / N_a = 0.344.
- `ksum3-n98` (candidate): `prime-projected-abscissa:size=98,scan_cap=1568` + `ksum:m=3`; cross-size transfer neighbourhood: N = 1.4 x 70 (ksum3 from k = 20).
- `ksum3-n49` (candidate): `prime-projected-abscissa:size=49,scan_cap=784` + `ksum:m=3`; cross-size transfer neighbourhood: N = 0.7 x 70 (ksum3 from k = 20).
- `ksum3-n35` (candidate): `prime-projected-abscissa:size=35,scan_cap=560` + `ksum:m=3`; cross-size transfer neighbourhood: N = 0.5 x 70 (ksum3 from k = 20).
- `mitmbatch-n203` (candidate): `prime-projected-abscissa:size=203,scan_cap=3248` + `mitm-prime-batch:m=2`; exploration: family mitmbatch untried at k = 28, N = 1 x N_a.
- `mitmblocked-n203` (candidate): `prime-projected-abscissa:size=203,scan_cap=3248` + `mitm-prime-blocked:m=2`; exploration: family mitmblocked untried at k = 28, N = 1 x N_a.
- Train spec `specs/train-k28.json` (sha256 `60539d08c7b395e8f57ab907b9a093fe7f0faed9e2fd20d892fefe422ccf4d1d`): `target_seed` 2026116428, 16 public targets x 1 measured round(s), 1 warm-up, timeout 1800 s per run. Holdout `target_seed` 2026116928 (reserved now, spec written at selection).

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

Exposures excluded: 35 target seeds already used on this family (ledger exposures); the seeds above are new.

## Amendment: selection (2026-11-04T04:22:03Z, after train, before holdout)

- k = 28: selected `ksum4-n15` (`prime-projected-abscissa:size=15,scan_cap=240` + `ksum:m=4`), train mean S = 6.665 / 7.484 / 14.852 against `inc` 10.869 / 11.141 / 13.595; Pareto alternative at w = 1: `ksum4-n15`. Holdout spec `specs/holdout-k28.json`, `target_seed` 2026116928.

Holdout spec sha256: {"holdout-k28":"55c1a36ae9ad20895df5882b861665a68b4108567b2437b6605d503abbbc23ef"}
