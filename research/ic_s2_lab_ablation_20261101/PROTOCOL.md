---
title: "S2 protocol: which part of the lab arm produces its edge over WDSat (2026-11-01)"
---

Committed before any S2 run. Owner approved S2 on 2026-11-01. Branch `research/ic-s2-lab-ablation-20261101` off `bb718445e3` (R2W). Pre-freeze; nothing here is a Register-E row.

## Question

R2W: the lab arm (`ic.pipeline`, oracle `pdp3-binary:m=3,engine=inherited-f4,degree=3,node_budget=8192`) is 2.0 to 8.7 times cheaper per target than the published-method arm with WDSat (whole run, each arm at its own best base dimension l). Which component of the lab arm produces that edge?

## What was looked at before this protocol was written (disclosed)

- The code of the lab arm (listed below) and the R2W report and records.
- **Free decomposition of the R2W WDSat whole-run figure** from the committed R2W records (child = ecbench child Ir less 3.51e7; WDSat processes = `wdsat_ir`; start-up = calls x the R2W per-l figure). Medians over 8 targets, WDSat best-whole l: K_1 n=11 l8: child 2.02e9, WDSat processes 1.83e9 (of which start-up 4.7e8); K_0 n=13 l8: child 2.93e9, WDSat 2.71e9 (6.0e8); K_0 n=15 l10: child 6.05e9, WDSat 7.39e9 (7.8e8). The lab arm's entire run at its best l is 4.6e8, 1.03e9, 1.85e9. So the Rust child of the WDSat arm alone is 2.9 to 4.4 times the whole lab run. This is observed, not an ablation; the S2 runs below test it.

## Components of the lab arm that differ from the published (WDSat) arm

Found in `src/bin/ecbench/pdp3_koblitz.rs`, `koblitz_index_calculus::groebner_decompose`, `koblitz_groebner.rs`, `inherited_f4.rs`, `polynomial_reuse.rs`:

| id | component | lab arm | WDSat arm (R5 glue) | switch for the ablation |
|:--|:--|:--|:--|:--|
| F | formulation | `S_3` chain `S_3(x1,x2,e), S_3(e,x3,x_R)`, 3l+n Boolean unknowns, degree 3, interleaved variable order | `S_3`-symmetrised `S_4`, e-form, 9l-3 unknowns, ANF text | **no switch**: feeding F4 the e-form needs 9l-3 > 64 unknowns (`u64` monomial masks) for l >= 8, and the x-form has degree > 3. Confounded with the solver; probed only through the paired per-system cost (E3). Not ablated. |
| T | template reuse across systems | `polynomial_reuse`: target-independent template built once per process, each system instantiated by word XORs | S4 system rebuilt and re-rendered to ANF text per system | `IC_TEMPLATE_MEMO=0` (X1) |
| H | harness / process | one in-process solve, no process, no files | one `wdsat_solver` process per system (start-up 1.4e7 to 2.1e7 Ir), ANF written to disk, model text parsed | not switchable in the lab; measured on the WDSat side (E0 decomposition, E3 in-process emitter cost, process start-up) |
| I | inherited bases | children specialise the parent's reduced Macaulay basis | n/a (CDCL + XOR-Gauss search) | `KIC_F4_INHERIT=0` with `KIC_LINEAR_ELIM=1`, `SOLVER_SPLIT_RULE=highest` (X3: from-scratch matrix F4 per node, other policies kept) |
| L | linear elimination at each node | on under inherited F4 | WDSat's XOR-Gauss (`-x`) | `KIC_LINEAR_ELIM=0` (X2) |
| S | split rule | highest-free (`Auto` under inherited F4) | WDSat branching (CDCL), `-b` symmetry breaking | `SOLVER_SPLIT_RULE=lowest` (X4) |
| O | chain variable order | interleaved | n/a | `KIC_CHAIN_ORDER=layout` (X5) |
| M | support-local multipliers | on | n/a | `KIC_F4_MULTIPLIERS=occurring` (X6) |
| U | UNSAT detection | degree-3 Macaulay refutation at the root and at nodes (`1` in the basis). **No dedicated early detection**: `KIC_F4_EARLY_ABORT` defaults to 0 and the F6-IC support gate belongs to `engine=f6-ic`, not to this arm | CDCL conflict proof | not a switch: tested by measuring the SAT/UNSAT cost split (E3) |
| B | factor base | `binary-standard-subspace:dimension=l`, the same in both arms | same | none (identical by construction) |
| R | stopping rule | first root that lifts (non-lifting roots are skipped inside the search) | first model (`find=first`); a non-lifting model costs an extra call | counted (`lift_failures`) |

Everything not in the table is shared: the `ic.pipeline` driver, targets, group arithmetic, relation linear algebra and `[d]G = Q` check.

## Ablations (one component switched off or replaced at a time)

Implemented as a new oracle parameter `abl=<id>` (`research/ic_s2_lab_ablation_20261101/abl.patch` on `pdp3_koblitz.rs`, applied in the build tree only): in `prepare`, after the existing refusal check, the listed variables are set with `std::env::set_var` before the first solve (the solver reads them once per process; one ecbench record is one child process, checked by a counter). Absent `abl` = the unchanged lab arm, same method id.

