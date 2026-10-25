# Quartic-twist mixed polynomial IC: S4/S5 completion and full cost

- **Status:** `investigating`
- **Kind:** native solver failure; implementation and measurement gaps; external-tool availability
- **Affected work:** [mixed polynomial report](../research/twist_index_calculus_20261020/MIXED_POLYNOMIAL_REPORT.md), [larger-curve scaling and occupancy report](../research/twist_index_calculus_20261020/K1_SCALING_REPORT.md), [peeled-orbit reuse report](../research/twist_index_calculus_20261020/K1_PEELED_ORBIT_REUSE.md), [frozen protocol](../research/twist_index_calculus_20261020/QUARTIC_TWIST_LINE_PROTOCOL.md)
- **Next action owner:** quartic twist IC study
- **Last updated:** 2026-10-25

## Problem

The canonical `p=37,b0=2,r=1,873,633` quartic twist has a complete
mixed-pattern `k=1` polynomial one-target solve, but a complete
line-union S5 relation finder and an admitted IC/rho cost comparison
cannot yet be established. The historical native S5 solver misses a verified
`k=0` relation at frozen residual index 2. A separately constructed complete
source-block union now reproduces that exact tuple and public input; the
old solver and its complete 61-query stream have not been repaired/repriced.
Hash/allocator CPU, peak bytes, calibrated GAE and complete same-input
external DLP comparisons remain absent.

The original broader request asks whether twist symmetry speeds relation
finding and index calculus across relevant regimes. The p=73/p=97
three-seed `k=1` panel is now independently audited. Its p=73 seed-one
historical S4 collector misses two verified relations at one failed system; the
p=97 paths are complete on their processed prefixes but sparse in exact
physical relation yield. Three small primes and incomplete cost fields
still prevent scaling or robust crossover claims.

The [exact S4/F4 follow-up](../research/twist_index_calculus_20261020/s4_twist_elimination/REPORT.md)
now repairs that S4 miss and matches every query's complete signed set on
all seven original processed prefixes in two byte-identical runs. The
remaining gaps include S5, the 21 thinned-base replays, complete cold
cost, external solvers and broader scaling; the historical misses and
their original costs remain preserved.

The [endomorphism-image study](../research/twist_index_calculus_20261020/endomorphism_image_base/PROTOCOL.md)
has now completed the resumed 189-case nine-arm polynomial panel and 21
generic-doubling controls, all on the 21 frozen p=37…337 instances.
All 210 independent processed-prefix signed-set and rank audits pass;
189 full-rank scalar/factor recoveries verify, and 21 inactive-column
controls remain censored as prescribed. The original arm matches all
621 earlier S4 prefix queries. Full-stream relation counts come from the
separate complete scalar/group oracle, not from unexecuted polynomial
suffixes. Its cold elapsed comparison now admits all 105 complete
eleven-execution rounds and 630 non-timing replays. Its msolve comparison
checks all 3,024 complete reduced-basis pairs for 378 fixed inputs. The
[final image-base report](../research/twist_index_calculus_20261020/endomorphism_image_base/README.md)
gives every paired per-target cost and its actual supplied-encoding clock
boundary. Complete GAE pricing, matched strongest rho and full external
DLP cost remain open; these image-base controls do not discharge them.

The [zero-fiber polynomial collector](../research/twist_index_calculus_20261020/zero_fiber_collector/REPORT.md)
now completes all 42 registered case-arms on the same seven curves and
three presentations. Separate pair-partition, physical-set and matrix
audits pass for every case. This closes the polynomial implementation and
complete-set replay obligation for that five-factor zero fiber. Charged
target descent, the matched complete IC price and the full formal bridges
remain open; zero-only rows cannot recover the target by themselves.

