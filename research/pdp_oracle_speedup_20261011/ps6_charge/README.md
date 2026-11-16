# PS-6: the pairs-and-solve kernels charged in the counted unit

Protocol: [`../protocols/PS-6.md`](../protocols/PS-6.md) (frozen; not edited).  Plan: [`../PLAN.md`](../PLAN.md).  Leads: [`../LEADS.md`](../LEADS.md) L1, L8.  Kernels: [`../ps2_scalar/`](../ps2_scalar/README.md), [`../ps3_bitsliced/`](../ps3_bitsliced/README.md), the pair table: [`../ps8_mitm/`](../ps8_mitm/README.md).
Results report (graphs, PDF): [`report/report.pdf`](report/report.pdf).  Run reports: [`../reports/run-20261018T2327Z/`](../reports/run-20261018T2327Z/) (counted sessions), [`../reports/run-20261031T1925Z/`](../reports/run-20261031T1925Z/) (calibration, timed sessions, memory).
Benchmarks copy and gate artifacts: [`../../../benchmarks/20261031-pdp-ps6-charge/`](../../../benchmarks/20261031-pdp-ps6-charge/).
Class: **engineering** (AGENTS.md section 3) for the oracle stage; everything about the kernel is a **stage diagnostic** (its pairs have no pinned price).  Nothing here is a claim about ECC2K-130 or the section 8a gate (`m = 83`, an enumerator would need `l ~ 28` at `m = 3`).

**One-paragraph result (observations).**  On the boundary ledger's char2 ladder, `n = 15, 18, 21, 24, 27` (random binary curves, `l = n/3`, `m = 3`), the baseline pairs-and-solve costs `S = 1,270 ... 114,574`, the group-law kernel's recorded `S` (its pairs left out) is `33.8 ... 22.4`, and the repository's meet-in-the-middle table costs `S = 57.9 ... 197.0` (full table) at `0.0 ... 2.1` MiB; the matched `rho.negation` costs `S = 2.1 ... 1.1`.  Pricing every arm's unpriced operations at 0, 0.1 and 1 GAE each, the baseline and the kernel and the table are above matched rho at every rung and weight, the kernel is below the baseline at every weight (by `9.1-11.6` at 1 GAE per pair, `38-5,119` at 0), and the kernel against the table is **unresolved** inside the band at every rung.  The baseline's total-cost exponent reproduces the ledger's (`e = 1.063`); the kernel's slope of `S`/floor over these rungs is below the baseline's by `0.03-0.17` depending on the price of a pair (no change if a pair costs what a baseline pair does).

## 1. Requested and executed

Host: one class, an Intel Xeon 2.10 GHz KVM guest, 4 vCPU (`raw/host.txt`, `raw/manifest.json`, `raw/host_premerge.txt`, `raw/manifest_premerge.json`).  Counted sessions 2026-10-18 23:27-23:44 UTC (commit `9e7e9a8a5`, binary `c914f6328634`), timed sessions and calibration 2026-10-31 19:24-20:16 UTC (commit `7ded1ca84`, binary `c8a7e9b9000f`); the PS-8 base branch (which had merged main) was merged between the two groups, so the counted sessions were replayed by the later binary.  Later commits changed only `src/bin/ps6_tool.rs` (analysis, not an oracle) and documents.

