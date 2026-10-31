# PS-3: the bit-sliced CPU kernel at 64, 256 and 512 lanes

Protocol: [`../protocols/PS-3.md`](../protocols/PS-3.md).  Plan: [`../PLAN.md`](../PLAN.md).  Baseline: [`../baseline_v1/`](../baseline_v1/README.md).
Scalar group-law kernel and the program: [`../ps2_scalar/`](../ps2_scalar/README.md).  Run reports with graphs and PDFs:
[`../reports/run-20261012T0216Z/`](../reports/run-20261012T0216Z/) (host A), [`../reports/run-20261018T1627Z/`](../reports/run-20261018T1627Z/) (host B).
Class: **engineering**.  Commands: [`run_ps3.sh`](run_ps3.sh), [`run_counts.sh`](run_counts.sh); tuning history: [`tuning_log.md`](tuning_log.md).
Raw records: [`raw/`](raw/) (host A), [`raw_hostB/`](raw_hostB/) (host B).

## 0. Two host classes, never pooled

The session's container was stopped and came back on a different machine part-way through the timing stage, so the work was measured on two host classes.
Rows from the two are **never pooled**; every table is per host.

| | host A | host B |
|:--|:--|:--|
| CPU model line | Intel Xeon 2.80 GHz (KVM guest, 4 vCPU), AVX-512 F/BW/CD/DQ/VL/VNNI | Intel Xeon 2.10 GHz (KVM guest, 4 vCPU), AVX-512 incl. IFMA, VBMI, BF16, FP16, VPCLMULQDQ, GFNI |
| kernel | `6.18.44-fc-v114` (manifest: [`../baseline_v1/manifest.json`](../baseline_v1/manifest.json)) | `6.18.44-fc-v139` ([`manifest_hostB.json`](manifest_hostB.json)) |
| run | 2026-10-12 02:16-02:52 UTC | 2026-10-18 16:27-17:55 UTC |
| rows | 169 timing rows of 180 planned (the container stopped during `l = 13`, round 4); `l = 13` has **3** complete rounds | 180 interleaved rows + 30 thread rows, all completed |
| contended runs (by the tool) | 4 of 169 (kept; [`raw/ab_summary_excl_contended.md`](raw/ab_summary_excl_contended.md) drops their rounds) | 0 of 210 |
| binary | the same file on both hosts, SHA-256 `c40434d66da31bf619d79a3c27ee617e19b36a061067fac075d409391734a6c1` (`raw/binary_hashes.txt`) | same |

Host A's thread stage never ran (the host was gone before it); the thread results below are host B only.

## 1. Requested and executed