The public-only Phase 2 pipeline is now implemented. Its tightened native
gate passes 89 tests, including actual p37 zero/plain phi/doubling recovery,
separate complete-physical-set/RREF replay and deterministic A/A. Duplicate
physical tuples are explicitly rejected. These are small positive controls,
not the full registered comparison. The frozen best admitted parent-family
selection gives 85 distinct configurations and 255 first/A/A executions;
the workload is frozen in `af66996a19` before its managed launch, and terminal
panel results remain pending. Globally strongest IC/rho
pricing remains separate and open. See the
[current implementation evidence](../research/twist_index_calculus_20261020/zero_fiber_collector/IMPLEMENTATION_STATUS.md).

A separate exact repeated-factor certificate panel now runs on all 42
registered phi/doubling zero-fiber arms. The first fully audited/repeated
p37 seed0 pair returns no certificate within raw degrees 4..8 and powers
1..3. That is a bounded search result, not an ideal-membership obstruction.
The [degree/closure diagnostic](../research/twist_index_calculus_20261020/zero_fiber_collector/degree_closure_probe/PROTOCOL.md)
therefore fixes indices 0,1,2 from both arms of that same fixture, with
raw16 versus closure8 and original-product cap 16: 12 configurations and
24 producers. Five native algebraic controls pass, including a prelaunch
regression for search cap versus actual original-product degree. Actual comparison
production remains pending at this checkpoint. This supplements both full
panels; no new relation rank, target recovery or complete cost is measured
by those controls. The existing full-price and proof exit criteria remain.
The fixed diagnostic subsequently launched under
[one managed owner](../research/twist_index_calculus_20261020/zero_fiber_collector/degree_closure_probe/RUNNING.md)
at 2026-10-25T04:08:23Z, after its committed source/input freeze. Its
complete comparison results remain pending; the full panels continue.

The first fixed p37 seed0 phi system now has passing first/audit/A/A
records for both diagnostic methods. Raw16 finds q squared at degree 12
with 33,609,631 search multiplications; closure8 finds it at working
degree 7, actual original-product degree 12, with 916,980. Sparse checks
use 15,211 and 15,983 multiplications respectively. These are one-system
certificate counts, not a complete filter or DLP comparison; even closure
search exceeds this system's partial old F4 row count of 125,327. The
registered doubling cases and full diagnostic remain under their owner.

The [concrete instance verification](../research/twist_index_calculus_20261020/zero_fiber_collector/certificate_instance_proof/REPORT.md)
independently verifies all 2,197 dense integer coefficient positions of
`sum(h_i*f_i)=q^2+37*R`, with 454 nonzero coefficients of R. Both native
gates pass 13 tests. Both actual Lean 4.34.1 encodings fail with an
out-of-memory panic within their original 8 GiB address-space bounds;
maximum RSS is 3,775,120 KiB (flat v1) and 3,772,400 KiB (balanced/product
v2). Their statement hash is identical. Neither yields an accepted
instance theorem, axiom audit or IDC. The original sources, integer lift
and failed receipts are retained. This is a proof-encoding resource
obstruction, not a failed arithmetic identity or mathematical impossibility.
The next concrete proof obligation is a kernel-checked, bounded expansion
of the same literal products and integer lift, followed by characteristic
37 and repeated-factor implications, all ten axiom audits and the standard
integer-lift IDC. Splitting normalization into small independently proved
coefficient or product blocks is a candidate encoding; it has not been
implemented or run. The same aggregate proof limit and full statement
must be retained. A successful instance proof would still leave the
native parser, field/group/decoder and parametric-family bridges open.

## Evidence and uncertainty

- **Joint S5 construction, independently validated:** The
  [joint-image continuation](../research/twist_index_calculus_20261020/joint_image_elimination/README.md)
  completes all 297 frozen constructions, 694 system files and 63 case-arm
  sets. Every arm has 84 fixed-query positions plus separately labelled
  positive/regression controls. The native validator checks workload and
  source seals, public input hashes, coefficients, dimensions, statistics,
  msolve inputs and terminal exits. A separate incidence audit checks
  4,110 unique signed relations in 5,433 presentations and actual group
  sums. These are record/coefficient and incidence checks, not an independent
  exact derivation of the generic S5 ideal or a complete-root certificate.