| requirement | status | evidence / exact gap |
|:--|:--|:--|
| (a) baseline `semaev_decomp::decompose` as an ecbench oracle on a random binary curve, factor base `<1, z, ..., z^(l-1)>`, `l = n/3`, `m = 3` | **verified complete** | `semaev-s4-baseline`; the ledger's code path (`Oracle::SemaevS4`); its `S` at `n = 27`, 114,574, against the ledger's 115,459 (section 4) |
| (b) PS-2 group-law scalar kernel as an oracle, verdict parity with (a) | **verified complete** | `semaev-s4-grouplaw`; same witness, not only verdict: 60,128 non-degenerate targets of 30 random curves, 0 mismatches (tests); audit arm 27,938 witness comparisons, 0 mismatches; 72 of 72 (workload, round) groups identical in targets tried, relations, rank, scalar (section 3).  Deviation: a generalised copy of PS-2's code, not `#[path]` reuse (section 6, D1) |
| (c) the existing meet-in-the-middle oracle as matched incumbent | **verified complete** | library `mitm:m=3` and `mitm:m=3,negation_folded=1`, unmodified, same specs |
| binary-side plug-ins; no frozen library file edited | **verified complete** | no library file changed (`git diff` against the PS-8 base touches `src/bin/` and documents only); `src/bin/ecbench.rs` gains two `mod` lines and one `.or_else` in `binary_plugins()` |
| every phase inside `S`, cold first, native work through `price_phase` at pinned conversions, the rest `*_uncharged`, rows marked lower bounds | **verified complete** for the baseline and the table; **partial** for the kernel | the kernel's pairs have no pinned conversion: counted (`solver.ops`), left out of `S`, row a lower bound; the section 5a band and four labelled further points stand in (section 2) |
| memory beside every table row; not in `S` | **verified complete** | `max_rss_kib` in every record (100-108 MiB, the process); the table's entries and growth (section 4); said to be outside `S` |
| one target per run; same curves, subgroups, factor bases, targets, seeds for every arm; `[k]P = Q` verified for every completed run | **verified complete** | all 432 counted and 1,296 timed runs verified by the runner; the 1,512 measured scalars re-verified by separate arithmetic (`validation/replay-certificate.json`); warm-up runs (216) only by the runner |
| rungs where the oracle dominates, `n <= 53` | **partial** | the ledger's own char2 S4 rungs, `n = 15 ... 27` (five sizes; `s4_max_degree` is 31 but the ledger's ladder stops at 27).  Not run: any `n > 27`; the baseline's cost grows 18-20 times per three bits (46 s per target at `n = 27`, about 15 min at `n = 30`), so the rung is not infeasible, only not attempted |
| paired 95% interval on every timed row | **verified complete** | cluster bootstrap over targets of the paired wall ratios, for every arm pair and rung (`raw/summary.md`), isolation levels of the pairs printed beside; ecbench's own wall status is `admitted` only with at least five L2 pairs (section 4) |
| `--cpus none` for counts; the repository's isolation tooling for wall time | **verified complete** | counted sessions `--cpus none`; timed sessions `ecbench run --cpus auto` under `tools/isolated_bench.py busy`, calibration `isolated_bench.py run --cpus 3` (not contended) |
| any stage speedup labelled a stage diagnostic unless every phase is priced | **verified complete** | every baseline/kernel ratio is so labelled |
| AGENTS.md section 5a band (0, 0.1, 1 GAE per unpriced operation, every arm, rho included) | **verified complete** | `raw/summary.md` per rung; order tests; unpriced counters listed per arm (section 2) |
| no claim about ECC2K-130; section 8a not discharged | **verified complete** | stated here and in both reports |
| code, runner, raw outputs, README, host manifest | **verified complete** | `run_ps6.sh`, `raw/`, this file, `raw/manifest*.json` |
| scoreboard and `docs/ic/progress-timeline.json` updated from source data | **verified complete** | updated in this PR (section 7): the timeline points and the page's panel, embedded copy and point table are written by `../tools/ps6_timeline.py` from `raw/summary.json`, none by hand; no leaderboard row is added because no arm is a new whole-pipeline method (section 7) |
| run report with graph and PDF; results report | **verified complete** | `../reports/run-*`, `report/` |
| `PLAN.md` PS-6 row | **verified complete** | edited |
| rustfmt, clippy, ecbench tests | **verified complete** (tree of commit `ef04f04af`, which merges `main`) | `rustfmt --edition 2021 --check` clean on the four Rust files; `cargo clippy --release --bin ecbench --bin ps6_tool --tests` exit 0, no errors, nothing reported in `ps6_*` files (the single warning per target is the library's pre-existing unknown lint, `raw/logs/clippy_merged.log`); `cargo test --release --bin ecbench ps6`: 4 passed; `--bin ps6_tool`: 4 passed; `--test ecbench`: 25 passed (`raw/logs/tests_*merged.log`, `raw/logs/merged_checks.status`).  `--lib ecbench` (102 passed) was run before the merge only |
| validation gate artifacts for the benchmarks row | **implemented; independence limited** | replay certificate, validator report, `VALIDATION.json`; `scripts/validation_gate.py evidence` says `valid`; produced by the author on one host class (section 6) |
| PS-3 bit-sliced kernels charged | **not attempted** | the build list named the PS-2 scalar kernel; PS-3 found its vector arms no faster than the scalar kernel per pair (`../ps3_bitsliced/README.md`); the bit-sliced program is specific to PS-2's fixed curve and would need the same generalisation |
| counted carry-less-multiply ratio (a deterministic conversion) | **blocked** | the `pdp-opcount` counters of PS-0 are not in the merged library (`semaev_decomp.rs` has no `opcount` module, `Cargo.toml` still declares the feature); re-adding them is a library edit this study did not make |
| second host class / second-party replay | **not attempted** | no second host available; no one but the author replayed |
| Conductor task | **blocked** | `conductor check` failed on every attempt (`connection refused` at `localhost:8080`); `.conductor/WORKFLOW.md` is absent from the tree.  Scope used: `src/bin/ecbench.rs`, `src/bin/ecbench/ps6_*.rs`, `src/bin/ps6_tool.rs`, `docs/ecbench/README.md`, `research/pdp_oracle_speedup_20261011/**`, `benchmarks/20261031-pdp-ps6-charge/`, `benchmarks/README.md`, `docs/index-calculus-scoreboard.html`, `docs/ic/progress-timeline.json` (the last four beyond the first reservation) |

## 2. What was built, and what is charged

All code is binary-side, as `src/bin/ecbench/pdp3_koblitz.rs` is.  No frozen library file (`ic_boundary.rs`, `ic_framework/*`, `ecbench/*`, `semaev_decomp.rs`, `docs/ic/calibration.json`) is edited, so no committed `ecbench` session and no IC1 candidate identity moves.

| file | what it is |
|:--|:--|
| `src/bin/ecbench/ps6_charge.rs` | two `ic.pipeline` oracles, `semaev-s4-baseline:m=3` and `semaev-s4-grouplaw:m=3[,audit=1]` |
| `src/bin/ecbench/ps6_kernel.rs` | the PS-2 group-law kernel, generalised from the program's fixed curve (`a = b = 1`) to any `y^2 + xy = x^3 + a x^2 + b`; the header names the source commit `2a14b74c7`; four unit tests |
| `src/bin/ecbench.rs` | two `mod` lines and one `.or_else(...)` in `binary_plugins()` |
| `src/bin/ps6_tool.rs` | `calibrate`, `memory`, `analyze`, `certify`, `validate`, `gate-json` (native; the plotting script is the only Python) |
| `docs/ecbench/README.md` | the charge of the two oracles, section 7 |
| `make_specs.sh`, `specs/` | the ten ecbench specs, written and committed before any session ran |
| `run_ps6.sh` | the runner (stages `build tests manifest counted reaudit calibrate memory timed analyze`) |
| `../tools/plot_ps6.py` | plotting only: draws `raw/summary.json` |

The third arm is the library's own `mitm` oracle (`MitmOracle`, `PairTable`), unmodified.

**The oracles.**
* **(a) `semaev-s4-baseline`** is `Oracle::SemaevS4` as the boundary ledger runs it: `SubspaceOracle::decompose` on `V = <1, z, ..., z^(l-1)>`, then the framework's sign lift.  Pairs go to `OracleCounters::s4_pairs` and are priced by `price_phase` at the pinned `ns_per_s4_pair` of the curve.
* **(b) `semaev-s4-grouplaw`** is the PS-2 scalar kernel: for each pair of rational base points the four candidates `x(P1 +- P2 +- R)` by the group law (row points `P1 +- R` once per row, pair denominators inverted a row at a time), membership in `V` as `x < 2^l`.  It returns the baseline's **witness**, not only its verdict: pairs are visited in the baseline's order and a pair with several roots in `V` reports the smallest, which is what the baseline reports.  A pipeline run with (a) or (b) therefore draws the same targets, finds relations on the same targets, builds the same matrix and ends at the same rank.  A *degenerate* target (`x_R = 0`, `x_R` in `V`, or `sqrt(b)/x_R` in `V`; PS-1 with `1/x` replaced by `sqrt(b)/x` for general `b`) is decided by the baseline inside the call and charged as the baseline is.  `audit=1` also decides every target with the baseline and compares the witnesses; that work is counted (`audit_baseline_pairs`) and never charged.
* **(c) `mitm`** is `PairTable::build` over all signed base points (group additions in the `oracle_setup` phase) and, per target, one subtraction `R - P_i` and one probe per signed base point (group additions and pinned `lookups`).

**What is charged**, all inside one cold process per run: factor base (`as_solves`, pinned), oracle set-up, target generation (`[a]G + [b]Q` per trial, group operations), decomposition including failed attempts, the sign lift of every witness, linear algebra (`row_ops`, pinned), verification `[d]G = Q`, and the runner's external check.  `S = total operations / sqrt(r)`.

| native work | where counted | priced? |
|:--|:--|:--|
| group additions, doublings | `GroupOps` | yes, 1 each |
| baseline S4 pairs | `s4_pairs` | yes, pinned `ns_per_s4_pair` of the curve |
| MITM probes, target-guard probes | `lookups`, `target_guard_probes` | yes, pinned `ns_per_lookup` |
| Artin-Schreier solves, elimination | `as_solves`, `row_ops` | yes, pinned |
| **group-law kernel pairs** | solver block, unit `group-law kernel pairs` | **no pinned conversion**: counted, recorded, left out of `S`; the row is a lower bound |
| one field inversion per target (the kernel's degeneracy test) | `inversions` | no pinned ratio for char2: `inversions_uncharged` |
| construction of the baseline's `SubspaceOracle` and of the kernel's base list | `setup_native`, `*_uncharged` | no unit; counted and listed, so both S4 rows are lower bounds (a few hundred words against 10^4 to 10^8 pairs) |
| `rho.negation` canonicalisations | `canonicalisations_uncharged` | the reference is itself a lower bound (ecbench README section 7) |

**The section 5a band.**  Every arm's unpriced native operations (the `unpriced` list of each record: the kernel's pairs, the `_uncharged` words, rho's canonicalisations) are priced at 0, 0.1 and 1 GAE each, the same way for every arm, rho included: `S(w) = S + w U / sqrt(r)`.  An order between two arms is reported only where it holds at all three weights; otherwise the pair is unresolved.  Four further kernel points use conversions that are **not pinned** and are not ranked: (1) each kernel pair priced as one baseline pair (the pinned `ns_per_s4_pair`, 2.8-3.1 GAE); (2) the *anchored estimate*, the pinned baseline-pair price times the measured kernel/baseline per-pair time ratio (`ps6_tool calibrate`: same process, interleaved, 11 repetitions, one reserved core; host speed cancels); (3) the measured kernel pair over a measured group addition, which mixes this host's addition speed (it measures a baseline pair at 1.6-1.8 additions where the pin says 2.8-3.1) with the pinned price, a sensitivity only.

## 3. Correctness

* `cargo test --release --bin ecbench ps6` (4 tests, `raw/logs/tests_ps6.log`): the group-law formulas equal `FastCurve::add` for `a` in {0, 1}; translation by the 2-torsion point is `x -> sqrt(b)/x` on every rational point of four random curves; the kernel's roots equal an exact pairwise enumeration; and the kernel returns the baseline's witness on **60,128 non-degenerate targets** of 30 random binary curves (n = 6...10, both values of `a`, `l = 2 ... min(n-1, 7)`; 22,920 with a witness; 9,404 degenerate targets skipped, which the oracle sends to the baseline).
* In the counted sessions (`raw/summary.md`, "Verdict parity"): 72 of 72 (workload, round) groups have identical targets tried, relations found, matrix rank and recovered scalar across `s4-baseline`, `s4-grouplaw` and `s4-grouplaw-audit`; the audit arm compared 27,938 witnesses with the baseline's, **0 mismatches**; 4 degenerate targets went to the baseline.
* Every measured recovered scalar (1,512) re-verified by `ps6_tool certify` with the group law of `ps6_kernel.rs` (separate from the harness's `FastCurve`): `[d]G = Q`, `[d+1]G != Q`, each curve's generator on the curve of order `r`.  0 errors.
* `ecbench verify` audits (`raw/sessions/*.audit.json`, `*.audit_postmerge.json`): all `ok`.  Counted sessions replayed by the pre-merge binary (all 96 runs at `n <= 21`, 12 at `n = 24, 27`) and by the post-merge binary (all 96 runs at `n <= 24`, 12 of 48 at `n = 27`): every replay identical.  Timed sessions: all 240 replays at `n <= 21`, 12 at `n = 24, 27`.  Receipt digests (first 16 hex digits of the SHA-256) of the counted sessions, pre-merge: `be49b457e2b4d769`, `6686f21dce4ae3bb`, `9acc9c9c07731799`, `5ec68b0096e7f87c`, `1df2cd75de8b705d` (n = 15 ... 27); post-merge: `f80b67e2299fd9e6`, `8ba46151c51e005a`, `29cbdb69a67dda63`, `8e489f57365b0ca5`, `38412ef53c2e463f`; timed: `e13312f6bd7c9e7f`, `93d81cc5aef445de`, `b4b7ac6ebd10a5bc`, `ac3a72c6409bca33`, `b34aad453f2fdd11`.
* `validation/validator-report.json`: all 432 counted records' totals recomputed from raw counters and the pinned calibration with separate arithmetic equal the recorded totals; 65 headline numbers of `raw/summary.json` agree to 1e-9.

## 4. Results (observations)

Curves (registered ICV1 slugs; the ledger's `random_binary_instance(n, 0x1CB00BDA7A, 8)`): `icv1-f2m15-t131-ed7256f0` (`r = 16,319`, `h = 2`), `icv1-f2m18-t393-d2c04152` (32,719, 8), `icv1-f2m21-t2591-de54d72e` (1,047,281, 2), `icv1-f2m24-tm3047-2d0c0e8c` (2,097,533, 8), `icv1-f2m27-t12543-569dca8b` (22,367,531, 6).  Floor `S = sqrt(pi/4) = 0.886` (negation, `A = 2`).  Sessions `ECBS1h35288032d470`, `ECBS1h9c1f6e592bd2`, `ECBS1h513dba00a329`, `ECBS1h08f67a4fa7eb`, `ECBS1ha751ceb1f30b` (counted) and `ECBS1hb584409b0e02`, `ECBS1h781f91c31e92`, `ECBS1hc6a454ec1cbc`, `ECBS1h358d02883765`, `ECBS1h08dbafdea64f` (timed).  Mean `S` over targets (16 runs per arm, 8 at `n = 27`); 95% bootstrap intervals over targets are in `raw/summary.md`.

| n | rho (negation) | S4 baseline | kernel, w = 0 (recorded) | kernel, w = 0.1 | kernel, w = 1 | MITM | MITM, neg.-folded |
|--:|--:|--:|--:|--:|--:|--:|--:|
| 15 | 2.10 | 1,270 | 33.8 | 44.4 | 139.7 | 57.9 | 56.1 |
| 18 | 1.97 | 3,041 | 26.9 | 55.0 | 308.3 | 65.9 | 60.2 |
| 21 | 1.31 | 8,560 | 22.9 | 94.6 | 740.0 | 73.6 | 69.9 |
| 24 | 1.29 | 19,067 | 16.3 | 211.1 | 1,964 | 93.9 | 80.9 |
| 27 | 1.09 | 114,574 | 22.4 | 1,012 | 9,915 | 197.0 | 182.4 |

Intervals at `n = 27`: baseline 107,540-121,184; kernel w=0 21.1-23.5, w=1 9,306-10,486; MITM 186.5-206.8; rho 0.59-1.70.  `S / floor` is `S / 0.886`.  Matched rho at weights 0.1 and 1 is 2.25, 2.13, 1.42, 1.41, 1.19 and 3.63, 3.53, 2.45, 2.46, 2.10 (canonicalisations priced).

**Ratios to matched rho and the order tests** (sum `S` over targets, each arm and rho at the same weight; below 1 = cheaper):

| n | baseline / rho (w=0) | kernel / rho, w = 0, 0.1, 1 | MITM / rho (w=0, 1) | kernel / baseline, w = 0, 0.1, 1 | kernel / MITM, w = 0, 0.1, 1 | order kernel vs MITM |
|--:|--:|:--|:--|:--|:--|:--|
| 15 | 605 | 16.1, 19.7, 38.4 | 27.6, 15.9 | 0.027, 0.035, 0.110 | 0.58, 0.77, 2.41 | unresolved |
| 18 | 1,544 | 13.6, 25.9, 87.4 | 33.5, 18.7 | 0.009, 0.018, 0.101 | 0.41, 0.83, 4.68 | unresolved |
| 21 | 6,560 | 17.5, 66.7, 302 | 56.4, 30.1 | 0.003, 0.011, 0.086 | 0.31, 1.29, 10.1 | unresolved |
| 24 | 14,806 | 12.6, 150, 800 | 72.9, 38.2 | 0.001, 0.011, 0.103 | 0.17, 2.25, 20.9 | unresolved |
| 27 | 105,128 | 20.5, 850, 4,721 | 180.8, 93.8 | 0.0002, 0.009, 0.087 | 0.11, 5.14, 50.3 | unresolved |

The baseline, kernel and table are above matched rho at every rung and weight; the baseline is above the kernel and above the table at every weight; the negation-folded table costs 3-14% less than the full table (`mitm-negfold / mitm` 0.97, 0.91, 0.95, 0.86, 0.93).

**Stage ratios, baseline over kernel** (sum `S`, same targets; **stage diagnostics**; narrow intervals because every arm walks the same trial stream, so they reflect the targets only): at w = 0: 37.6, 113, 374, 1,171, 5,119; w = 0.1: 28.6, 55.3, 90.5, 90.3, 113.3; w = 1: 9.1, 9.9, 11.6, 9.7, 11.6 for `n = 15 ... 27`.  Not pinned: a kernel pair priced as a baseline pair, 3.6, 3.6, 4.2, 3.4, 3.7 (the pair-count ratio); anchored estimate, 10.8, 17.8, 28.7, 31.2, 42.3; measured kernel pair over a measured addition, 15.2, 26.0, 43.7, 53.6, 74.7.  The measured kernel/baseline per-pair time ratio is 0.264, 0.177, 0.137, 0.107, 0.088 (`raw/calibrate.jsonl`: 52.7, 39.4, 31.8, 27.0, 24.2 ns per kernel pair against 196, 209, 230, 252, 277 ns per baseline pair).

**Exponents** (total operations `T = S sqrt(r) ~ r^e`, five rungs; 95% bootstrap intervals over targets, which carry neither the five-rung model nor the calibration spread; the slope of `S`/floor is `e - 1/2`):

| arm | e | 95% | e minus the baseline's (paired) |
|:--|--:|:--|:--|
| rho (negation) | 0.405 | 0.332-0.465 | -0.660 |
| S4 baseline | 1.063 | 1.049-1.078 | 0 |
| kernel, w = 0 (lower bound) | 0.436 | 0.424-0.447 | -0.628 (-0.634, -0.623) |
| kernel, w = 0.1 | 0.895 | 0.882-0.908 | -0.168 (-0.172, -0.165) |
| kernel, w = 1 | 1.036 | 1.022-1.050 | -0.027 (-0.028, -0.026) |
| kernel, pair = baseline pair (not pinned) | 1.059 | 1.045-1.073 | -0.004 |
| kernel, anchored estimate (not pinned) | 0.892 | 0.878-0.906 | -0.171 (-0.173, -0.170) |
| kernel, add-ratio estimate (not pinned) | 0.861 | 0.847-0.874 | -0.203 |
| MITM | 0.645 | 0.633-0.657 | -0.418 (-0.421, -0.416) |
| MITM, negation-folded | 0.639 | 0.626-0.653 | -0.424 |

The baseline's `e = 1.063` reproduces the ledger's exponent (1.06, five sizes); the table's 0.645 is the ledger's 0.62.

**Wall time** (timed sessions, solve wall per run, descriptive; 40 paired runs per row, 20 at `n = 27`; paired median and 95% cluster-bootstrap interval over targets):

| n | baseline ms | kernel ms | MITM ms | rho ms | kernel / baseline | MITM / kernel | A/A: control / baseline | L2 pairs of the baseline/kernel row |
|--:|--:|--:|--:|--:|:--|:--|:--|--:|
| 15 | 14.5 | 5.8 | 5.1 | 38.4 | 0.385 (0.363-0.444) | 0.885 (0.806-0.941) | 0.999 (0.966-1.037) | 28 of 40 |
| 18 | 41.2 | 6.9 | 5.6 | 38.2 | 0.163 (0.154-0.179) | 0.807 (0.779-0.860) | 1.014 (0.984-1.056) | 20 of 40 |
| 21 | 749 | 33.8 | 13.8 | 40.9 | 0.044 (0.043-0.046) | 0.418 (0.389-0.468) | 1.019 (0.993-1.055) | 17 of 40 |
| 24 | 2,521 | 86.0 | 23.3 | 39.2 | 0.035 (0.034-0.036) | 0.272 (0.261-0.283) | 1.006 (0.983-1.017) | 5 of 40 |
| 27 | 46,151 | 1,156 | 132.5 | 41.6 | 0.025 (0.024-0.025) | 0.117 (0.113-0.126) | 0.999 (0.991-1.010) | 0 of 20 |

A one-tick hypervisor steal on the run CPU loses L2, and a long run is likely to have one (389 of 1,296 runs; the other L2 blocker was run-queue delay above 0.5%, 18 runs), so the baseline's runs at `n >= 21` are mostly L1.  ecbench's own wall status is `admitted` where it has five L2 pairs and `descriptive` otherwise (`n = 27`: every comparison that includes the baseline).  The A/A intervals cover 1 at every rung.  Rho's wall is 38-42 ms at every `n` while its `S` is about 1.  The `n = 27` kernel/baseline wall ratio, 0.025, is within 6% of the anchored estimate's `S` ratio, 1/42.3.

**Memory (not in `S`).**  The library's pair table (`ps6_tool memory`): 220, 1,016, 3,721, 18,711, 69,118 entries at `n = 15 ... 27`; resident growth 0.0, 0.0, 0.1, 0.5, 2.1 MiB (30-39 bytes per entry at `n >= 21`; full and negation-folded builds give the same entries with 465 ... 138,601 and 240 ... 69,432 group additions).  Every run's peak resident set is 100-108 MiB (the process, `max_rss_kib`).  Keys grow as `|B|^2`: PS-8 measured 16.4 M keys at `l = 13`.

## 5. Reading (interpretation, labelled; scope: these five rungs, this host class, this unit)

* The kernel lowers the S4 oracle's cost against the baseline at every rung and weight; how much depends on the price of a kernel pair: about 3.4-4.2 if a pair costs what a baseline pair does (the smaller candidate set alone), 9.1-11.6 at 1 GAE per pair, 11-42 at the anchored estimate, 38-5,119 if pairs are free.  None is a pinned figure: it is a stage diagnostic, and no speedup is claimed.
* **Finding rule (slope of the ratio to the floor).**  Over these rungs the kernel's `S`/floor slope is below the baseline's by 0.03 (1 GAE per pair) to 0.17 (anchored estimate), intervals excluding zero, and by 0.004 if a pair is priced as a baseline pair.  What moves the slope is the per-pair cost ratio (0.264 to 0.088); the pair-count ratio is flat (3.4-4.2).  The likelier cause is a row set-up amortised over longer rows; if so the difference should vanish as the ratio flattens, which five rungs ending at `l = 9` cannot show.  Reported as an observation, not as a change of exponent; the plan's expectation (the slope does not change) is neither confirmed nor refuted at larger `l`.
* Against the incumbent: the table is cheaper than the baseline at every rung (21.9 to 582 times in `S`) with exponent 0.645 against the kernel's 0.89-1.04 at the priced points.  Kernel versus table is unresolved at every rung inside the band; at the anchored estimate the kernel is above the table at every rung (2.0 to 13.7 times) and the table's whole-pipeline wall time is 8.5 times lower at `n = 27`.  This is the PS-8 reading (the table replaces the kernels where it fits in memory) now in the counted unit.
* Against rho: the gap grows with `n` for the baseline, the table and the kernel at `w >= 0.1`; the kernel's lower bound (w = 0) stays at 12.6-20.5 times rho.
* AGENTS.md section 3: engineering for the oracle stage (cost fell, the ratio to the floor still rises with `r`); not an advance.

## 6. Deviations and gaps (every one)

* **D1, generalised copy.**  The PS-2 kernel hard-codes `y^2 + xy = x^3 + x^2 + 1`; the ledger's curves are random binary curves, so `a` and `b` became parameters and the 2-torsion translation `x -> 1/x` became `x -> sqrt(b)/x`.  The change is checked by the exhaustive test above, not by PS-2's corpus.  Also changed: a pair with several roots reports the smallest (baseline order), the base comes from the run's factor base, and the kernel counts its own work.
* **D2, the kernel is a lower bound.**  No pinned conversion exists for its pairs and adding one is a library edit (`Calibration`, `docs/ic/calibration.json`).  The band and the labelled estimates stand in; **no pinned price of the kernel is established**.
* **D3, rungs.**  `n = 15 ... 27` only; see section 1.
* **D4, sample sizes.**  Counted: 8 targets x 2 rounds (x 1 at `n = 27`); timed: 8 targets x 5 rounds + 1 warm-up (4 targets at `n = 27`).  Chosen before the runs from the cost of the baseline; `n = 27` has 8 and 4 targets.
* **D5, isolation.**  Wall time is below the spec's L2 for the long runs (L1, steal ticks); the interval and the levels are printed, and ecbench marks those comparisons descriptive.  No L3 (a lab host is needed), one host class.
* **D6, validation artifacts.**  `replay-certificate.json` and `validator-report.json` were produced by `ps6_tool certify` and `ps6_tool validate`, the author's code, on the machine that measured; the scalar check is a separate implementation of the group law and the totals check a separate arithmetic, but no second party and no second host class replayed anything.  The gate's script finds the evidence structurally valid; the independence the gate intends is limited accordingly, and a reviewer may archive the row instead (as was done for PS-3's).
* **D7, tests and lints.**  The integration test `a_public_target_session_yields_a_checked_vs_rho_claim` failed in a run on the pre-merge base (`raw/` has no log of it) and passes after the base was merged (`raw/logs/tests_integration_ecbench.log`: 16 passed); it was not investigated beyond that.  The opcount stage failed to build (no instrumentation, section 1), `src/bin/ps6_tool.rs` no longer carries it.  Clippy: `cargo clippy --release --bin ecbench --bin ps6_tool --tests` on the tree before the last edit exited 0 with no errors and two warnings in `src/bin/ps6_tool.rs` (`map_entry`, `unnecessary_map_or`; `raw/logs/clippy_first_final.log`); the other warning in every target is the library's pre-existing `unknown lint clippy::chunks_exact_to_as_chunks` and is not from this change.  Both warnings were fixed by two behaviour-neutral edits (an `Entry::Vacant` insert, `is_none_or`); the fixed tree is rustfmt-clean, and its clippy and test results are the next-to-last line of this section.
* **D8, history and base.**  PS-8 (PR 2199) was merged into `main` and its branch deleted on 2026-10-26, so this PR's base is `main`, not `exp/pdp-ps8-mitm`.  `main` was merged into this branch at `ef04f04af` (six conflicts: `src/bin/ecbench.rs` keeps main's `structured_s4` plug-in beside the two PS-6 ones, `docs/ecbench/README.md` and `benchmarks/README.md` keep both sides with the PS-6 row first, and the three generated scoreboard files are main's versions with the PS-6 rows written again by `../tools/ps6_timeline.py` and `scripts/site/panel_index.py`, not merged by hand).  The measured sessions were recorded before that merge (commits `9e7e9a8a5` and `7ded1ca84`); the merged tree was compiled, linted and tested again (the "Clippy and tests, final tree" line below) but **its binary was not used to re-run the sessions**, so the numbers describe the library as of `7ded1ca84` (the PS-8 base with the `main` it had merged), not as of today's `main`.  The counted sessions were recorded before the base was merged; the merge touched `docs/ecbench/README.md` and `src/bin/ecbench.rs` (conflicts resolved keeping both sides).  A first batch of work on this branch stopped at API limits; its snapshot commits are in the branch history.
* **D9, calibration noise.**  The anchored estimate uses the median of 11 repetitions per rung; the min-max range of the kernel/baseline per-pair time ratio is in `raw/summary.md` (for example 0.080-0.124 at `n = 27`); it is not propagated into the exponent intervals.
* **D10, local checks were partial.**  The worktree was a sparse checkout (about 2-3 GB free; a full-tree merge of `main` filled the disk twice and was redone with `scripts/sparse-checkout.sh apply --profile minimal`, then widened path by path).  Run and passing on the final tree: `scripts/validation_gate.py evidence benchmarks/20261031-pdp-ps6-charge` (valid), `scripts/site/panel_index.py --check` (310 panels, current), `node scripts/check_ic_dashboard_coverage.mjs` (20,817 curves registered, rostered and searchable; 99 timeline points).  Passing before the merge only: `scripts/check_curve_names.py --diff origin/main` over the files then present (0 problems) and `scripts/build_ic_leaderboard.py --check`.  **Did not run to completion after the merge** because each stops at a path the sparse checkout lacks (for example `ecdb/README.md`, `research/f6_ic_geometric_closure_20261003/small_cold/measurements.jsonl`, `docs/backend-leaderboard/leaderboard.json`): `scripts/check_curve_names.py` over the tree, `scripts/build_ic_leaderboard.py --check`, `scripts/site/test_build.py`, and `scripts/validation_gate.py check` over all rows (51 rows fail there on missing provenance files, none of them this row).  A curve-names run over a partial `docs/curves` also reported `curve_aliases.json` out of step with the registry, which this PR does not touch; that was not resolved here.  CI is the first full-tree run of these on the merged tree.
* **Clippy and tests, final tree.**  After the two fixes and the merge of `main` (`ef04f04af`) clippy exits 0 with no warning in a `ps6_*` file, and the three test runs above pass (`raw/logs/*merged*`); they ran under `tools/isolated_bench.py busy`, once the lock was free, after two earlier attempts failed only because the sparse checkout lacked workspace members (no code result).  The `--lib ecbench` tests were not repeated on the merged tree.
* **Not measured:** relation yield beyond what the pipeline needs, linear algebra beyond the pinned `row_ops`, warm amortisation (every run is cold; the table's build is inside `S`, and a multi-target break-even is PS-8's `N*`), any GPU row (PS-5), anything on a second host.

## 7. Dashboard, graphs checked, benchmarks

* `docs/ic/progress-timeline.json`, the scoreboard's embedded copy and its plotted-point table, and a new panel `#lab-ps6-charge` on `docs/index-calculus-scoreboard.html` carry this study (the points are quoted from `raw/summary.json`; the script that edits them from that file is `../tools/ps6_timeline.py`).  The IC leaderboard, the lab browser and the best-of table are unchanged: nothing here is a whole-pipeline cold cost of a new method (the ledger's own S4 and MITM rows reproduce), the kernel is a stage diagnostic, and `scripts/build_ic_leaderboard.py --check` is unchanged.
* `benchmarks/20261031-pdp-ps6-charge/` holds the PDFs, every graph as SVG and PNG (150 dpi, from `tools/plot_ps6.py`), `copies.json`, and `VALIDATION.json`; its row is in `benchmarks/README.md`.
* Graphs checked and left unchanged: the scoreboard's ledger table row for the S4 pairs-and-solve at `n = 27` (115,459, whose counts this study reproduces at 114,574) and its exponent row (1.06, reproduced as 1.063).

## 8. Commands

```bash
bash scripts/sparse-checkout.sh apply   # this worktree used the harness profile (disk)
bash research/pdp_oracle_speedup_20261011/ps6_charge/make_specs.sh                                   # the ten specs (already committed)
PS6_STAGES="build manifest tests counted reaudit" bash research/pdp_oracle_speedup_20261011/ps6_charge/run_ps6.sh   # counted group
PS6_STAGES="manifest calibrate timed" PS6_LOCKED=1 python3 tools/isolated_bench.py busy --wait -- bash research/pdp_oracle_speedup_20261011/ps6_charge/run_ps6.sh
PS6_STAGES=memory bash research/pdp_oracle_speedup_20261011/ps6_charge/run_ps6.sh
PS6_STAGES=analyze bash research/pdp_oracle_speedup_20261011/ps6_charge/run_ps6.sh                   # compare --save, table, ps6_tool analyze
target/release/ps6_tool certify --sessions raw/sessions/{counted,timed}_n* --out validation/replay-certificate.json
target/release/ps6_tool validate --counted raw/sessions/counted_n* --summary raw/summary.json --evidence-dir ../../../benchmarks/20261031-pdp-ps6-charge --out validation/validator-report.json
target/release/ps6_tool gate-json --summary raw/summary.json --certificate validation/replay-certificate.json --report validation/validator-report.json --row 20261031-pdp-ps6-charge --out ../../../benchmarks/20261031-pdp-ps6-charge/VALIDATION.json
python3 scripts/validation_gate.py evidence benchmarks/20261031-pdp-ps6-charge
/tmp/report-venv/bin/python research/pdp_oracle_speedup_20261011/tools/plot_ps6.py raw/summary.json report   # plotting only
```

The binaries embed the source commit at build time (`raw/build_env*.txt`); the runner exports it from the file for every later stage.  Spec of each session: `raw/sessions/<name>/spec.json`; records: `records.jsonl`; saved comparisons: `raw/sessions/timed_n*/comparisons/`, `counted_n*/comparisons/`; ecbench tables: `raw/table_counted.txt`, `raw/table_timed.txt`; every table of this file: `raw/summary.md` (generated, do not edit) and `raw/summary.json`.
