# Validation report: prefiltered symmetric j=0 IC (claims A and B)

**Disposition:**
- **Claim A:** validated, with qualifiers.
- **Claim B:** validated only as pre-registered (baseline c=0.5). Not validated as a general "beats `ic_multi_solve_ex`" claim: with the baseline's c tuned, the T=512 gain is 1.15x at 30 bits and 1.35x at 32 bits.
- **BSGS side claim:** direction confirmed. The stated 2.0x-3.4x range does not match my measurement of 2.3x-4.9x.
- **Gate eligibility:** `insufficient_evidence` (see section 6).

**Binding**
- Commit: `ee67edf0563227fb4c45be2d2119d8b95595f2b6`, branch `claude/prime-field-factor-base-calculus-wy2own`.
- The tracked tree was clean at start and end. The working-tree blobs of the three examples and `Cargo.lock` equal HEAD.
- Binaries were built with `cargo build --release --locked --offline`. They were built into `CARGO_TARGET_DIR=<scratchpad>/target`, which I deleted afterwards.
- Binary sha256 values:
  - `prime_fast_j0_sym`: `70d2dff2…1fbf`
  - `prime_fast_j0_multi`: `6fb2f733…5831`
  - `prime_fast_j0_bsgs_multi`: `71c3c23a…ac50`
- Environment: Firecracker VM, 4 vCPU Intel Xeon 2.1 GHz, 15 GB RAM, rustc and cargo 1.97.0, default release profile, no RUSTFLAGS. Runs were sequential, with no isolation and no EKS. This is local validation, not a benchmark of record.
- Scratchpad root: `/tmp/claude-0/-home-user/ecc58476-b907-5471-a536-d3549b06e740/scratchpad`. It also holds many unrelated pre-existing files. I wrote only to `out/`, `certout/`, `certfrozen/`, `csweep/`, `replay/`, `neg/`, `cert/`, `scripts/`, `build.log` and `cert_build.log`.

## 1. Frozen files and ordering

| Item | Result | Evidence |
|---|---|---|
| `frozen_prefilter_holdout.json` sha256 | PASS: `d5eb7a62…b007`, matches its `.sha256` and the dispatch | `sha256sum -c` from repo root, under `research/ic_e222_m221_20261106/results/pf_j0/` |
| `frozen_prefilter_multi_holdout.json` sha256 | PASS: `4ccebf84…cfd66` | same |
| Single-target ordering | PASS: freeze `d5f47c3b75` (23:13:33Z) predates results `5b05b3c91c` (23:14:39Z) | `git log` |
| Multi-target ordering | PASS: freeze `bae578d3f6` (23:19:53Z) predates results `83c981d2fe` (23:21:21Z) | `git log` |
| Single-target tuning rows `t_*.jsonl` | PASS: committed in the freeze commit | `git log --diff-filter=A` |
| Code unchanged since freeze | PASS: `prime_fast_j0_sym.rs` unchanged since `b5a7480004`. `prime_fast_j0_multi.rs`, `prime_fast_j0_bsgs_multi.rs` and `src/` are unchanged since `bae578d3f6`. `Cargo.*` is unchanged since `d5f47c3b75`. | `git diff` |

Findings:
- **Multi freeze not separate from code.** The multi freeze is in the same commit as the code it freezes. The file says so.
- **Multi tuning evidence postdates the freeze.** The rows behind B=350 (`ms_*.jsonl`) were first committed in the results commit `83c981d2fe`, after the freeze. So the B=350 choice is not verifiably pre-registered.
- **B=220 is not tuned.** It is the cube-root scaling of 350, with no 30-bit tuning data.
- **Commit times are not proof.** Commit timestamps are producer-controlled, and the rows carry no start times or load metadata.

## 2. Prefilter soundness (read of both diffs)

- **Single-target:** `examples/prime_fast_j0_sym.rs`, commits `549d558ed4` and `b5a7480004`.
- **Multi:** `examples/prime_fast_j0_multi.rs`, commit `bae578d3f6`.

