RED-TEAM REVIEW: GB-SOLVER-SHOOTOUT-20261116, amendment-1 local diagnostic run (post-data). I am not declaring anything supported or refuted; the Coordinator and Validator decide. Every number below is [pcc:unverified].

## Evidence status: not durable
- Every artifact is untracked in the working tree: AMENDMENT1.md, protocol.amend1.json, run_local.py, run-local/, analysis-local/, gc_native_solve.rs, native_worker.py and build_native.sh. `freeze.commit` f8480787ec does not contain the amendment.
- Both binaries exist only in the session scratchpad (`.../scratchpad/native_build/tree/...`).
- I reviewed these files by sha256:
  - rows.jsonl `7c5c98f9…2aedf1`
  - cells.json `87d4b7a8…b71c3f0f`
  - protocol.amend1.json `1d9b44fa…caf812`
  - AMENDMENT1.md `3cbf3d4b…5cb`
  - run_local.py `ed27d2e7…28120ac`
  - analyze_amend1.rs `d62848b4…5f3`
- The Validator must recheck these hashes at the commit that lands them.
- `REPORT.md` does not exist. The report is `analysis-local/report.typ` and `report.pdf`.
- `analyze.py` v1 (`0b0eee39…`) was not used. The verdicts come from `analyze_amend1.rs`, which was written after the data existed.

## Findings by severity

### CRITICAL

**1. Wrong (compliance): the run broke the frozen rule that should have stopped it.**
- protocol.amend1.json, stopping_rules[3], reads: "Run blocked, not substituted: if PA-1..PA-5 are not met, the affected arm is UNAVAILABLE (inherited-f4, rho-matched) or the run does not start (oracle, driver, analysis). Without rho-matched, C-vs-rho is not reported at all."
- Amendment-2 was never filed (grep finds no amendment-2 anywhere). So:
  - the driver (PA-4) and the analysis script (PA-5) were never pinned before data;
  - PA-1 was never fixed; the build was moved to an older base instead (finding 2).
- Under the frozen rule the run should not have started, inherited-f4 and rho should be UNAVAILABLE, and C-vs-rho should not be reported. The report presents all three.
- Its "Deviations" list does not mention this rule.
- Cheapest settle (no compute): the Coordinator compares stopping_rules[3] with the run. Any salvage is a post-data amendment that labels every verdict "post-data, not preregistered".

### HIGH

**2. Unsupported: the incumbent was swapped for an older build, and the swap is hidden.**
- `groebner_compare/build_native.sh` builds inherited-f4 and the rho binary from base `eb01341d3a` (2026-11-12). Only `f4_trace.rs`, `gc_native_solve.rs` and `gc_rho_match.rs` come from HEAD or the working tree.
- `git diff --stat eb01341d3a 94479f7e3e` on koblitz_groebner.rs, solvers.rs and pq_groebner_f2.rs gives +2340/−172 lines. The range includes "perf(f4): integrate f4-parallel, f4-memory and gf2-m4ri speed-up leads".
- The base commit is recorded only in `scratchpad/native_build/build_identity.json`. It is not in cells.json and not in the report.
- So "no solver beat the incumbent" is about a Nov-12 engine.
- Cheapest test: record the base in the report. Once any newer engine builds, run 20 targets with both binaries and compare CPU and root sets.

**3. Wrong as stated: the headline "inherited-f4 is fastest per verified relation" leaves out the preregistered fb-enum row.**
- AMENDMENT1 (H1 and the design summary) says "fb-enum is reported as a descriptive row". The report has no fb-enum row.
- From cells.json `oracle`: build_cpu_s = 0.519 for the whole table (13440 additions; all 65572 targets; 2030 relations). That is 2.6e-4 CPU-s per relation against inherited-f4's 1.272.
- For the 24-relation phase, inherited-f4 needs about 863 targets × 0.0353 s ≈ 30.5 CPU-s. fb-enum needs 0.52 s, about 59× less.
- Cheapest test: none needed; compute the row from cells.json.

