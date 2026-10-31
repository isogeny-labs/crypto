---
title: "R5: WDSat on the binary Weil-descended S4 reference cells, 2026-10-31"
---

Experiment R5 of the ECDLP index-calculus program (leads `LEADS_20261030.md`, round 4). Card
`XP-20261024-published-ic-reference-arm`; read `PILOT_20261030.md` sections 6-7 and 9 and `E1_SCALING_20261030.md`
(branch `research/published-ic-reference-arm-20261024`) first. **Pre-freeze: nothing here is a Register-E row.**
Everything below is *measured* (recorded values) unless marked *derived*. Host: Apple M4 Pro, aarch64, instructions retired
from `/usr/bin/time -l`, not Callgrind `Ir`; counts are comparable only within this pilot family.

## 1. Answer

WDSat is a much better solver than msolve on these systems and still does not catch the lab arm, except in the smallest cell on solver time.

- **Solver against solver** (the fair comparison: WDSat's and msolve's own process instructions, nothing of the harness): WDSat (l = 4, first-solution) is **3.7x to 5.0x cheaper than the tuned msolve arm** of E1 on K_0 n=9, K_1 n=11 and K_0 n=13, and 34x cheaper on K_1 n=9.
  That takes E1's solver-only gap to the lab arm (17x to 19x, msolve) down to **4.2x (K_0 n=9), 5.4x (K_1 n=11), 3.7x (K_0 n=13)** at the same l = 4, and to 0.46x (WDSat ahead) on K_1 n=9, where r = 37 needs only 9 mostly satisfiable systems.
- **Whole run** (child + solver, paired per target, 8/8 verified everywhere): WDSat costs **4.8x to 7.0x** the lab arm at l = 4 on K_0 n=9, K_1 n=11, K_0 n=13 and 2.2x on K_1 n=9. This figure carries the harness's own ANF construction, which is far lighter for WDSat (about 4.5e8 per target) than for the E1 msolve arm (x-form expansion, 2^spec substitutions, rendering, parsing and root extraction, about 1.1e10 per target at n = 9, PILOT section 9.1). The "12x to 29x cheaper whole-run than msolve" in the table is therefore mostly harness, not solver, and should not be quoted as a solver ratio.
- Against the lab arm at its best l (E1) the whole-run ratios are 7.0, 2.3, 18.1, 9.5 (K_0 n=9, K_1 n=9, K_1 n=11, K_0 n=13) and solver-only 4.9, 0.48, 14.1, 7.2. The lab's own l-sweep moves its optimum (l = 5, 4, 5, 6) while WDSat's best l stays 4.
- Four cells, one per n, two curve families: no statement about how the gap moves with n is made.

## 2. Provenance of WDSat

| item | value |
|:--|:--|
| URL | https://github.com/mtrimoska/WDSat |
| commit | `61c6ff3f49445af2729274819edb27b85a89efc1` ("Add symmetry-breaking technique", 2024-06-20), checked out detached after a plain clone |
| licence | GNU GPL-3.0 (`LICENSE`, sha256 prefix `3972dc97`). Used only as an external process; **none of its source is in this repository** (the committed `wdsat_cell.py` writes a generated `config.h` into a copy outside the repo and runs the upstream `make`) |
| size | 2.7 MB clone: `.git` 572 KiB, `src/` 164 KiB (4,101 lines of C in 15 files), `benchmarks/` (IC_S4, MQ ANF instances), README, LICENSE, plus an 80 KB prebuilt `wdsat_solver` committed in the repo root, **never run** |
| Step 0 | `/Volumes/SSD990-2/ext/WDSat` did not exist; cloned to a new empty directory `$TMPDIR/wdsat_src` (`/tmp/claude-501/wdsat_src`). No fetch, build or run was refused by a permission check |
| audit before building | `src/makefile` is `gcc -O3 -Wall`, output `../wdsat_solver`; grep of all `.c`/`.h` for `system`, `popen`, `exec*`, `socket`, `fork`, `getenv`, `curl`, `http`, `remove(`: only `fopen(input, "r")` in `main.c`; no network, no process creation |
| build | per-size binaries in copies of `src/` outside the repository (`make -C copy/src`), `config.h` generated per (l, `__MAX_ID__`) and per mode; macOS clang 4 warnings (unused variables, format), no errors |
| start-up | 1.348e7 / 1.385e7 / 1.391e7 instructions per process (find-all l = 4 / 5 / 6 binaries, contradictory one-line instance, "UNSAT on XORGAUSS init"); first-solution binaries 1.28e7 to 1.38e7 over 4 runs (l = 4, 5), within 5% of the value subtracted for solver-only |