**Argument that it cannot drop a valid relation.**
- `canon(x)` returns the minimum of {x, βx, β²x}. So a key equal to some pair-table key `k0` forces x ∈ {k0, β²k0, βk0}, because β³=1.
- The bitset is built from exactly `{k0, β·k0, β²·k0}` for every entry, with the same hash and mask as the probe.
- A probe on raw x3 that returns false therefore means no entry can match, so there are no false negatives. False positives are only extra work.
- The gate covers only the 3-summand branch.
  - In `sym_solve_ex`, `summands >= 3 && pass`, and the 2-summand `tab.get(x3)` path is ungated.
  - In the multi example, `!pass` leaves `nh = 0`, and the 2-summand path still runs. The 16-hit cap is unchanged.
- Multi-example specifics:
  - `pf = !rot && NOPREFILTER unset`, so `ROTKEYS` mode is unfiltered.
  - Bits are built from `ents` keys, which are canonical in non-rot mode.
  - The probe is on raw x3, before `canon`.
- Scan order and RNG consumption are unchanged, so any dropped relation would change the target and relation counts.

**Empirical confirmation.**
- Single-target: 288/288 ON/OFF row pairs have identical `sym_targets`, `sym_rels`, `sym_inv`, curve and instance.
- Multi-target: 224/224 pairs have identical `sym_determined`, `sym_solved` and `base_fb`.
- With the instrumented harness, scan counts are identical in 32/32 multi rows (ON vs OFF).
- Recovered logs are identical ON vs OFF in all harness rows (sym 48/48, multi 32/32).

## 3. Fresh-curve runs

Setup:
- New curves: CSKIP=800 and CSKIP=900, two curves each.
- Arms were interleaved in ABBA order, 3 reps. ON is the default; OFF is `NOPREFILTER=1`.
- Reps repeat identical instances, so rep spread is timing noise only.
- Single-target ran with `NOPLAIN2=1`, extra=40, summands=3, frozen B and C3.
- Multi ran with extra=40, c=0.5, 4 instances per curve.
- Raw rows: `<scratchpad>/out/A_cs*_b*_{on,off}_r*.jsonl`, `B_cs*…`, `BS_cs*…`. Drivers are in `scripts/run_a.sh` and `scripts/run_b.sh`. Analysis scripts are `scripts/analyze_a.py` and `scripts/analyze_b.py`.
- Verification: every sym, plain3, base and BSGS solve verified. That is 288 rows for A and 224 for B. There were 0 failures.

### A. Single-target, pooled plain3/sym (32 rows per size per rep)

| bits | B / c | ON ratio per rep | mean | 95% bootstrap CI over instances | prediction | OFF ratio (mean) | prefilter speedup of sym (mean) |
|---|---|---|---|---|---|---|---|
| 28 | 116 / 0.7 | 3.494, 3.491, 3.525 | 3.50 | 3.40-3.61 | >=3.2 | 2.50 | 1.44 |
| 30 | 184 / 0.8 | 3.983, 4.068, 3.930 | 3.99 | 3.86-4.14 | >=3.5 | 2.62 | 1.53 |
| 32 | 292 / 0.5 | 4.006, 4.120, 4.067 | 4.06 | 3.96-4.18 | >=3.7 | 2.37 | 1.72 |

- Per-CSKIP pooled ratios:
  - 28 bits: 3.52 and 3.49.
  - 30 bits: 3.99 and 4.00.
  - 32 bits: 4.07 and 4.06.
- Per-curve range across the 6 cells is 3.37-4.18. Per-row ratios run from 2.67 to 6.4 (minimum 2.67, 2.77, 2.80 by size). The abandon threshold is 3.0.
- OFF would fail the frozen thresholds at every size. The claimed gain is the prefilter plus the retuned B, not the symmetric arm alone.
- Replay of the exact frozen holdout (CSKIP=600, same commands, same source):
  - All deterministic fields match the committed `h28/h30/h32.jsonl` in 48/48 rows. The fields are curve, n, `sym_targets`, `sym_rels`, `sym_inv` and the `*_ok` flags.
  - My pooled ratios there are 3.42/4.44/4.19, against the producer's committed 3.29/4.15/4.04.
  - Files: `replay/h*.jsonl`.
