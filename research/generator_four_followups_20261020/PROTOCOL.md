# Generator-effect follow-up: four discriminating experiments

Status: **protocol / implementation backlog only**. No runs, speedup claims, or validation artifacts are asserted by this file. Date: 2026-10-20.

## Dependencies and provenance
- Existing known-log factor-base work: PR #2297 (not assumed merged).
- P-192/P-224 generator statistics and model: PR #2301 (stacked on #2297).
- Cofactor-class yield analysis: PR #2288.
- Screenshot-derived E2 result is **not independently replayed here**. Treat its reported h=1,2,4,8,16 yield ratios and A/A discrepancy as hypotheses until raw receipts are available.
- Respect AGENTS.md, docs/VALIDATION_GATE.md, and docs/ic/boundary_targets.json. Keep frozen evidence immutable. Do not promote a leaderboard row without replay certificate, validator report, and matched rho reference.

## E6: generator dependence
**Question:** Can a choice of generator G'= [u]G and a generator-tailored base change cold arbitrary-target cost after normalizing the same group and budget?

- Use prime-order toy curves with independently verified group orders, at least three curve instances per bit-size, and P-192/P-224 only for bounded structural checks (no infeasible full DLP benchmark).
- For each curve sample >=16 distinct invertible u modulo subgroup order using precommitted seeds. Include u=1, u=-1, random u and matched permuted-scalar controls.
- Use the *same geometric factor base* under relabeling, and separately a base reconstructed using the new generator. Distinguish these arms; never conflate generator effect with base selection.
- Sample targets Q=[k]G, convert planted answer k'=k/u mod r for G', and verify [k']G'=Q. Do not supply k or k' to an arbitrary-target solver.
- Paired targets and RNG streams; record factor-base construction, relation finding, rank, target recovery, peak memory and total GAE S. Hold out independent seeds.
- Negative control: pure coordinate relabeling must produce the same mathematical relations after canonicalization. Positive control: deliberately biased toy base with known nonuniform coverage.
- Decision: only a statistically supported reduction in verified cold S against the same-curve current IC baseline counts as a pipeline gain; report rho gap separately.

## E7: useful independent relations
**Question:** Does distinct-sum yield translate into full-rank equations?

- Record each relation as sparse coefficients modulo prime subgroup order r, RHS scalar if legitimately known, canonical sum point, and its provenance.
- Compute incremental modular row rank, nullity, dependent/duplicate counts, rank per 1e6 group operations, and rank growth versus samples.
- Include explicit target coefficient and enforce correct augmented-system consistency; never count a relation whose RHS contains the unknown target log as known.
- Cross-check ranks with an independent elimination implementation on a fixed subset; verify every row as an actual curve group identity.
- Compare abscissa, known-log progression, known-log random, and uniformly sampled bases at equal base size, memory and construction budget.
- Negative control: duplicate each row; raw count doubles, rank must not. Positive control: planted independent identity rows with known rank.
- Report full linear algebra and failed target-recovery attempts; rank alone does not establish key recovery.

## E8: cofactor, collision and symmetry accounting
**Question:** What explains the divergence between pair-count and distinct-sum models at h=16, and the reported h=4 A/A ratio 0.971?

- For each synthetic h in {1,2,4,8,16}, verify #E(Fp), prime subgroup order r, cofactor h, base subgroup membership, and distinct-sum counts.
- Partition pair collisions into identical pairs, negation, torsion/cofactor classes, known automorphism orbits, and residual collisions; ensure classes are disjoint with explicit priority.
- Compute additive energy and orbit-stabilizer predicted counts; test pair-count Np versus exact |S+S|/r on independent holdout targets.
- A/A: independently reproduce identical arms with separately seeded RNG and compare across hosts; report confidence intervals, multiplicity corrections and whether 1 is excluded.
- Negative control: randomized subgroup labels preserving cardinality; positive control: intentionally duplicated pairs.
- Do not extrapolate h>1 synthetic results to standard P-192/P-224 (h=1).

## E9: cold arbitrary-target end-to-end comparison
**Question:** Can the candidate recover an unknown target log more cheaply than the current IC baseline and matched rho?

- Freeze a protocol before measurement. Use tractable toy sizes (e.g. 18, 20, 22, 24 bits where supported); multiple independently generated curves and target seeds.
- Arms: best verified current IC pipeline, known-log random, known-log progression, equal-memory BSGS (including negation), and strongest applicable rho with negation/automorphism folds.
- Inputs to solver: E, G, Q, public precomputed tables allowed by protocol; planted k only in external verifier. Reject any solver that receives the answer.
- Charge setup/base construction, failed relation attempts, relation verification, linear algebra, target descent, memory and all group operations; report cold S at w=0,0.1,1 GAE, plus amortized multi-target results separately.
- Verify each output [k]G=Q; record failures, censored timeouts, wall-clock and isolation tier. Compare the fastest external implementation of the same computation on matched inputs/host where available; otherwise mark the external comparison missing.
- Negative control: hidden random target with no known-log hint. Positive control: small planted solvable instance.
- Acceptance: independent raw-record validator, independent replay, matched rho, precommitted thresholds, and honest CI. No deployed-curve or asymptotic-security claims from toy runs.

## Execution checklist (currently NOT DONE)
- [ ] Reserve scope via conductor (if reachable).
- [ ] Rebase/coordinate with PRs #2297, #2301, #2288 and select exact runnable entrypoints.
- [ ] Implement E6-E9 harnesses and unit/positive/A-A controls.
- [ ] Freeze per-experiment protocol commit before measured runs.
- [ ] Run required Rust/Python suites and the experiments on identified hosts.
- [ ] Commit raw data, manifests, seeds, hashes, failures, environment, and cost accounting.
- [ ] Independently replay and validate; compare external tools and rho.
- [ ] Open implementation/result PRs or extend existing PRs; promote leaderboard only if gate passes.

**Blocking distinction:** this document specifies experiments; it does not implement or execute them. GitHub file-edit connector alone does not provide a runnable checkout or conductor reservation.
