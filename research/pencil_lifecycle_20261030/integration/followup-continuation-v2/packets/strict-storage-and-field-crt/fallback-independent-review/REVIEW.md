# Independent review: constructed quintic fallback reduction

Producer commit: 55b56b1e04 (full SHA in commit.txt). Reviewer /root/quintic_fidelity_review. Plan /var/tmp/quintic-execution-review-plan.md read first. No sibling reports read. Producer context inherited; this is not mutually blind derivation. No repository edits, cloud actions or scientific trials.

## Source-fidelity result

**faithful_scoped** for the five stated declarations and the report's modeled trace claim. No sign, identity orientation, or Divided projection error found.

- step_exists constructs exactDiv(residual(f,v),u), normalizes q, and computes remainder(pneg(v),monic(q)). The residual identity gives q dividing f-v*v; nonzero quintic residual proves nonzero q. Negation preserves v squared. remainder_valid then carries congruence to the computed r. Canonicality and normalization of r are proved inside the construction, not assumed.
- Run.step guard is 3<u.length, matching native while u.len()>3. Its successor state is precisely monic(q),r. Run.stop requires u.length<=3 and does not itself assert validity, which is appropriate for a raw control-flow relation; initialized_run supplies invariants separately. The proof explicitly constructs stop or step followed by stop.
- reduction_exists includes the native entry normalization and initial remainder of v modulo monic(u); these are not skipped. Empty v is separately handled when natural degree, unlike degree, represents zero by zero. The degree-four case admits residual degree six; no accidental degree-five-only reduction bound appears.
- fallback_reduced uses the actual two-EEA chain and Divided. hdiv.2.2.1 is Canon u; hdiv.2.2.2.2.1 is Normal u; hdiv.2.2.2.2.2.2.1 is the correct product identity Ua*Ub=U*(D*D). composed_degree cancels no possibly zero factor: nonzero Ua,Ub imply nonzero U and D squared first. The sum of input degrees is at most four. Native fallback performs first prem(W,U), then reduce(U,R), whose entry performs second prem(R,monic(U)); both calls appear in the theorem.
- The new report explicitly limits native dispatch to Field.n=1 and no diagnostic tables, and excludes compiled equivalence, allocation/counters/copies, optimized branches and divisor-class meaning. These exclusions match the actual proof.

## Remaining interface detail

The endpoint returns a nonempty a, a.length<=3, b.length<=2 and decoded Valid. It does not expose Normal b or the stricter b.length<a.length required by Divisor::from. Those bounds are established internally at the remainder calls, and the actual constructed witness has the needed property, but the published endpoint's outward conjunction does not state it. For a direct consumer theorem about native Divisor::from, add the strict inequality (and Normal output if canonical normalized storage is claimed), or derive it separately from the trace plus entry invariants. Merely using the endpoint's size-and-Valid conjunction is insufficient: a=[1], b=[0] has valid decoded polynomials and fits both arrays but fails the strict assertion. This is a missing exported bridge, not a counterexample to fallback_reduced, whose witnesses are tied to actual calls.

## Independent fresh machine evidence

Six modules compiled successfully in this review session: ReductionStep, ReductionRun, ReductionEntry, FallbackReduced, ReductionExecutionControls and ReductionExecutionReadback. ExpectedFailure returned 1. All nine named axiom readbacks passed the published native checker, which additionally exercised forbidden-axiom, empty-output and wrong-name rejection controls. The checker acceptance JSON retains its original producer-oriented keys; interpret producer_controls=4 as the four fixed theorem controls replayed independently, not newly designed controls.

Sources were extracted with git show from the immutable producer commit into input/. Neighboring helper caches were copied only after matching their source to the committed copy. External helper source/cache roots were copied to deps/. Shared standard-library/Mathlib dependency-union caches were reused, recursively hashed before and after, and checked unchanged. Toolchain binary and all private bound inputs were also checked unchanged. This is a fresh kernel elaboration of the named modules using reused dependency caches, not an independent fresh build or binary recheck of all dependencies. The copied source tree was not changed during the replay.

Initial attempt failed because setup copied only regular top-level files and omitted symlinked Init/Mathlib union entries. It reported unknown module prefix Init, before theorem checking. attempt1-logs preserves it. The revised search path explicitly includes the existing pinned unions; their content hashes are in reused-union-before.sha256 and checks in reused-union-check.txt. No mathematical failure was observed. logs/ contains successful second-attempt exits and stdout/stderr; acceptance.json is the checker result.

Machine status for the nine named declarations: passed (fresh reviewer run with reused dependencies). Fidelity: faithful_scoped for the five modeled declarations. Overall review outcome: scoped source/kernel checks passed; full native execution and direct Divisor::from-interface closure remain separate. Fixed controls cover stop cases, guard and zero negation, not an independently instantiated complete nontrivial one-step fallback. Do not call this end-to-end empirical evaluation or mutually blind derivation.

## Sources and next obligations

Read: prewritten review plan; same AGENTS/review-evidence Lean-review instructions as prior task; immutable packet REPORT.md, all four main new source modules, controls, readback, inventory, check.rs/check.sh/replay.sh, helper Divided definition and dependency inventory; native fallback and reduce at the pinned source. No sibling verdicts consulted. Reused prior local source-fidelity knowledge was not called fresh independent derivation.

Next: export strict final storage inequality; bind model dispatch to native prime-field runtime; derive array-copy postcondition; establish divisor-class interpretation. Preserve frozen full cohort and existing expired scientific limits. Independent source/kernel review does not renew those authorizations.
