#set page(margin: 1.8cm, numbering: "1")
#set text(size: 9.5pt)
#set heading(numbering: "1.")
#show raw: set text(size: 8pt)

#align(center)[
  #text(size: 15pt, weight: "bold")[PS-7 run report: CDCL with an algebraic leaf, and dynamic branching] \
  #text(size: 10pt)[PDP oracle speedup program, `research/pdp_oracle_speedup_20261011/ps7_cdcl/` --- run of 2026-10-31, 21:14--21:46 UTC]
]

= What ran

`ps7_cdcl/run_ps7.sh` (stages `build test graph fidelity counts verify reg clippy summary` in one lock acquisition, then the `timing` block), `examples/pdp_ps7` (binary SHA-256 in `raw/binary_hash.txt`), on the tree of `origin/main` `f6d0610d1` plus the PS-7 files. Host: Intel Xeon 2.10 GHz KVM guest, 4 vCPU, kernel 6.18.44-fc-v185, rustc 1.97.0; counts do not depend on the host, wall is a note. Protocol: `protocols/PS-7.md` (frozen, unedited); decisions fixed before the run: `ps7_cdcl/DESIGN.md`. Frozen inputs: the first 16 uniform targets of each PS-0 corpus (`ℓ = 8, 9, 10`; 13 refuted and 3 found per rung) and the 60 regression inputs. Arms: `a1` PS-2's plain loop; `a1u` the same loop over the shell's one-pair leaf (control); `a2` the library solver with the leaf as lazy-clause theory; `a2p` its bare-guard encoding; `a2l` the same solver code with a lexicographic decider; `a3` the same with the re-evaluated minimum vertex cover decider. Requested and completed: 6 arms × 48 uniform targets (first model) and again with every model (full root set), 6 arms × 60 regression inputs twice, 5 arms on the 46 applicable ones, the library-versus-copy fidelity check on 48 targets, 54 timed runs; all completed, none failed. Earlier attempts (an interrupted run on another tree, a run whose every-model records had a bookkeeping defect, two failed builds) are kept under `raw/` and not used.

= Observations

#figure(
  image("leaves.svg", width: 100%),
  caption: [Leaf visits (solid) and conflicts (hatched) divided by the pair count, mean over the 13 refuted targets of each rung (13 × 3 rungs × 6 arms recorded values; source `ps7_cdcl/summary_rows.jsonl`, copy `summary_rows.jsonl` here). Leaf visits equal the pair count in every arm. `a1`, `a1u` have no conflicts; `a2p`'s solver counts none for the guards it installs without analysis.],
)

#figure(
  image("bitops.svg", width: 100%),
  caption: [Counted bit operations per refuted target: leaf in PS-1's gate model (solid) and shell at 64 bit operations per counted solver step (hatched, a stated convention). Log scale. `a1`'s leaf figure is low by about 0.5--1.6% (section 9 of the README).],
)

#table(
  columns: (auto, auto, auto, auto, auto, auto, auto),
  align: (right, left, right, right, right, right, right),
  table.header[*ℓ*][*arm*][*pairs P*][*leaves L*][*conflicts*][*P / learned*][*total bit ops (ratio to a1)*],
  [8], [a1], [8,256], [8,256], [-], [-], [62,477,440 (1)],
  [8], [a1u], [8,256], [8,256], [-], [-], [90,689,728 (1.45)],
  [8], [a2], [8,256], [8,256], [8,660], [0.953], [852,449,984 (13.64)],
  [8], [a2p], [8,256], [8,256], [6,369], [1.000], [519,230,272 (8.31)],
  [8], [a2l], [8,256], [8,256], [9,193], [0.898], [635,153,792 (10.17)],
  [8], [a3], [8,256], [8,256], [9,193], [0.898], [1,061,066,240 (16.98)],
  [9], [a1], [32,131], [32,131], [-], [-], [294,779,914 (1)],
  [9], [a1u], [32,131], [32,131], [-], [-], [406,799,206 (1.38)],
  [9], [a2], [32,131], [32,131], [33,083], [0.971], [6,257,520,358 (21.23)],
  [9], [a2p], [32,131], [32,131], [23,626], [1.000], [2,389,939,622 (8.11)],
  [9], [a2l], [32,131], [32,131], [35,536], [0.904], [3,750,586,150 (12.72)],
  [9], [a3], [32,131], [32,131], [35,536], [0.904], [6,021,078,694 (20.43)],
  [10], [a1], [107,880], [107,880], [-], [-], [1,111,892,016 (1)],
  [10], [a1u], [107,880], [107,880], [-], [-], [1,608,046,520 (1.45)],
  [10], [a2], [107,880], [107,880], [111,483], [0.968], [42,425,100,920 (38.16)],
  [10], [a2p], [107,880], [107,880], [75,246], [1.000], [19,898,423,736 (17.90)],
  [10], [a2l], [107,880], [107,880], [122,845], [0.878], [17,837,296,248 (16.04)],
  [10], [a3], [107,880], [107,880], [122,845], [0.878], [28,275,573,240 (25.43)],
)

