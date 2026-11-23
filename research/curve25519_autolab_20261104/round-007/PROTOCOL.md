# ic_autolab round 7: protocol (generic track)

Frozen 2026-11-04T04:23:34Z before any data of this round. This file is not edited after that; the selection step appends a dated amendment (after train, before holdout), and nothing else is added.

Loop home: `research/curve25519_autolab_20261104` (`ledger.json` names the incumbents; `families.json` the candidate families). Success criterion: AGENTS.md, "Success criterion: iterative improvement of the index-calculus baseline"; tracks: AGENTS.md, "Tracks of index calculus". The gap to matched rho is reported as the distance remaining, never as a gate.

## Question

Does any candidate configuration of the generic track lower the counted end-to-end cost S (GAE / sqrt r, every phase, every solve verified) below the incumbent on the same fresh public targets of the Curve25519-family miniature at each size below, with the order holding across the AGENTS.md section 5a pricing band (w = 0, 0.1, 1 GAE per unpriced native operation)?

## Incumbents and candidates

Factor base and oracle strings are ecbench `ic.pipeline` parameters. Every candidate is a configuration never tried at its size in this loop (ledger trials).

### k = 20 (p = 1048573, r = 131203, log2 r = 17.00, N_a = 32)

- `inc` (baseline): `prime-projected-abscissa:size=11,scan_cap=176` + `ksum:m=3` (family `ksum3`, N = 11); S = 8.987 / 9.710 / 16.217 since round 0.
- `rho-neg` (reference): `rho.negation`, the matched rho.
- `ksum3-n9` (candidate): `prime-projected-abscissa:size=9,scan_cap=144` + `ksum:m=3`; incumbent neighbourhood: N = 0.85 x 11.
- `ksum3-n13` (candidate): `prime-projected-abscissa:size=13,scan_cap=208` + `ksum:m=3`; incumbent neighbourhood: N = 1.2 x 11.
- `ksum4-n9` (candidate): `prime-projected-abscissa:size=9,scan_cap=144` + `ksum:m=4`; cross-size transfer: ksum4 is incumbent at 3 sizes; ln N fitted on log2 r (slope 0.097) and extrapolated.
- `ksum4-n8` (candidate): `prime-projected-abscissa:size=8,scan_cap=128` + `ksum:m=4`; cross-size transfer neighbourhood: N = 0.85 x 9 (ksum4 from k = 24).
- `ksum4-n11` (candidate): `prime-projected-abscissa:size=11,scan_cap=176` + `ksum:m=4`; cross-size transfer neighbourhood: N = 1.2 x 9 (ksum4 from k = 24).
- `ksum4-n13` (candidate): `prime-projected-abscissa:size=13,scan_cap=208` + `ksum:m=4`; cross-size transfer neighbourhood: N = 1.4 x 9 (ksum4 from k = 24).
- `ksum4-n18` (candidate): `prime-projected-abscissa:size=18,scan_cap=288` + `ksum:m=4`; cross-size transfer neighbourhood: N = 2 x 9 (ksum4 from k = 24).
- `ksum5-n8` (candidate): `prime-projected-abscissa:size=8,scan_cap=128` + `ksum:m=5`; exploration: family ksum5 untried at k = 20, N = 0.08 x N_a.
- `rfsub2-n32` (candidate): `prime-projected-abscissa:size=32,scan_cap=512` + `rootfind-subtract:m=2`; exploration: family rfsub2 untried at k = 20, N = 1 x N_a.
- Train spec `specs/train-k20.json` (sha256 `6c5f7335b29bc661c2360bcc311d96e09746b1e750d0bc4a0083e7b29eb3cc2a`): `target_seed` 2026117420, 16 public targets x 1 measured round(s), 1 warm-up, timeout 1800 s per run. Holdout `target_seed` 2026117920 (reserved now, spec written at selection).

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

Exposures excluded: 37 target seeds already used on this family (ledger exposures); the seeds above are new.

## Amendment: selection (2026-11-04T04:23:43Z, after train, before holdout)

- k = 20: selected `ksum4-n8` (`prime-projected-abscissa:size=8,scan_cap=128` + `ksum:m=4`), train mean S = 8.184 / 8.774 / 14.079 against `inc` 9.311 / 10.089 / 17.089; Pareto alternative at w = 1: `ksum4-n9`. Holdout spec `specs/holdout-k20.json`, `target_seed` 2026117920.

Holdout spec sha256: {"holdout-k20":"b72c053da7d45287f9a40853aad3df23a083c0a49e4dfc96da733c51580dce0c"}
