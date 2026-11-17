# Validation report: 4-summand relation collection (Claim A)

All figures below are [pcc:unverified]. No pcc claim id exists for sym4: `pcc/claims`, `pcc/ledger.jsonl` and `tools/pcc` have no entry for it.

## Disposition

- **Claim A (wall-clock, local, 28/30/32 bits):** validated with qualifiers.
  - Every frozen prediction was met on replay, no abandon rule triggered, and all solves were independently certified.
  - The qualifiers are in sections 4 and 6.
- **Gate eligibility:** insufficient_evidence. The missing items are in section 7.

## Binding

| Item | Value |
|---|---|
| Commit and tree | HEAD `3039cfa8714ffdd319fc20a9eb996a0ff422b89c`, branch `claude/prime-field-factor-base-calculus-wy2own`; `git status` was clean before and after. |
| Build | `cargo build --release --locked --offline`, rustc/cargo 1.97.0, no RUSTFLAGS, 10m50s. |
| Binary | `.../scratchpad/target/release/examples/prime_fast_j0_sym`, sha256 `40bd5b649584d66a3ea14f7e01e3802706fa54ae694843cc493bf6fe425e6414`. |
| Hashes of inputs | `examples/prime_fast_j0_sym.rs` `3c9c9bef2f89dfba14584…` (full: `3c9c9bef2f89dfba14584f9ca96101a26064872b30072d113271da69e1519416`); `src/cryptanalysis/prime_fast.rs` `56bf5abc…`; `Cargo.lock` `cf805b65…`. |
| Host | Firecracker VM, 4 vCPU Intel Xeon 2.1 GHz. Runs were sequential and unisolated, with no EKS and no A/A. A few short Python checks ran beside the timed runs. |

Scratchpad root: `/tmp/claude-0/-home-user/ecc58476-b907-5471-a536-d3549b06e740/scratchpad/`. Its subdirectories hold my files: `out/`, `out1000/`, `outg/`, `sweep/`, `neg/`, `scripts/`, `analysis.txt`. Pre-existing unrelated files from earlier sessions are also there.

## 1. Requirement table

| Requirement | Result | Evidence |
|---|---|---|
| Frozen file hash | PASS. `e9e54a22d4f13550c471766498a4719f329e72679f5cfab44d30e41176feaac3` matches the `.sha256`. | `results/pf_j0/frozen_sym4_holdout.json` |
| Freeze predates rows | PASS, by commit time only. Code `036fd2f27` 20:18:20Z, freeze `c475944d45` 20:24:55Z, rows `239539d45d` 20:27:10Z. Commit times are producer-controlled. | `git log` |
| Frozen file and tuning rows untouched afterwards | PASS. Each is touched by exactly one commit. | `git log` |
| Code unchanged freeze to results (and to HEAD) | PASS. `git diff --name-only 036fd2f27 HEAD -- src Cargo.* examples crates tools` is empty. | `git diff` |
| Tuning numbers recompute | PASS. 8.4/6.5/6.7; 22.8/19.0/13.5/17.3; 41.1/38.0/64.8/175.8. | `results/pf_j0/sym4_tune/` |
| Producer holdout headline recomputes | PASS. plain3/sym4 7.530/10.401/12.423; sym3/sym4 2.101/2.300/2.989. | `results/pf_j0/sym4_holdout/h{28,30,32}_{s4,s3}.jsonl` |
| Row set complete | PASS. 6 files, 16 rows each, no null `sym_ms`, all `sym_ok`/`plain3_ok` true. Parameters equal the frozen ones, including `plain3_fb` 452/819/813. | same |
| Rows bound to this code | PASS. I re-ran the exact `CSKIP=1000` commands: 192/192 row comparisons match exactly on `cert` (g, q, k, recovered scalars), `sym_targets`, `sym_rels`, `sym_inv`, p, n and B. | `scratchpad/out1000/` |
| Soundness of `sym4_solve_ex` | PASS (read, section 2). | `examples/prime_fast_j0_sym.rs` lines 141-431 |
| Timing scope | PASS with a note (section 3). | |
| Fresh-curve replay | PASS with the freshness qualifier in section 4. | `scratchpad/out/`, `outg/`, `analysis.txt` |
| Independent certificates | PASS for 1728 replay rows plus 96 producer rows. A checker gap was found (section 5). | `scripts/strict_cert.py` |
| Reporting contract: one unit, S, §5a band, phases, rho at §5a pricing | FAIL. Only ms is reported. | section 7 |
| m=83 / ECC2K-130 rules | N/A. This is a prime-field j=0 claim and carries no ECC2K-130 transfer claim. It must not be cited as m=83 evidence. | |

