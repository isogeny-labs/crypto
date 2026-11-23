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

## 2026-11-23 matched F4 continuation

The follow-up did run a same-size, same-arity actual Semaev F4 comparison. The fixed generic 10-bit instance was `icv1-fp10-t5-192cb216`. The baseline was `prime-abscissa:below=8`; a size-only preflight selected the first ordinary auxiliary-curve 2-isogeny tower with four lifting abscissae, `prime-isogeny:t=3,seed=1,aux=ordinary`. This map composes **three degree-2 Vélu maps into degree 8**. Both arms used m=2, degree bound 8, identical planted target seed, one rho context run and a 300-trial cap. Both verified scalar 468. The interval needed 85 target trials and 3,023,034 total GAE; the isogeny needed 166 and 4,831,568 total GAE, a candidate/control ratio of 1.598. The isogeny's F4 subtotal was slightly smaller (6,404,385 versus 6,704,397 multiply-adds), but that did not offset its extra relation attempts and setup. The first 100-trial attempt was censored for the isogeny and remains in `f4_isogeny.json`; the 300-trial pair was rerun cold under `AMENDMENT_F4_300_20261123.md`.

`f4_sweeper.py` then tested size-matched ordinary degree-8 towers for auxiliary seeds 0–31 using disjoint three-target screening and 24-target confirmation sets. There were 23 size mismatches, seven screen rejects and two confirmation rejects. Seeds 7 and 14 had aggregate charged-cost ratios 0.817 and 0.777 on their 24 confirmation targets, but only 14/24 paired cost wins each (one-sided sign p=0.2706); neither passed the frozen gate. These ratios are selection-panel estimates, with broad target-to-target variation. Full JSON and every decision are in `matched_f4.jsonl` (SHA-256 `22d6e58037373d3099394fd6ffeaff92148106bd95ee71c2a60ed3e72150c078`).

A separate protocol selected seed 14 for **64 fresh planted targets** (20261150–20261213), with no overlap with the selection targets. All 64 candidate/control pairs completed and verified matching scalars. The candidate/interval ratio of summed total GAE was **0.8937**, external wall ratio **0.9159**, and child CPU ratio **0.7602**. The predeclared paired-bootstrap 99% interval for the GAE ratio was **[0.6382, 1.2368]**, and 39/64 individual targets favored the isogeny. The point estimate met the <0.90 threshold, but the interval did not exclude 1, so the replication gate failed. The `replication_seed14.jsonl` SHA-256 is `d2654ba55987e0accc3fa50b623bef959cea6b753358616c8471695193cd9e5a`. This remains a small-instance lead to investigate, not a confirmed faster relation map; it supplies no P-192, s59 or NIST inference.

## Degree-16 map and random-scalar validation

The next frozen rung changed map degree, not just the auxiliary search seed: `prime-isogeny:t=4,seed=1,aux=ordinary` composes **four degree-2 Vélu isogenies, total degree 16**. `prime-abscissa:below=16` and the chosen isogeny fiber each have six lifting abscissae on the same 10-bit instance. The identical m=2 Semaev F4 degree-8 chain oracle, target seed, rho context, trial and solver caps applied to both arms. Of 32 auxiliary seeds, 28 had a different base size, three failed the three-target screen, and seed 1 reached but failed the original 24-target sign-test gate (16 wins, p=0.0758 versus its allocated alpha). The ratio of aggregate total GAE on that selected panel was 0.786, but it was not a qualifying result. Full decisions: `matched_f4_height4.jsonl`, SHA-256 `36d467045cd1c033d359a739768d6f1bac6ca52a23547be7d8395ebebe83d05d`.

An initial 128-target replication accidentally ran a literal t=3 candidate against the t=4 interval. Its four-versus-six base-size mismatch failed the final gate. That invalid run is preserved in `replication_height4_seed1.jsonl` (SHA-256 `c1fe16a489cc10551b8256e56f6d56bde3317f197852988d4e2939c4ed8ee571`), with no performance estimate used. `AMENDMENT_F4_HEIGHT4_REPLICATION_RETRY_20261123.md` fixed the command and added an immediate size check. The corrected 128 consecutive-scalar panel passed its component gate: charged GAE ratio 0.6536, paired-bootstrap 99% interval [0.5119,0.7990], external wall ratio 0.7002, child CPU ratio 0.7879, 92/128 paired wins. Its SHA-256 is `44bbbc19289a0d9bd907f2c19e515f6394fa4f245bc94cd60f07ef3f211c8b00`. Source audit then showed `ic bench` plants `1+seed mod (r−1)`, so consecutive CLI seeds were consecutive scalars, a limitation of that panel.