**4. Unsupported: "M8 abandon (clock > 2^20) not triggered, narrowly (2^19.54)".**
- (i) The IC side is declared a lower bound: target generation, encoding, projection and linear algebra are uncharged. A lower bound below the threshold cannot clear it.
- (ii) The rule's consequent is about *external* Boolean solvers. analyze_amend1's own `abandon_check_clock_log2_gt_20` flags are true for msolve-f4 (2^26.61) and cms-xor (2^28.84) and false only for the in-house inherited-f4. The report picked the reading that does not trigger and did not discuss the other.
- (iii) The 0.46-bit margin is smaller than the choice of rho scope. From cells.json `per_dlp`:
  - measured CPU: 2^19.54;
  - ns timing (setup + walk + verification): 2^19.65;
  - walk-only ns: 2^20.48;
  - A_rho × t_GA: 2^22.59.
- (iv) The rho CPU timer is quantised: 990 of 1000 per-DLP `cpu_s` values are 0 and 10 are 0.004. The ns cross-check agrees within 7%. The clock CI [19.47, 19.61] resamples only the IC side.
- Cheapest test: none needed; recompute from cells.json and state which reading the rule meant.

**5. Unsupported: the falsification and abandon targets are not preregistered in any verifiable way.**
- The 2^20 threshold and the msolve<CMS expectation exist only in AMENDMENT1.md. protocol.amend1.json just says "M8: falsification/abandon target stated in AMENDMENT1.md". It carries no threshold and no hash of that file.
- AMENDMENT1.md's mtime is 06:11:45, 50 s after the freeze at 06:10:55, and the file is untracked.
- Cheapest settle: none possible after the fact. Record it as a deviation.

**6. Wrong (fact) under §15: msolve 0.11 is "UNAVAILABLE / not provisioned" in the report, but it is built and runs in this container.**
- `/tmp/claude-0/-home-user/a3ce588d-65c6-5d5a-a7d0-cb936deafa77/scratchpad/scout/src/msolve/.libs/msolve -h` prints "version 0.11.0". configure checks -mavx/-mavx2. It was built 2026-11-16 16:04.
- The external_worker docstring shows 0.11.0 was already used in scouting.
- The measured arm is Debian 0.6.5 (libneogb-0.6.5.so).
- It cannot plausibly close a 134× gap, but it changes the msolve gaps and its distance to rho.
- Cheapest test: msolve 0.11.0 `-g 2 -t 1` on 10 run targets, about 1 CPU-min.

### MEDIUM

**7. Unsupported: the report states an order between arms at w = 0.1 and 1 ("inherited-f4 < cms-xor < msolve-f4").**
- The native units are not comparable:
  - msolve counts matrix cells, Σ rows×cols ≈ 2.0e9 per target, not operations;
  - CMS counts propagations parsed from rounded "70M"-style printouts;
  - inherited-f4 counts word XORs, ≈ 4.06e6 per target.
- One GAE weight across these units yields no order. Drop the sentence.

**8. Unpriced: the rho reference.**
- U_rho = 55.10 partition hashes + 56.20 canonicalisations.
- cells.json also emits, per DLP, 955.3 Frobenius maps, 955.3 negations examined, 17 setup group additions and 34 setup scalar multiplications. None of them is in A_rho or U_rho (§5a: list every counter for every arm).
- Including the emitted counts gives C_rho(0.1) ≈ 274 instead of 66.3 and C_rho(1) ≈ 2094 instead of 166.5.
- Every distance to rho is overstated by at least 2.05 bits at w = 0.1 and at least 3.65 bits at w = 1. Inherited-f4 moves from 22.33/24.33 to at most 20.28/20.68.
- The scalar multiplications' own additions are not emitted, so the true overstatement is larger. The "unresolved in band" verdict is unchanged.
- Cheapest test: none needed; recompute from cells.json, and have gc_rho_match emit the additions inside its scalar multiplications.

