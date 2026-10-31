# PS-7: CDCL with an algebraic leaf, and dynamic branching

Protocol: [`../protocols/PS-7.md`](../protocols/PS-7.md) (frozen by the plan; not edited).  Plan: [`../PLAN.md`](../PLAN.md).  Baseline:
[`../baseline_v1/`](../baseline_v1/README.md).  Leaf: PS-2's group-law kernel ([`../ps2_scalar/`](../ps2_scalar/README.md)).  Decisions fixed before
the first measured run: [`DESIGN.md`](DESIGN.md).  Run report with graphs and PDF: [`../reports/run-20261031T2114Z/`](../reports/run-20261031T2114Z/).
Class: **boundary** (the protocol: "boundary if negative"; the result is negative).  Conductor task T-4 (program task, not claimed: see section 9).
Commands: [`run_ps7.sh`](run_ps7.sh); raw records: [`raw/`](raw/); code: [`../../../examples/pdp_ps7/`](../../../examples/pdp_ps7/).

Everything below is an **observation** of one run of `run_ps7.sh` on one host class, or arithmetic on those observations shown with its
inputs.  Interpretation is in its own section and says so.  The measurement is an oracle-stage one (a decision problem on one abscissa); it is not
an end-to-end ECDLP cost, `S` is not computed, and nothing here touches the ratio to a floor or to rho.

## 1. Requested and executed

