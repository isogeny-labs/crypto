# ic_autolab round 36: protocol (generic track)

Frozen 2026-11-10T03:59:40Z before any data of this round. This file is not edited after that; the selection step appends a dated amendment (after train, before holdout), and nothing else is added.

Loop home: `research/curve25519_autolab_20261104` (`ledger.json` names the incumbents; `families.json` the candidate families). Success criterion: AGENTS.md, "Success criterion: iterative improvement of the index-calculus baseline"; tracks: AGENTS.md, "Tracks of index calculus". The gap to matched rho is reported as the distance remaining, never as a gate.

## Question

Does any candidate configuration of the generic track lower the counted end-to-end cost S (GAE / sqrt r, every phase, every solve verified) below the incumbent on the same fresh public targets of the Curve25519-family miniature at each size below, with the order holding across the AGENTS.md section 5a pricing band (w = 0, 0.1, 1 GAE per unpriced native operation)?

## Incumbents and candidates

Factor base and oracle strings are ecbench `ic.pipeline` parameters. Every candidate is a configuration never tried at its size in this loop (ledger trials).

### k = 20 (p = 1048573, r = 131203, log2 r = 17.00, N_a = 32)

- `inc` (baseline): `prime-projected-abscissa:size=2,scan_cap=32` + `wsum:coef=15,free=1` (family `wsum2g-b15`, N = 2); S = 2.575 / 2.691 / 3.741 since round 28.
- `rho-neg` (reference): `rho.negation`, the matched rho.
- `tw2l-x10-j8-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=1.0,jumps=8,g=1,grow=2,tw=2,cj=1,lazy=1,seed=run`; x/jumps sweep of tw2cj-lazy after the SNC panel (PCC-20261110-978c29: x = 2.0, 16 jumps gave 0.936/0.914/0.909/0.922 of the incumbent at k = 20/24/28/32).
- `tw2l-x10-j16-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=1.0,jumps=16,g=1,grow=2,tw=2,cj=1,lazy=1,seed=run`; x/jumps sweep of tw2cj-lazy after the SNC panel (PCC-20261110-978c29: x = 2.0, 16 jumps gave 0.936/0.914/0.909/0.922 of the incumbent at k = 20/24/28/32).
- `tw2l-x15-j16-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=1.5,jumps=16,g=1,grow=2,tw=2,cj=1,lazy=1,seed=run`; x/jumps sweep of tw2cj-lazy after the SNC panel (PCC-20261110-978c29: x = 2.0, 16 jumps gave 0.936/0.914/0.909/0.922 of the incumbent at k = 20/24/28/32).
- `tw2l-x20-j8-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=2.0,jumps=8,g=1,grow=2,tw=2,cj=1,lazy=1,seed=run`; x/jumps sweep of tw2cj-lazy after the SNC panel (PCC-20261110-978c29: x = 2.0, 16 jumps gave 0.936/0.914/0.909/0.922 of the incumbent at k = 20/24/28/32).
- `tw2l-x20-j16-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=2.0,jumps=16,g=1,grow=2,tw=2,cj=1,lazy=1,seed=run`; x/jumps sweep of tw2cj-lazy after the SNC panel (PCC-20261110-978c29: x = 2.0, 16 jumps gave 0.936/0.914/0.909/0.922 of the incumbent at k = 20/24/28/32).
- `tw2l-x20-j32-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=2.0,jumps=32,g=1,grow=2,tw=2,cj=1,lazy=1,seed=run`; x/jumps sweep of tw2cj-lazy after the SNC panel (PCC-20261110-978c29: x = 2.0, 16 jumps gave 0.936/0.914/0.909/0.922 of the incumbent at k = 20/24/28/32).
- `tw2l-x30-j16-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=3.0,jumps=16,g=1,grow=2,tw=2,cj=1,lazy=1,seed=run`; x/jumps sweep of tw2cj-lazy after the SNC panel (PCC-20261110-978c29: x = 2.0, 16 jumps gave 0.936/0.914/0.909/0.922 of the incumbent at k = 20/24/28/32).
- Train spec `specs/train-k20.json` (sha256 `bb306e6228bce1be664900febd0b1c204df055de945273766bf9da63c9a07bc5`): `target_seed` 2026146420, 16 public targets x 1 measured round(s), 1 warm-up, timeout 1800 s per run. Holdout `target_seed` 2026146920 (reserved now, spec written at selection).