| PS-3 requirement | status | evidence |
|:--|:--|:--|
| Portable `u64` path; AVX2 and AVX-512 paths behind `is_x86_feature_detected!` and `#[target_feature]`, runtime dispatch, **no global `target-cpu`**; threads by `std::thread`; one-time table builds reported separately | **verified complete** (code and build: plain `cargo build --release`; every vector entry point asserts the detected feature; `setup_secs` is in every timing record) | §2, `bskernel.rs`, `lanes_x86.rs` |
| Frozen input: PS-2's program, hash recorded | **partial** (named deviation): the hash of the recorded program reproduces on every rung on both hosts, but the vector kernel is a hand-written kernel of the *same function* (same fused product, same fold, same inversion chain), **not an interpreter of the recorded operation sequence**; the product is scheduled in register tiles and the linear steps compute full-width temporaries the recorded program had dead-code-eliminated | §2, `raw*/programs_equal_ps2.txt` |
| Frozen inputs: PS-0 corpus and baseline; rungs `l = 8…12`, plus `l = 13` for timing only | **verified complete** for `l = 8…12`; `l = 13`: verdicts replayed on a 3-target sample (6 arms, 0 mismatches), timing 5 rounds on host B, **3 rounds on host A (partial)** | §3, §4 |
| Identical digests on the whole corpus at every width and thread count | **verified complete**: per host 30 `verify-arm` records (`bs64`, `bs256`, `bs512`, each at 1 and 4 threads, five rungs, 80 targets each, verdict digest and full root sets) plus 6 `l = 13` records; **0 mismatches in 36 records on each host** | §3 |
| Unit: pairs per second per core | **verified complete** (ns per own pair and ns per target, medians of 5 rounds) | §4 |
| Unit: counted bit operations per pair | **derived, not executed-counted**: PS-2's `G` per task divided by lanes and by lane utilisation; the executed kernel's own gate count was not instrumented | §6 |
| Unit: instructions retired under `callgrind` | **partial**: `bs64`, `bs256`, `group-scalar` and the baseline at `l = 9, 10, 11` (host B); **`bs512` cannot run under valgrind 3.22** (exit 252, recorded) | §6, `raw_hostB/counts_callgrind.jsonl` |
| Wall figures only from `isolated_bench` runs, ABAB with at least five rounds, with the A/A spread | **verified complete** except host A `l = 13` (3 rounds) | §4 |
| Thread counts 1 and 4 | **blocked at 4 isolated threads**: `isolated_bench` refuses to reserve all four vCPUs ("leave at least one free"; 200 refused starts in `raw_hostB/refused_starts.log`).  Measured instead: 1, 2, 3 reserved cores (isolated) and 4 threads **without isolation**, labelled and never pooled | §5 |
| Success: >= 10x the PS-0 baseline per core at `l = 11` outside the A/A spread | **verified complete at all three widths on both hosts** (table in §4); see §7 for what the factor consists of | §4 |
| Success: four-thread efficiency >= 85% | **not established at 4 isolated threads (blocked)**; efficiency at 2 and 3 isolated cores is 0.97-1.11 as medians of paired rounds (individual rounds 0.84-1.22; the single-core spread is 14-16%); the non-isolated 4-thread diagnostic gives 0.98 and 0.91 | §5 |
| Success: no regression of the single-thread figure when threads are enabled | **partial**: the one-thread path takes the direct-worker branch (no thread spawned); there is no build without thread support to compare against | §5 |
| Stop: below 5x after tuning at all three widths | **not triggered** | §4 |
| Hardware classes not measured are named | Arm64 NEON, Metal, a host with two 512-bit logic ports (if host B is not one), AMD: **not measured** | §8 |
| Inadmissible: the 512-lane figure without AVX2 and portable ones; pooling contended and uncontended runs; global `target-cpu` as the headline | respected: all three widths in every table; contended runs flagged and an excluding summary kept; no `target-cpu` flag | §4 |

## 2. What was built

`examples/pdp_speedup/`: `bitslice.rs` (the `Lane` trait, counting and recording lanes, the field `BsField<N>`, products), `bspairs.rs` (stage A: row-minus-base
differences; Montgomery inversion; stage C: the four candidates and the hit mask), `bskernel.rs` (tasks of a row and a lane block, batches of 8, threads, dispatch),
`lanes_x86.rs` (`W256` AVX2, `W512` AVX-512F with `vpternlogq` for the fused XOR-AND), `arms.rs` (`bs64|bs256|bs512` with a `-tK` thread suffix), `gatecount.rs` (`bs-micro`, `bs-util`, program export).
The field degree is a const generic (`n = 24, 27, 30, 33, 36, 39`).  A hit or a zero denominator is re-derived with the exact reference (`group::thirds`), which also confirms every hit.

Tuning (kept, with the rejected attempts, in [`tuning_log.md`](tuning_log.md)): a closure in a generic body is compiled without the caller's `#[target_feature]`, so vector
intrinsics inside it did not inline (found with callgrind; fixed: `bs256` 63 -> 33 ns/pair); a column-wise product was slower at every width (rejected); a register tile with an accumulator array
was not promoted to registers (rejected); the tile written out with named accumulators was kept.

## 3. Identical outputs

