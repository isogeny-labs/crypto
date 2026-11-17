# Lead crossbred-m3 — notes  [pcc:unverified]

HEAD: a53cc49711 (branch claude/funny-brown-735uvs)

## Context read
- src/cryptanalysis/crossbred.rs, koblitz_groebner.rs (SolverEngine default = InheritedF4{3}),
  koblitz_index_calculus.rs: `KoblitzIcOptions::default().strategy = DecompositionStrategy::PairTable`
  for every m (the production default oracle is pair-table root finding, not Groebner, at m=2 AND m>=3);
  Groebner (engine InheritedF4{3}) runs only when a caller selects `DecompositionStrategy::Groebner`.
  `DecompositionStrategy::Crossbred` ALREADY exists (crossbred_decompose; params None -> D = system degree,
  k = v/2 capped at 22, max_rows 4000 from CrossbredParams::default()).
- research/notes/ecc2k130/RESEARCH_ECC2K130_CROSSBRED.md (2026-09-20): crossbred vs MatrixF4 only
  (the default engine then); m=3 n=7 (D=3,k=6) and n=9 (D=3,k=12); xb/F4 = 0.023 bit ops, 0.696 wall at n=9;
  zero filters at every (D,k); pipeline: crossbred worst arm at m=3 (0.37x of Groebner).
  It never compared with InheritedF4 (default since later) nor with pair-table at m=3.
- RESEARCH_INHERITED_F4.md: current default on icv1-f2m9 m=3 354,877 word ops; icv1-f2m15 m=3 7,187,062.
- research/rootfind_f456_batch_combined_20261115: frozen ecbench K13 panel (m=3/m=2 batched root finding
  vs F4/F5/InheritedF4/F6-IC at n=13), no data yet; this lead does not duplicate it (different n, adds crossbred,
  oracle-level deterministic counts, not an ecbench session).

## Prediction (written before any measurement)
1. Verdicts: every completed arm agrees with exhaustive enumeration on every target; production crossbred
   (max_rows 4000, k = v/2) may return Unknown (no crossbred space under the row cap) at n >= 9.
2. Bit ops per target, n=9 m=3: InheritedF4 decomposition (first lift, interleaved order) ~ 20-130M bit ops;
   tuned crossbred ~ 30M bit ops; so crossbred within 3x of InheritedF4 either way, no clear win.
   MatrixF4{3} >= 10x crossbred.  n=15 m=3 (ell=4, v=27): InheritedF4 ~ 0.4G bit ops; crossbred maybe
   lower (more equations per variable) — uncertain, guess crossbred 2-5x fewer bit ops.
3. Pair table at m=3: <= |F| probes/target, microseconds; >= 1000x below every algebraic engine in wall time
   at every cell. So even a crossbred win would not change the production default.

## Harness
New example `examples/crossbred_m3_bench.rs` (only file touched; additive). Build 1:
`locked_build.sh $L "crossbred_m3_bench" examples/crossbred_m3_bench.rs`

## Commands run
Previous sessions (exact flags inferred from the outputs; all with `bin/crossbred_m3_bench` from build 1, patch.diff = example only):
- `bin/crossbred_m3_bench 5 --roots`, `7 --roots`, `9 --roots`, `15 --roots` -> runs/n{5,7,9,15}.txt (11-12 targets)
- probes: `11`, `17`, `21` -> "no K_0 curve"; `13 --targets=2 --skip=mf4,xbp` -> runs/probe_n13.txt;
  `31` -> runs/n31.txt (killed by the container restart after the header; MatrixF4 / crossbred_prod at n=31 are minutes per target)
This session (resume, no rebuild):
- `timeout 900 bin/crossbred_m3_bench 13 --targets=6` -> runs/n13.txt
- `timeout 900 bin/crossbred_m3_bench 31 --targets=4 --skip=xbp,mf4` -> runs/n31b.txt
- `timeout 600 bin/crossbred_m3_bench 31 --targets=12 --skip=xbp,mf4` -> runs/n31_t12.txt
- `timeout 400 bin/crossbred_m3_bench 31 --targets=1 --skip=xbp,xbt,inh` -> runs/n31_mf4.txt
- `timeout 400 bin/crossbred_m3_bench 31 --targets=1 --skip=mf4,xbt,inh` -> runs/n31_xbp.txt

## Results  [pcc:unverified]
Units: native bit ops/target = native word ops x 64 (F4: f4_word_ops_total delta; crossbred: extraction+transform+filter+solve-row);
aux = bitop_tally word ops x 64. Wall ms/target indicative only (shared VM).
Oracle level = the decomposition functions as the pipeline calls them (Groebner stops at the first lifting root; crossbred solves the whole
system then lifts).

