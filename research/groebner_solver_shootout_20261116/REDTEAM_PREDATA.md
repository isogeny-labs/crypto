RED-TEAM REVIEW: GB-SOLVER-SHOOTOUT-20261116 (pre-data design review). I do not declare the protocol supported or refuted; the Coordinator and Validator decide.

EVIDENCE STATUS: not durable
- Every artifact exists only in the working tree. `freeze.commit` 66d8635ef2 contains neither the study nor the harness. The S3 checkpoint failed. No pcc claim exists for the freeze.
- What I checked myself:
  - The canonical hash recomputes to 116b33f7…bfee.
  - The vendored checker reports 0 findings.
  - The sha256 of analyze.py and of the 5 harness files match their pins today.
- The freeze cannot be audited until protocol.json, PROTOCOL.md, grid.tsv, analyze.py, preflight/ and groebner_compare/* are committed and pushed and that commit is recorded as an amendment.
- Classes used below: WRONG, UNSUPPORTED, UNPRICED.

## CRITICAL (fix before launch; each one fixes or voids an outcome)

**C1. WRONG / UNSUPPORTED. Spurious algebraic roots, combined with CMS decision mode and the literal invalidation and completeness rules.**
- Evidence:
  - /home/user/crypto/groebner_compare/semaev_verifier.py docstring: spurious roots exist (an intermediate U that is not rational; an e-triple that does not split over the FB).
  - The producer's build note [pcc:unverified]: on the e-only 9-variable instance, 17 algebraic roots and 1 re-addition.
  - protocol.json stopping_rules: "a returned decomposition failing re-addition, a complete arm whose root set differs from the oracle's … invalidates".
- Consequences:
  - On the 4 symmetrised-e cells, cms-xor (`--maxsol 1`) returns whichever model it finds first, likely a non-relation. That fails the 5/5 planted control, or is read as a failed re-addition, so CMS is invalidated or loses recall because of the encoding, not the solver.
  - msolve's root set (exhaustive basis_roots) contains the spurious roots, so the literal rule "root set differs from oracle" also invalidates msolve.
  - In every cell, Boolean roots (ordered X tuples, the U intermediates, x=0 → 2-torsion (0,1), which readdition accepts while plant() excludes it) are never projected onto the oracle's abscissa multisets with signs.
- Fix:
  - Amendment: roots are mapped to verified relations by readdition before any rule applies.
  - Completeness is judged against an exhaustive Boolean root set; e-only systems are quadratic, so fes-f2 can enumerate them. Relation sets are compared with the fb-enum oracle after projection, with x=0 handling stated.
  - CMS runs in enumerate-until-a-verified-relation-or-UNSAT mode, with all calls charged.
- Cheapest test: on the preflight m3-symmetrised-e-n11 instance (21 variables), enumerate all 2^21 roots, count how many re-add, and run CMS `--maxsol 1` under 20 values of `--random`, counting the re-adding returns. This takes minutes.

**C2. WRONG. The matched rho cannot be built on 7 of 15 cells, and is trivial on the rest.**
- Evidence: grid.tsv gives (a=0, n=11) #E = 2116 = 4·23² and (a=0, n=17) #E = 130972 = 4·137·239, so r = 239 < h = 548. /home/user/crypto/src/cryptanalysis/koblitz_index_calculus.rs lines 1066 and 1230-1236 show `KoblitzCurve::new` returns None when r² | #E or r ≤ h. So P5 (`koblitz_signed_frobenius_rho_reference`) cannot be met for the 4 n=11 cells or the 3 n=17 cells.
- r = 23 and r = 37 give E_steps ≈ 1.3 to about 6 steps. Every C-vs-rho verdict is therefore "above" before any data exists.
- Protocol says "7 distinct curves"; there are 5.
- Fix (pre-data amendment): use (a=1, n=11), #E = 2·991; (a=1, n=17), #E = 2·65587; (a=0, n=9), #E = 4·127; keep (a=0, n=13). Then recheck fb ≥ m+2 and the rate.
- Cheapest test: a 2-line cargo test that calls `KoblitzCurve::new(0,11)` and `KoblitzCurve::new(0,17)`.

**C3. WRONG. The §5a band is applied only to rho.**
- Evidence: in /home/user/crypto/research/groebner_solver_shootout_20261116/analyze.py (`arm_cell_metrics`), `gae_per_rel = cpu/t_ga` is used unchanged at every w, while `c_rho = expected_steps*(1+w*u)`.
- The IC side is a clock-calibrated cost and rho is band-priced. §5a requires the same pricing for every arm and forbids "calibrated on one side, uncalibrated on the other" (lesson L-1abba43f8286). The solver arms emit no native operation counts, so §5a says they cannot be ranked.
- The code fails open:
  - `rho.get("unpriced_emitted", True)`
  - a missing `unpriced_per_step` gives u = 0, which collapses the band.
- "Below matched rho" can be issued from an IC lower bound. Linear algebra, generating R = aP + bQ, per-target Weil descent and system generation, the ANF→CNF and .ms encoding, and verification are all uncharged.
- Fix:
  - Report the same-host clock ratio only, labelled "diagnostic, IC lower bound".
  - Alternatively emit native counters (msolve: per-step rows×cols×density; CMS: propagations and conflicts) and price them at 0, 0.1, 1, w_cal and 10·w_cal.
  - Remove "below" from the verdict vocabulary unless every phase is priced.
  - Make missing fields fail closed.

## HIGH

**H1. UNSUPPORTED. The comparison uses a weaker baseline; the incumbent and the fastest tools are missing.**
- The repo's own binary decomposition engines are absent: `ic descent --solver fes-f2|f4-f2|inherited-f4|m4gb-f2|exhaustive`. They run on the direct S_{m+1} encoding with m·l x-only unknowns. /home/user/crypto/research/m4gb_descent_ladder_20261110/RESULTS.md shows inherited-f4 at a 28-225 ms median on m=3 n=11/13/15 [pcc:unverified].
- fb-enum is "not ranked", although at |FB| ≤ 32 it costs about 10^4-10^5 additions and may beat both arms.
- repo-boolean-f4 is the Python prototype that hit the watchdog on 11-24 variables, and it consumes about 13.5k of the 54k CPU-s.
- Under §15:
  - msolve is pinned to the Debian 0.6.5 generic build, while a source-built msolve 0.11.0 with AVX2/AVX-512 is on this host (scratchpad scout/src/msolve). The packaged libneogb has 194 ymm/zmm instructions against 855 in the source build.
  - The CMS Debian build has ENABLE_ASSERTIONS=ON.
  - M4GB and M5GB are marked UNAVAILABLE only because an environment variable is unset; the repo has build paths for both (build_m5gb.py, research/m4gb_integration_20261110).
- Fix:
  - Add inherited-f4, m4gb-f2 and fes-f2/exhaustive on the same targets, plus the x-only encoding family.
  - Rank fb-enum as a row.
  - Add msolve 0.11.0 with native SIMD as a second configuration.
  - Cut repo-boolean-f4 to one probe.
- Cheapest test: on 5 targets of m3 n13 l5, time `ic descent` (inherited-f4, exhaustive) against the preflight msolve and CMS timings.

**H2. UNPRICED / transferred. The two arms do different tasks.**
- msolve plus basis_roots enumerates every root. The Python extraction is charged to the msolve arm, and the build note reports re-addition at about 16 s against msolve's 3.07 s. CMS stops at its first model, and the metric counts at most 1 relation per target.
- Extraction limits (4096 roots, 200k nodes) raise TimeoutError, which is recorded as status unknown, not censored. That is a third outcome class the protocol does not have.
- Fix: preregister both tasks, decision and enumeration. Count every distinct verified relation per target for both arms. Report extract_cpu_s separately. Classify extraction overflow as censored.

**H3. UNSUPPORTED. The caps are not implemented as written.**
- /home/user/crypto/groebner_compare/runner.py (`Worker.request`) has only a wall-clock watchdog over the whole Python request. That window includes encoding, msolve output parsing and log printing, so msolve gets less solver budget than CMS.
- When the watchdog kills the process group, rusage, the msolve log and the degree reached are all lost.
- There is no 8 GiB cap anywhere. An msolve run past the 10 GiB pod limit gets OOM-killed, is classified as an infrastructure failure, retried, and its rows excluded as "preempted". That removes the hardest targets (survivorship bias).
- Fix:
  - Set RLIMIT_CPU = 300 on the solver child; record SIGXCPU as censored and keep the wait4 rusage.
  - Use a per-call cgroup with memory.max = 8G and memory.peak; classify OOM as censored.
  - Parse the msolve log on every kill.

**H4. WRONG (rule design). The censoring stop rule hides the largest effects.**
- When 3 planted calls are censored, the natural targets are never run, `pair_stats` finds no common keys, and the verdict is "unresolved (no paired targets)". That happens exactly where msolve was censored in preflight (7 of 15 cells).
- The bootstrap median keeps the both-censored zeros that the Wilcoxon test drops.
- Fix:
  - Count a stopped arm-cell's natural targets as censored at the cap (a lower bound).
  - Allow a directional verdict "A < cap; B censored".
  - Drop both-censored pairs from the bootstrap.

## MEDIUM

**M1. False-negative UNSAT is treated asymmetrically.**
- A CMS UNSAT on an oracle-positive target is status unknown and costs only recall. An msolve basis of {1} on the same target invalidates. An over-constrained encoding would make CMS fast and wrong without triggering anything.
- Fix: any no-relation answer on a target whose oracle decomposition is a root of the encoded system invalidates the arm-cell.
- Add an encoding-equivalence control per cell: compare full CMS enumeration with msolve roots and with exhaustive search.

**M2. The Williams interleaving is not implemented.**
- `runner.run` is backend-major, and no prerequisite P1-P7 covers interleaving. analyze.py never uses `order_position`.
- Fix: add an interleaving prerequisite with a launcher order derived from the hash, and an order-effect check.

**M3. analyze.py leaves forks open.**
- A/A pairs are supplied with `--aa` at unblinding; if omitted, the floor is 0.
- Verdicts are emitted for unregistered pairs, such as aa-vs-other.
- TESTS_IN_FAMILY = 315 assumes 7 labels; 8 labels give 420 tests, which is anti-conservative if every arm is available.
- The CI for rel_per_cpu_s is declared but not implemented.
- The invalidation rules (planted 5/5, oracle-empty, tamper) are not implemented. They are delegated to P3's unfrozen `status` field.
- Fix: hard-code the A/A pairs and the registered comparisons, and implement invalidation and the CI. This changes analyze.py, so record it as an amendment with a new sha.

**M4. Harness defaults contradict the protocol.**
- /home/user/crypto/groebner_compare/external_worker.py `main()` defaults `max_solutions` to 64, and `default_backends` passes no `--max-solutions`.
- There is no `--random` passthrough.
- Extract and verify time are lumped into one wall-clock `basis_verification` phase.
- The seed rule keys on "protocol_sha256", not the literal v1 hash, so an amendment could reseed.
- Fix: pin the literal hash 116b33f7… and make P3 tests assert each default.

**M5. CMS Gauss-Jordan elimination may be silently off on larger cells.**
- The defaults are `--maxmatrixcols 1000` and `--autodisablegauss 1`. Preflight CNF sizes reach 989 variables (m3-plain-n17) and 1371 (m4-plain-n13), so the arm configuration may vary by cell.
- Fix: log Gauss activation for every call and preregister a raised-limit configuration.

**M6. Startup, parsing and printing are timed as the algorithm.**
- §15 says to use the tool's internal timer. CMS's preflight times of 0.00-0.08 s on 11 of 15 cells are dominated by startup.
- The encoding conversion and per-target generation are uncharged.
- Fix: record msolve's `-v 2` total and CMS's "Total time", plus an empty-input startup baseline per tool and host, and report both readings.

**M7. Two requested arms (trace-f4, Magma) are UNAVAILABLE with no problems/ record ("Name the obstruction").**
- If trace becomes available, the runner learns on instance 0, which is the discarded warm-up, so the learn cost would be lost.
- P8's same-support template changes one arm's encoding, which the inadmissible list forbids.
- Amortising over 40 targets is arbitrary.
- Fix:
  - Write problems/<slug>.md with unblock criteria.
  - Learn on a named target and charge it in its own row.
  - Run every arm on the template encoding.
  - Amortise over (fb + 10) / P(SAT).

**M8. There is no §4 falsification or abandon target, and no path to the success criterion.**
- The in-repo IC baseline is excluded, so nothing in this study can register an improvement.
- The preflight already shows the direction: CMS ≤ 12.5 s, while msolve was killed on 7 cells.
- Fix: declare H1 and its numeric abandon condition before launch.

**M9. WRONG. C-dreg uses the wrong yardstick.**
- /home/user/crypto/src/cryptanalysis/dreg_forecast.rs:164-167: `semi_regular_degree_field` is "no field equations", but these are Boolean systems with field equations.
- Fix: use the boolean series (1+t)^v / ∏(1+t^{d_i}).

## LOW

- **L1.** Only the two executables are pinned. libneogb-0.6.5.so, libflint.so.18 and libcryptominisat5 are not; pin the ldd closure.
- **L2.** protocol.json `measurement.isolation_required: "L2"` contradicts the planned command Jobs, which have no L2.
- **L3.** The rho prerequisite is P5 in PROTOCOL.md but P6 in the rho-matched arm. PROTOCOL.md says grid.tsv carries the decomposition-rate column; it does not.
- **L4.** No ICV1 slugs are used (§11). n = 15 is composite, and its subfield structure is not disclosed (§8b).
- **L5.** VmHWM sampled every 2 ms misses short runs and late peaks; prefer cgroup memory.peak.
- **L6.** No SAT/UNSAT-stratified readout is preregistered. P(no decomposition) ranges from about 7% to about 83% across cells.
- **L7.** Blinding is cosmetic: CMS rows carry null degree and matrix.

## CHEAPEST FALSIFICATION OF THE DESIGN
Three local checks, under 10 minutes in total. They are development checks and use no main-run seeds:
1. Run `KoblitzCurve::new(0,11)` and `KoblitzCurve::new(0,17)`. If they return None, C2 is settled.
2. On preflight m3-symmetrised-e-n11, compare the number of exhaustive roots with the number that re-add, and record how often CMS `--maxsol 1` returns a re-adding model across 20 seeds. This settles C1.
3. Time `ic descent` (inherited-f4, exhaustive) against msolve and CMS on 5 targets of m3 n13 l5. This settles H1.

## LEDGER
- Lesson tests recorded: T-a168a3e79ee6 (confirms L-1abba43f8286) and T-66368bcf4091 (confirms L-5ee9408bdbb8, now adopted).
- New candidate lessons: L-1b40a6c821f7 (spurious roots with decision-mode arms) and L-3ad339a84447 (check the rho constructor's preconditions for every grid cell).
- Outcome recorded: O-d43d5ee12000.
- All of these are untracked files under /home/user/crypto/docs/lessons/entries/.

Files reviewed:
- /home/user/crypto/research/groebner_solver_shootout_20261116/protocol.json
- /home/user/crypto/research/groebner_solver_shootout_20261116/PROTOCOL.md
- /home/user/crypto/research/groebner_solver_shootout_20261116/analyze.py
- /home/user/crypto/research/groebner_solver_shootout_20261116/grid.tsv
- /home/user/crypto/research/groebner_solver_shootout_20261116/preflight/timings.tsv
- /home/user/crypto/groebner_compare/external_worker.py
- /home/user/crypto/groebner_compare/runner.py
- /home/user/crypto/groebner_compare/semaev.py
- /home/user/crypto/groebner_compare/semaev_verifier.py
- /home/user/crypto/groebner_compare/certificate.py