- The tuning evidence for the frozen B and baseline c reproduces from `t_*.jsonl`: baseline optimum c = 0.7/0.8/0.5 and sym optimum B = 116/184/292.

### B. Multi-target, pooled base_total/sym_total (16 rows per cell per rep)

| bits, B | T | ON per rep | ON mean | 95% CI | OFF mean | claim |
|---|---|---|---|---|---|---|
| 30, 220 | 64 | 1.893, 1.837, 1.789 | 1.84 | 1.76-1.92 | 1.21 | >=1.3 |
| 30, 220 | 512 | 1.580, 1.687, 1.594 | 1.62 | 1.56-1.68 | 1.05 | >=1.3 |
| 32, 350 | 64 | 1.720, 1.735, 1.722 | 1.73 | 1.67-1.78 | 1.00 | >=1.3 |
| 32, 350 | 512 | 1.591, 1.591, 1.530 | 1.57 | 1.51-1.64 | 0.84 | >=1.3 |
| 32, 350 | 4096 (not in claim) | 1.461, 1.467 | 1.46 | 1.40-1.53 | 0.69 | none |

- Per-curve cells are all >=1.42 (1.49 for T<=512).
- The producer's rows replay with identical deterministic fields in 32/32 rows (CSKIP=700, `replay/m*.jsonl`). My ratios there are 1.74/1.63/1.94/1.65, against the producer's 1.80/1.63/1.72/1.59.
- Without the prefilter, symmetric multi is not faster than the baseline at 32 bits. The whole B gain is the prefilter.

### Timing-scope check
- **Single-target:** no inequality found.
  - `sym_ms` covers rep selection, pair table, bitset build, collection, linear algebra and the final `mul` verification.
  - `plain3_ms` wraps the whole `ic_solve`: factor base, PairTable, collection, linear algebra and verify.
  - Both arms include the pair table. Sym runs first, which if anything penalises sym.
  - The `plain3` A/A (same code and seeds, ON-run vs OFF-run) shows run-to-run noise: ratio 0.89-1.06 at 28 bits, 0.96-1.06 at 30, 0.97-1.03 at 32.
- **Multi-target:**
  - Both arms include setup, pair table, collection, per-target work and per-target `mul` verification.
  - The ON bitset build is inside sym time. It adds about 9 ms at 32 bits (pair 38.8 vs 29.9 ms).
  - The baseline is hard-coded to extra=6, while sym uses 40. That slightly favours the baseline.
  - The baseline batches all targets in lock-step, while sym walks them sequentially. This is a design difference, not a scope inequality.
  - The target counts the claim cites are not in the multi rows. I confirmed them with scan counts from the instrumented harness.

### Controls
- **Baseline c at 30 bits.** At 30 bits the baseline was never swept.
  - The `ee67edf056` sweep was 32 bits only, on a 5-point grid, with 6 rows per cell.
  - I swept c (single run per cell, 8 rows, ratio = base(c) / sym ON). Raw rows are in `csweep/`.
  - Selecting the best c on the same data favours the baseline, but the c=0.5 vs c=0.8 gap at 30 bits (36%) is well outside noise.

| cell | c=0.4 | 0.5 (frozen) | 0.65 | 0.8 | 1.0 | 1.2 |
|---|---|---|---|---|---|---|
| 30b T=512 (pooled over CSKIP 800 and 900) | n/a | 1.60 | 1.27 | **1.15** (1.14 and 1.16) | 1.36 | 1.46 |
| 32b T=512 (pooled over CSKIP 800 and 900) | n/a | 1.59 | 1.53 | **1.35** (1.36 and 1.34) | 1.52 | n/a |
| 30b T=64 (CSKIP 800 only) | 2.38 | 1.93 | **1.78** | 1.80 | n/a | n/a |
| 32b T=64 (CSKIP 800 only) | 1.99 | 1.77 | **1.74** | 1.94 | n/a | n/a |