## 2. Soundness read of `sym4_solve_ex`

**What a row requires.**
- A row enters the system only if the signed sum of the four chosen `cand` points, computed by full affine `c.add`, equals R exactly (lines 327-334).
- Rows with all-zero F coefficients are skipped.
- Duplicate rows are dropped within a target.

**The final answer is checked.**
- The answer must pass `c.mul(g, xsol) == q` (line 416), else the function returns None.
- A wrong relation therefore cannot reach an accepted answer.

**The solver never sees k.**
- The function receives only `(c, g, q, lam, B, extra, seed)`.
- The seed is `ins+1`, independent of k.

**The coefficient model holds.**
- `lams = [1, lam, lam^2]` matches `cand[3i+k] = (beta^k x, y)`.
- `beta_of` ties beta to lam by an exact multiplication.
- All points lie in a prime-order group: n is in the Hasse interval and n > 4 sqrt(p), so n = #E.

**Failures and the free-column case.**
- Failures return None, so `sym_ms` is null and `sym_ok` is false. There are no silent drops: 0 of 1728 replay rows failed.
- A free-column pivot-row mistake would be caught by the kG=Q check.

## 3. Timer scope

| Arm | What is inside the timer |
|---|---|
| sym4 and sym3 (`ms_total`, `t0` to the end) | Orbit representatives, candidate points, table build, prefilter bitset, collection, Gauss-Jordan, and the final kG=Q check. |
| plain3 (`p3_ms`, wall clock around `ic_solve`) | Factor-base build, pair table, collection, linear algebra and verify. |

- Excluded from every arm: curve search, `q = k*G`, and `beta_of` (one scalar multiplication and one inversion, microseconds).
- Scopes are equivalent.
- The sym arm runs first in each row, cold, which if anything penalises sym.

**Instances are not paired between sym3 and sym4.**
- The seed includes B, so k differs between the s3 and s4 files.
- The sym3/sym4 ratio therefore compares two independent instance sets. I resampled them independently in the bootstrap.
- plain3/sym4 is row-paired.

**Attribution of the gain.** plain3/sym4 is the product of the earlier plain3/sym3 gain and the new sym3/sym4 gain.
- plain3/sym3 is 3.5/4.3/4.2 in my replay.
- Only sym3/sym4 (2.1/2.4/2.9) isolates the 4-summand change, since both run in the same function with the same code style.

## 4. Fresh-curve replay

**Protocol.** Frozen B4=24/28/44, B3=116/184/292, C3=0.7/0.8/0.5, args `<out> <bits> <B> 8 40 <4|3>`, `NOPLAIN2=1`.
- CSKIP=1100 and 1200, two curves each.
- 3 reps, ABBA on odd reps and BAAB on even reps.
- 72 runs, 1152 rows; 32 instances per arm per size, each run 6 times.
- Reps repeat identical instances, so rep spread is timing noise only.
- Bootstrap: 10000 resamples over instances; sym3 and sym4 resampled independently.

| bits | plain3/sym4 (95% CI) | prediction | producer | sym3/sym4 (95% CI) | prediction | producer |
|---|---|---|---|---|---|---|
| 28 | **7.32** [7.08, 7.60] | >=6.0 | 7.53 | **2.12** [2.02, 2.23] | >=1.8 | 2.10 |
| 30 | **10.03** [9.64, 10.44] | >=7.0 | 10.40 | **2.39** [2.26, 2.54] | >=1.8 | 2.30 |
| 32 | **12.01** [11.45, 12.55] | >=8.0 | 12.42 | **2.88** [2.71, 3.06] | >=1.8 | 2.99 |