`raw/verify_arms.jsonl`, `raw_hostB/verify_arms.jsonl`: for each of `bs64, bs256, bs512, bs64-t4, bs256-t4, bs512-t4` and `l = 8…12`, the arm's verdict on all 80 PS-0 targets equals the corpus verdict, the
verdict digest equals PS-0's ([`../baseline_v1/README.md`](../baseline_v1/README.md) §4), and the arm's full root set equals the corpus root set.  `verify_l13.jsonl`: the baseline, `group-scalar`, the three widths and `bs512-t4` on the
3-target `l = 13` sample (`raw/corpus_l13.txt`, built by the independent enumerator).  **0 mismatches in 30 + 6 records on each host.**  Program hashes: `raw*/programs_equal_ps2.txt` reads `true` on both hosts.

## 4. Timing (interleaved rounds, one pinned core, per host)

Unit: wall nanoseconds on one reserved core (`tools/isolated_bench.py run --cpus 3`), per refuted target; medians over rounds; each round runs `baseline`, `baseline2` (the baseline again: the A/A), `group-scalar`,
`bs64`, `bs256`, `bs512` once, in that order.  Workloads: the first 16 refuted PS-0 targets (6 at `l = 12`, 3 at `l = 13`); baseline arms use PS-0's pass counts, the others repeat passes until 3 s.
Full tables with min...max and bootstrap intervals: `raw/ab_summary.md`, `raw_hostB/ab_summary.md`.

**Host A** (milliseconds per refuted target):

| l | rounds | baseline | group-scalar | bs64 | bs256 | bs512 | A/A spread (baseline) | contended runs |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 8 | 5 | 11.13 | 0.351 | 0.808 | 0.652 | 0.802 | 7.95% | 2 |
| 9 | 5 | 48.88 | 1.248 | 2.906 | 1.524 | 1.862 | 8.72% | 2 |
| 10 | 5 | 206.44 | 3.504 | 9.574 | 4.035 | 3.796 | 5.42% | 0 |
| 11 | 5 | 897.32 | 15.863 | 38.494 | 14.711 | 14.498 | 5.01% | 0 |
| 12 | 5 | 3839.87 | 60.292 | 160.481 | 59.347 | 50.358 | 9.30% | 0 |
| 13 | **3** | 16167.60 | 245.456 | 749.956 | 262.647 | 245.700 | 2.06% | 0 |

**Host B** (milliseconds per refuted target):

| l | rounds | baseline | group-scalar | bs64 | bs256 | bs512 | A/A spread (baseline) | contended runs |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 8 | 5 | 8.96 | 0.249 | 0.705 | 0.688 | 0.690 | 6.15% | 0 |
| 9 | 5 | 37.19 | 0.828 | 2.584 | 1.478 | 1.584 | 9.76% | 0 |
| 10 | 5 | 159.53 | 2.615 | 9.395 | 4.170 | 3.514 | 6.34% | 0 |
| 11 | 5 | 687.65 | 12.156 | 39.036 | 17.242 | 13.712 | 4.48% | 0 |
| 12 | 5 | 2979.92 | 46.021 | 175.385 | 72.145 | 47.463 | 6.14% | 0 |
| 13 | 5 | 12426.10 | 178.175 | 784.152 | 304.325 | 208.012 | 4.80% | 0 |

**The protocol's headline: the baseline against each width at `l = 11`**, per target, B/A medians as recorded and their reciprocals (derived: `1/(B/A)`; the baseline's own run-to-run spread is the A/A bound):

| host | arm | B/A median (min...max) | baseline / arm |
|:--|:--|:--|--:|
| A | group-scalar (PS-2) | 0.0175 (0.0170...0.0179) | 57.1 |
| A | bs64 | 0.0420 (0.0402...0.0463) | 23.8 |
| A | bs256 | 0.0165 (0.0160...0.0170) | 60.6 |
| A | bs512 | 0.0160 (0.0150...0.0176) | 62.5 |
| B | group-scalar (PS-2) | 0.0177 (0.0158...0.0181) | 56.5 |
| B | bs64 | 0.0570 (0.0506...0.0648) | 17.5 |
| B | bs256 | 0.0241 (0.0228...0.0266) | 41.5 |
| B | bs512 | 0.0199 (0.0169...0.0220) | 50.3 |