- **Cheaper baseline for A.** `ic_solve_table` (c in {0.5, 1.0, 1.5}) is 1.8x slower than `ic_solve` at 3 summands at all three sizes (best table/s3 = 1.78, 1.92, 1.76). So `ic_solve` at 3 summands is the stronger in-repo single-target baseline. Rows: `certout/ctl_b*.jsonl`.
- **Rho reference (negation, 1 thread, median of 10, from the same rows).** Not part of the claim, but it sets the distance remaining.

| bits | mean sym ms | mean plain3 ms | rho ms | sym/rho | plain3/rho |
|---|---|---|---|---|---|
| 28 | 17.5 | 61.5 | 3.7 | 4.7x | 16.5x |
| 30 | 43.7 | 174.7 | 5.1 | 8.6x | 34.5x |
| 32 | 124.0 | 504.2 | 6.9 | 18.1x | 73.5x |

- The gap to rho widens with bits. The producer's own log says the folded rho is faster still, so the rho in the rows is not the strongest available.

### BSGS, same CSKIP, bits, T and targets (mean ms/row, all targets verified, n=48 per cell, 32 at T=4096)

| 32 bits | BSGS | sym multi (ON) | `ic_multi_solve_ex` (c=0.5) | sym/BSGS (per rep) | claimed |
|---|---|---|---|---|---|
| T=64 | 26.3 | 130.2 | 224.7 | 4.94x (4.66-5.29) | 2.8x (46 vs 128 ms) |
| T=512 | 104.9 | 254.8 | 400.0 | 2.43x (2.34-2.58) | 2.0x (123 vs 241) |
| T=4096 | 367.3 | 1259.1 | 1843.1 | 3.43x (3.33-3.53) | 3.4x (333 vs 1119) |

- At 30 bits BSGS is 3.7x (T=64) and 2.6x (T=512) faster than sym multi.
- BSGS is faster at every size and T tested, so the order is confirmed.
- The producer's committed BSGS rows (`bs_*.jsonl`) were on different curves (b296/b301 at 32 bits). They are not the CSKIP=40 rows the 46/123/333 ms figures came from.
- The BSGS table (about sqrt(T·n/6) entries, 1.7M at T=4096) is unpriced.

## 4. Requirement table

| # | Requirement | Result |
|---|---|---|
| 1 | Frozen sha256 and ordering | PASS, with the multi-tuning provenance gap above |
| 2 | Prefilter cannot drop a relation | PASS by proof and by 288 + 224 identical ON/OFF pairs |
| 3 | Clean build, new curves, interleaved ON/OFF, both examples | PASS |
| 4a | Every solve verified, counts identical ON/OFF | PASS (0 failures; scans identical) |
| 4b | A thresholds (3.2/3.5/3.7) | PASS at all sizes, all reps, and at the CI lower bound |
| 4c | B prediction >=1.3 at c=0.5 | PASS in all four claimed cells |
| 4d | B vs tuned baseline c | FAIL at 30 bits T=512 (1.15) and marginal at 32 bits T=512 (1.35) |
| 4e | Timing-scope equality | PASS, no inequality found (minor baseline-favouring extra=6) |
| 4f | BSGS order and magnitude | Order PASS. Stated magnitudes do not match: 2.4x-4.9x measured vs 2.0x-3.4x stated |
| 5 | Independent certificate check | PASS via instrumented copy (below); rows alone are not enough |
| 6 | Gate artifacts | Not present (below) |

## 5. Certificate check

**What the rows print.** Rows carry only `*_ok` booleans (solver output equals the planted k, plus an in-solver `mul` check). They carry no recovered scalar, no G or Q coordinates and no k. An independent certificate cannot be derived from the committed rows. Their `p` and `n` are printed; `b` is only in the curve name.

**What I did.**
- I made scratch copies that differ from the committed examples only in `main` prints. Solver code is byte-identical. The diffs are in `cert/harness_diff.txt`, the sources in `cert/src/{sym,multi,bsgs}.rs`, and the build is `cert/Cargo.toml`.
- The copies emit p, n, G, Q, k and every recovered scalar.
- A pure-Python checker (`scripts/cert_verify.py`) shares no code with the solver. For each row it checks:
  - p is prime and ≡ 1 mod 3.
  - n is prime and within the Hasse interval, with n > 4√p, so n·G = O fixes the group order.
  - G and Q lie on the curve.
  - x·G == Q.
  - x == k.

