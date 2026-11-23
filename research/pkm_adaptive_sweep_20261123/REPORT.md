# Adaptive PKM map continuation — 2026-11-23

## Question and construction

Can a genuinely different PKM §3.4 auxiliary-curve isogeny chain deliver more useful two-summand relations per setup-charged CPU than a same-size interval factor base? The source is [Petit–Kosters–Messeng, *Algebraic approaches for the Elliptic Curve Discrete Logarithm Problem over prime fields*, §3.4](https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf): use the x-coordinates of a coset of a smooth-order subgroup on an auxiliary curve, represented as a composition of low-degree Vélu maps. This is the paper's isogeny construction, implemented here as a relation-enumeration component. The separate F4 qualification tests the repository's existing `prime-algebraic` implementation of Semaev plus low-degree membership equations; it does not execute this same candidate panel.

The fixed target is `E/F_1000003: y²=x³+2x+3`; the generator selects an auxiliary `E'`, subgroup and Vélu chain for each seed. Low map degrees in the measured chains were **2, 3, 5, 7, 11 and 13**, composing to high degrees **500–6370**. For example, seed 14 used `[5,5,5,7]`, high degree 875, 455 lifting abscissae; seed 66 used `[2,5,7,7,13]`, high degree 6370, 3155 lifting abscissae; seed 103 used `[2,2,2,2,3,3,7]`, high degree 1008, 490 lifting abscissae. Every fiber point was checked through the composed rational map. The comparator has exactly the same number of lifting abscissae.

## Frozen protocol and raw evidence

`PROTOCOL.md` preceded the one-trial F4 call; `AMENDMENT_112_160.md` preceded the second relation batch. `adaptive.py` resumes an append-only JSONL store, uses the same 48 screening and 96 confirmation target points for every candidate, verifies fibers, enumerates two-summand relations, charges generator and query CPU, and spends a total one-sided paired-test alpha below 0.01 over an unbounded seed sequence. It stops when a seed passes both yield and cost screens, so the search can be continued with a larger `--through-seed`; a screen hit would trigger independent replay and a complete-cost study. Each invocation has a wall budget. Only `pkm-smooth-tower` is a valid generator. A diagnostic invocation accidentally used the generic `ic` binary; its eight usage failures remain in `adaptive.jsonl` and are excluded from the valid panel.

```bash
python3 research/pkm_adaptive_sweep_20261123/adaptive.py \
  --bin /Volumes/SSD990-2/crypto/target/release/pkm-smooth-tower \
  --out research/pkm_adaptive_sweep_20261123/adaptive_pkm.jsonl \
  --through-seed 200 --max-wall-seconds 120
```

The generator SHA-256 is `82ae63d60b04e3fbf3c8ad2ada00ab2cd09a03a1609e8ad511a83eab1495aed0`. The completed store `adaptive_pkm.jsonl` SHA-256 is `e07396918946ccae06c2609f62f52f42da4927f11395207f1d2ecbc289080410`. The existing `ic` F4 binary SHA-256 is `06439ae5119b5ed69c413df90e7a65dc67541dd1dfb9f6ac8d77cf2bd4441c59`; its JSON says `git_commit: null`, so the executable is hash-pinned but not source-commit-pinned. No source or performance claim is inferred from that missing field.

## Measured relation screen

| Item | Result |
|:--|:--|
| New seeds | 9–160 inclusive; 152 attempts |
| Complete distinct chains | 151; seed 111 censored by a one-second end-of-call timeout |
| Matched interval controls | 138 distinct sizes, reused for equal sizes |
| Actual lifting base size | 220–3155 |
| Candidate screen wins / confirmation wins / both | 45 / 78 / 23 of 151 |
| Smallest paired confirmation p | 0.0137666 at seed 103; its alpha allocation was 0.00000109649 |
| Smallest setup-charged CPU per relation ratio | 1.86942 at seed 66, candidate/control |
| Median CPU per relation ratio | 7.18722, candidate/control |
| Candidates passing the frozen component gate | 0 |

The ratio uses the same Python relation enumeration on both factor bases, plus the measured native generator CPU for the candidate. It is a component cost and does not include F4 extraction, linear algebra, descent or a full cold target solve. The 48/96 point set is fixed and shared; its reuse is acceptable for paired measurement, with the correction across searched seeds, but the panel cannot be treated as an independent curve-family replication.

An independent direct ordered-pair loop replayed the positive seed-14 target at index 1, target `(167680,66479)`: **two** signed ordered pairs, matching the sweep row. The source hash was checked before replay. See `replay.py`, `replay_seed14_source.json` and `replay_seed14.json`.

## F4 qualification and limits

The existing `ic sweep run --space prime-algebraic` one-trial qualification used generic prime rungs 10,11,12,13; `m=3`, full Semaev system, sampled `t_frac=0.3511368`, degree bound 7; baseline used `m=2` and its fixed interval sizes. Baseline rungs 10 and 11 completed. Baseline rungs 12 and 13 reached the 30-second per-trial limit. The sampled isogeny trial reported ratios 1.667 and 2.316 on the two usable rungs, but its recorded `solver_ops` were zero with 21 and 38 undecided systems. These are not measured F4 successes. The study reported `projected_to_target=false`. Candidate and baseline also differ in factor-base size and Semaev arity, so this call is a diagnostic of the existing sweep path, not a matched PKM-vs-interval speed comparison. See `qualifier.jsonl` and `qualifier.stdout.json`.

This new panel covers the isogeny-kernel construction on one small prime field. It does not add a new p−1 or p+1 measurement, nor a new s59, P-192 or NIST measurement. Those require separate frozen same-target protocols. The previous 56-candidate panel is in `research/pkm_map_sweep_20261123/REPORT.md` and should not be pooled into this panel's 151-chain statistics.

## Next decision

The current chain-family screen has produced no confirmed relation/cost candidate. A genuinely faster approach must reduce algebraic extraction work while preserving same-size relation yield. The next qualified experiment should compare one fixed candidate and same-size interval through identical Semaev arity, solver, timeouts and target walks, retaining every undecided system and setup cost. Further seed-only searches can be resumed, but relation-count fluctuations alone cannot establish a PKM improvement.