| n | |F| | v | eqs | cell | pair_table ms | inherited_f4_3 native (aux) | matrix_f4_3 native | crossbred tuned (D,k) native | crossbred prod (k=v/2) native | xb_tuned/inh native |
|--|--|--|--|--|--|--|--|--|--|--|
| 5 | 21 | 17 | 10 | 11/11 yes | 0.026 | 0.104M (3.18M) | 10.3M | (3,8) 1.12M | 1.12M | 10.7x worse |
| 7 | 1 | 16 | 14 | 11/11 no | 0.013 | 0.235M (4.36M) | 4.75M | (3,6) 0.320M | 1.26M | 1.36x worse |
| 9 | 55 | 27 | 18 | 12/12 yes | 0.013 | 2.59M (24.0M) | 17.5M | (3,12) 30.3M | 61.1M | 11.7x worse |
| 13 | 4005 | 49 | 26 | 6/6 yes | 0.181 (+377 ms table build) | 62.7M (236M) | 821M | (4,2) 112G, 6/6 UNKNOWN | 65.3G, 6/6 UNKNOWN | fails |
| 15 | 1 | 27 | 30 | 12/12 no | 0.006 | 11.7M (43.7M) | 186M | (3,8) 10.1M | 741M | 1.16x better |
| 31 | 63 | 46 | 62 | 12/12 no | 0.009 | 1107M (1403M) | 3.36T (1 target, 162 s) | (3,10) 336M | 2.76T (1 target, 343 s) | 3.29x better |

Indicative wall ms/target (same order): n9 inh 1.03 / xb_tuned 71.3; n15 1.74 / 1.87; n31 61.1 / 88.1 (crossbred slower in wall
even where its native counter is 3.3x lower, so the two native counters do not price the same work); n13 inh 7.5 / xb 7266 (unknown).

Verdicts: 0 disagreements vs exhaustive enumeration and 0 bad witnesses on every arm and cell; the only non-agreement is
Unknown (crossbred, both parameter sets, at n=13 where v=49 > eqs=26: kernel beyond max_kernel_dim / extraction short).
Root level (`--roots`, full Boolean root sets, n=5,7,9,15): InheritedF4, MatrixF4 and tuned crossbred root sets equal on
11+11+12+12 targets (n9: 628-707 roots each). Full root sets: crossbred tuned is cheaper than InheritedF4 at every cell
(n9 30.3M vs 522M native bit ops, 17x; n5 1.12M vs 7.27M, 6.5x; n7 0.32M vs 0.62M; n15 10.1M vs 22.7M).
Fingerprints: n5 bc2d6fe8cbe28533, n7 ae3c2751cd35c47e, n9 c7f309329df4dcde, n13 cf640bdf35936099, n15 053ae233138eff15,
n31(12 targets) c3622b9a72e17605.

Caveats: n=7 and n=15 have |F|=1 (degenerate factor base, every cell "no"); n=11,17,21 have no K_0 curve in the library;
n=31 cells are all "no" (|F|^3/6 << 2^31). Largest n where every arm finishes in minutes: 31 (MatrixF4 and production crossbred
need 162 s and 343 s per target there).

## Comparison with the prediction
1. Verdicts: confirmed (all agree; Unknowns only from crossbred). Prod crossbred did NOT go Unknown at n=9 (it did at n=13).
2. n=9: crossbred tuned 30.3M bit ops — matches the ~30M guess. InheritedF4 2.59M native (26.6M with aux): native below the
   20-130M band, total inside it. "Crossbred within 3x either way": falsified on native (11.7x worse), holds on native+aux (1.14x).
   "MatrixF4 >= 10x crossbred": falsified at n=9 (MatrixF4 17.5M < crossbred 30.3M). n=15 InheritedF4 0.4G guess: falsified (11.7M);
   crossbred 2-5x fewer: only 1.16x at n=15, 3.3x at n=31.
3. Pair table >= 1000x below every algebraic engine in wall: falsified as a uniform bound (40x at n=13, 79x at n=9, 290x at n=15,
   6800x at n=31) but pair table is the fastest arm at every cell; its n=13 table build (377 ms) breaks even against InheritedF4
   after ~50 targets.

## Conclusion
No clear crossbred win at m=3 in the oracle the pipeline uses: crossbred beats InheritedF4{3} on native bit ops only on
overdetermined, no-decomposition cells (n=15: 1.16x, n=31: 3.3x), loses ~11x on decomposable cells (it solves the whole system;
Groebner stops at the first lift) and returns Unknown on the underdetermined n=13 cell, and is slower in indicative wall time at
every cell where InheritedF4 runs. The production default (PairTable) is 40-6800x faster than every algebraic engine. So no library
code was changed (status measurement_only). Side finding: `DecompositionStrategy::Crossbred` with default params (k = v/2, capped 22)
is 7-8000x worse than the minimal-k tuned choice (n=31: 2.76T vs 336M bit ops; n=15: 741M vs 10.1M); if anyone selects that strategy,
choosing the smallest k with kernel_dim >= v-k (as the harness does) is the fix — a possible future lead, not justified as a default now.
Full-root-set solving (not used by the pipeline) is where crossbred wins clearly (6.5-17x fewer bit ops than InheritedF4).

## Risks
- Native counters of F4 and crossbred count different inner loops; wall time disagrees with the bit-op ranking at n=31.
- Few targets per cell (6-12); n=7/15 degenerate (|F|=1); n=31 has no "yes" cells; m=3 only, K_0 only.
- Crossbred extraction ops are re-measured by a second deterministic extraction in the harness (not returned by crossbred_decompose).
- MatrixF4 and crossbred_prod at n=31 measured on 1 target.