Means per refuted target (13 per rung), from `counts_summary.md`; ratios are arithmetic on the column. For `a2p` the learned clauses are the guards installed (equal to $P$), its conflicts are the solver's own.

*Pairs removed without a leaf call:* 0 on every refuted target of every rung for `a2`, `a2p`, `a2l`, `a3` (the logged per-clause maximum of such pairs is 0 over 78 records). *Pairs in the cube of a learned clause* (`a2l`, `a3`, mean): 181.7, 360.3, 610.5 at `ℓ = 8, 9, 10`: the clauses are mostly shorter than the $2ℓ$ pair literals (resolvents), and every pair in their cubes had already been evaluated. *Dynamic order:* the monomial graph of the descended symmetrised $S_4$ has only cross-summand edges ($K_(ℓ,ℓ,ℓ)$; 75, 108, 108, 192, 243, 300 edges at $(n,ℓ) = (15,5),(17,6),(19,6),(24,8),(27,9),(30,10)$, none inside a summand, minimum vertex cover $2ℓ$); over the 48 uniform targets `a3` took 2,361,391 decisions, 0 of which differ from the lexicographic first unassigned bit, and its conflict, decision and leaf counts equal `a2l`'s. *Fidelity:* library solver against the example-local copy with hooks inactive, every counter equal on 48 of 48 targets. *Verdicts:* every record of every arm equals the PS-0 corpus' verdict (witness in the root set; every-model runs equal the full root set); regression corpus 60 of 60 (31 found including the algebraic-only input, 29 refuted), group-law leaf 46 of 46 where applicable.

#figure(
  image("wall.svg", width: 92%),
  caption: [Wall nanoseconds per target (16 targets, found and refuted), three rounds, `a1b` = `a1` again (A/A); log scale; source `ps7_cdcl/raw/times.jsonl`. All runs inside one `isolated_bench run --cpus 3` block, not marked contended (33.3 other-CPU-seconds in 685 s); a note, not the metric.],
)

Round-1 wall ratios to the plain loop: `a2`, `a2l`, `a3` = 315, 270, 389 at `ℓ = 8`; 852, 620, 819 at `ℓ = 9`; 2,430, 1,603, 1,900 at `ℓ = 10`. A/A (`a1`, `a1b`) differs by up to 12% between a pair of runs (`ℓ = 9`, round 3).

= Criteria, as measured

*Success* (pre-registered reading: pairs removed without a leaf call, more than one per learned clause, at two rungs): not met, 0 at every rung. *Stop* (no pruning at three rungs): met, leaves $=$ pairs on all 39 refuted targets for every realisation of arms 2 and 3. The cube reading (181.7--610.5 pairs per learned clause) exceeds 1 and corresponds to zero avoided leaf calls; both are printed in the README (section 5) and the frozen sentence does not say which is meant.

= Missing or not done

The instrumented field-operation cross-check was done on another tree only (`raw/interrupted_ps2tree/count_check.jsonl`; the one-pair leaf's counts are exact, the plain loop's products are 0.5--1.8% low) and cannot be rebuilt on `main`, whose `semaev_decomp.rs` lacks the `opcount` module. Bit operations were not computed for the regression corpus (no gate model at $n = 15, 17, 19$). The upstream WDSat input files, WDSat/Xorricane/Bosphorus arms (out of scope in the protocol), rungs above $ℓ = 10$ and theories that fire earlier than $m - 1$ summands were not run. The shell's weight (64 bit operations per counted step) is a convention; the ordering `a1` $<$ `a1u` $<$ every shell arm does not depend on it. The solver copy used by `a2l`, `a3` is generated from the library file (`gen_sat_hooked.sh`). Wall is one isolated block, not per-run records, on a different host class from PS-0..PS-3. No `benchmarks/` entry was added (counted-unit null result, no validation artifacts exist; see README section 10). Nothing end to end; nothing for ECC2K-130.