### k = 24 (p = 16777213, r = 2097349, log2 r = 21.00, N_a = 81)

- `inc` (baseline): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.9,g=1,grow=2,seed=run` (family `wwalkGg-x09-sr`, N = 2); S = 2.105 / 2.148 / 2.531 since round 33.
- `rho-neg` (reference): `rho.negation`, the matched rho.
- `tw2l-x10-j8-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=1.0,jumps=8,g=1,grow=2,tw=2,cj=1,lazy=1,seed=run`; x/jumps sweep of tw2cj-lazy after the SNC panel (PCC-20261110-978c29: x = 2.0, 16 jumps gave 0.936/0.914/0.909/0.922 of the incumbent at k = 20/24/28/32).
- `tw2l-x10-j16-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=1.0,jumps=16,g=1,grow=2,tw=2,cj=1,lazy=1,seed=run`; x/jumps sweep of tw2cj-lazy after the SNC panel (PCC-20261110-978c29: x = 2.0, 16 jumps gave 0.936/0.914/0.909/0.922 of the incumbent at k = 20/24/28/32).
- `tw2l-x15-j16-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=1.5,jumps=16,g=1,grow=2,tw=2,cj=1,lazy=1,seed=run`; x/jumps sweep of tw2cj-lazy after the SNC panel (PCC-20261110-978c29: x = 2.0, 16 jumps gave 0.936/0.914/0.909/0.922 of the incumbent at k = 20/24/28/32).
- `tw2l-x20-j8-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=2.0,jumps=8,g=1,grow=2,tw=2,cj=1,lazy=1,seed=run`; x/jumps sweep of tw2cj-lazy after the SNC panel (PCC-20261110-978c29: x = 2.0, 16 jumps gave 0.936/0.914/0.909/0.922 of the incumbent at k = 20/24/28/32).
- `tw2l-x20-j16-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=2.0,jumps=16,g=1,grow=2,tw=2,cj=1,lazy=1,seed=run`; x/jumps sweep of tw2cj-lazy after the SNC panel (PCC-20261110-978c29: x = 2.0, 16 jumps gave 0.936/0.914/0.909/0.922 of the incumbent at k = 20/24/28/32).
- `tw2l-x20-j32-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=2.0,jumps=32,g=1,grow=2,tw=2,cj=1,lazy=1,seed=run`; x/jumps sweep of tw2cj-lazy after the SNC panel (PCC-20261110-978c29: x = 2.0, 16 jumps gave 0.936/0.914/0.909/0.922 of the incumbent at k = 20/24/28/32).
- `tw2l-x30-j16-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=3.0,jumps=16,g=1,grow=2,tw=2,cj=1,lazy=1,seed=run`; x/jumps sweep of tw2cj-lazy after the SNC panel (PCC-20261110-978c29: x = 2.0, 16 jumps gave 0.936/0.914/0.909/0.922 of the incumbent at k = 20/24/28/32).
- Train spec `specs/train-k24.json` (sha256 `7f5821f25fb14ceab64e3dc35bdf469631475a8ba754bd137081c019f467faa9`): `target_seed` 2026146424, 16 public targets x 1 measured round(s), 1 warm-up, timeout 1800 s per run. Holdout `target_seed` 2026146924 (reserved now, spec written at selection).

### k = 28 (p = 268435157, r = 33557809, log2 r = 25.00, N_a = 203)

- `inc` (baseline): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.9,g=1,grow=2,seed=run` (family `wwalkGg-x09-sr`, N = 2); S = 1.883 / 1.926 / 2.309 since round 33.
- `rho-neg` (reference): `rho.negation`, the matched rho.
- `tw2l-x10-j8-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=1.0,jumps=8,g=1,grow=2,tw=2,cj=1,lazy=1,seed=run`; x/jumps sweep of tw2cj-lazy after the SNC panel (PCC-20261110-978c29: x = 2.0, 16 jumps gave 0.936/0.914/0.909/0.922 of the incumbent at k = 20/24/28/32).
- `tw2l-x10-j16-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=1.0,jumps=16,g=1,grow=2,tw=2,cj=1,lazy=1,seed=run`; x/jumps sweep of tw2cj-lazy after the SNC panel (PCC-20261110-978c29: x = 2.0, 16 jumps gave 0.936/0.914/0.909/0.922 of the incumbent at k = 20/24/28/32).
- `tw2l-x15-j16-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=1.5,jumps=16,g=1,grow=2,tw=2,cj=1,lazy=1,seed=run`; x/jumps sweep of tw2cj-lazy after the SNC panel (PCC-20261110-978c29: x = 2.0, 16 jumps gave 0.936/0.914/0.909/0.922 of the incumbent at k = 20/24/28/32).
- `tw2l-x20-j8-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=2.0,jumps=8,g=1,grow=2,tw=2,cj=1,lazy=1,seed=run`; x/jumps sweep of tw2cj-lazy after the SNC panel (PCC-20261110-978c29: x = 2.0, 16 jumps gave 0.936/0.914/0.909/0.922 of the incumbent at k = 20/24/28/32).
- `tw2l-x20-j16-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=2.0,jumps=16,g=1,grow=2,tw=2,cj=1,lazy=1,seed=run`; x/jumps sweep of tw2cj-lazy after the SNC panel (PCC-20261110-978c29: x = 2.0, 16 jumps gave 0.936/0.914/0.909/0.922 of the incumbent at k = 20/24/28/32).
- `tw2l-x20-j32-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=2.0,jumps=32,g=1,grow=2,tw=2,cj=1,lazy=1,seed=run`; x/jumps sweep of tw2cj-lazy after the SNC panel (PCC-20261110-978c29: x = 2.0, 16 jumps gave 0.936/0.914/0.909/0.922 of the incumbent at k = 20/24/28/32).
- `tw2l-x30-j16-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=3.0,jumps=16,g=1,grow=2,tw=2,cj=1,lazy=1,seed=run`; x/jumps sweep of tw2cj-lazy after the SNC panel (PCC-20261110-978c29: x = 2.0, 16 jumps gave 0.936/0.914/0.909/0.922 of the incumbent at k = 20/24/28/32).
- Train spec `specs/train-k28.json` (sha256 `2f48318c6c7506e59cfa4ed1ad8dfed194be587db8721fac720362b0beef4bc4`): `target_seed` 2026146428, 16 public targets x 1 measured round(s), 1 warm-up, timeout 1800 s per run. Holdout `target_seed` 2026146928 (reserved now, spec written at selection).