Binary hashes (sha256 prefix): find-all l4/id300 `5eb5238e`, l5/id502 `3044a4f5`; first-solution l4/id300 `c1d37ed9`, l5/id502 `7ef29a56`.

## 3. The emitter (`wdsat_s4.rs`)

The emitter writes **the same system the msolve arm solves in its e-form** (`binary_semaev_s4::weil_descend_s4`: the descended,
`S_3`-symmetrised `S_4`, quadratic in the bits of `e_1, e_2, e_3`, plus the cubic correspondence `e_i = sigma_i(x)`), in WDSat's ANF format,
the encoding of WDSat's own `benchmarks/IC_S4` (checked: same variable layout, 3l + (6l - 3) unknowns, `.d` product terms, `T` convention).

- Header `p cnf <3l + 6l - 3> <#equations>`. Variables `1..3l`: bits of X_1, X_2, X_3 in the subspace `<1, z, .., z^(l-1)>` (block i, bit j is `i*l + j + 1`); then the bits of e_1 (l), e_2 (2l-1), e_3 (3l-2).
- The `6l - 3` definitional lines `e_{i,d} + sigma_{i,d}(x) = 0` first, then every non-zero Semaev equation in the e bits. Line `x [T] terms 0` asserts the XOR of its terms (plus the constant 1 when `T` is present) is TRUE, so a polynomial `P = 0` is written **with** `T` iff `P` has no constant term. A polynomial that is the constant 1 refutes the system without a WDSat call (never occurred).
- Run line: `wdsat_solver -i file -x -b -n n -l l -m 3` (`-x` XOR-Gauss module, the published recommendation for ANF input; `-b` symmetry breaking of the three summands; `-n -l -m` always passed, because `-l` defaults to n and `-b` depends on it). Without `-x`, the `-b` path prints `xorgauss_assignment`, which is unset in that mode (read from `wdsat.c`; seen as 5 failed parses in 10 find-all runs), so `-x` is kept with `-b`.
- Static allocation (`config.h`): only `__MAX_ID__` changes the instruction count and it must be exact (upstream warns otherwise). `__MAX_BUFFER_SIZE__`, `__MAX_XEQ_SIZE__` (checked 302 against 113) do not. `wdsat_cell.py` computes every constant from the ANF text following upstream `dimacs.c`'s counting rules, builds one binary per distinct `__MAX_ID__` seen over 240 sampled systems per l (plus 4 either side), and the runner picks the exact one (`wdsat_inexact` = 0 in every record; a larger one would be counted).
- Modes: **find-all** builds with `__FIND_ALL_SOLUTIONS__` (every model, like msolve returns every root); **first** is WDSat's default (stop at the first model). The pipeline takes the first liftable root, and in 7 of 8 sessions every root lifted (summed over the records: `lift_failures` = 0, `refuted_by_emitter` = 0, `wdsat_inexact` = 0). **K_1 n=11 has 5 roots that did not lift** (4 targets at l = 4, 1 at l = 5; identical in the find-all and the first session; spurious roots, `x` in the subspace with no factor-base point behind it). Find-all and first-solution sessions have identical solver calls and relations on every target at l = 4 and on 7/8 at l = 5 (the exception: K_1 n=11, l = 5, target 1, where a first model that failed to lift cost one extra call in first mode). So first-solution is the solver as normally used and is not distorted in these data, but it can in principle lose a relation behind a spurious first model; find-all is the like-for-like with msolve.
- Process accounting is the msolve arm's: one child per system under `/usr/bin/time -l`, `wdsat_ir` = its instructions retired.

