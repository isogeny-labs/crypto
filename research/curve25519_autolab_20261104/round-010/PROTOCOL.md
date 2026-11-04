# ic_autolab round 10: protocol (generic track)

Frozen 2026-11-04T04:28:44Z before any data of this round. This file is not edited after that; the selection step appends a dated amendment (after train, before holdout), and nothing else is added.

Loop home: `research/curve25519_autolab_20261104` (`ledger.json` names the incumbents; `families.json` the candidate families). Success criterion: AGENTS.md, "Success criterion: iterative improvement of the index-calculus baseline"; tracks: AGENTS.md, "Tracks of index calculus". The gap to matched rho is reported as the distance remaining, never as a gate.

## Question

Does any candidate configuration of the generic track lower the counted end-to-end cost S (GAE / sqrt r, every phase, every solve verified) below the incumbent on the same fresh public targets of the Curve25519-family miniature at each size below, with the order holding across the AGENTS.md section 5a pricing band (w = 0, 0.1, 1 GAE per unpriced native operation)?

## Incumbents and candidates

Factor base and oracle strings are ecbench `ic.pipeline` parameters. Every candidate is a configuration never tried at its size in this loop (ledger trials).

### k = 32 (p = 4294967197, r = 536884639, log2 r = 29.00, N_a = 512)

- `inc` (baseline): `prime-projected-abscissa:size=26,scan_cap=416` + `ksum:m=4` (family `ksum4`, N = 26); S = 8.296 / 8.811 / 13.446 since round 5.
- `rho-neg` (reference): `rho.negation`, the matched rho.
- `ksum4-n22` (candidate): `prime-projected-abscissa:size=22,scan_cap=352` + `ksum:m=4`; incumbent neighbourhood: N = 0.85 x 26.
- `ksum4-n31` (candidate): `prime-projected-abscissa:size=31,scan_cap=496` + `ksum:m=4`; incumbent neighbourhood: N = 1.2 x 26.
- `ksum4-n36` (candidate): `prime-projected-abscissa:size=36,scan_cap=576` + `ksum:m=4`; incumbent neighbourhood: N = 1.4 x 26.
- `ksum4-n18` (candidate): `prime-projected-abscissa:size=18,scan_cap=288` + `ksum:m=4`; incumbent neighbourhood: N = 0.7 x 26.
- `ksum4-n13` (candidate): `prime-projected-abscissa:size=13,scan_cap=208` + `ksum:m=4`; incumbent neighbourhood: N = 0.5 x 26.
- `ksum3-n150` (candidate): `prime-projected-abscissa:size=150,scan_cap=2400` + `ksum:m=3`; cross-size transfer neighbourhood: N = 0.85 x 176 (ksum3 from k = 20).
- `ksum3-n211` (candidate): `prime-projected-abscissa:size=211,scan_cap=3376` + `ksum:m=3`; cross-size transfer neighbourhood: N = 1.2 x 176 (ksum3 from k = 20).
- `ksum5-n41` (candidate): `prime-projected-abscissa:size=41,scan_cap=656` + `ksum:m=5`; exploration: family ksum5 untried at k = 32, N = 0.08 x N_a.
- `rfmitm2-n512` (candidate): `prime-projected-abscissa:size=512,scan_cap=8192` + `rootfind-mitm:m=2`; exploration: family rfmitm2 untried at k = 32, N = 1 x N_a.
- Train spec `specs/train-k32.json` (sha256 `378c8fd59caf9cef1fd33c8d7796c0fc85fd53db792fb6e179edf67612fe39c4`): `target_seed` 2026120432, 32 public targets x 1 measured round(s), 1 warm-up, timeout 1800 s per run. Holdout `target_seed` 2026120932 (reserved now, spec written at selection).

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

Exposures excluded: 42 target seeds already used on this family (ledger exposures); the seeds above are new.

## Amendment: selection (2026-11-04T04:36:30Z, after train, before holdout)

- k = 32: selected `ksum4-n22` (`prime-projected-abscissa:size=22,scan_cap=352` + `ksum:m=4`), train mean S = 7.196 / 8.052 / 15.751 against `inc` 8.179 / 8.671 / 13.096; Pareto alternative at w = 1: `ksum4-n22`. Holdout spec `specs/holdout-k32.json`, `target_seed` 2026120932.

Holdout spec sha256: {"holdout-k32":"99429f238dbf6684602442e16b575f31da8fe289ff27b453062f64b350c42dcb"}
