# Validator report: J2 (factor-base constant c_D = 1.60 vs the frozen 2.00, N4v2 plane-quartic DL1)

Validated artifact set: git commit `44d35033fa06b9796c6a4bf7696056d7a6e595a6`, producer paths clean, hashes in `FROZEN.sha256`.
Date 2026-11-14/15. **Unsigned**: no keyring signature is available in this session. All numbers `[pcc:unverified]`.
Details and commands: `RESULTS.md`; machine files: `validator-report.json`, `replay-certificate.json`, `VALIDATION.json`; evidence: `evidence/`.

## Disposition: `insufficient_evidence` (for the claim "engineering improvement"); the measurement itself reproduces

The recorded runs are valid and reproducible: 84 of 84 units replay identically, 22 of 22 instances pass an independent certificate check, and every [pcc:unverified]
number in the producer's J2 table, held-out paragraph and bench record recomputes. The improvement claim is not established, because the gain exists only in
the frozen N4v2 M-eq unit, which counts but does not price F_q and mod-ell additions; at c = 1.60 those additions rise 1.38-1.49x (denser matrix) and the measured
CPU time of the solve does not fall (0.95-1.13 per seed at k = 21, 22). The rho reference is partial.

## Pass / fail per requirement

| id | requirement | result | evidence |
|---|---|---|---|
| V0 | frozen committed artifact set; hashes (binary, raw tarballs, manifests) | pass | `evidence/v0_v8.json`, `FROZEN.sha256` |
| V1 | rebuild from source, compare sha256 | pass: `183810cf...` bit for bit (source-directory invocation; other path forms embed the path and differ) | `RESULTS.md` section 2 |
| V2 | replay of at least 6 units, every counter identical | pass: 84/84 units, 27,612 field comparisons, A/A identical; the six mandated units included; comparator negative controls pass | `evidence/replay_compare.json`, `evidence/replay/`, `evidence/tool_controls.txt` |
| V3 | c = 2.00 reproduces the committed N4v2 records | pass: 12/12, 293 fields each | `evidence/v3_compare.json` |
| V4 | certificate discipline (independent code) | pass: 22/22 instances (order = 4 ell, ell prime, G, planted k from the seed, k_found G = k G, negative controls rejected, tries equal the committed tries) | `evidence/certcheck.jsonl`, `tools/certcheck.c` |
| V5 | recompute every ratio, count, range | pass: all printed numbers reproduced to their rounding; `j2/summary.json` equal in 18 cells; pricing code reproduces `summary_supp.json` (<= 3.5e-7) | `evidence/recompute.json`, `evidence/tables.md` |
| V6 | w-band order c=1.60 < 1.75 < 2.00 | pass under the frozen rule (k >= 18 and held-out; k = 16 fails at w = 0, 0.1 as claimed; robust to calibration); **fail under the strict reading** (additions priced): reversed at w = 0.1 and 1 at every size | `evidence/recompute.json`, `evidence/tables.md` T2, T3 |
| V7 | threshold law and success counts | pass with a scope limit: counts reproduced; the 1.50/1.60 flip is observed at k = 16, 18, 20 only (no c <= 1.50 data at k = 21, 22); 95% lower bound of first-attempt success at c = 1.60: 0.873 (22 units), 0.741 (10 held-out) | `evidence/recompute.json` |
| V8 | held-out discipline | pass: amendment committed 04:54:39Z before launch 04:54:56Z; fresh seeds; predictions met; failed outputs empty and unused | `evidence/v0_v8.json` |
| V9 | matched rho reference with the 5a band | partial: N4v2 frozen rho at the same q (k = 22 reference is the post-hoc supplement), recomputed equal to committed; 5 rho job files replayed identically; DL1 / rho = 0.0011-0.0029 at w = 0, 0.1, 1 (strict 0.0011-0.0073), order holds; J2 protocol names no rho; different binary and host | `evidence/recompute.json`, `evidence/rho_replay/`, `VALIDATION.json` |
| V10 | reporting contract | gap: no rho column, no phase split, no unpriced-counter list, class asserted; m = 83 / ECC2K-130 not applicable | `RESULTS.md` section 11 |
| V11 | the gain survives measured time (post-hoc) | **fail**: total CPU c=1.60 / c=2.00 = 1.00-1.07 (medians) at k = 21, 22; LA phase 1.34-1.56x; in-situ 1.08 M-eq per counted ladd | `evidence/recompute.json`, `evidence/tables.md` T6, `evidence/ratio_vs_addition_price.svg` |

## Numbers the validator stands behind (recomputed from raw counters, frozen unit, w = 0, paired by seed)

| k | seeds | c = 1.60 / c = 2.00 | c = 1.75 / c = 2.00 | first-attempt success at 1.60 / 1.75 |
|---|---|---|---|---|
| 21 (2,097,091) held-out | 4-8 | 0.660-0.685 | 0.771-0.775 | 5/5, 5/5 |
| 22 (4,194,271) held-out | 4-8 | 0.655-0.666 | 0.771-0.773 | 5/5, 5/5 |
| 22 grid | 1-3 | 0.653-0.658 | 0.771-0.772 | 3/3, 3/3 |
| 16 (65,479) grid | 1-3 | 0.869-1.119 (no gain) | 0.907-1.003 | 3/3, 3/3 |

## Findings the artifacts do not support (details in `RESULTS.md` section 12)

F1 gain depends on leaving additions unpriced; measured CPU shows none (decisive for the class label). F2 threshold flip "at every k" is unobserved at k = 21, 22.
F3 "N4v2 DL1 about 0.005x rho-neg" is DL's figure; DL1 is 0.0028. F4 "seeds agree within 1%" holds for c = 2.00 only. F5 J2 protocol names no rho; rho ran in
another binary. F6 the c = 2.00 control against the committed records was required and not reported (identical here). F7 the amendment's time label (05:00) is later than its commit (04:54:39Z).

## What would make it `validated`

1. An amendment, frozen before data, that names a unit including the additions (or a calibrated weight for them, 5a "calibrated weight") and shows c = 1.60 below c = 2.00 in it; or an L2-isolated CPU
   measurement with an A/A interval that shows the gain at k >= 21. A retune of the SGE merge parameter gamma at lower c is the obvious candidate to recover the linear-algebra phase.
2. A J2 protocol naming the negation-map rho as reference, with the rho arm run in the same binary at k = 21, 22.
3. Observed lower brackets (c = 1.40, 1.50) at k = 21, 22 if the threshold statement is to stand at every k.

## Gate descriptor

`python3 scripts/validation_gate.py evidence research/leads_followup_20261114/item03_j2_validation` lists exactly six open reasons: the validator report status is not `pass`, three headline numbers that
do not agree with the producer's text (F2, F3, F4), and the two rho-reference fields `matches_protocol` and `same_binary_or_process` (false, as measured). The replay certificate and the raw-data hashes pass the gate.