- X1 `memo0`: `IC_TEMPLATE_MEMO=0` (T).
- X2 `nolin`: `KIC_LINEAR_ELIM=0` (L).
- X3 `noinh`: `KIC_F4_INHERIT=0`, `KIC_LINEAR_ELIM=1`, `SOLVER_SPLIT_RULE=highest` (I).
- X4 `splitlow`: `SOLVER_SPLIT_RULE=lowest` (S).
- X5 `chainlayout`: `KIC_CHAIN_ORDER=layout` (O).
- X6 `occmult`: `KIC_F4_MULTIPLIERS=occurring` (M).
- X7 `alloff`: cumulative: `IC_TEMPLATE_MEMO=0`, `KIC_F4_INHERIT=0` (which also restores the historical LowestFree rule and no linear elimination), `KIC_CHAIN_ORDER=layout`, `KIC_F4_MULTIPLIERS=occurring`. Distinguishes "the edge is spread over components" from "no component matters".
- A ablated record that hits the node budget (8192) is reported as `exhausted`; a session with fewer than 8/8 verified targets is reported as failed at that cell with its partial cost, and is not used to satisfy the decision rule.

## Cells

K_1 n=11, K_0 n=13, K_0 n=15 (R2W cells, `target_seed 202610111901`, 8 public targets, `m = 3`, session law of E1/R2/R5/R2W). Lab base dimension l = 5, 6, 7 (R2W lab best). WDSat arm at its R2W whole-run best l = 8, 8, 10, and at its solver-only best (l = 8, 3, 10). Ablations are run at the lab's best l only.

## Metrics

Counted instructions retired (`/usr/bin/time -l`, as R2W), median over the 8 targets: **whole run** = ecbench child Ir + every WDSat process Ir - 3.51e7; **solver-only** = WDSat process Ir less one measured start-up per call (R2W's per-l figures), compared against the lab's whole run as in R2W. Every record verified `[d]G = Q` by ecbench. Each ablated lab number is also compared with a same-session baseline lab run (A/A) of the new binary against R2W (accepted if within 3 %).

**SAT/UNSAT split (E3)** by a per-system probe (new example `s2_split_probe`): random abscissae with a curve point (seeded, 30 systems per cell and l), each system solved by (lab) `groebner_decompose` with in-process instruction counts (`proc_pid_rusage` `ri_instructions`, checked against `time -l` on one session), (WDSat a) and (WDSat b) through the R5 emitter with per-process Ir and the emitter's own in-process Ir. A system is SAT iff WDSat (a) returns a model; the lab's found/none outcome is recorded next to it and disagreements are reported. Probe l: the lab's best l and the WDSat best l of each cell (so that both arms are compared on the same systems at both). Mean and median cost per class, and share of cost in UNSAT systems.

## Decision rule (fixed here)

Let `L0` = baseline lab median Ir (new binary, same metric), `W` = reference WDSat median Ir, `A` = ablated lab median Ir. The ablation moves the lab arm "at least halfway to WDSat" on a cell iff `A - L0 >= 0.5 (W - L0)` (linear reading; the log reading `A/L0 >= sqrt(W/L0)` is reported as secondary). A component **explains** the edge iff this holds on every discriminating cell. A cell is *discriminating* for a reference iff `W/L0 >= 1.5`; non-discriminating cells are reported and excluded (the solver-only K_0 n=13 ratio is 1.16 in R2W and is such a cell). Two references: (i) whole run, WDSat at its whole-run best l; (ii) solver-only, WDSat at its solver-only best l. "Best WDSat" is the lower of config (a) and config (b) below. Both (a) and best-WDSat verdicts are reported; the verdict in the report names which reference it uses.

## Amendment 1 (coordinator note, committed before any run): WDSat configuration

The WDSat authors (CP 2020, Table 5) recommend disabling the XOR-Gauss module (XG) for m = 3 (13x to 32x cheaper than XG on in their cell). R2W/R5 ran `-x -b` everywhere. Config (a) = `-x -b` (R2W; `xg=1,sym=1`). Config (b) = no XG, no `-b` (`xg=0,sym=0`: R5 found `-b` without `-x` unusable). Config (c) (`-x` alone) is being probed by agent S1 and is not repeated here. S2 runs: the SAT/UNSAT probe under (a) and (b) on all cells; whole-run sessions of (b) at (a)'s whole-run best l (8, 8, 10), **labelled not swept** (no l sweep of (b)). The WDSat binaries are the same builds (XG is a run-time flag). (b)'s root sets are checked against (a)'s by the probe (SAT/UNSAT agreement, every model re-checked against the symmetrised S4). If (b) fails (parse error, wrong status), the verdict continues with (a) and says so.

## Build and environment

Build tree `/Volumes/SSD990-2/s2-build/tree` = R2W's tree (R5 glue + E1 child-time shim + `WDSAT_TMP_DEFAULT` patch, copied read-only from S1's tree and checked identical to `git archive bb718445e3` + those patches) plus `abl.patch`; `CARGO_TARGET_DIR=/Volumes/SSD990-2/s2-build/target`, `TMPDIR` and `WDSAT_TMP_DEFAULT` in `/Volumes/SSD990-2/s2-build`. The WDSat source is a read-only copy of S1's `wdsat_src` (`/tmp/claude-501/wdsat_src` no longer exists); its commit cannot be checked (no `.git`), only file sizes against R5's account. Sandbox off for builds and `time -l`. No push, no PR, no download; WDSat's prebuilt binary is never run or committed.

## Order

(1) this protocol and RESUME, S3 checkpoint gate; (2) build, A/A of the lab at its best l against R2W; (3) probes E3; (4) ablation sessions X1..X7, largest expected effect first (X7, X1, X3, X2, X4, X5, X6); (5) (b) whole-run sessions; (6) report. Budget about 3 h of compute; a session projected over 1 h is skipped and reported.
