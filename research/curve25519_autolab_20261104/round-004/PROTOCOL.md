# ic_autolab round 4: protocol (generic track)

Frozen 2026-11-04T03:30:01Z before any data of this round. This file is not edited after that; the selection step appends a dated amendment (after train, before holdout), and nothing else is added.

Loop home: `research/curve25519_autolab_20261104` (`ledger.json` names the incumbents; `families.json` the candidate families). Success criterion: AGENTS.md, "Success criterion: iterative improvement of the index-calculus baseline"; tracks: AGENTS.md, "Tracks of index calculus". The gap to matched rho is reported as the distance remaining, never as a gate.

## Question

Does any candidate configuration of the generic track lower the counted end-to-end cost S (GAE / sqrt r, every phase, every solve verified) below the incumbent on the same fresh public targets of the Curve25519-family miniature at each size below, with the order holding across the AGENTS.md section 5a pricing band (w = 0, 0.1, 1 GAE per unpriced native operation)?

## Incumbents and candidates

Factor base and oracle strings are ecbench `ic.pipeline` parameters. Every candidate is a configuration never tried at its size in this loop (ledger trials).

### k = 28 (p = 268435157, r = 33557809, log2 r = 25.00, N_a = 203)

- `inc` (baseline): `prime-projected-abscissa:size=203,scan_cap=3248` + `mitm:negation_folded=1` (family `mitm2`, N = 203); S = 16.057 / 17.564 / 31.130 since round 0.
- `rho-neg` (reference): `rho.negation`, the matched rho.
- `mitm2-n284` (candidate): `prime-projected-abscissa:size=284,scan_cap=4544` + `mitm:negation_folded=1`; incumbent neighbourhood: N = 1.4 x 203.
- `mitm2-n142` (candidate): `prime-projected-abscissa:size=142,scan_cap=2272` + `mitm:negation_folded=1`; incumbent neighbourhood: N = 0.7 x 203.
- `mitm2-n102` (candidate): `prime-projected-abscissa:size=102,scan_cap=1632` + `mitm:negation_folded=1`; incumbent neighbourhood: N = 0.5 x 203.
- `mitm2-n406` (candidate): `prime-projected-abscissa:size=406,scan_cap=6496` + `mitm:negation_folded=1`; incumbent neighbourhood: N = 2 x 203.
- `ksum4-n30` (candidate): `prime-projected-abscissa:size=30,scan_cap=480` + `ksum:m=4`; cross-size transfer: ksum4-n12 is the k = 24 incumbent at N / N_a = 0.148.
- `ksum4-n42` (candidate): `prime-projected-abscissa:size=42,scan_cap=672` + `ksum:m=4`; cross-size transfer neighbourhood: N = 1.4 x 30 (ksum4 from k = 24).
- `ksum4-n21` (candidate): `prime-projected-abscissa:size=21,scan_cap=336` + `ksum:m=4`; cross-size transfer neighbourhood: N = 0.7 x 30 (ksum4 from k = 24).
- `rfsub2-n203` (candidate): `prime-projected-abscissa:size=203,scan_cap=3248` + `rootfind-subtract:m=2`; exploration: family rfsub2 untried at k = 28, N = 1 x N_a.
- `sub2-n203` (candidate): `prime-projected-abscissa:size=203,scan_cap=3248` + `subtract`; exploration: family sub2 untried at k = 28, N = 1 x N_a.
- Train spec `specs/train-k28.json` (sha256 `a274fa2480d37c18fa07e587633632ab927431c520cd7fff6981e975dfe6626f`): `target_seed` 2026114428, 16 public targets x 1 measured round(s), 1 warm-up, timeout 1800 s per run. Holdout `target_seed` 2026114928 (reserved now, spec written at selection).

### k = 32 (p = 4294967197, r = 536884639, log2 r = 29.00, N_a = 512)

- `inc` (baseline): `prime-projected-abscissa:size=512,scan_cap=8192` + `mitm:negation_folded=1` (family `mitm2`, N = 512); S = 23.310 / 25.520 / 45.410 since round 0.
- `rho-neg` (reference): `rho.negation`, the matched rho.
- `mitm2-n717` (candidate): `prime-projected-abscissa:size=717,scan_cap=11472` + `mitm:negation_folded=1`; incumbent neighbourhood: N = 1.4 x 512.
- `mitm2-n358` (candidate): `prime-projected-abscissa:size=358,scan_cap=5728` + `mitm:negation_folded=1`; incumbent neighbourhood: N = 0.7 x 512.
- `mitm2-n256` (candidate): `prime-projected-abscissa:size=256,scan_cap=4096` + `mitm:negation_folded=1`; incumbent neighbourhood: N = 0.5 x 512.
- `mitm2-n1024` (candidate): `prime-projected-abscissa:size=1024,scan_cap=16384` + `mitm:negation_folded=1`; incumbent neighbourhood: N = 2 x 512.
- `ksum4-n76` (candidate): `prime-projected-abscissa:size=76,scan_cap=1216` + `ksum:m=4`; cross-size transfer: ksum4-n12 is the k = 24 incumbent at N / N_a = 0.148.
- `ksum4-n106` (candidate): `prime-projected-abscissa:size=106,scan_cap=1696` + `ksum:m=4`; cross-size transfer neighbourhood: N = 1.4 x 76 (ksum4 from k = 24).
- `ksum4-n53` (candidate): `prime-projected-abscissa:size=53,scan_cap=848` + `ksum:m=4`; cross-size transfer neighbourhood: N = 0.7 x 76 (ksum4 from k = 24).
- `rfsub2-n512` (candidate): `prime-projected-abscissa:size=512,scan_cap=8192` + `rootfind-subtract:m=2`; exploration: family rfsub2 untried at k = 32, N = 1 x N_a.
- `sub2-n512` (candidate): `prime-projected-abscissa:size=512,scan_cap=8192` + `subtract`; exploration: family sub2 untried at k = 32, N = 1 x N_a.
- Train spec `specs/train-k32.json` (sha256 `65dc7a99b8a6c7430d62eb8dd8bcd4a59ea964d48ab1157505b881cc3c8c0b96`): `target_seed` 2026114432, 16 public targets x 1 measured round(s), 1 warm-up, timeout 1800 s per run. Holdout `target_seed` 2026114932 (reserved now, spec written at selection).

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

Exposures excluded: 30 target seeds already used on this family (ledger exposures); the seeds above are new.