**The bit-sliced arms against PS-2's scalar group-law kernel**, per own pair (ns), `l = 11…13` (the rungs where the kernel is not starved of lanes), B/A of `group-scalar` -> arm:

| host | l | group-scalar ns/pair | bs64 | bs256 | bs512 | bs512 / group-scalar (min...max) |
|:--|--:|--:|--:|--:|--:|:--|
| A | 11 | 31.3 | 75.8 | 29.0 | 28.6 | 0.935 (0.841...1.038) |
| A | 12 | 30.4 | 80.8 | 29.9 | 25.4 | 0.806 (0.610...0.974); group-scalar spread 39.8% |
| A | 13 (3 rounds) | 29.9 | 91.3 | 32.0 | 29.9 | 0.980 (0.938...1.044) |
| B | 11 | 24.0 | 76.9 | 34.0 | 27.0 | 1.117 (0.957...1.393) |
| B | 12 | 23.2 | 88.4 | 36.3 | 23.9 | 1.031 (0.914...1.118) |
| B | 13 | 21.7 | 95.5 | 37.1 | 25.3 | 1.137 (1.065...1.242) |

At `l = 8, 9` the vector arms are slower than the scalar kernel on both hosts (host A `l = 8`: 2.3x for `bs512`; host B 2.6x): there are only 128 and 253 base points, so a 512-lane block is mostly empty (`raw/utilisation.jsonl`).

## 5. Threads (host B only)

`bs512` on 1, 2 and 3 reserved cores (`isolated_bench run --cpus 3 | 2-3 | 1-3`), five interleaved rounds, `l = 11` and `l = 12`; medians of the per-round ratio `1c / Nc` (`summarize`), efficiency = speed-up / N:

| l | cores | ms per target (median) | speed-up vs 1 core | efficiency | min...max of B/A |
|--:|--:|--:|--:|--:|:--|
| 11 | 1 | 12.90 | - | - | single-core spread 14.33% |
| 11 | 2 | 5.82 | 2.21 | 1.11 | 0.4087...0.4999 |
| 11 | 3 | 4.16 | 3.22 | 1.07 | 0.2837...0.3581 |
| 12 | 1 | 44.42 | - | - | single-core spread 16.14% |
| 12 | 2 | 22.88 | 2.02 | 1.01 | 0.4543...0.5540 |
| 12 | 3 | 15.75 | 2.90 | 0.97 | 0.2972...0.3945 |

Efficiency above 1.0 is not a physical result: the single-core arm's spread is 14-16%, so medians from 0.97 to 1.11 are consistent with linear scaling; individual rounds range from 0.84 (`l = 12`, 3 cores) to 1.22 (`l = 11`, 2 cores).  **The protocol's 4-thread isolated run could not be made:** `isolated_bench` refuses to reserve all four vCPUs.
Beside it, `bs512-t4` pinned with `taskset -c 0-3` but **not isolated** (`raw_hostB/times_4thread_not_isolated.jsonl`, five rounds, the benchmark lock held, nothing else running by inspection but no tool-recorded evidence):
3.29 ms per target at `l = 11` (speed-up 3.92, efficiency 0.98 against the median 1-core figure) and 12.22 ms at `l = 12` (3.64; 0.91).  These two numbers are a diagnostic, not a discharge of the requirement.

## 6. Counted units