- Per-rep plain3/sym4 ranges are 7.24-7.39, 9.83-10.26 and 11.95-12.10.
- Per-rep sym3/sym4 ranges are 1.98-2.38, 2.32-2.51 and 2.77-3.12.
- A cluster bootstrap by shared k value (8 clusters per arm) gives the same intervals within about 0.3.
- Estimator choice moves the ratios by less than 5%. Per-instance min across the 6 runs gives plain3/sym4 7.64/10.59/12.58 and sym3/sym4 2.16/2.29/2.82.
- Extra runs, beyond the dispatch:
  - Exact producer commands at `CSKIP=1000`, ABBA, 2 runs per arm: plain3/sym4 7.38/9.44/11.92, sym3/sym4 1.95/2.11/2.83.
  - `CSKIP=2500` and `4000`, one ABBA rep each, on groups with different p:

| Size | cs=2500 plain3/sym4 | cs=4000 plain3/sym4 | cs=2500 sym3/sym4 | cs=4000 sym3/sym4 |
|---|---|---|---|---|
| 28 | 6.57 | 7.83 | 2.11 | 2.08 |
| 30 | 9.28 | 10.96 | 2.56 | 2.77 |
| 32 | 12.41 | 12.11 | 3.11 | 3.21 |

- Pooled over all my groups, stratified bootstrap: plain3/sym4 7.25/9.88/12.08 and sym3/sym4 2.08/2.43/2.97.
- Pooled over groups whose p differs from the producer's holdout: plain3/sym4 7.22/10.00/12.26 and sym3/sym4 2.11/2.65/3.16.
- Smallest margins seen:
  - Per-curve sym3/sym4 was 1.79 on `j0-28-b1228`, just under the 1.8 prediction; the pooled value is 2.12.
  - Smallest single-group plain3/sym4 was 6.57 (28 bits).
  - Smallest row-level plain3/sym4 was 5.18, 6.26 and 7.03 by size.
- **Abandon rules:** none triggered. Every plain3/sym4 is above 4.0 and every sym3/sym4 above 1.3. There were 0 failed verifications.

**Baseline-tuning sensitivity** (one pass, `CSKIP=1100`, host noise about 10%):
- The best `ic_solve` c on this sweep was 0.6/0.7/0.6, against frozen 0.7/0.8/0.5. Frozen c was 1.07x/1.03x/1.03x the best.
- With the best c, plain3/sym4 would be 6.9/9.8/12.2, still above 6/7/8.
- Frozen B3 was within noise of the best at 28 and 30 bits. At 32 bits B3=365 beat 292 by 12%, which would lower sym3/sym4 at 32 bits to about 2.6, still above 1.8.
- The frozen baselines are therefore not materially mistuned.

**Freshness qualifier.**
- All curves within a CSKIP pair share one (p, n). The producer's "two curves, 16 rows per arm" is therefore one group with 8 distinct k values, 2 curve names each.
- The same holds for each CSKIP pair I ran. The frozen file's "one twist group per size" already admits this.
- At 30 and 32 bits my CSKIP=1100/1200 curves have the same (p, n) as the producer's holdout. They are fresh b values and generators, not a fresh group. At 28 bits the p differs.
- Only my `CSKIP=2500` and `4000` runs add genuinely new groups.
- I did not test isomorphism over F_p; the matching (p, n) is consistent with sextic twists.

## 5. Certificates and negative controls

- `tools/verify_pf_cert.py` was run on all 1728 replay rows (2304 + 384 + 768 = 3456 recovered scalars, covering sym and plain3) and on the producer's 96 rows (192 scalars). Failures: 0.
- My wrapper `scripts/strict_cert.py` also requires non-null sym and plain3 scalars equal to k, `recovered_scalars_checked` equal to 2 times the rows, and n in the Hasse interval with n > 4 sqrt(p).
- Negative controls on a copy of a producer file, one row mutated:
  - sym_rec+1, plain3_rec+1, q.x+1, k changed, g off-curve, n composite, and curve b changed: the checker exits 1 for all seven.
  - A recovered scalar corrupted in a copy of a replay file: the checker exits 1.
