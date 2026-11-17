# Amendment 1: GB-SOLVER-SHOOTOUT-20261116 (local diagnostic)

| | |
|---|---|
| Amends | `protocol.json` v1. This file is frozen and not edited: canonical sha256 `116b33f754becbd9d7e76cbd80bfff8e323cb79ab0ee0c57481c0616d3d9bfee`, file sha256 `9ab64347…ee9341a` |
| Amendment record | `protocol.amend1.json`, `experiment.protocol/v1`, version 2, `amendments[0].id = amendment-1` |
| Frozen | 2026-11-17T06:10:55Z with the vendored checker `research/primefield-companion-followup-20261102/tools/experiment_protocol.py freeze --commit auto --filed-before-affected-data`; `validate` reports 0 findings |
| Canonical sha256 | **`c521e04c643ff8b3028426b1d9c8b85a3f92fabc5e6e91e124498c13524fe858`** |
| File sha256 | `1d9b44faed4b22accd755d038f7b2c88b824e0a939aef316a296af4c01caf812` |
| `freeze.commit` | `f8480787ec` (HEAD at freeze). This commit does not contain the amendment; the main session commits it |
| Filed before affected data | yes. No main-run data exists for v1 or for this amendment |
| Superseded draft | a first freeze, canonical `e57df666…72ea8`, was replaced before any commit or data. That draft ran inherited-f4 on the x-only descent; the replacement runs it on the same Boolean system, through the adapter the main session is building (`examples/gc_native_solve.rs`) |

Every number below comes from a command run in this container on 2026-11-17, using
development seeds only (`dev-sizing`, seed 90001), never main-run seeds. All of it is
**[pcc:unverified]**: no pcc claim was filed.

## Owner decisions applied (2026-11-17)

- AWS does not work, and Magma is not available.
- Consequences:
  - No isolab-eks, EKS, S3 or USD budget.
  - The run becomes a **local diagnostic** in this container: 4 vCPU on a shared VM, 15 GiB of memory, no PMU, isolation `L0`.
  - It is explicitly non-admitted. It never becomes an IC leaderboard row or a `benchmarks/README.md` row.
- Magma stays UNAVAILABLE.
- trace-f4 and julia-f4-warm stay UNAVAILABLE: Julia is absent and its download host is proxy-blocked.
- If a repository hook refuses an action, the refusal is recorded verbatim and is not bypassed.

## Cell (C2): curve facts computed with the repository's own code

`groebner_compare.semaev.setup(n,a,1)` followed by `Curve.order()` (exhaustive trace count), then trial division:

| (a, n) | #E | r | h | `KoblitzCurve::new` preconditions (r > h, r² ∤ #E) |
|---|---|---|---|---|
| (0, 11) (v1) | 2116 = 4·23² | 23 | 92 | fail |
| (0, 17) (v1) | 130972 = 4·137·239 | 239 | 548 | fail |
| (1, 15) (v1) | 32494 = 2·7·11·211 | 211 | 154 | hold (r tiny) |
| (0, 13) (v1) | 8012 = 4·2003 | 2003 | 4 | hold (r < 2^12) |
| (1, 11) | 1982 = 2·991 | 991 | 2 | hold (r < 2^12) |
| (0, 9) | 508 = 4·127 | 127 | 4 | hold (r < 2^12) |
| **(1, 17)** | **131174 = 2·65587** | **65587 (≈2^16)** | **2** | **hold** |
| (0, 19) | 523492 = 4·130873 | 130873 | 4 | hold |

**Chosen cell `K17-m3-plain-l5-a1`.** The curve is K1: y² + xy = x³ + x² + 1 over GF(2^17), modulus z^17 + z^3 + 1 (`semaev.irreducible(17)` = 131081).

- Encoding: m = 3, plain chained-S3 encoding, l = 5, 32 Boolean variables.
- Factor base: 14 abscissae (the nonzero x in ⟨1, z, …, z^4⟩ that lift).
- Exhaustive fb-enum oracle: every 3-multiset with repetition and every sign pattern. Of the 65572 liftable abscissae outside V, **1757 have at least one decomposition**, so p_sat = 0.026795. The mean number of decompositions, given at least one, is 1.04.
- Matched rho:
  - expected steps `rho_expected_steps(65587, 17)` = √(πr/2)/√(2n) = **55.05**;
  - plus 1000 measured repository signed-Frobenius rho DLPs on `KoblitzCurve::new(1,17)`, run locally.

Other candidates and why they were rejected:

- Dropped:
  - (0, 19): oracle rate 0.0067 at l = 5, and its systems are larger.
  - l = 6 at n = 17: CMS enumeration took more than 119 s.
- Other rates checked: (17, 6, a1) has rate 0.263, and (13, 5, a0) has rate 0.499 but r = 2003.
- m = 4 at n = 17: rate 0.0038 at l = 4, and 0.0002 at l = 3.