* **Lane utilisation** (valid pairs / lane slots; `raw/utilisation.jsonl`, exact arithmetic from the base-point count): at `l = 11` 0.913 (64 lanes), 0.780 (256), 0.653 (512); `l = 8`: 0.672 / 0.252 / 0.126; `l = 9`: 0.788 / 0.496 / 0.248; `l = 10`: 0.747 / 0.585 / 0.454; `l = 12`: 0.919 / 0.847 / 0.766; `l = 13`: 0.964 / 0.923 / 0.872 (64 / 256 / 512 lanes).
* **Gates per valid pair** (derived from PS-2's program: 12,879 fused gates per task at `l = 11`, a batch of 8 tasks = 103,030): 12,879 / lanes / utilisation = **220** (64), **64.5** (256), **38.5** (512).  The plan's model puts the 512-lane time at `G/(512·2·2.8)` ns per lane slot = 4.5 ns; the measured time per slot at 512 lanes is 18.7 ns (host A: 28.6 ns x 0.653), 4.2x the model's 4.5 ns, and 17.6 ns (host B: 27.0 x 0.653), 2.9x the model evaluated at host B's 2.1 GHz nominal clock (6.0 ns) (arithmetic on the recorded medians).
* **Instructions retired** (`callgrind`, host B, one refuted target, `Ir(passes = 2) - Ir(passes = 1)`; `raw_hostB/counts_callgrind.jsonl`):

| l | baseline Ir/pair | group-scalar | bs64 | bs256 | bs512 |
|--:|--:|--:|--:|--:|--:|
| 9 | 2,570 | 290 | 824 | 649 | cannot run under valgrind 3.22 |
| 10 | 2,787 | 283 | 889 | 559 | same |
| 11 | 3,007 | 278 | 814 | 440 | same |

  Caveat: `callgrind` counts each iteration of a `rep`-prefixed string instruction as an instruction.  A profile of an earlier build of `bs256` attributed 33% of its instructions to `memset` of the 2.5 KB plane arrays
  (a handful of cycles each on hardware); the share was **not re-measured on the final binary**, so the bit-sliced columns overstate their instruction count by an unknown amount and Ir is not a time proxy across these arms.
* **Product micro-benchmark** (`bs-micro`, `raw*/micro.jsonl`, 3,000 chained products at `l = 11`, a single run per host, diagnostic): host A 313 / 352 / 519 ns per product at 64 / 256 / 512 lanes; the `mul u64` row of host B (1,535 ns) is an outlier against that host's other `u64` product rows (about 360-380 ns), reported as recorded.

## 7. Reading the numbers (interpretation, labelled)

* **Observation.**  Against the PS-0 baseline the three vector arms and PS-2's scalar group kernel are all faster per target by at least the protocol's factor of 10 at `l = 11` on both hosts (17.5x to 62.5x).
* **Observation.**  Against PS-2's scalar group kernel (same candidates, same exact reference) the vector arms are **not distinguishable** from it at `l >= 11` on host A (0.81-0.98, with the group kernel's own spread up to 39.8% at `l = 12`) and are **slower or equal** on host B (1.03-1.14 at `bs512`, 1.4-1.7 at `bs256`, 3.3-4.4 at `bs64`).
  So the factor over the baseline is carried by the change of algorithm made in PS-2 (liftable candidates and the group-law test); **the bit-sliced evaluation has not, in this implementation, added a measured factor on top of it** on either host.
* **Measured, not explained further than the logs allow.**  The residual gaps recorded in `tuning_log.md`: about 30% of a batch outside the products, 65% lane utilisation at 512 lanes, and a product that retires about one fused gate per cycle at 512 lanes on host A.
  A lean `stage_c`, in-place temporaries and a diagonal-block lane map are not built; they are leads (L3), not results.  This README makes **no statement** about what an optimised vector kernel would reach.
* **Not extrapolated.**  Nothing here is an end-to-end or ECC2K-130 figure; the stage is the oracle's pairs-and-solve step on one host class per table.

## 8. Not done, not claimed

* The 4-thread isolated measurement (blocked by the tool's refusal, §5); the host A thread stage (host gone); host A `l = 13` rounds 4-5 (host gone).
* `bs512` instruction counts (valgrind 3.22 has no AVX-512); the executed kernel's own counted bit operations (only PS-2's `G`, derived).
* The library module `semaev_gray_pairs` and the cryptanalysis-suite port (PS-3b) are not done; the kernel lives in the example crate.
* Arm64 NEON, Apple Metal, AMD x86 and a host with two 512-bit logic ports (if host B is not one) were not measured and nothing is implied for them.
* Wall figures are exploratory in the repository's isolation vocabulary (shared container with the benchmark lock, not the receipt service).