- **Bounded global S5 results:** In the p37/seed0/query2 global regression,
  five native processes hit allocation failure under 8 GiB; the remaining
  pure finite-domain process returns an explicit cooperative incomplete
  result. An empty returned root set is not zero physical yield. All six
  terminal results and the failed complete-set check remain retained.
  Previously interrupted source-block unions now complete in fresh records.
  All three independent signed-set/RREF audits pass: pure gives one
  relation and rank 1/3, phi two and rank 2/3, doubling one and rank 1/3.
  Both full-symmetry pure encodings also complete and pass their independent
  audits, each at rank 1/3. This is one selected positive residual, not a
  population or full-cost gain. The separate historical-match-v3 checker now
  binds all 13 public keys and 61 query pairs, the old miss and the complete
  new pure source-block set. All four corruption controls reject. The old
  report has no full runtime-input seal, so no retroactive seal or repair of
  its whole solver stream is claimed.
- **Fair representation controls:** The implicit mixed encodings retain
  and reduce their inverse equations. The fully symmetric pure-L0 control
  eliminates the distinguished fourth source coordinate by an exact
  coefficient check. On the constructed p37 regression it lowers total
  degree 16 to 8, with 1,485 to 495 extension terms. An exponent-arity
  exporter defect is preserved in v6, repaired, and checked by an actual
  export test; v8 passes 69 tests and writes all six corrected inputs.
  All four mixed implicit native attempts fail allocation under the
  unchanged 8 GiB limit. The full 297-position variant population now
  passes independent native record/export consistency validation: 294 new
  and three reused inputs, with no missing or invalid positions. This
  does not certify complete roots or a generic ideal derivation. Remaining
  complete collection and cold prices remain open. All 21 official msolve
  positions are now terminal: 11 complete physical-set audits pass, nine
  external solves time out, and one native-root audit times out. Reverse
  ideal containment remains unverified. The earlier accidental retry of that
  terminal root timeout was stopped; its partial work and correction are
  preserved, not counted as a new successful computation.
- **Proof and attribution gaps:** Three exact integer identities and IDC
  mutation checks pass. The combined Lean check hits memory limits;
  separately bounded faithful targets retain the original statements.
  The split coordinate gate passes all four theorems. The finite-count
  proof exposes and repairs an arithmetic goal; its new gate passes all
  12 audited declarations. The factored quartic v6 module now passes the
  build and axiom audit: seven named theorems and two local AC declarations.
  It preserves the universal discriminant identity and nonrepetition
  consequence. Earlier memory/PATH failures remain recorded. Group,
  finite-base membership, complete-root and cost bridges remain explicit.
  Equal-size disjoint images have the same uniform signed-tuple total
  mass; this does not equate rank, higher moments or conditioned traces.
  The 21-case zero-fiber rank audit and three p37 full-population censuses
  are explicitly separate oracle diagnostics, with no target coefficient
  in zero-only rows. They are not priced polynomial algorithms.
- **Five-factor zero relations, independently replayed:** All 21 pure/phi/
  doubling censuses complete. Each mixed arm has 504 signed physical tuples
  and 15 projective rows in aggregate, but per-curve ranks differ. At p37,
  phi spans 2/2 factor-log columns versus doubling's 1/2; at p157, 2/8 versus
  1/8; at p337, 1/14 versus 3/14. Other cases have rank zero; pure has no
  five-factor relation on this panel. An independently partitioned public-
  point meet-in-the-middle enumerator and separate matrix elimination match
  all 21 cases and 63 arm sets/ranks, including the full-factor point checks.
  Its enumeration uses no factor logs or BSGS input, but shares the field/
  curve kernel and orbit builder. This is bounded structural evidence and an
  executable group-based enumerator, not a measured polynomial/complete-IC
  price. All zero-only target coefficients are zero.