- **Finding:** a row with `sym_rec = null`, meaning a failed solve, makes `verify_pf_cert.py` exit 0 with `failing_rows=0`. Only the lower `recovered_scalars_checked` count (31 against 32) shows it. The checker therefore passes failed solves silently, and the wrapper catches that case.
- The checker takes b from the curve name and p, n, g, q, k from the row. This is self-consistency of the reported row, not binding to a solver input.

## 6. Distance to the negation-only rho in the rows

| bits | sym4/rho (producer) | sym4/rho (mine, mean ms ratio) | sym3/rho (mine) | plain3/rho (mine) |
|---|---|---|---|---|
| 28 | 1.96 | 2.14 | 4.4 | 15.5 |
| 30 | 3.46 | 3.57 | 8.7 | 35.2 |
| 32 | 5.54 | 5.76 | 17.1 | 69.6 |

- rho is still faster at every size, and the gap grows with bits.
- The "about 2 ms" fixed rho overhead was not independently measured, and the following is my inference. Subtracting a fixed overhead from rho would increase these ratios, so they understate the distance at 28-30 bits. They are indicative only.
- The rho used is negation-only. j=0 admits the 6-fold automorphism fold, and the repo has a fold-rho (`results/pf_j0/foldrho_hold_28.jsonl`). At 28 bits it measures 3.1-3.4 ms, equal to negation rho, so the fold is overhead-dominated at that size.
- The producer's 36-bit scaling rows (`sym4_scale/`, 6 rows per setting, B not finely tuned, not a holdout) give about 14x behind rho. I did not replay them and they are not part of Claim A.

## 7. Discrepancies and what is missing for the gate

**Discrepancies**
1. The session log says the 3-summand solver "was 4.7 / 8.6 / 18.1" times rho. The same-holdout s3 rows give 4.26/7.55/14.98 against their own rho. The 4.7/8.6/18.1 figures match the older CSKIP=600 prefilter holdout (`results/pf_j0/prefilter/h*.jsonl`: 4.68/8.51/20.02). The improvement quoted against rho is therefore not like-for-like.
2. "Two fresh curves" per CSKIP are one group with 8 k values (section 4). My bootstrap CIs treat the instances as independent. The cluster-by-k intervals are similar, but variation across groups is sampled by only a few groups.
3. The producer's rows carry no commit sha, binary hash, host, or timestamps. I bound them to this code by the exact deterministic replay above. The producer ran each arm once, in non-interleaved order, and my host ran about 10-20% slower than their timings on sym4 at 30-32 bits. Their ratios sit at the upper end of my reproduction, within 2-10%.

**Missing for `docs/VALIDATION_GATE.md`**
- **Replay certificate.** No sha-pinned, independent, committed certificate file for the sym4 rows. `results/pf_j0/cert_replay/` covers the earlier 3-summand claims. The `cert` fields and `tools/verify_pf_cert.py` exist, and the checker's null-scalar hole needs fixing.
- **Validator report.** This text is not a `validator-report.json` with `raw_data` and `headline` entries.
- **Matched rho reference and §5a band.**
  - No rho at 0, 0.1 and 1 GAE for either arm.
  - No `order_holds_across_band`, and `kind` should be `automorphism_fold` rather than negation.
  - No counted-operation or S unit.
  - No plain3 phase split in the rows.
  - Not measured through `ecbench`.
- **Measurement admission.**
  - No `bench.record/v1`.
  - No S3 raw data and no `.artifacts.json`.
  - No isolab-eks, L2 or A/A admission.
- **Evidence directory.** No `VALIDATION.json` in the evidence directory and no pcc claim id.
- **Leaderboard and benchmarks.** No row for sym4 exists in the IC leaderboard or `benchmarks/README.md`. That is correct until the above lands.
