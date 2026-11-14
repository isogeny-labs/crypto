# L3 protocol: incremental SAT across decomposition targets (frozen 2026-11-14, before any measured run)

Lane L3 of `research/ecc2k130_open7_20261114`. Rules: `../COMMON.md` (with Amendment 1, so the harness is Rust).
Every number this protocol leads to is `[pcc:unverified]`.

## 0. Prior-art audit (summary; details in RESULTS.md)
- `research/sat_ladder_scaling_20261110`: one incremental CaDiCaL per curve, target bits as assumptions,
  learnt clauses kept. That is the **ladder** formulation (prime field, the bits of k are the unknowns),
  not point decomposition. The prescribed replication is pending, and the result there may be table
  exhaustion of tiny groups.
- `research/polynomial_reuse_20260914` + `cryptanalysis:suite/src/cryptanalysis/polynomial_reuse.rs`:
  the target-independent part of the S3 chain is compiled once (the template). Its README says:
  "SAT answers and learned clauses are not persisted by this implementation".
- `cryptanalysis:ecc2k130/runner/research/candidates/goal22/codegen/indexcalc.py` `solveCnf`: a fresh
  pysat `CryptoMinisat` for each target, with R folded in as constants (`decomp.encode`).
- `cryptanalysis:suite/src/cryptanalysis/sat.rs`: native CDCL with no assumption interface.
  `reset_search` plus blocking clauses is used only to enumerate models **within** one target
  (`koblitz_index_calculus.rs`, enumeration loop near line 5343).
- No prior run found that keeps solver state across decomposition targets. research-kb and the lead
  atlas are unavailable in this container, so this is not a novelty claim.

**Residual question:** on the Koblitz S3-chain decomposition system with a weight factor base, do we
spend fewer conflicts per target if we keep one incremental solver across targets, with the target
bits as assumptions and the learnt clauses kept, than if we solve each target cold? And what share of
the retained learnt clauses can apply to a different target at all?

## 1. Instance
- Curve y^2 + xy = x^3 + 1 over GF(2^m), type-II optimal normal basis (as in goal22 `field.Onb`).
  The factor base is {x : HW(x) <= w}. For k points, the system has k-1 S3 links chained through
  k-2 intermediate x-coordinates, encoded at bit level in the same way as `decomp.py`.
  ONB multiplication uses the direct m^2-AND formula, Tseitin encoded. Squaring is a coordinate
  permutation. Weight uses a Sinz counter. A lex order on the points (`cnf.lexLeq`) removes the k! orderings.
  Re-implemented in Rust (`tool/src/main.rs`). This is not the codegen Karatsuba circuit, which is a stated deviation.
- Configurations (m prime with 2 primitive mod m, 2m+1 prime, so type-II ONB, mirroring m = 131):
  - C1 m=11, k=2, w=2
  - C2 m=11, k=3, w=2
  - C3 m=29, k=2, w=3 (primary): mostly non-decomposable targets, the regime of a real IC run
  - C4 m=53, k=2, w=2 (optional, run only if C1 to C3 finish): essentially every target is UNSAT
- Target stream: 40 x-coordinates of uniformly random curve points. The seed is `L3/targets/<tag>`
  (splitmix64 over FNV-1a). Then a **repeat block**: the first 5 targets again (positions 40 to 44).
- Per target, every arm **enumerates all decompositions**: solve, add a blocking clause on the
  projection (the p and t bits), and repeat until UNSAT. In incremental arms the blocking clauses are
  guarded by a per-target activation literal, which is deactivated afterwards.

## 2. Arms (same targets, same instrument, arm order rotated per target)
Solver: CaDiCaL (vendored by `cadical-sys 0.8.0`), defaults except `factor=0, factorcheck=0`, set in
every arm so that no extension variables appear. Conflict budget per solve call: 2,000,000. A
censored call is reported as censored, never imputed.
- **A0** cold, current practice: R folded in as constants, fresh solver per target.
- **AA** A0 with the clause list shuffled (per-target seed): the A/A noise floor of cold solving.
- **A1** cold, template CNF (R as variables) with R given as assumptions, fresh solver per target.
  This isolates the encoding.
- **B** one persistent solver, template loaded once, R as assumptions, learnt clauses and heuristic state kept.
- **BS** B with the template clauses shuffled once: the A/A noise floor of the persistent arm.
- **CI** after every target, rebuild from the template plus the live derived clauses (taken from the
  DRAT trace) that mention **no** R variable and no activation literal. This carries only
  target-independent learnt clauses.
- **CA** as CI, but every live derived clause without an activation literal is carried. This is B
  without its heuristic state.
- **CK** B, flushed every K = 4 targets (rebuilt from the template alone). This is the periodic cleanup.