Engine glue: `pdp3_koblitz_wdsat.patch` adds `engine=wdsat-s4` (params `sym`, `xg`, `find=all|first`) to `src/bin/ecbench/pdp3_koblitz.rs`; `wdsat_probe.rs` is the per-system correctness probe. Both are applied only inside a `git archive` build tree (see 7), so no hard-linked worktree file was touched. `ecbench` scrubs the environment of measured children, so the binary directory is a build-time default (`WDSAT_BIN_DEFAULT`).

## 4. Verification

- **Root sets against brute force** (`wdsat_probe`, every ordered triple of V^3 against `symmetrised_s4_eval_b`, compared as sorted-unique triples because `-b` returns one representative per permutation class; each returned model also re-checked against S4): find-all, `-x -b`: **270/270 systems agree** (K_0 n=9, K_1 n=9, K_1 n=11, K_0 n=13 at l = 4, 5; K_0 n=13 at l = 6; 30 targets each; `probes/probe_n*.jsonl`); first-solution: **180/180** (models valid, SAT/UNSAT status equal to brute force; `probes/probe_first_*.jsonl`). Unsatisfiable systems are included (about half at n = 9, 97% at n = 13, l = 4).
- **Targets**: all 8 sessions x 32 records = 256 records `verified` (`[d]G = Q` checked by ecbench), 128 of them WDSat arms (4 cells x {find-all, first} x {l = 4, 5} x 8 targets).
- **Same query stream**: at l = 4 the WDSat arm and the lab arm of a session have the same `targets_tried`, `relations_found` and solver calls on 8/8, 5/8, 8/8, 8/8 targets (K_0 n=9, K_1 n=9, K_1 n=11, K_0 n=13; identical in the find-all and first sessions); the others differ because a system with several roots returns a different, equally valid first relation (the counted S therefore differs on some targets, and `same_S` is false). At l = 5 the streams differ on most targets for the same reason, so l = 5 ratios are not per-query pairs.
- **Lab A/A against E1**: the lab arm repeated in every session reproduces E1's medians to 0.3% to 2.8% (`lab_aa_vs_e1` in `analysis/*.json`).

## 5. Results

Median over the 8 targets of Ir per target. *Whole* = ecbench child + every WDSat process - 3.51e7 child start-up (E1 rule). *Solver-only* = WDSat processes' Ir less 1.35e7 per call (the published-arm-favourable bound; the harness's own ANF construction in Rust, comparable to the solver at small n, is excluded). msolve and lab values are E1's recorded medians (`analysis/e1_res_input.json` = `tuned_20261030/e1/e1_res.json`); the lab l = 4 column is this session's rerun.

| cell | r | systems/target | lab l=4 (this run) | lab best l (E1) | msolve x-form l=4 spec=4, whole (E1) | msolve solver-only (E1) | **WDSat first, l=4 whole** | WDSat first solver-only | WDSat find-all whole | WDSat find-all solver-only |
|:--|--:|--:|--:|:--|--:|--:|--:|--:|--:|--:|
| K_0 n=9  | 127  | 16 | 3.52e8 | 2.95e8 (l=5) | 2.68e10 | 6.26e9 | **2.07e9** | 1.45e9 | 2.65e9 | 2.02e9 |
| K_1 n=9  | 37   | 9  | 2.52e8 | 2.47e8 (l=4) | 1.68e10 | 4.04e9 | **5.77e8** | 1.19e8 | 1.64e9 | 1.20e9 |
| K_1 n=11 | 991  | 56 | 1.19e9 | 4.56e8 (l=5) | 1.04e11 | 2.39e10 | **8.27e9** | 6.45e9 | 8.70e9 | 6.87e9 |
| K_0 n=13 | 2003 | 71 | 2.04e9 | 1.03e9 (l=6) | 1.48e11 | 3.75e10 | **9.85e9** | 7.47e9 | 1.07e10 | 8.29e9 |

Ratios (medians; the paired column is the median of 8 per-target ratios from the same session, with its range):