- **Complete polynomial zero-fiber collection:** The new
  [native panel summary](../research/twist_index_calculus_20261020/zero_fiber_collector/results/panel-v2/native-summary.json)
  binds all 42 completed collectors and independent post-run audits. Each
  arm attempts 33,696 S4 systems after the same 24-action pair quotient,
  reproducing its 504 signed tuples, 15 projective rows and exact per-case
  ranks from the immutable physical census. These are case-local sums,
  not globally unique rows or independent secret samples. The audit
  reconstructs actions with scalar multiplication, checks every pair's
  physical relations and transported union, and uses separate RREF.
  Changed bindings, missing/incomplete systems, duplicate pairs, altered
  work counters and nonempty-set tuple/sign mutations are rejected. The
  journalled producer passes 84 native tests. Shared field/curve arithmetic
  and the equation deriver remain explicit. This is a complete polynomial
  implementation and replay, not a new complete IC speed result. Shared-host
  elapsed diagnostics are unadmitted and all target coefficients are zero.

- **Observed:** The [pilot audit](../research/twist_index_calculus_20261020/quartic_twist_tagged_s5_audit_p37_v1.json) enumerated all `r` subgroup logs and found five complete signed relations among the first four residuals. `k=1,2,3` reproduce their entire pattern-specific sets; `k=0` misses one and `k=4` has one system failure. The [one-target k=1 audit](../research/twist_index_calculus_20261020/quartic_twist_mixed_polynomial_ic_k1_audit_p37_v1.json) independently reconstructs all eight processed sets, rank four, all physical factor logs, scalar 271, and the same-target twist24 rho replay. The native producer counts 116,050,118 `Fp M` and 3,936 group operations against rho's 348,941 `Fp M` and 2,745 group operations, in separate units.
- **Bounded S5 retry:** [Degree 30](../research/twist_index_calculus_20261020/quartic_twist_tagged_s5_k0_q2_d30_p37_v1.json) and [degree 31](../research/twist_index_calculus_20261020/quartic_twist_tagged_s5_k0_q2_d31_p37_v1.json) still return “border unreachable at this degree”; the `k=4` control does likewise. The 8 GiB and 1,200 s caps were observed. A repeated quotient dimension of 4,090 is recorded; its cause is unknown.
- **Derived:** Four `k=1` rows suffice only after the 24-action folds the 48 physical factors to four columns and the public generator log anchors one of them. Flat and cubic-only encodings have 48 and 16 anchored unknowns respectively, so these same four rows cannot solve either. This statement is specific to this base and trace.
- **Larger-curve panel:** [Six independent audits](../research/twist_index_calculus_20261020/K1_SCALE_RUNS.md) verify six one-target scalar recoveries, five exact processed-prefix signed sets and one partial p=73 seed-one set (11 exact, nine polynomial; two missing at query 18). The [post-run difference record](../research/twist_index_calculus_20261020/quartic_twist_k1_missing_diagnostic_v1.json) isolates both missing relations to signed `L2` peel `(54,+1)` and finds no false polynomial relation. Counted IC field multiplications are 1,788–17,249 times matched twist24 rho's on these runs; hash/allocator and calibrated timing remain absent. The [full-stream exact census](../research/twist_index_calculus_20261020/K1_CENSUS_RUNS.md) finds 167 signed `k=1` relations in 359 p=73 residuals and 79 in 1,497 p=97 residuals. An independent BSGS-based scalar oracle matches all 1,856 per-query set digests; the p=97 sparsity is a physical occupancy observation, not a hidden polynomial miss. The [orbit-row audit](../research/twist_index_calculus_20261020/quartic_twist_k1_orbit_row_audit_v1.json) verifies that the 24 images of every selected folded row span only that row.
- **Exact peeled-target orbit diagnostic:** The [full-stream six-run test and subgroup ablation](../research/twist_index_calculus_20261020/K1_PEELED_ORBIT_REUSE.md) check 142,248 finite signed L2 peels. Identity, cubic-only, transported-Frobenius-only and combined-action caches find 20, 107, 243 and 790 duplicate targets respectively. Only two combined-action collisions occur within one residual; 788 more occur across residuals within their separate streams. Even an ideal unlimited combined cache yields only a 1.005585× aggregate S4 call-count ratio here, before cache overhead. This is no native speed measurement or general obstruction.
- **Native base-shape panel:** The [complete 21-case report](../research/twist_index_calculus_20261020/K1_THIN_NATIVE_REPORT.md) and [machine validator](../research/twist_index_calculus_20261020/K1_THIN_NATIVE_VALIDATION.json) show full anchored rank and independently verified scalar 271 on every preregistered prefix choice. Seventeen processed signed relation sets are exact; four p=73 seed-one choices remain partial from the same one failed S4 system. All 21 first/A/A non-timing records and independent audit outputs match, and six full-base controls reproduce the older scaling source. The largest observed proper-subset same-seed full/candidate counted-field ratios are 3.653× at p=73 and 3.575× at p=97; p=97 seed-one one-orbit regresses to 0.745×. This comparison charges full-base and subset-map construction but omits native hash/allocator CPU and a calibrated whole-method unit.
- **Suspected explanation:** The S5 closure failure may be a low-degree Macaulay border or specialization defect in the native solver; neither was established by the retries. A different order, degree, Gröbner engine or exact root algorithm could change the outcome.
- **Exact S4 repair:** [Seven complete first/A/A collectors](../research/twist_index_calculus_20261020/s4_twist_elimination/results/prefixes.json) execute 46,200 finite systems per run with no undetermined cases. Every signed set matches a post-run group-pair oracle; all seven anchored matrices and target/factor logarithms verify. The old p73/seed1 prefix's nine relations become the exact eleven and full rank first occurs at query 18 rather than 19. Eight separately frozen systems also match complete `Fp^3` root enumeration. The 49-term and 18-term resultant identities have zero exact integer remainders, separate IDC records and two checked Lean theorems. All 46,200 component-coefficient matches pass; simple linear equation reduction retains degree profile `[4,4,4,4]`. F4's counter covers row reduction only, so this is a completeness result, not a cold-cost improvement.
- **Missing capability:** A solver that returns complete signed sets for all five patterns on the frozen p=37 stream; F4 replay on the 21 thinned bases; complete F4 and cold-pipeline accounting including native hash/tree/allocator and rho hash CPU; peak memory; isolated ABAB wall intervals; a same-input msolve comparison (Magma if available); and tests on larger curves beyond p=97.
- **Not established:** Impossibility of complete S5 solving, polynomial IC superiority to rho, fastest solver status, large-size asymptotics, or an effect on deployed curves.
- **Image-base attribution control:** On the same 9,658-query full reference streams, phi(L0) supplies 1,865 unique signed tuples and 1,698 distinct projective augmented rows; [2]L0 supplies 1,804 and 1,705 respectively. Both active-column variants attain full rank on all 21 cases. These distinct rows are not all linearly independent. The psi control reproduces the original's exact sets; phi(L2) gives 1,472 tuples against the original's 1,473 with equal peel counts. All 210 candidate-prefix audits pass, both A/A trees match, and hash/omission negative controls reject. All 105 whole cold rounds and 630 non-timing records are admitted; original deadlines, isolation refusals and Spot reclaims are retained. Through first full rank, phi consumes 5.868804% more systems than doubling. Per-record paired doubling/phi median elapsed ratios span 0.405438–5.524357, with both gains and losses. The exact fixture boundary includes encoded public points for alignment while recomputing and charging every lift. The same-input GB control completes 378 fixed systems in eight passes, with 3,024 matching native/msolve bases and generally slower native timers. Complete operation pricing, strongest rho, S5 and full external DLP comparisons remain open.