| PS-7 requirement | status | evidence |
|:--|:--|:--|
| Arm 1: the plain pairs loop, PS-2's leaf | **verified complete** (`a1` is PS-2's `ScalarKernel` itself; `a1u` is the same pair loop over the shell's one-pair-at-a-time leaf, a control) | §2, §4 |
| Arm 2: CDCL shell, the leaf as a theory propagator that fires once `m − 1 = 2` summands are assigned, through `solve_with_lazy_clauses` as a library user | **verified complete** for `a2` (the library solver as shipped) and `a2p` (the bare-guard encoding, the library's level-0 path); `a2l` runs the same solver code from an example-local copy (needed for a branching hook, §2) | §2, §4 |
| Arm 3: the same with dynamic branching order, a re-evaluated minimum vertex cover of the monomial graph per node | **verified complete**, on the example-local solver copy (the library has no branching hook); the measured finding is that the rule reproduces the lexicographic order | §2, §6 |
| Frozen input: the 60-input regression corpus | **partial**: the 60 instances' parameters (field, target abscissa, label) from `semaev_corpus::CORPUS`, regenerated with the repository's own encoder-free leaf, **not** the upstream DIMACS/ANF files, and the leaf is PS-0's algebraic enumerator (quartic and `gcd`), not PS-2's, because 14 of the 60 targets are not abscissae of rational points or are degenerate (§7); no bit operations for this corpus | §7 |
| Frozen input: uniform targets at `ℓ = 8, 9, 10`, 16 per rung, seed 20261011, targets are curve points | **verified complete**: the first 16 uniform targets of the PS-0 corpus per rung (PS-0's convention: SplitMix64 seeded `20261011 + ℓ`), re-derived from the generator and compared with the stored file on every run; 13 refuted and 3 found per rung | §3 |
| Unit: leaves visited per target, pairs pruned per learned clause, conflicts, counted bit operations, wall as a note | **verified complete** on the uniform rungs; bit operations **not computed** for the regression corpus (no gate model at `n = 15, 17, 19`); wall is a note | §4, §5, §8 |
| Verdicts identical across arms, checked against the PS-0 corpus | **verified complete**: 6 arms × 3 rungs × 16 targets, first model and every model (full root set); 6 arms × 60 inputs (and the group-law leaf on the 46 applicable ones) | §3 |
| Inadmissible: conflicts without leaves; changing the branching order between arms of one comparison; leaf cost counted as zero | **not committed**: conflicts are always printed next to leaves; arms `a1`/`a2l` share the lexicographic order and `a2l`/`a3` differ only in the decider; the leaf is priced in every arm | §4, DESIGN D3, D6 |
| Success: any arm prunes more than one pair per learned clause on average at two rungs, identical verdicts | **not met** under the pre-registered reading (pairs saved without a leaf call): 0 at every rung, for every realisation; see §5 for the other readings | §5 |
| Stop: no pruning at three rungs: close the direction and record it | **met**: leaves visited equal the pair count on all 39 refuted targets (13 per rung) for every realisation of arms 2 and 3 | §5 |
| The library solver copy behaves as the library | **verified complete**: `a2h` against `a2`, 48 targets, every solver counter equal | §2 |
| Counted field operations cross-checked against instrumented counters | **partial**: done on the tree of `exp/pdp-ps2-scalar` (records kept), **not reproducible on `main`**: `main`'s `semaev_decomp.rs` has no `opcount` module although `Cargo.toml` still declares the feature | §9 |
| Wall time as a note, A/A | **see §8** | §8 |
| Report, graph, PDF (research-visuals) | **verified complete** | [`../reports/run-20261031T2114Z/`](../reports/run-20261031T2114Z/) |
| `benchmarks/` entry and row; validation gate | **not added, deliberately**: a counted-unit null result with a wall note, no rho reference; the gate keeps such results in their research directory | §10 |

## 2. What was built

`examples/pdp_ps7/` (one binary, `pdp_ps7`; `rustfmt --edition 2021` clean, `cargo clippy` clean for the example, 5 unit tests).

| arm | what runs | order | conflict explanation |
|:--|:--|:--|:--|
| `a1`  | PS-2's `ScalarKernel` (row-batched inversions) | lexicographic | none |
| `a1u` | the pair loop over the shell's leaf (control) | lexicographic | none |
| `a2`  | **library** `Solver::solve_with_lazy_clauses`, priority on the pair bits, phase 0 first | the solver's own (VSIDS within the priority class) | `g ∨ d`, `g ∨ ¬d` |
| `a2p` | the library, the bare guard `g` (the solver backjumps to level 0 and counts no conflict) | as `a2` | none analysed |
| `a2l` | the solver code from `examples/pdp_ps7/sat_hooked.rs`, `decide` hook = lexicographically first unassigned bit, 0 first | static, the plain loop's | as `a2` |
| `a2h` | the same copy with its hooks inactive (fidelity control) | as `a2` | as `a2` |
| `a3`  | the same copy, `decide` hook = the re-evaluated minimum-vertex-cover rule (§6) | dynamic | as `a2` |

* **Pair space.**  Sorted pairs `{(a, b) : a ≤ b}` over the leaf's candidate set: the rational base abscissae `B` of `V` (`|B| = 128, 253, 464` at `ℓ = 8, 9, 10`,
  so `8,256; 32,131; 107,880` pairs), the set PS-2's kernel enumerates.  The shell gets it as static clauses (`X₁ ≤ X₂`; one clause per summand and per value outside `B`).
* **Theory.**  When both bit-vectors `X₁, X₂` are assigned, the leaf returns the third abscissae in `V`.  None: the explanation `g` (the negation of the `2ℓ` pair literals) is
  installed as `g ∨ d` and `g ∨ ¬d` on one fresh variable `d`, so the solver propagates `d`, meets the second clause falsified and analyses an ordinary conflict.  Some third `t`:
  `g ∨ (X₃ = t)` per bit.  This is the only use of the library hook that produces analysed conflicts; the library's own path for a bare falsified clause is `a2p`.
* **Why a solver copy.**  `src/cryptanalysis/sat.rs` has no branching hook and no learnt-clause hook, and tracked files were off limits.  `gen_sat_hooked.sh` writes
  `sat_hooked.rs` = `sat.rs` (git blob `41c480f1e42b…`, identical on `main`) with the tests cut off, plus `Solver::solve_with_hooks` (a copy of `solve_with_lazy_clauses`
  with a `decide` hook and a `learnt` hook) and a trait.  `gen_sat_hooked.sh --check` passes on the tree of this PR.
* **Fidelity.**  `fidelity` runs `a2` (library) and `a2h` (copy, hooks inactive) with all models enumerated on the 3 × 16 targets: the verdict, the roots, the leaf visits and
  **every solver counter** (decisions, conflicts, restarts, propagations, learnt clauses, clause and literal visits, analysis visits, learnt literals, levels) are equal on all
  48 targets (`raw/fidelity.jsonl`, 48 `identical: true`, 0 differing).

## 3. Identical outputs

Checked against the PS-0 corpus (`../baseline_v1/corpus/corpus_l{8,9,10}.txt`) on every run: the arm's verdict equals the corpus verdict, and the witness is in the corpus' root set.

| run | scope | result |
|:--|:--|:--|
| `counts_l{8,9,10}.jsonl` | first model or refutation, 6 arms × 16 targets per rung | 288 records, all `ok` (`ok: 16/16` in each of the 18 summaries) |
| `verify_l{8,9,10}.jsonl` | every model with blocking clauses: the arm's full root set must equal the corpus' full root set | 6 arms × 16 targets per rung, `ok: 16/16` in each of the 18 summaries |
| `reg_counts.jsonl`, `reg_verify.jsonl` | the 60 regression inputs, first model and every model, 6 arms, PS-0's quartic leaf | `ok: 60/60` in each of the 12 summaries; reference = exhaustive search of every sorted triple of `V³` through the crate's general `S₄` evaluator (30 labelled satisfiable + the algebraic-only `n19l6-19-U` = 31 found, 29 refuted) |
| `reg_group.jsonl` | the group-law leaf on the regression inputs where it applies (liftable and not degenerate, PS-1's theorem) | 46 of 60 applicable, `ok: 46/46` in each of the 5 summaries; 14 skipped, named in the summaries |

The uniform targets are curve points (`ℓ = 8, 9, 10`: `icv1-f2m24-tm5983-28fa8b30`, `icv1-f2m27-t7555-cb6a4c9d`, `icv1-f2m30-t10089-8ab21610`, PS-0's registered models).
A first complete run of this tree had a bookkeeping defect in the every-model records (the verdict was read from the solver's final `Unsat`); it is kept in
`raw/first_main_run_binary_A/` with a note, and the run reported here is the rerun with the corrected binary.

## 4. Counted results on the uniform rungs

Refuted targets (13 per rung, every pair must be evaluated or removed; the quantity the protocol's pruning criterion is about).  Means per target.  `learned` = analysed conflicts
(for `a2p`: the guards installed).  Bit operations: the leaf in PS-1's gate model (fused ternary-logic issues, [`ps1_algebra/raw/gates.jsonl`](../ps1_algebra/raw/gates.jsonl)) from counted field
operations, plus the shell's counted steps at 64 bit operations each (a convention: the repository's word-XOR convention; a lower bound on what a clause visit costs).  Ratios are
arithmetic (`total / total of a1`).

| `ℓ` | arm | pairs `P` | leaves `L` | conflicts | learned | `P / learned` | pairs saved `P − L` | leaf bit ops | shell bit ops | total bit ops | total / `a1` |
|--:|:--|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 8 | `a1`  | 8,256 | 8,256 | - | - | - | 0 | 62,477,440 | - | 62,477,440 | 1 |
| 8 | `a1u` | 8,256 | 8,256 | - | - | - | 0 | 90,689,728 | - | 90,689,728 | 1.45 |
| 8 | `a2`  | 8,256 | 8,256 | 8,660 | 8,660 | 0.953 | 0 | 90,689,728 | 761,760,256 | 852,449,984 | 13.64 |
| 8 | `a2p` | 8,256 | 8,256 | 6,369 | 8,256 | 1.000 | 0 | 90,689,728 | 428,540,544 | 519,230,272 | 8.31 |
| 8 | `a2l` | 8,256 | 8,256 | 9,193 | 9,193 | 0.898 | 0 | 90,689,728 | 544,464,064 | 635,153,792 | 10.17 |
| 8 | `a3`  | 8,256 | 8,256 | 9,193 | 9,193 | 0.898 | 0 | 90,689,728 | 970,376,512 | 1,061,066,240 | 16.98 |
| 9 | `a1`  | 32,131 | 32,131 | - | - | - | 0 | 294,779,914 | - | 294,779,914 | 1 |
| 9 | `a1u` | 32,131 | 32,131 | - | - | - | 0 | 406,799,206 | - | 406,799,206 | 1.38 |
| 9 | `a2`  | 32,131 | 32,131 | 33,083 | 33,083 | 0.971 | 0 | 406,799,206 | 5,850,721,152 | 6,257,520,358 | 21.23 |
| 9 | `a2p` | 32,131 | 32,131 | 23,626 | 32,131 | 1.000 | 0 | 406,799,206 | 1,983,140,416 | 2,389,939,622 | 8.11 |
| 9 | `a2l` | 32,131 | 32,131 | 35,536 | 35,536 | 0.904 | 0 | 406,799,206 | 3,343,786,944 | 3,750,586,150 | 12.72 |
| 9 | `a3`  | 32,131 | 32,131 | 35,536 | 35,536 | 0.904 | 0 | 406,799,206 | 5,614,279,488 | 6,021,078,694 | 20.43 |
| 10 | `a1`  | 107,880 | 107,880 | - | - | - | 0 | 1,111,892,016 | - | 1,111,892,016 | 1 |
| 10 | `a1u` | 107,880 | 107,880 | - | - | - | 0 | 1,608,046,520 | - | 1,608,046,520 | 1.45 |
| 10 | `a2`  | 107,880 | 107,880 | 111,483 | 111,483 | 0.968 | 0 | 1,608,046,520 | 40,817,054,400 | 42,425,100,920 | 38.16 |
| 10 | `a2p` | 107,880 | 107,880 | 75,246 | 107,880 | 1.000 | 0 | 1,608,046,520 | 18,290,377,216 | 19,898,423,736 | 17.90 |
| 10 | `a2l` | 107,880 | 107,880 | 122,845 | 122,845 | 0.878 | 0 | 1,608,046,520 | 16,229,249,728 | 17,837,296,248 | 16.04 |
| 10 | `a3`  | 107,880 | 107,880 | 122,845 | 122,845 | 0.878 | 0 | 1,608,046,520 | 26,667,526,720 | 28,275,573,240 | 25.43 |

(`counts_summary.md` has every column, the found class, and the wall column; `summary_rows.jsonl` is the same as data.)  Observations:

* **Leaves visited equal the pair count in every arm on every refuted target** (`pairs saved = 0` in all 4 × 3 shell rows, `13` targets each).  No pair is removed without a leaf call.
* **Conflicts exceed leaves** in `a2`, `a2l`, `a3` (by 4.9%, 11.3%, 3.0%, 10.6%, 3.3%, 13.9% over the six rows `a2 8, a2l 8, a2 9, a2l 9, a2 10, a2l 10`, derived as `conflicts/P − 1`):
  the extra conflicts come from the static clauses (order and candidate-set clauses), not from the leaf.  `a2p`'s solver counts fewer conflicts (6,369; 23,626; 75,246) because the bare guard is
  installed without analysis; its 8,256; 32,131; 107,880 guards are its learned clauses.
* **The leaf priced alone** (`a1u` against `a1`) is 1.45×, 1.38×, 1.45× the plain loop's: the shell cannot batch the pair denominators' inversion across a row (one inversion per pair instead of
  one per row); this is a property of a per-pair theory, not of the CDCL machinery.
* **Shell work** (64 bit operations per counted step) is 4.7× to 25.4× the leaf's in the four shell arms (derived: `shell / leaf`, from 4.72 for `a2p` at `ℓ = 8` to 25.38 for `a2` at `ℓ = 10`).  The ordering `a1 < a1u < every shell arm` holds for **every** weight of a shell step, including 0,
  because the shell arms evaluate the same leaves as `a1u` and add shell work to them.  At weight 1 (one bit operation per counted step) `a2l` at `ℓ = 10` is `(1,608,046,520 + 16,229,249,728/64)/1,111,892,016 = 1.67×` the plain loop.
* **Found targets** (3 per rung; leaves until the first model, mean): `ℓ = 8`: `a1` 3,124.7, `a2` 1,522.7, `a2p` 1,602.7, `a2l` 3,124.7, `a3` 3,124.7; `ℓ = 9`: 9,768.3, 10,834.0, 11,662.3, 9,768.3, 9,768.3;
  `ℓ = 10`: 24,663.0, 20,604.7, 9,783.0, 24,663.0, 24,663.0.  The numbers differ by search order (the native and bare-guard arms use the solver's own order); the unvisited remainder after the first
  model is early exit, **not** pruning.  Arms with the same order (`a1`, `a2l`, `a3`) visit the same number of leaves.

## 5. Pairs pruned per learned clause, and the criteria

"Learned clause" is read as in a CDCL solver, one clause per analysed conflict.  Three readings of "prunes", all measured:

| reading | `ℓ = 8` | `ℓ = 9` | `ℓ = 10` | what it is |
|:--|--:|--:|--:|:--|
| **pairs removed without a leaf call, per learned clause** (the pre-registered reading, DESIGN D6): `(P − L) / learned` | 0 | 0 | 0 | all arms 2 and 3, refuted targets |
| pairs excluded per learned clause, `P / learned` | `a2` 0.953, `a2l`/`a3` 0.898, `a2p` 1.000 | 0.971, 0.904, 1.000 | 0.968, 0.878, 1.000 | below or at 1 everywhere |
| pairs in the cube of a learned clause, mean (`a2l`, `a3`; the learnt-clause hook is not in the library, so not for `a2`) | 181.7 | 360.3 | 610.5 | pairs whose literals the clause falsifies, whatever their history |

* The third reading exceeds 1 by two orders of magnitude, and it is **not** pruning: nearly every learned clause is shorter than the `2ℓ` pair literals (`shorter than pair`: 119,483 of the
  13 × 9,193 = 119,509 learned clauses at `ℓ = 8` for `a2l`), because conflict analysis drops literals implied by earlier decisions; each such clause is a resolvent that covers pairs the leaf has **already** evaluated
  (the logged count of pairs a clause excludes that were not yet evaluated is 0 for every clause, `acct.saved_sum`).  Reported alone this number would be the "conflicts without leaves" figure the protocol
  rules inadmissible; here it stands beside leaves visited, which equal the pair count.
* **Success (pre-registered reading): not met.**  No realisation (`a2`, `a2p`, `a2l`, `a3`) removes a single pair without a leaf call at any of the three rungs, so there is no rung, let alone two, with
  `P / learned > 1` and `L < P`.  Under the cube reading alone the number is above 1 at all three rungs for `a2l` and `a3`, with `L = P`; the frozen sentence does not say which reading is meant, DESIGN D6 fixed the first before the run, and
  both are printed so that the reader can apply the other.
* **Stop: met.**  `L = P` on all 13 refuted targets at each of `ℓ = 8, 9, 10` for every realisation of arms 2 and 3: no pruning at three rungs.

## 6. The dynamic order (arm 3)

* **The monomial graph** of the library's descended symmetrised `S₄` (`graph_stats.jsonl`): the `3ℓ` coordinate bits are joined only **across** summands: `x`-edges 75 / 108 / 108 / 192 / 243 / 300 for
  `(n, ℓ) = (15, 5), (17, 6), (19, 6), (24, 8), (27, 9), (30, 10)`, **0** edges inside a summand, i.e. the complete tripartite graph `K_{ℓ,ℓ,ℓ}` (not the complete graph on `3ℓ` vertices: 105, 153, 153, 276, 351, 435 edges would be complete);
  its minimum vertex cover has `2ℓ` vertices (two summands).  The Semaev equations form a separate component on the `e` variables (45 to 57 vertices at `ℓ = 8…10`) that no `x` assignment changes.  The survey's remark that the
  monomial graph of `S′₄` is complete is therefore not what this system gives for the `x` bits (a structural fact of the library's two independent variable spaces, plus the measured edge counts above).
* **Re-evaluated per node**, the cover of the residual graph is two of the three summands' unassigned bits; the rule (DESIGN D5) chooses the cover vertex of largest residual degree, ties to the lowest lexicographic position.
* **Measured:** over the 48 uniform targets, `a3` took **2,361,391** MVC decisions, **0** of which differ from the lexicographically first unassigned bit; `a3` and `a2l` have identical conflict, decision and leaf counts on every target
  (the rows above); the 60 regression inputs with the quartic leaf: 0 of the decisions differ (`reg_summary.md`); with the group-law leaf (static clauses present) 14 of 6,648 decisions differ on refuted targets, with no change in the counts.
  The ablation `a3` against `a2l` therefore changes nothing but the decider's own cost (the shell column of `a3` is the larger).

## 7. The regression corpus (60 inputs)

Inputs: `n, ℓ = 15, 5 / 17, 6 / 19, 6`, 20 each, the Koblitz-family curve `y² + xy = x³ + x² + 1`, each instance with its own modulus; 31 found (the labelled-satisfiable 30 and the algebraic-only
`n19l6-19-U`, whose root does not lift to a point), 29 refuted.  Leaf: PS-0's independent enumerator (quartic and `gcd`) over all of `V`, because 14 of the 60 targets are not abscissae of rational points or are
degenerate (PS-1's theorem), so PS-2's leaf does not apply to them, and the corpus' question is algebraic (`reg_summary.md`; `raw/reg_*.jsonl`).  Means per refuted input (29; pairs 1,544.8 on average):

| arm | leaves | conflicts | learned | `P / learned` | pairs saved | cube per learned |
|:--|--:|--:|--:|--:|--:|--:|
| `a1`/`a1u` | 1,544.8 | - | - | - | 0 | - |
| `a2`  | 1,544.8 | 1,556.0 | 1,556.0 | 0.993 | 0 | not logged |
| `a2p` | 1,544.8 | 842.4 | 1,544.8 | 1.000 | 0 | - |
| `a2l`, `a3` | 1,544.8 | 1,544.8 | 1,544.8 | 1.000 | 0 | 82.3 |

On the 46 inputs where the group-law leaf applies (17 refuted), `a2l`/`a3`: 383.6 conflicts for 337.8 pairs, `P / learned` 0.881, pairs saved 0.  The same as the uniform rungs: nothing pruned.
The regression corpus is reported with the same quantities and does not enter the success/stop rule (DESIGN D6).  Bit operations: **not computed** (the quartic leaf has no gate model; PS-1 has none for `n = 15, 17, 19`).
Not done: the upstream WDSat inputs themselves, and any WDSat/Xorricane/Bosphorus arm (out of scope in the protocol: sources unreachable).

## 8. Wall time (a note)

@@WALL@@

## 9. Counted field operations against instrumented counters (cross-check)

`count-check` (built with `--features pdp-opcount`) runs the plain loop and the one-pair leaf on 2 refuted targets per rung and compares the harness' counts with the library's instrumented counters, after subtracting the products
and squarings inside inversions (measured once).  **It could only be run on the tree of `exp/pdp-ps2-scalar`**; the records are in `raw/interrupted_ps2tree/count_check.jsonl` (binary hashes there).  On the tree of this PR the
instrumentation is absent: `main`'s `semaev_decomp.rs` has no `opcount` module, while `Cargo.toml` still declares `pdp-opcount` and `examples/pdp_speedup` still has its `#[cfg(feature = "pdp-opcount")]` code; building
this example with the feature fails on `main`.  That is a **named gap** (the stage `countcheck` of `run_ps7.sh` would fail to build), not a silent skip.  Observations from the recorded run:

| `ℓ` | one-pair leaf (`a1u`): products, squarings, inversions per pair | plain loop (`a1`): products per pair, harness vs instrumented | squarings, inversions |
|--:|:--|:--|:--|
| 8 | 7.062, 4.031, 1.031: harness = instrumented | 10.0155 vs 10.2016 | equal |
| 9 | 7.0315, 4.0157, 1.0157: equal | 10.0079 vs 10.1024 | equal |
| 10 | 7.0172, 4.0086, 1.0086: equal | 10.0043 vs 10.0559 | equal |

So the **one-pair leaf's counts are exact**, and the **plain loop's products are low by 1.8%, 0.9%, 0.5%** (12 products per row; 3 of them are the library's `3k` against the harness' `3(k − 1)` for a batch of `k`, the other 9 are not isolated).
In bit operations that is about +1.6%, +0.9%, +0.5% on `a1`'s leaf figure; every ratio to `a1` above is overstated by at most that much.  Nothing else depends on it.

## 10. Deviations, gaps and decisions, each named

1. **Solver copy for arms `a2l`, `a3`** (§2): the library has no branching hook; verified equal to the library by `a2h`/`a2`.  Arm 2 as a pure library user is `a2` and `a2p`.
2. **"Seed 20261011"** is read in PS-0's convention (base seed plus `ℓ`); the 16 targets are the first uniform ones of the PS-0 corpus, found and refuted as drawn (13 + 3 per rung).  The pruning rule uses the refuted ones.
3. **Regression corpus**: parameters, not the upstream files; PS-0's quartic leaf instead of PS-2's where the target is not a point; no WDSat run; no bit operations (§7).
4. **Plain-loop field operations are derived** from the structure of `kernel.rs`, cross-checked once on another tree (§9), not instrumented on this one.
5. **Shell cost** is a convention (64 bit operations per counted step); raw counters are in every record so another weighting can be applied (§4).
6. **The wall note is one isolated block** (§8), not per-run records.
7. **Instrumented count-check not reproducible on `main`** (§9).
8. **Conductor**: `conductor check` for the scopes `examples/pdp_ps7` and `ps7_cdcl` returned "Clear" at the start; the control plane was unreachable after the container restarts and no task was claimed (T-4 is the program task and is unclaimed).
9. **Hosts**: the first interrupted attempt (`raw/interrupted_ps2tree`) ran on the tree of `exp/pdp-ps2-scalar` in an earlier container; the run reported here is on one container (`raw/host_per_stage.log`, `raw/host_revision.txt`): Intel Xeon @ 2.10 GHz, 4 vCPU, kernel 6.18.44-fc-v185, rustc 1.97.0, binary SHA-256 in `raw/binary_hash.txt`.
10. **Two failed builds** of the sparse source tree (missing path dependency; missing `include_str!` inputs) are kept in `raw/failed_attempts/`; they produced no records.
11. **`benchmarks/` and the validation gate** (AGENTS.md "Performance results also land in `benchmarks/`", "Validation gate"): no entry or row was added.  The result is a counted-unit null (nothing pruned) with wall only a note and no speedup; a row would
    need a `VALIDATION.json` with a replay certificate, a validator report and a matched rho reference, none of which exist for an oracle-stage decision problem, and rows frozen in `benchmarks/PRE_GATE.json` may only shrink, so a new row cannot be
    listed as "unvalidated (pre-gate)".  The gate states that it "never" decides "whether an honest null or negative result may be recorded: those stay in their research directories".  This is the rule applied, flagged for the owner in the PR.
12. **Not attempted**: pruning variants outside the frozen protocol (a theory that fires at `m − 2` summands and installs a whole row, which would raise the clause count removed per learned clause while leaving leaf calls equal, the very "conflicts without leaves" artefact the protocol excludes), rungs above `ℓ = 10`,
    other curves, and any end-to-end cost.

## 11. Interpretation (separate from the observations; bounded by them)

On these inputs the shell adds a conflict-analysis and propagation cost to the same leaf evaluations the plain loop does, and prunes none of them.  The mechanism the numbers show: the leaf's refutation explains only the pair it was asked about
(the guard), nothing in the polynomial structure of the monomial graph (`K_{ℓ,ℓ,ℓ}`) offers a shorter reason, and the learned clauses that are shorter than a guard are resolvents of pairs already evaluated.  This does not show that no pruning theory
exists: a theory that fires earlier or reasons about a coset of pairs is outside the frozen protocol and was not tried.  The result is a stage-level, oracle-stage statement on one host class (x86-64 Xeon, 4 vCPU); nothing was measured on Arm64, GPU or FPGA classes.

## 12. Not claimed

No speedup; no end-to-end or `S` figure; nothing about ECC2K-130 (the `AGENTS.md` section 8a gate is untouched); no statement about the upstream WDSat benchmark runs; no statement beyond `ℓ = 10` and the 60 regression inputs; wall figures are a note on a shared container,
not the repository's isolation receipt.

## Reproduce

```bash
export PS7_MAIN_SHA=$(git rev-parse origin/main)
python3 tools/isolated_bench.py busy --wait -- env PS7_LOCKED=1 PS7_STAGES=core research/pdp_oracle_speedup_20261011/ps7_cdcl/run_ps7.sh   # build, test, graph, fidelity, counts, verify, reg, clippy, summary
PS7_STAGES=timing research/pdp_oracle_speedup_20261011/ps7_cdcl/run_ps7.sh                                                                  # the wall note
```
