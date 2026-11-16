# Curve25519 torus four-point decomposition cost and distribution bridge

- **Status:** open
- **Kind:** proof gap; implementation gap; measurement gap
- **Affected work:** [native study](../research/curve25519_torus_native_20261115/README.md), [frozen energy draft](../research/curve25519_torus_native_20261115/theory-v1/energy-lemma.md), [related factor-base obstruction](factor-base-shape-to-successful-work.md)
- **Next action owner:** Curve25519 torus campaign
- **Last updated:** 2026-11-16

## Problem

The four-point quadratic circuit does not yet have a checked distribution
theorem or a budgeted decomposition algorithm with cost D=soft-O(B^gamma),
gamma<1, on an admissible growing family. Those gaps prevent using the
conditional relation cost N*D/B^3 and linear algebra B² to establish an
attack exponent 2/(5-gamma). [pcc:unverified]

The actual instance is E: v²=u³+486662u²+u over p=2^255-19, prime subgroup
order ell=2^252+27742317777372353535851937790883648493 and cofactor eight.
Its candidate raw base is four cosets of the norm-one-torus subgroup of
order M=60824497*213156431. The proposed energy bound is over admissible
random multipliers, not a bound for the fixed multiplier one.
[pcc:unverified]

## Evidence and uncertainty

- **Observed:** [four-point-v1](../research/curve25519_torus_native_20261115/four-point-v1/) records 454 variables, 455 quadratic equations, a checked known witness and eight raw sections of the prime target. No solver was invoked. Producer claim PCC-20261115-02fa5a remains unverified. [pcc:unverified]
- **Derived or proved:** [Lean arithmetic](../research/curve25519_torus_native_20261115/lean/Arithmetic.lean) covers concrete integer dependencies only. The geometric collision bound remains a draft; [source addendum](../research/curve25519_torus_native_20261115/theory-v1/SOURCE_ADDENDUM.md) names its map-degree dependency. [pcc:unverified]
- **Suspected explanation:** fewer terms or degree-two auxiliaries may help a solver, but neither implies a bounded solving degree or gamma<1.
- **Missing capability:** cost-accounted isolated full-base solver runs; an actual-object cover/morphism proof and independent review; family generation and full phase/rank bounds. Generic four-point recovery is implemented in the adjacent cryptanalysis study and has bounded M=1 native controls; full-base discovery remains untested. [pcc:unverified]
- **Not established:** route exclusion, a fixed-base yield theorem, asymptotic improvement, or novelty.

## What would unblock it

- [ ] Review and formalize the actual multiquadratic cover, independent sign automorphisms, projective elliptic maps and total degree bound; resolve every review counterexample.
- [ ] Freeze a label-hidden budgeted oracle and independently check recovered roots, four signed point sums and cofactor transport on its public inputs.
- [ ] Test its registered success/cost promise with all failures retained, then meet the repository's six-size/24-order-bit scaling requirement with held-out predictions and complete cold work accounting.
- [ ] Supply a growing admissible family and bounds for setup, indexing, rank, sparse linear algebra and descent. A finite favorable fit alone does not discharge this condition.

The first condition would establish a distribution component. The others are
still needed for the requested exponent change.

## Next discriminating step

The [16 November scaling audit](../research/curve25519_torus_native_20261115/reports/scaling-audit-v1/REPORT.md)
adds a parameter gate for the proposed six-summand alternative: cap yield
at one, charge all setup/kernel/memory work, and establish a suitable base
cardinality before transferring a numeric model to an actual solver.
The current transport lower-bound integer exceeds the unit-constant
sixth-root target; this is a finite arithmetic observation, not exclusion
of an asymptotic family. Many-coset candidates still lack admissibility and
recognition-cost certification. PCC-20261116-ebfd31, 76dc04 and d45270.
[pcc:unverified]

The [private-coefficient clarification](../research/curve25519_torus_native_20261115/energy-test-v1/RANKFREE_REDUCTION.md)
proposes replacing a statistical row-rank assumption with a common-column,
tag-blind kernel protocol. Scalar-field endpoints and finite controls are
available, while the actual point/selection bridge remains open. This is
not a new decomposition-cost bound. PCC-20261116-f0439a. [pcc:unverified]

- **Action:** freeze the four-point public-input solver protocol and qualify its isolated runtime. Generic recovery, union membership, mutation controls and actual-field M=1 native diagnostics are implemented in cryptanalysis; they do not replace full-base discovery. [pcc:unverified]
- **Inputs and controls:** actual-field known-witness construction, all eight cofactor sections, M=1 complete-oracle diagnostics, and separately sampled label-hidden targets. The diagnostics do not substitute for the full base.
- **Limits:** development checks only until the [isolated runner gate](curve25519-singular-isolated-runner.md) is satisfied. Every experimental run must have explicit CPU, memory, wall, query and root-recovery limits before launch.
- **Outcomes:** a malformed root invalidates that run; a checked decomposition is positive evidence only; budget exhaustion parks that implementation; a counterexample rejects only the exact universal lemma it violates. No timeout rejects all index calculus.
- **Recheck trigger:** completed native recovery, independent geometric review or an admitted scaling dataset.

## Resolution and history

Unresolved. 2026-11-15: native construction and narrow arithmetic proofs
preserved; geometric and solver-cost obligations separated explicitly.