## What would unblock it

The S4 component-ideal follow-up now has two checked consequences:
2048 times the discriminant lies in the component ideal (independent
IDC1h09d838b7dca9895f and two Lean theorems pass), and 64 times the cube
of S=64*s4-(4*s2-s1²)² lies there (native exact search and combined
10-theorem Lean gate pass). The latter excludes three distinct roots
in odd characteristic when all four components vanish. Full curve/group
and field-embedding formalization is separate. The stronger independent
replay now passes as IDC1h65c11f01d0b1e67a, with zero integer remainder,
32 extra points and both mutation controls. Its finite-Fp residual audit now retains
infinity and preserves every saved physical presentation, unique row,
rank and recovery on all 21 cases. It rejects one phi and two doubling
systems over the full tape, and zero systems before first full rank in
either arm. No filter is deployed and no through-rank saving results. This neither rules
out higher-arity precomputation nor establishes a twist-specific gain.

- [ ] Reproduce every `k=0…4` signed decomposition on the frozen 61-residual stream with no failed algebraic systems, retaining all extraneous, duplicate, and invalid roots and a scalar-keyed independent audit.
- [ ] Export identical Weil-restricted S4/S5 systems to the native engine and msolve on the same host, compare complete roots, memory and all solver phases; include Magma if available and name its absence otherwise.
- [ ] Emit native hash/tree/allocator and rho-partition CPU/work counters, requested and observed peak bytes, exclusive setup and online phases; run at least five interleaved ABAB repetitions under the repository's isolation tool, A/A controls, same target and resource envelope, and price `w=0,0.1,1` GAE.
- [x] Run a preregistered p=73/p=97 one-target panel with independent signed-set/rank/scalar audits and a cap that records censored outcomes. Five processed-prefix relation sets pass; one is retained as partial.
- [x] Run every preregistered orbit-complete L2 prefix choice with native polynomial collection, full-base construction, independent exact signed-set and rank replay, full-base source parity and A/A. All 21 recover the target; four retain the known relation miss.
- [x] Repair and independently audit both missing p=73 seed-one S4 relations, retaining the historical failure and cost record; complete on all seven original processed prefixes in first/A/A.
- [ ] Extend the repaired collector to all 21 thinned bases and the unused frozen-stream suffixes; extend the polynomial panel beyond p=97 where feasible.
- [x] Execute all 189 image-base cases plus 21 doubling controls through rank or their frozen cap on all seven primes, with complete independently checked processed sets, rank progress, source parity and A/A.
- [x] Reconcile and admit all 105 complete same-origin cold rounds and 630 non-timing replays for the ten image-base arms; check complete elapsed phase sums and retain every censored, interrupted and refused record. Complete GAE prices are still missing.
- [x] Compare complete native and msolve reduced bases on all 378 preselected image-study systems in eight passes, preserving losing rows and internal timer ranges. This is a GB component comparison, not all S5 roots or external DLP cost.
- [x] Bind the historical p37 pure S5 miss to the new complete source-block set with full public projection and corruption controls; preserve the missing runtime seal and incomplete old whole-stream solver.
- [x] Reconcile all 21 joint external positions and audit every completed available basis's physical roots; retain nine external timeouts, one native-root timeout, and the stopped duplicate retry.
- [x] Complete all 21 four-factor zero fibers, three full p37 populations, all 21 five-factor censuses and 63 independently enumerated public-point set/rank comparisons; count zero/projective rows separately.
- [x] Implement a polynomial five-factor zero collector with the same orbit quotient for phi and doubling and independently preserve every complete physical set and factor-log rank on all 42 registered case-arms.
- [x] Implement public-only stopped zero precomputation plus target descent and verify all four p37 zero/plain phi/doubling positive controls, separate replay and A/A in the tightened 89-test native gate.
- [ ] Execute and charge the complete 85-configuration stopped-zero/target panel, including setup, every failed system and matrix work, against matched plain phi/doubling and the strongest existing verified IC baseline; retain every loss and separate counters from admitted timing.
- [ ] For any priced leaderboard result, provide the replay certificate, validator report and strongest matched rho receipt in `VALIDATION.json` under `docs/VALIDATION_GATE.md`.