| cell | WDSat first / lab, same l=4, whole (paired) | WDSat solver-only / lab l=4 whole | WDSat whole / lab best l | WDSat solver-only / lab best l | msolve / WDSat first, whole | msolve / WDSat first, solver-only | WDSat find-all / lab l=4, whole | WDSat first l=5 / lab l=5, whole |
|:--|:--|--:|--:|--:|--:|--:|--:|--:|
| K_0 n=9  | 5.83 (5.12-7.19) | 4.17 | 7.00  | 4.92  | 13.0 | 4.3  | 7.26 | 14.9 |
| K_1 n=9  | 2.22 (1.50-2.61) | 0.46 | 2.34  | 0.48  | 29.2 | 33.9 | 6.26 | 2.6  |
| K_1 n=11 | 6.95 (6.55-7.25) | 5.42 | 18.14 | 14.14 | 12.5 | 3.7  | 7.25 | 27.4 |
| K_0 n=13 | 4.83 (4.50-4.92) | 3.66 | 9.54  | 7.24  | 15.1 | 5.0  | 5.22 | 27.9 |

The two "msolve / WDSat, whole" columns include the harness work discussed in section 1 and are not solver ratios; the solver-only column is.

Verified targets: 8/8 in each of the four WDSat arms (find-all l4, l5; first l4, l5) in every cell.

What drives the numbers:

- **Per system** (probe medians, find-all `-x -b`): l = 4 about 1.4e8 Ir (of which 1.35e7 process start-up), l = 5 about 2.1e9, l = 6 about 3.4e10, essentially independent of n and of the curve; the search is a full proof for the unsatisfiable systems that dominate at larger n (86% to 88% of the l = 4 systems of K_1 n=11 and K_0 n=13 are unsatisfiable). WDSat costs 15x per extra l-bit while the number of systems per target falls only 2x (71 to 36 at n = 13), so l = 4 wins at every n here and the lab's l-sweep gain is not matched.
- **First solution vs find-all** matters only where systems are mostly satisfiable: K_1 n=9 (97% SAT) is 5x to 7x cheaper per system (2.6e7 mean over 20 systems and 1.9e7 median over 30, against 1.4e8), K_0 n=9 (47% SAT) 1.3x whole-run, n >= 11 nothing.
- **K_1 n=9** is the one cell where WDSat's own time is below the lab arm's whole run (solver-only 0.46x), because r = 37 needs 9 systems and mostly satisfiable ones, so both arms are close to the harness floor.

## 6. Caveats (recorded, not hidden)

1. **Tuning is partial.** WDSat swept only l in {4, 5} (whole solves) and l = 6 (per-system probe at n = 13, 3.4e10 Ir per system, not run as a whole solve) with `-x -b`; no custom branching order `-g` (irrelevant under `-b`, which branches on the 3l x-bits), no hybrid specialisation, no `T`-invariants (D6a: the 2-torsion action does not preserve the x-base), `m = 3` only. The E1 side keeps its own sweep asymmetry (E1 section "Verdict").
2. **The whole-run figures are not solver comparisons.** They contain this harness's own Rust work: for WDSat the e-form ANF construction and rendering (about 4.5e8 per target at these sizes), for the E1 msolve arm much more (x-form expansion, substitutions, parsing, root extraction, about 1.1e10 per target at n = 9). msolve/WDSat whole-run ratios therefore overstate the solver gap; the solver-only ratios (3.7x to 5.0x, 34x at K_1 n=9) are the fair ones. Solver-only against the lab arm's whole run is the published-arm-favourable bound, as in E1. The lab "whole run" is the lab's complete solve including its F4 splitting search.
3. **The lab arm has no separate solver-only measurement**, so "WDSat solver-only / lab whole" compares unlike scopes in the published arm's favour, as in E1.
4. Instructions retired, aarch64, not Callgrind (D5); counted metrics only (no wall times used). WDSat and msolve are both external processes priced identically (child `time -l`).
5. msolve numbers are E1's records (msolve 0.9.5), not re-run here; the WDSat and lab arms were run in new sessions (lab A/A within 2.8% of E1).
6. Four cells (K_0 n=9, K_1 n=9, K_1 n=11, K_0 n=13); K_0 n=13 is the largest. A higher-n WDSat cell was not run (cost per system is flat so the projection is the systems-per-target growth, not computed here).
7. Repeats of one WDSat call differ by 0.2% (probe, 1.404e8 and 1.407e8); the first launch of a fresh binary was 10% higher (cold). The lab A/A bounds the harness noise; a WDSat session A/A was not repeated.

