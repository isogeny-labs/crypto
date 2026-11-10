# Protocol: SNC-1 and SNC-2 shared panel (frozen 2026-11-10, before any data)

Stage 1 of the two engineering records in `../ideas/SYNTHESIS-NC-20261108.md` (section 4) after both passed Stage 0 (`../snc/RESULTS.md`) [pcc:unverified]. Nothing below changes after the first session runs. Failed predictions stay failed. Before freezing, the only executions of the new options are unit tests and replay checks on committed sessions; no measured curve of this panel was run.

## 1. Questions

- SNC-1: does skipping the per-collision recomputation (`lazy=1`), with the final check `[d]G = Q` still charged and a charged re-verification on failure, lower the C = 2 incumbent's S by its verify share (derived 0.067 / 0.023 / 0.005 sqrt(r) at k = 24 / 28 / 32) with identical walk and probe counters on the same draw and no wrong answer accepted? The final check must still pass on every accepted answer [pcc:unverified].
- SNC-2: does a two-seed target walk with chained jumps (`tw=2,cj=1`: entries `u1 P1 + u2 P2 + z G + b Q`, every collision a row over `(s1, s2, log Q)`, stop at rank 3, no probe phase) cost less than the incumbent table-and-probe arm at C = 2?

Neither record can reach the owner's 0.75x goal; the K = 3 collision floor is 1.662 sqrt(r). Any measured gain counts for the C = 2 baseline under the success rule, not as a step toward 0.75.

## 2. Design (frozen)

- Binary: `ecbench` release from the branch `claude/snc-panel` at the commit recorded in `data/binary.sha256` when the sessions start. Existing arms (`wwalk` without `tw=2`/`lazy`, `wsum`) must replay byte-identically on committed sessions with the new binary; that check is a precondition, recorded in `data/replay-precondition.txt`.
- Curves: the five fresh sibling curves per size of `../c1/confirm-curves.json`, k = 20, 24, 28, 32 (`spec-k*.json`).
- Targets: 12 public targets per curve (`target_seed` 20261120 + k), 8 measured rounds plus 1 warm-up, `order = alternate`. ecbench seeds runs by workload and round, so arms are paired run for run and an A/A arm measures nothing (as in `../../curve25519_f1_targetwalk_20261108/PROTOCOL.md` section 3); none is included.
- Arms. All pipeline arms use `prime-projected-abscissa:size=2,scan_cap=32`; `targets=stride-generator` where the incumbent spec has it (not at k = 32, matching `../multicurve/spec-k32.json`); `seed=run` everywhere.
  - `rho-neg` (reference): `rho.negation`.
  - `inc` (baseline): the multicurve incumbent: `wsum:coef=15,free=1` at k = 20; `wwalk:x=0.9,g=1,grow=2,seed=run` at k = 24, 28; `wwalk:x=0.6,g=1,grow=2,seed=run` at k = 32.
  - `inc-lazy` (k = 24, 28, 32 only): `inc` plus `lazy=1`. There is no box lazy arm: the box recorded 0 collisions in all 120 k = 20 confirmation records.
  - `tw2cj`: `wwalk:x=2.0,g=1,grow=2,tw=2,cj=1,seed=run` (cap 6 sqrt(r)).
  - `tw2cj-lazy`: `tw2cj` plus `lazy=1`.
  - If the implementation lands under different option names before any data, the names are updated here and the change is recorded in section 7 as non-substantive.
- Unit and pricing: ecbench GAE, S = cost / sqrt(r), every comparison at w = 0, 0.1, 1 with the 2026-11-08 reading (table inserts and probe lookups priced on every arm, rho-neg's canonicalisations likewise); `ecbench band --ranks` is the band verdict.
- Audit: `ecbench verify --replay 8` per size; every record must carry the audit status `verified` [pcc:unverified].
- Statistic (frozen): for a pair of arms (A, B) at one size, R = (sum of S over all measured runs of A) / (same for B) at w = 0, 0.1 and 1; its 95% interval is the percentile bootstrap over (curve, target) blocks (each block carries its 8 paired rounds), 4000 resamples, seed 20261110. The upper end of the w = 0 interval is the gate named below; w = 0.1 and 1 are reported with it and a gain holds only where the order holds across the band (AGENTS.md section 5a).
- Pre-registered pairs: (`inc-lazy`, `inc`) at k = 24, 28, 32; (`tw2cj-lazy`, `inc`) at k = 20; (`tw2cj-lazy`, `inc-lazy`) at k = 24, 28, 32; (`tw2cj`, `tw2cj-lazy`) as the lazy effect inside the target walk; every IC arm against `rho-neg` as distance remaining.

## 3. Predictions (from the records, before data) [pcc:unverified]

| pair | k = 20 | k = 24 | k = 28 | k = 32 |
|:--|:--|:--|:--|:--|
| `inc-lazy` / `inc`, w = 0 | n/a | 0.968 [0.95, 0.99] | 0.987 [0.97, 1.00] | 0.997 [0.99, 1.00] |
| `tw2cj-lazy` / incumbent (box at 20; `inc-lazy` at 24 to 32), w = 0 | 0.88 [0.83, 0.95] | 0.94 [0.89, 0.99] | 0.96 [0.92, 1.01] | 0.92 [0.88, 0.97] |
| `tw2cj` walk mean, sqrt(r) | 1.75 [1.70, 1.82] | 1.75 [1.70, 1.82] | 1.75 [1.70, 1.82] | 1.75 [1.70, 1.82] |
| `tw2cj-lazy` / `rho-neg`, w = 0 | 1.38 [1.30, 1.48] | about 1.65 | 1.71 [1.66, 1.79] | about 1.75 |

The w = 1 ratio of `tw2cj-lazy` to the incumbent is predicted within 0.03 of its w = 0 ratio at k >= 24 (one insert per stored entry against the incumbent's inserts plus probe lookups). Probability that 0.75x of rho-neg is reached by any arm: below 0.01.

## 4. Falsification (frozen)

- SNC-1 is dead at Stage 1 if the w = 0 interval of `inc-lazy` / `inc` has its upper end at or above 1 at k = 24; or if walk or probe counters of `inc-lazy` differ from `inc` on any paired run; or if any accepted answer fails ecbench's check.
- SNC-2 is dead at Stage 1 if the w = 0 upper end of `tw2cj-lazy` against the incumbent is at or above 1 at k = 20 and at two of k = 24, 28, 32; or if the measured walk mean lies outside [1.66, 1.85] sqrt(r); or if any used row has zero seed part; or if any answer is unverified.
- A gain that holds at w = 0 but reverses at w = 1 is reported as unresolved inside the band, not as a gain.

## 5. Controls

- `inc` against `inc-lazy` on the same draw (exact pairing by workload and round).
- `rho-neg` on the same targets.
- Fault injection in the unit tests: a corrupted stored coefficient must be rejected by the lazy path.
- Replay precondition (section 2) on existing arms.

## 6. Reporting

`README.md` with the pair table at w = 0, 0.1, 1, per-size S of every arm, the walk-mean diagnostics of the `tw2` arms, the audit results, a PDF and SVG, a bench record, a pcc statement claim, a row in `../README.md`, and the session log. The main-track ledger is touched only if the owner rules on the classification of `tw=2` (the targetwalk README calls the one-seed version "rho with one superfluous unknown"; the two-seed version carries two base logarithms and the obstruction note's K = 3 floor applies).

## 7. Amendments

None yet.