`PROTOCOL_F4_HEIGHT4_RANDOM_20261123.md` froze an additional panel of **128 unique subgroup scalars sampled uniformly without replacement**, excluding all earlier panel scalars. The exact list was written to the store header before any benchmark row. All 128 cold candidate/control pairs completed, had six-abscissa bases, and verified the selected identical scalar. Results:

| Random-scalar 128-pair metric | Degree-16 isogeny / interval |
|:--|--:|
| Sum of complete, setup-charged total GAE | **0.62857** (720,512,071 / 1,146,280,036) |
| Paired-bootstrap 99% interval, 10,000 resamples | **[0.54982, 0.72185]** |
| Sum of external wall time | **0.67846** |
| Sum of child CPU | **0.72280** |
| Per-target charged-cost wins | **102 / 128** |
| F4 multiply-adds | **4,249,506,827 / 35,223,981,352** |
| Relation target trials | **8,455 / 9,112** |
| Raw relations found | **703 / 726** |

The predeclared **small-instance relation-extraction component gate passed**. The useful result is lower total cost per completed relation pipeline, driven mainly by much cheaper F4 systems; raw relation probability changed modestly. The random-panel store SHA-256 is `de0291fd492c7b534515ed29e3012ce5cc7c6d0ad1eb8bb73467a72b230adcdf`. The binary's embedded `git_commit` is null, so its SHA-256 pins execution but source provenance requires an independent clean rebuild. This benchmark is on a ~10-bit subgroup. The candidate's median reported `s_over_rho` is about **53,088**, so even this improved map is far from the same-target rho reference in the repository's counted-cost model. It does not establish a faster complete same-target ECDLP than optimized rho, an asymptotic advantage, or performance on s59, P-192 or NIST curves. Those remain distinct gates.

## Source replay and first larger-rung diagnostic

An isolated `cargo build --release --locked --bin ic` from the clean PR source tree (`src` tree hash `c544168a0d8c2babc0a5fd26ad793fe59701e66b`) with Rust 1.99.1 failed. The first pass also lacked `include_str!` inputs because the worktree was sparse; after those paths were added, the second pass failed solely with ten unresolved-name `E0425` errors in unrelated `src/cryptanalysis/prime_root_trace.rs`. The retry stderr SHA-256 is `db43408a3ab9695962d933cbb5e3f5826581d9cc8dc94d8197be46f12b016d71`. Thus an independently rebuilt-binary replay has **not** occurred. The old executable remains pinned by its SHA-256 and the full raw JSON records remain replayable when source compilation is repaired.

For an 11-bit diagnostic, `ic bench` changed its generated generic curve when either `--seed` or `--repeats` changed. The first scalar-by-seed runner stopped on a size mismatch after one row (`scale11.jsonl`). A fixed-seed 32-repeat pair then used the same instance `icv1-fp11-t55-257fce57` and equal ten-abscissa bases in both arms. All 32 paired rows completed and verified matching scalars. The interval had total 779,093,756 GAE and 9,787,844,546 F4 multiply-adds; the isogeny had 240,493,964 GAE and 1,236,616,534 F4 multiply-adds, a charged-cost ratio **0.3087**. External wall was 33.48 versus 9.48 seconds, while child CPU was 47.48 versus 60.60 seconds, so the CPU direction is opposite. The initial size-only preflight was on a different curve and cannot qualify the run. A later exact 32-repeat preflight confirmed that auxiliary seed 10 is the *first* size-matched seed on the timed curve as well; this is post hoc verification, not a predeclared selection. The 11-bit row is exploratory scale evidence only. It does not extend the 10-bit random-scalar confidence interval.