**Results.**
- 46,272 solve certificates from my fresh-curve runs (CSKIP=800, 6 curves) verified. The count covers sym, plain3, base and BSGS arms, ON and OFF, with 0 failures. Files: `certout/`.
- 18,528 further certificates on the exact frozen holdout instances (CSKIP=600 and 700, 10 curves) verified. Curve, n and instance match the committed rows (`certfrozen/`).
- Negative controls reject: a scalar off by one, a swapped Q, a wrong n, and a missing scalar (`neg/tamper.jsonl`).
- The recovered logs equal k in every sym, base and BSGS row.

**What is still missing.** The committed rows contain no certificate. A committed replay certificate needs the instrumented prints, or an equivalent, in the example itself.

## 6. Verdicts and what the gate still needs

**Claim A: validated, with qualifiers.**
- Replicated on fresh curves and on the replayed holdout; all thresholds met.
- The 28-bit and 32-bit validator runs used primes different from the tuning and holdout primes. At 30 bits my runs share the holdout's p and n.
- Qualifiers:
  - Wall clock on a shared VM with no isolation; A/A noise is up to about ±11% at 28 bits.
  - The result is a constant factor over `ic_solve`, not an advance.
  - The gap to rho is 4.7x/8.6x/18.1x (sym vs the negation-rho reference in the rows) and widens with bits. The rho in the rows is not the strongest available.
  - B and c were tuned on a different prime per size.
  - The baseline was tuned only over a coarse grid.

**Claim B: validated only as pre-registered; not validated as a general claim.**
- The frozen comparison (baseline c=0.5, B=350/220) holds at all four cells.
- Against a baseline with tuned c, the T=512 gain is 1.15x at 30 bits (below the 1.3 prediction, close to the 1.1 abandon line) and 1.35x at 32 bits.
- T=64 holds at about 1.7-1.8x.
- The "1.4x-1.6x vs best c" statement from `ee67edf056` comes from a 32-bit-only coarse sweep and does not transfer to 30 bits.
- The sym arm's B was also untuned at 30 bits.

**BSGS claim.** The folded BSGS is faster than symmetric multi-target IC at every setting tested. The magnitude range should read about 2.3x-4.9x at 32 bits, not 2.0x-3.4x. The IC arms are therefore behind a generic baseline in the multi-target setting.

**What the validation gate (`docs/VALIDATION_GATE.md`) still needs:**
- **Replay certificate.** A committed `replay-certificate.json` with scalars verified and a non-empty `runs` list, bound to committed rows. The example does not print scalars, so the certificate comes from a separate harness. The harness above is a supplement, not that file.
- **Validator report.** A committed, signed `validator-report.json` with `raw_data` and a `headline` list of claimed vs recomputed values. The current Markdown validator reports are not that.
- **Matched rho reference with the §5a band.**
  - The strongest rho for the curve, i.e. the order-6 folded rho, in the same unit and binary.
  - Native-operation counters for every arm, rho included, priced at 0, 0.1 and 1 GAE, plus the calibrated weight.
  - The rows do not emit the counters. `plain3`'s `IcReport` target and relation counts are unused, and sym emits only `sym_targets` and `sym_inv`.
- **`VALIDATION.json` and the benchrec record.** A `VALIDATION.json` in the evidence directory, plus a `bench.record/v1` record, S3 raw-data hashes and the progress report. The producer's notes say S3 and AWS identity are unavailable.
- **Isolated run.** Rows from isolab-eks with A/A intervals. Current rows are wall clock on a VM with no isolation.
- **pcc claim ids and standing.** All rows are labelled `[pcc:unverified]`.
- **Baseline c re-swept per size and per T.** A sweep at 30 bits and T=512 is needed for claim B, with the c grid refined around 0.65-0.8, since the optimum moves with T.
- **Tuning provenance for B.** Tuning evidence for B=350 committed before the freeze, and a justification or tuning for B=220.

**Not done:** no multi-target rho comparison, no per-operation counts, single-run c-sweep cells, and BSGS vs sym only at the stated bits and T.