### k = 32 (p = 4294967197, r = 536884639, log2 r = 29.00, N_a = 512)

- `inc` (baseline): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=0.6,g=1,grow=2,seed=run` (family `wwalkG-x06-sr`, N = 2); S = 1.842 / 1.932 / 2.740 since round 33.
- `rho-neg` (reference): `rho.negation`, the matched rho.
- `tw2l-x10-j8-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=1.0,jumps=8,g=1,grow=2,tw=2,cj=1,lazy=1,seed=run`; x/jumps sweep of tw2cj-lazy after the SNC panel (PCC-20261110-978c29: x = 2.0, 16 jumps gave 0.936/0.914/0.909/0.922 of the incumbent at k = 20/24/28/32).
- `tw2l-x10-j16-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=1.0,jumps=16,g=1,grow=2,tw=2,cj=1,lazy=1,seed=run`; x/jumps sweep of tw2cj-lazy after the SNC panel (PCC-20261110-978c29: x = 2.0, 16 jumps gave 0.936/0.914/0.909/0.922 of the incumbent at k = 20/24/28/32).
- `tw2l-x15-j16-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=1.5,jumps=16,g=1,grow=2,tw=2,cj=1,lazy=1,seed=run`; x/jumps sweep of tw2cj-lazy after the SNC panel (PCC-20261110-978c29: x = 2.0, 16 jumps gave 0.936/0.914/0.909/0.922 of the incumbent at k = 20/24/28/32).
- `tw2l-x20-j8-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=2.0,jumps=8,g=1,grow=2,tw=2,cj=1,lazy=1,seed=run`; x/jumps sweep of tw2cj-lazy after the SNC panel (PCC-20261110-978c29: x = 2.0, 16 jumps gave 0.936/0.914/0.909/0.922 of the incumbent at k = 20/24/28/32).
- `tw2l-x20-j16-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=2.0,jumps=16,g=1,grow=2,tw=2,cj=1,lazy=1,seed=run`; x/jumps sweep of tw2cj-lazy after the SNC panel (PCC-20261110-978c29: x = 2.0, 16 jumps gave 0.936/0.914/0.909/0.922 of the incumbent at k = 20/24/28/32).
- `tw2l-x20-j32-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=2.0,jumps=32,g=1,grow=2,tw=2,cj=1,lazy=1,seed=run`; x/jumps sweep of tw2cj-lazy after the SNC panel (PCC-20261110-978c29: x = 2.0, 16 jumps gave 0.936/0.914/0.909/0.922 of the incumbent at k = 20/24/28/32).
- `tw2l-x30-j16-n2` (candidate): `prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=3.0,jumps=16,g=1,grow=2,tw=2,cj=1,lazy=1,seed=run`; x/jumps sweep of tw2cj-lazy after the SNC panel (PCC-20261110-978c29: x = 2.0, 16 jumps gave 0.936/0.914/0.909/0.922 of the incumbent at k = 20/24/28/32).
- Train spec `specs/train-k32.json` (sha256 `7c9058e016c753e773d2d34177d2d467b020bd35d096861b969e64265a442b8e`): `target_seed` 2026146432, 16 public targets x 1 measured round(s), 1 warm-up, timeout 1800 s per run. Holdout `target_seed` 2026146932 (reserved now, spec written at selection).

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

Exposures excluded: 120 target seeds already used on this family (ledger exposures); the seeds above are new.

## Amendment: selection (2026-11-10T04:00:56Z, after train, before holdout)

- k = 20: selected `tw2l-x10-j16-n2` (`prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=1.0,jumps=16,g=1,grow=2,tw=2,cj=1,lazy=1,seed=run`), train mean S = 2.284 / 2.448 / 3.925 against `inc` 2.574 / 2.823 / 5.063; Pareto alternative at w = 1: `tw2l-x10-j16-n2`. Holdout spec `specs/holdout-k20.json`, `target_seed` 2026146920.
- k = 24: selected `tw2l-x20-j32-n2` (`prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=2.0,jumps=32,g=1,grow=2,tw=2,cj=1,lazy=1,seed=run`), train mean S = 1.773 / 1.921 / 3.258 against `inc` 2.218 / 2.408 / 4.117; Pareto alternative at w = 1: `tw2l-x20-j32-n2`. Holdout spec `specs/holdout-k24.json`, `target_seed` 2026146924.
- k = 28: selected `tw2l-x30-j16-n2` (`prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=3.0,jumps=16,g=1,grow=2,tw=2,cj=1,lazy=1,seed=run`), train mean S = 1.614 / 1.770 / 3.166 against `inc` 1.997 / 2.187 / 3.896; Pareto alternative at w = 1: `tw2l-x30-j16-n2`. Holdout spec `specs/holdout-k28.json`, `target_seed` 2026146928.
- k = 32: selected `tw2l-x20-j8-n2` (`prime-projected-abscissa:size=2,scan_cap=32` + `wwalk:x=2.0,jumps=8,g=1,grow=2,tw=2,cj=1,lazy=1,seed=run`), train mean S = 1.621 / 1.781 / 3.226 against `inc` 1.807 / 2.020 / 3.942; Pareto alternative at w = 1: `tw2l-x20-j8-n2`. Holdout spec `specs/holdout-k32.json`, `target_seed` 2026146932.

Holdout spec sha256: {"holdout-k20":"444625f3bbff542344b6df7a5e4e2e8e81293948f18cf4647f1effcaade6fe52","holdout-k24":"2884b3fddbf2652e41dbe1654766277ec58241ff317af3f9c231285d279a84a8","holdout-k28":"3c9ef1ac58a93a35c5a2f0f837963f5d6aac7b469f442dcab67d73db522cb74d","holdout-k32":"e1259b26a0e0a1bc600dd94647ae0bac0265556a2d4a4c5b198e692d9f5a968b"}