**9. Unsupported as a solver-quality claim: encoding and algorithm class are confounded.**
- msolve gets field equations as generic char-2 polynomials, with no Boolean ring and no bit-packing.
- CMS gets a naive CNF: one Tseitin gate per monomial (705 auxiliary variables, 34 XOR rows), no `c ind` sampling set, `--random 0` fixed in both passes, and a Debian build with ENABLE_ASSERTIONS=ON.
- inherited-f4 is a Boolean-native hybrid: Macaulay matrices up to degree 3, then splitting (261 splits and 250 infeasible branches on a sample target). It is the Koblitz oracle's default for these very systems.
- Calculation (mine, not run): once the 15 X bits are fixed, both chained S3 equations are affine in the 17 U bits, because u ↦ u² is GF(2)-linear. The cell is therefore a 2^15 guess plus a linear solve, which favours branching solvers.
- The 134× and 629× gaps therefore measure configuration and algorithm class.
- The msolve<CMS ratio (per target 3.76× to 6.55×) could move under a better CNF; that is untested.
- Cheapest test, on 10 UNSAT targets:
  - CMS with `--random 1..3`, with a `c ind 1..32` sampling set, and with `--maxsol 1`;
  - a hybrid msolve that guesses the 5 x1 bits (32 calls per target) on 5 targets.

**10. Unsupported precision.**
- The CIs, for example "134× [131, 138]", resample targets within one run, one host and one build. They exclude run-to-run, host and build variance.
- Cheapest test: rerun 20 targets on another day.

### LOW

- **11.** M6 asks for both readings to be reported. The report omits the internal-timer reading. Inherited-f4's "internal" figure is the adapter's wall-clock ns, not CPU (0.842 vs 0.786 rel/CPU-s), so it is not comparable with msolve's `overall(cpu)` or CMS's "Total time".
- **12.** Startup cannot flip the order. Empty-input medians are 2.0 ms (gc_native_solve) to 3.3 ms (CMS). That is about 6% of inherited-f4's 33.5 ms charge and under 0.1% for the other two arms.
- **13.** Shared-VM noise cannot flip the order.
  - A/A floors are 0.008–0.024 log2. The smallest per-target |log2| gaps are 1.91 (msolve/CMS), 6.52 and 8.64.
  - Load stayed at about 3 runnable tasks. Median CPU differs by ≤2% between high-PSI and low-PSI rows.
  - There is no uncontended control. msolve's 235 MB footprint against inherited-f4's 5.6 MB could bias the ratios by a modest amount.
  - Cheapest test: a 1-worker rerun of 10 targets.
- **14.** The msolve<CMS falsification is weak. It repeats dev sizing on the same cell, host and tools (4.1–4.8 s vs 19–24 s).
- **15.** Blinding was not effective:
  - the pilot (`pilot_local_20261117`, unmasked rows) is not in the amendment;
  - the analyst printed the key before phase 1 (the report admits this);
  - C-dreg was not computed (admitted).
  - Williams sequences are balanced overall (27/27/27/27/26/26) but not within strata.
- **16.** The benchrec record is OPEN and there is no S3 copy, as the owner decided. REFUSALS.md records the refusals verbatim. `benchmarks/20261117-gf2-semaev-solver-shootout/` must not get a README row (no VALIDATION.json).

## (a) Did the run follow amendment-1?

**Followed:**
- the cell and seed domain;
- 80 + 80 natural targets, 5 planted, 1 warm-up, 2 tamper checks per arm;
- Williams A/B passes with reversal;
- RLIMIT_CPU 60/62 s and RLIMIT_AS 4 GiB (cgroup v1 was not writable);
- startup baselines at start, middle and end;
- the ldd closure, and binary pins matching at start and end;
- the p_sat assertion (1757/65572);
- rho with 1000/1000 verified DLPs, and t_GA as best of 7;
- no censoring and no truncation (56:17 wall);
- every correctness control passed.