## Finding-by-finding

- **C1 (spurious roots and decision mode).**
  - Every arm's Boolean roots are projected to abscissa multisets by group re-addition (memoised per target and multiset) before any rule applies.
  - Completeness: an arm is complete when its verified relation set equals the oracle's set for that target.
  - Spurious roots are counted and never invalidate a run.
  - x = 0 is not in the factor base. A relation that uses it is projected out of both sides and counted separately.
  - CMS runs in full-enumeration mode (`--maxsol 1000000`, until UNSAT), and every call is charged.
  - The dev evidence supports this rule:
    - On the two oracle-SAT plain targets (1 multiset each), CMS enumeration returned 3 and 6 Boolean models.
    - On symmetrised-e targets, CMS enumeration was still running after 90 s with 161 and 266 models, while the oracle has 1 relation.
- **C2.** One cell with r = 65587. The facts and the rejected candidates are in the table above.
- **C3.** The §5a band (0, 0.1 and 1 GAE per unpriced native op) applies to every arm. Each arm's native unit:
  - msolve: Σ rows×cols over F4 rounds (the `-v 2` "mat" column);
  - CMS: propagations, with conflicts recorded beside them;
  - inherited-f4: word XORs (`SolverCost.ops`);
  - fb-enum: group additions;
  - rho: walk additions, priced at 1, plus unpriced partition hashes and canonicalisations.

  Consequences:
  - Solver arms do no priced group additions, so C_arm(0) = 0. The band therefore cannot rank any solver against rho, and the report will say so.
  - The clock ratio is reported as "diagnostic, IC lower bound".
  - "Below matched rho" is removed from the verdict vocabulary.
  - Missing fields fail closed.
- **H1.**
  - Added: the incumbent inherited-f4 (`ic_framework::solvers::HybridF4`, kind InheritedF4, engine defaults) as the baseline arm, on the **same** Boolean system.
  - fb-enum is reported as a descriptive row.
  - Dropped: repo-boolean-f4.
  - Not provisioned here: msolve 0.11 SIMD.
- **H2.** Both solver arms enumerate. Every distinct verified relation counts. Extraction CPU is reported separately. Extraction overflow is classed as censored.
- **H3.**
  - Per call: RLIMIT_CPU 60 s on the solver child, and 4 GiB of memory (a per-call cgroup `memory.max` if writable, else RLIMIT_AS).
  - SIGXCPU and OOM are classed as censored. The rusage is kept and the logs are parsed on every kill.
- **H4.**
  - No arm stops early because of censoring; censored calls are charged at the cap.
  - A directional verdict "A < cap; B censored" is allowed.
  - Both-censored pairs are dropped from both the test and the bootstrap.
- **M1.** A "no relation" answer on an oracle-SAT target whose oracle multiset is a root of the encoding invalidates the arm. This applies to every arm alike.
- **M2 (Williams interleaving).**
  - The three solver arms use all 6 sequences, balanced over the 160 targets.
  - Each target runs pass A, then pass B with the order reversed, on the same worker. Pass B forms the A/A arms.
  - The order-position effect is reported.
- **M3.**
  - The A/A pairs and the registered comparisons (3) are hard-coded.
  - The CI is implemented.
  - analyze.py v1 is **not** used. A new analysis script must be pinned by amendment-2 (PA-5).
- **M4.**
  - Seeds are keyed on the literal v1 hash plus the domain `:amend1:K17-m3-plain-l5-a1:`.
  - Tests in PA-4 assert the harness defaults: `max_solutions`, the `--random` passthrough, and separate extract and verify timers.
- **M5.** CMS keeps its default Gauss options (`--maxmatrixrows 2000 --maxmatrixcols 1000 --autodisablegauss 1`). The `[matrix] Good matrix` and `Using N matrices` lines are logged per call. The dev check on one plain l5 target shows 4 Gauss matrices active, the largest 17×107.
- **M6 (startup).** Both readings are reported: wait4 CPU of the solver child, and the tool-internal timer (msolve `overall(cpu)`, CMS `Total time`). Per-tool empty-input startup baselines are taken at the start, middle and end of the run. The dev empty-input startup for both tools was below `/usr/bin/time` resolution (0.00 s).
- **M7.** The UNAVAILABLE arms are named with their obstruction in `protocol.amend1.json` (C-out). A `problems/` record is still owed; see open items.
- **M8 (falsification and abandon target).**
  - The preregistered expectation from dev sizing, msolve about 4.5 s per call against CMS enumeration about 23 s, is that msolve uses less CPU per target than CMS.
  - The amendment is falsified if CMS ≤ msolve outside the A/A floor.
  - **Abandon:** suppose the best available arm's projected relation-phase clock ratio to the measured matched rho exceeds 2^20. Then external Boolean solvers on m = 3 Semaev systems at n ≤ 19 are recorded as no route to the IC success criterion. No further solver shootout at these sizes is run without a new mechanism.
  - This study can register a component-level improvement only if a solver beats the incumbent inherited-f4 outside the A/A floor. That improvement is diagnostic tier.
