# Curve25519 coefficient discovery: candidate algebra versus solution algebra

- **Status:** investigating
- **Kind:** algorithm/proof gap; complete-cost gap; separate artifact access dependency
- **Affected work:** [original pinned problem](curve25519-section-coefficient-selection.md), [spectral assessment](../research/curve25519_torus_native_20261115/coset-spectrum-v1/REVIEW.md), [qualified compiler](../research/curve25519_torus_native_20261115/section-compiler-v1/NEXT.md)
- **Next action owner:** existing Curve25519 coefficient task / PR5052
- **Last updated:** 2026-11-23

## Problem

A public-input coefficient root-discovery algorithm with a complete growing-
family cost/yield bound is still missing. A dimension6 candidate quotient is
not the quotient algebra of all unknown coefficient solutions. Symmetric
encoding, low equation degree and small candidate operators alone do not
supply that algorithm. [pcc:unverified]

Keep the original p=2^255-19,A486662,q213156431,a60824497,k20633 contract,
the original row/index/sign aliases and common-column/private-tag/rank/
recovery requirements. The new proposal quantifies six bounded UNKNOWN
coset labels and retains five unknown normalized section coefficients. It
does not replace the huge original base by a q1 or small-column fixture.

## Evidence and uncertainty

- **Implemented earlier:** supplied-section query, coefficient predicate and
  guarded quadratic compiler, including fixed original-q/k20633 indexed
  replay. Native7f4e38 and earlier evidence remain producer-unverified.
  The original November17 implementation TODOs are historical, not current.
- **Written:** the guarded spectrum/reducedness equivalence in the linked
  review. The native spectral compiler and full quotient/curve Lean bridge
  have not been implemented. The finite Lean counterexample concerns only
  the explicit2x2 ZMod101 matrix.
- **Specific invalid inference:** treating the6x6 candidate multiplication
  matrix as an FGLM multiplication matrix of the entire solution ideal.
  The latter dimension D and construction cost are unknown. A guessed D=6
  is not an instance of the cited sparse-FGLM algorithm.
- **Missing:** a concrete costed coefficient-elimination step, public-only
  unknown-root qualification and original projection, matched point-circuit
  comparison, complete setup/work/yield/peak-memory/family analysis.
- **Separate access dependency:** genuine trusted publication/replay of the
  preceding117571164-byte full archive. This blocks raw publication, not
  the present algebraic assessment or independent source development.
- **Not inferred:** low solving degree, gamma, a full-base solve, signed
  standing, novelty or a universal negative result. [pcc:unverified]

## What would unblock it

- [ ] Specify an executable elimination operation that takes only the public
  target/specification and limits, without known coefficients, labels, scalar,
  candidate list or witness. Charge its complete construction and failed work.
- [ ] Bind guarded spectral extension/projection to the unchanged original
  predicate. Include nilpotents, repeated coset labels, original raw aliases,
  pole/identity/target/sign controls and original indexed/group replay.
- [ ] Qualify actual unknown-coefficient discovery on a declared blinded
  finite domain. Compare the existing point target circuit on the same inputs;
  distinguish finite exhaustion, unsupported extraction, cap and incomplete
  observations. Infrastructure failure is not a mathematical negative.
- [ ] Supply a complete growing-family useful-yield/rank/recovery/memory
  bound meeting the original criteria. Under the explicitly conditional
  six-sum/constant-yield model B=ell^(1/6), the relation phase needs complete
  query exponent gamma<2. This is not an asserted yield theorem.
- [ ] Independently validate and formally bind the actual algebra/compiler,
  then complete the separate raw publication and downstream solve gates.

These are acceptance conditions for this route. Passing a finite comparator
alone advances its component; it does not meet the family-cost conditions.

## Next discriminating step

- **Action:** write the proposed elimination operation before implementing a
  new large compiler. A proposal consisting only of `FGLM on the6x6 matrix`
  fails the object/algorithm contract.
- **Controls:** same public instances in original point and section encodings;
  no planted tuple in the query API; nilpotent and repeated-label countercases.
- **Limits:** no new campaign, timing, paid job or huge-base enumeration is
  scheduled by this record. New source/run work needs its own versioned
  prospective qualification under the standing execution rules.
- **Outcomes:** correct public roots advance root discovery; complete matched
  costs allow comparison; caps/incomplete bases leave the problem open. A
  necessary overhead lower bound plus a declared family can reject a named
  implementation, whereas an unsuccessful sufficient upper budget cannot.
- **Recheck trigger:** a concrete elimination construction, a faithful full
  semantic proof, or new source-bound public-input root evidence.

## Resolution and history

Unresolved. The old problem is a subject of PCC-20261117-cd9874 and remains
byte-identical. This companion updates the SAME obstruction without replacing
its evidence or retroactively changing its assumptions. The original uniform-
coefficient sampling rejection remains scoped to its stated model. The new
spectral representation and solver-dimension assessment do not resolve the
constructor gap. [pcc:unverified]