The first item would establish a complete tagged-line-union polynomial
finder on the tested stream; it would not by itself establish a practical
or asymptotic advantage. The other items address ranking and transfer.

## Next discriminating step

- **Action:** Continue Phase 2 of the
  [frozen zero-fiber protocol](../research/twist_index_calculus_20261020/zero_fiber_collector/PROTOCOL.md#phase-2-complete-one-target-pipeline).
  Stop zero precomputation at determined factor logs or the declared pair
  cap, retain its independent rows, and execute the original target trace
  through verified recovery or its original cap. Freeze the strongest
  verified existing IC baseline selection before the comparison. Charge
  setup, every failed system, action bookkeeping, matrix work and target
  descent; compare phi, generic doubling with the same quotient, and their
  plain controls. Orbit copies add no independent folded rows. The completed
  189+21 panel, zero-fiber polynomial collection and physical censuses must
  not be repeated merely to produce new receipts.
  The exact parent-family baseline is now frozen; execute all 85 distinct
  configurations with 255 candidate runs, 85 independent first-run audits
  and 170 A/A checks. A lower F4-system count alone does not close the
  complete-price requirement.
- **Inputs and controls:** The completed 297-construction workload, frozen
  p37/seed0/query2 regression, pure L0 with full permutation symmetry, and
  generic doubling with the same image size and represented logs. Keep
  original failures and scalar/group oracle separate from candidate inputs.
- **Limits and stop:** Phase 2 retains one science worker, 8 GiB per case,
  a 1,200-second outer watchdog and 1,180-second cooperative case deadline;
  each S4 call receives the remaining budget. Keep the busy reservation and
  separate timeout/OOM outcomes. Check managed
  owners and receipts before retrying after app/host recovery. Complete
  physical equality motivates full-stream collection and complete costing;
  a rank increase in zero-only rows motivates an actual invariant collector
  plus charged target descent. A bounded failure or zero-row concentration
  does not establish an obstruction or an IC improvement.
- **Recheck trigger:** Completion of stopped target-descent execution and cost instrumentation, solver repair, or a new available external-solver comparison. The shared migrated host currently permits correctness/counter diagnostics, not new admitted timing.

## Resolution and history

2026-10-25: The public-input stopped-precomputation/target pipeline passes
the tightened 89-test gate and actual four-way p37 recovery/replay controls.
The fixed 85-configuration comparison is frozen before its managed launch;
its terminal results remain pending.
No complete GAE price or newly admitted timing is inferred from these tests.

2026-10-24: All 42 polynomial zero-fiber collectors and separate physical/
rank audits complete successfully. The same quotient and full pair coverage
are checked for both arms; all failed launches, native tests and the stopped
nested-lock attempt remain preserved. Collection is complete at this bounded
scale. Charged target descent and a matched complete IC price remain open.

2026-10-24: All 297 joint S5 constructions pass independent validation.
Six native global solves remain bounded failures. All three source-block
unions and both pure full-symmetry encodings have complete signed-set/RREF
audits. Phi inverse-coordinate attempts fail allocation. The exact quartic
discriminant/nonrepetition Lean proof now passes. Remaining comparisons
continue under managed owners after explicit migration reconciliation.
The broader IC gain and full-price
requirements remain unresolved.

2026-10-22: The restricted `k=1` one-target path and independent replay
landed as evidence; S5 degree 29–31 retries remained partial. Unresolved.

2026-10-23: The six-run p=73/p=97 polynomial panel, twelve-run
point/scalar-keyed full-stream census, and deterministic 24-action row
audit were added. One p=73 S4 system remains partial; complete costing
and external comparisons remain unresolved.

2026-10-23: The 21-case native orbit-thinning grid and separate
identity/cubic/Frobenius/full peeled-target cache ablation were added.
The restricted S4 path has counted base-shape improvements on selected
frozen cases and a regression on another; the p=73 algebraic miss,
full pricing and external comparison remain unresolved.

2026-10-23: Exact coefficient derivation and two Lean identities, an
eight-system exhaustive field-root comparison, and complete seven-prefix
F4 first/A/A collectors repair the S4 miss. Full cold costing and external
comparisons are still open. The proposed endomorphism-sum image base is
recorded as draft card
[`XP-20261023-quartic-twist-image-base`](../backlog/XP-20261023-quartic-twist-image-base/README.md),
with an unimplemented harness and explicit prerequisites; no image-base
result or launch is implied.