- **M9.** C-dreg uses the Boolean semi-regular series (1+t)^v / ∏(1+t^{d_i}).
- **L-items.**
  - The ldd closure hashes are recorded (L1).
  - Isolation is stated as L0 local (L2).
  - L4 and L5 are covered by the memory-source rule and the cell facts.
  - The SAT/UNSAT stratified readout is the design itself (L6).
  - The blinding is declared partial (L7).

## Design summary

- **Targets:** 80 SAT-stratum and 80 UNSAT-stratum natural targets, drawn from one natural stream and labelled by the exact oracle. There are also 5 planted targets (the positive control), 1 warm-up, and 2 tamper checks per arm. The UNSAT stratum doubles as the negative control.
- **Arms:**
  - msolve-f4: `-g 2 -v 2 -t 1` plus in-house root extraction;
  - cms-xor: full enumeration;
  - inherited-f4: through the `gc_native_solve` adapter;
  - A/A: each of the three again in pass B;
  - references: fb-enum oracle and the matched rho.
- **Primary endpoint:** stratified verified relations per CPU-second per arm:
  p_sat·E_SAT[relations] / (p_sat·E_SAT[CPU] + (1−p_sat)·E_UNSAT[CPU]).
  It comes with stratified paired bootstrap CIs from 20000 resamples.
- **Secondary endpoints:** degree reached, largest matrix, peak memory, native ops, spurious roots, projection CPU, and startup baselines.
- **Decision:**
  - Test: Wilcoxon signed-rank on per-target paired log2 charged CPU, at Bonferroni 0.05/3, required to lie outside the A/A floor of both arms.
  - Rho: "above at every reading" or "unresolved in band".
  - Every verdict is labelled "diagnostic tier (local, non-admitted)".
- **Budget:**
  - Dev per-call CPU: msolve 4.10–4.77 s; CMS enumeration 19.4–24.5 s.
  - Re-addition in the v1 runner took 10.85 s for 6 roots, so projection is memoised per multiset.
  - About 66 CPU-s per target over both passes, × 160 targets, plus planted and fixed work, gives about 11.5k CPU-s.
  - On 3 workers that is about 64 min; with contention, about 77 min.
  - Cost guard: if the planted phase projects more than 100 min, both strata are cut symmetrically, down to no fewer than 40 each.
  - Hard wall ceiling: 105 min.
  - inherited-f4 timing is unmeasured: the build is blocked (PA-1). The 60 s cap bounds it.

## Prerequisites (must exist and be pinned by amendment-2, hashes only, before any data)

- **PA-1. The build.** At `94479f7e3e` (main), `cargo build --release --bin ic` fails, and `--lib` fails the same way. `crates/hashed-ecdl-tensor/src/lib.rs` does not declare `mod metrics;`, but `src/metrics.rs` exists and `curve.rs` and `partial.rs` call `crate::metrics::…`. The compiler reports 8 × E0433 "could not find `metrics` in the crate root". This blocks inherited-f4 and rho-matched. Until it is fixed, both arms are UNAVAILABLE, and C-vs-rho is not reported.
- **PA-2. A rho and t_GA adapter.**
  - It runs 1000 DLPs on `KoblitzCurve::new(1,17)` using `koblitz_signed_frobenius_rho_with_progress`.
  - It reports walk additions, the unpriced counters and CPU per DLP.
  - It measures t_GA as the best of 7 batches of 10^6 FastCurve additions.
  - It checks the curve modulus against z^17 + z^3 + 1.
- **PA-3. inherited-f4 adapter:** `examples/gc_native_solve.rs` plus `groebner_compare/native_worker.py`. These are untracked work of the main session and not reviewed here. A node-budget hit is classed as censored.
- **PA-4. Local driver** `run_amend1.py`. It must implement:
  - the seed rule;
  - the oracle table, with an assertion that p_sat = 1757/65572;
  - stratified sampling;
  - the Williams A/B passes;
  - the RLIMIT and cgroup caps;
  - projection and tamper checks;
  - `rows.jsonl`;
  - startup baselines;
  - the ldd and binary hash checks;
  - tests that assert the harness defaults (M4).
- **PA-5. Analysis script**, implementing the decision rule exactly, with its sha pinned.
- **Open items for the main session:**
  - `problems/<slug>.md` for the UNAVAILABLE arms (M7).
  - The resume checkpoint and benchrec steps: S3 is unavailable under the owner decision. If a hook refuses, record the refusal verbatim.