**Not followed:** findings 1, 2, 3, 5, 11 and 15 above.

## (b) Could a confounder flip the order?

- Shared-VM noise and startup charging: no (12, 13).
- CMS enumeration vs F4 basis: all three arms return the same 456 roots and 0 spurious roots, so the work product is the same.
- What is untested is configuration and encoding (9), the stale incumbent build (2) and msolve 0.11 (6). Only finding 9 could plausibly reverse msolve<CMS.

## (c) Is each stated conclusion supported?

- **Charged CPU per target orders inherited-f4 < msolve-f4 < cms-xor on this cell, host and builds:** the observation reproduces. I recomputed from rows.jsonl: ratios 134.37×, 4.68× and 629.08×; median log2 −7.139, −2.255 and −9.418. All 960 natural rows match the oracle, with 0 M1 violations and 0 spurious roots. Procedurally it is post-data and not preregistered.
- **"inherited-f4 is fastest per verified relation":** wrong (3).
- **The CIs:** wrong-scoped (10).
- **"Band does not hold":** supported. The order stated at w > 0: unsupported (7).
- **Distances to rho:** unpriced (8).
- **"Abandon not triggered":** unsupported (4, 5).
- **"msolve < CMS, not falsified":** supported in scope, but a weak test (14) against an unhashed target (5).
- **"msolve 0.11 UNAVAILABLE":** wrong (6).
- **The deviations list:** incomplete (1, 2, 3, 5).

## (d) Cheapest falsification, in order of cost

1. No compute, from cells.json and summary.unblinded.json:
   - the fb-enum row;
   - the rho band with every emitted counter;
   - the clock ratio under walk-only rho;
   - the per-arm abandon flags.
2. About 1 CPU-min: msolve 0.11.0 on 10 targets.
3. About 10 min: a 1-worker uncontended rerun of 10 targets.
4. About 20 CPU-min: the CMS seed, sampling-set and `--maxsol 1` variants on 10 UNSAT targets.
5. About 8 min to build, then 1 min to run: inherited-f4 from a second engine commit on 20 targets.

The order is falsified if any of these gives CMS ≤ msolve, or inherited-f4 ≥ msolve, outside the A/A floor.

## Ledger
- Tests: T-325337499d3e (L-245e36ed80ab), T-f3c03a3353ec (L-1abba43f8286, now adopted), T-2ff023b403fd (L-5ee9408bdbb8).
- New candidate lessons: L-f06252a2fd1b (an abandon threshold judged on a lower bound) and L-6e958e45ca18 (compute the reference rows and look for an already-built faster binary before accepting a "fastest" headline).
- Outcome: O-c2254158c565.
- All are untracked files under `/home/user/crypto/docs/lessons/entries/`.

Files reviewed:
- /home/user/crypto/research/groebner_solver_shootout_20261116/AMENDMENT1.md
- /home/user/crypto/research/groebner_solver_shootout_20261116/protocol.amend1.json
- /home/user/crypto/research/groebner_solver_shootout_20261116/run_local.py
- /home/user/crypto/research/groebner_solver_shootout_20261116/run-local/{rows.jsonl,cells.json,driver.log,blinding_key.json,manifest.json}
- /home/user/crypto/research/groebner_solver_shootout_20261116/run-local.launch/{REFUSALS.md,host.json,driver_time.stderr}
- /home/user/crypto/research/groebner_solver_shootout_20261116/analysis-local/{analyze_amend1.rs,extract.sh,report.typ,out/summary.unblinded.json,out/registered.tsv,out/params.tsv}
- /home/user/crypto/groebner_compare/{build_native.sh,external_worker.py,semaev.py}
- /home/user/crypto/examples/gc_native_solve.rs
- /home/user/crypto/problems/gf2-fixed-schedule-f4-unavailable.md