## 3. Metrics
Primary: **conflicts per target** (sum over all calls of the enumeration). Also decisions and
propagations, the first-call cost, and splits by decomposable (SAT) and non-decomposable (UNSAT)
target. Learner hook on B, BS, CI, CA and CK: for each learnt clause, whether it mentions an R
variable or an activation literal, how many R literals it has, and its applicability
sum 2^-(#R literals): the share of uniformly random targets whose assumptions do not satisfy the
clause. Wall time is a note only.

## 4. Controls
- **Correctness (fatal):** the model set of every arm and every target equals a brute-force ground
  truth, computed by enumerating the factor-base tuples and solving the S3 quadratics. The number of
  models that lift to an actual curve relation (±R) is reported.
- **A/A:** AA/A0 and BS/B, with the same statistic as the primary one.
- **Positive control:** the repeat block. On a repeated target, B should cost less than A1 on the
  same target. Pass when the geometric mean of (B+1)/(A1+1) over the repeat block is < 0.5 in at
  least one configuration. If it fails in every configuration, the result is INCONCLUSIVE (the
  instrument cannot see reuse).
- **Determinism:** B and CA are identical at position 0, because CA differs only by proof tracing
  before its first rebuild.

## 5. Statistic and decision rule
For each configuration, over stream positions 1 to 39, the statistic is the geometric mean of the
paired ratio (c_X + 1)/(c_Y + 1) of total conflicts. The 95% CI is a percentile bootstrap over
targets (2000 resamples, fixed seed). The verdict is read on **C3 (B vs A0)**. C1, C2 and C4 are
supporting evidence.
- **IMPROVES:** upper CI < 0.8 and the CI is disjoint from the AA/A0 CI.
- **NEGATIVE:** lower CI > 1.25 and the CI is disjoint from the AA/A0 CI.
- **NO-CHANGE:** the CI lies inside [0.8, 1.25].
- **INCONCLUSIVE:** anything else, or a failed positive control, or censoring on more than 10% of
  C3 targets in A0 or B.

Secondary statistics, reported but not deciding: B/A1 (pure reuse, same encoding), CI/A1, CA/A1,
CK/A1 and A1/A0. Each is also split by SAT and UNSAT target.

## 6. Pre-registered predictions
- P1. A1/A0 > 1: folding the constants of R makes the cold solve cheaper than keeping R as assumptions.
- P2. B/A1 < 1 on C1 and C2 (small field, large share of the space covered), and closer to 1 on C3.
- P3. Share of learnt clauses in B with no R literal and no activation literal: < 25% on every
  configuration. Mean applicability of the non-activation learnts: < 0.1.
- P4. Verdict on C3: NO-CHANGE or NEGATIVE. Prior probability of IMPROVES: 0.25.
- **Falsifier of the "reuse is a toy-table effect" reading:** B/A0 upper CI < 0.8 on C3, with a
  B/A1 advantage at m=29 no smaller than at m=11.

## 7. Counted-cost mapping (AGENTS.md §5a)
Conflicts and propagations are unpriced native operations. Per target they are priced at 0, 0.1 and
1 GAE per propagation as a band. No end-to-end IC number exists in this lane (no relation-yield or
linear-algebra phase), so no gap to matched rho is reported: a saving here is a stage ratio of the
decomposition oracle only.

## Amendment 1 (2026-11-14, after the C1 data, before any C3 data; rules and predictions unchanged)
- **C3 changed from m=29, k=2, w=3 to m=23, k=2, w=2.** An unscored timing smoke at m=29, w=3
  (`$SP/lanes/L3/dev/dev_c3.jsonl`) censored the first A0 target at the 2,000,000-conflict budget
  (149 s). The section 5 censoring clause would then fire by construction. m=23 has a type-II
  ONB (47 is prime, ord_47(2) = 23), but 2 is not primitive mod 23. That breaks the "mirror
  m=131" preference, and it is the only odd m between 11 and 29 with a type-II ONB. An unscored
  smoke on 2 targets (`dev_m23.jsonl`) finished: A0 about 7e4 conflicts per target, A1 and B about
  1.3e5 to 2e5. The prediction P2 statements about "C3" now refer to m=23.
- **C4 (m=53) dropped**, because it is harder than m=29.
- **C3 runs as two processes,** {A0, AA, A1, B} and {BS, CI, CA, CK}. The target stream is
  identical (same seed), and the arm order rotates within each process. Counted metrics do not
  depend on process placement.
- Note recorded before the C3 data: the C1 positive-control ratio (0.438) is about the same as the
  C1 stream B/A1 ratio (0.426). So the repeat block does not separate target-specific reuse from
  general warm-up. The pass/fail criterion is unchanged.