Provenance of the sessions: the four find-all sessions (`R5_*`) ran on an ecbench built before the `find=` parameter existed (its default is find-all, so the arm is the same); rebuilding from the committed patch gives a different binary hash than their `host.json`. The first-solution sessions (`R5f_*`) ran on the rebuilt binary.

## 7. Reproduce

```
# external clone (untrusted; outside the repo)
git clone https://github.com/mtrimoska/WDSat.git $TMPDIR/wdsat_src && git -C $TMPDIR/wdsat_src checkout 61c6ff3f49445af2729274819edb27b85a89efc1
# build tree from a git archive of the reference-arm branch (src, tools/volcano-cli, isogeny_algos, crates, Cargo.lock, a Cargo.toml with the [[bin]]/[[example]] blocks removed,
# and the docs/ and research/ files the crate include_str!s), then
patch -p0 src/cryptanalysis/ecbench/runner.rs < pilot_20261030/ecbench_child_time_shim.patch      # E1's child-time shim
cp wdsat_s4.rs src/bin/ecbench/ ; cp wdsat_probe.rs examples/ ; patch src/bin/ecbench/pdp3_koblitz.rs < pdp3_koblitz_wdsat.patch
WDSAT_BIN_DEFAULT=<bins dir> CARGO_TARGET_DIR=<dir> cargo build --release --offline --bin ecbench --example wdsat_probe
# WDSat binaries: probe dumps ANF files (wdsat_probe ... <dump_dir>), then
python3 wdsat_cell.py build <anf dirs of one l> --src $TMPDIR/wdsat_src --out <bins>/l<l> --pad 4 [--first -> <bins>/first/l<l>]
python3 make_specs.py specs
ECB_TIME_DIR=<tm dir> ecbench run --spec specs/SPEC-R5_<cell>.json --out <out> --cpus none --wait        # unsandboxed (see 8)
python3 r5_analyze.py <out.json> analysis/e1_res_input.json "K0_n=9@<session dir>" ...
```

## 8. Environment notes and what was not done

- **Sandbox off** for every `ecbench run`, `wdsat_probe` run and `/usr/bin/time -l` call (instructions retired need a sysctl the sandbox refuses; also `/tmp/crypto-bench.lock`). Nothing was refused by a permission check. The WDSat sources were read first (section 2); WDSat itself was built in the sandbox.
- **Build location deviates from the instruction** (builds under `$TMPDIR`): the root disk had 146 MB free, so the cargo target (0.8 GB) and the build tree (30 MB) were under `/Volumes/SSD990-2/r5-wdsat-build`, capped well under 5 GB (about 0.9 GB used). **It was not deleted**: the delete (`rm -rf` of that directory) was refused by the safety check, so `/Volumes/SSD990-2/r5-wdsat-build` (target 0.8 GB, build tree, WDSat binaries, run logs; every committed result is a copy) is left for the owner to remove. The WDSat clone is in `/tmp/claude-501/wdsat_src` (2.7 MB).
- `ecbench` session lock `/tmp/crypto-bench.lock` was held by another agent for tens of minutes while a session of this experiment queued (`--wait`).
- Not done: Callgrind (no Valgrind on this host); WDSat l = 6 whole solve and K_0 n=15 or larger cells; msolve 0.6.5 A/A; branching-order / hybrid tuning of WDSat; a WDSat repeat for A/A; a Linux host; no push or PR.
- S3 rule: `scripts/checkpoint.sh init`, `.resume/RESUME.md` filled, `gate` passed (twice) before any run; `.resume/` and `scripts/checkpoint.sh` are not committed.

## 9. Files

`wdsat_s4.rs` (emitter and runner), `wdsat_probe.rs` (correctness probe), `pdp3_koblitz_wdsat.patch` (engine glue), `wdsat_cell.py` (config sizing, per-size builds), `make_specs.py` and `specs/`
(the sessions' own `spec.json` is in `runs/`), `r5_analyze.py`, `runs/R5_<cell>` (find-all) and `runs/R5f_<cell>` (first-solution): `host.json`, `plan.json`, `records.jsonl`, `session.json`, `spec.json`,
and `*_tm/` per-child `time -l` files; `probes/`; `analysis/` (`r5_find_all.{json,txt}`, `r5_first.{json,txt}`, `e1_res_input.json`).
