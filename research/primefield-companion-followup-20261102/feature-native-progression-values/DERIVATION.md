# Convolution evaluation of recurrence block progressions

T-1857, 2026-11-07. Producer derivation and deterministic native checks; unsigned evidence [pcc:unverified].

## Integration

The packed interval helper now detects batches of at least two points with its fixed nonzero step w modulo prime r. For these batches it lazily constructs ProgressionValues for the cached degree-w block polynomial. A nonsingular shift uses one low convolution. A mixed progression, singleton or zero denominator retains the original monic remainder-tree evaluator. Point-owner routing, request caps, interval products and coefficient recurrence are unchanged.

The new cached evaluator is per interval helper/coefficient stream. It is not shared across targets or advertised as free setup. The first eligible batch pays setup even if its shift subsequently uses the fallback. Original source/recovery/protocol/cohort remain unchanged.

## Scaled grid and cached weights

For polynomial P of degree at most d<r and nonzero step s, put H(X)=P(sX). Sample H(0),...,H(d). Production has d=s=w. Only d samples use a product/remainder tree; the final sample is computed by scalar Horner, preserving the d-point tree cap. Store

    W_i = H(i) (-1)^(d-i) / (i! (d-i)!),   0<=i<=d.

These are the usual Lagrange weights for integer nodes0..d. Factorial inverses use one checked inverse of d!, with a backward recurrence. The checked inverse of s is also cached. All factorial factors are nonzero because d<r. Persistent cached data is d+1weights and constant parameters; no base sample tree survives setup.

## Shift evaluation and proof

For first point x0, put a=x0/s. Request c values,1<=c<=d. If any element in a-d,...,a+c-1 is zero, return None and use the existing tree evaluator. Otherwise form

    S_j = 1/(a+j-d),  0<=j<d+c,
    Q = W*S mod X^(d+c).

Lagrange interpolation gives H(a+k)=Delta_k sum_i W_i/(a+k-i), where Delta_k=product_(i=0..d)(a+k-i). The sum is exactly coefficient d+k of Q. Compute Delta_0 from the same denominator prefix products, then update

    Delta_(k+1) = Delta_k (a+k+1)/(a+k-d).

The batch-inverted S supplies the inverse denominator already needed by that update. Thus the output is exactly H(a+k)=P(x0+ks), for0<=k<c. Canonical trimming of the low product is handled by reading absent coefficients as zero.

This is the arithmetic-progression Lagrange-convolution construction of Bostan-Gaudry-Schost, adapted to a bounded prefix of the progression and the existing low-product kernel. No novelty claim is made. Modular additions are reduced incrementally so offsets remain safe in the supported r<=u64::MAX/2 range. All native multiplication is u128 modulo r. Caller-certified primality and matching polynomial modulus remain inherited preconditions.

## Fallback and accounting

No singular denominator is inverted. None is an expected dispatch result, not an arithmetic failure. Mixed batches and singleton batches never initialize the progression cache. Actual inverse-check or polynomial errors return Arithmetic. The fallback works with repeated points and wrap because it uses monic remainder trees.

A cached evaluator costs O(M(d)logd+d) polynomial/scalar work, plus two explicit scalar inversions, to prepare: one d-point evaluation tree, one Horner sample, inverse step and inverse factorial. A nonsingular query uses one low product of length at most2d, O(d) scalar work and one scalar inversion, with O(d) auxiliary field elements excluding kernel workspace. Its polynomial/scalar cost is O(M(d)+d), versus the standard O(M(d)logd) evaluation-tree bound. In this implementation these three inversions use exponentiation by r-2, so each adds O(logr) modular multiplications. They are not free or omitted from the accounting. The cached block-polynomial construction is another earlier setup cost.

Fallback batches retain the previous cost. Packing can produce irregular batches, and small degrees may favor the old evaluator; no universal field-count or timing dominance is asserted. A workload crossover has not been measured. Production query point count remains at mostw, weights have w+1entries and the convolution/inverse buffers at most2w (prefix has2w+1). Temporary setup-tree storage remains O(wlogw), with kernel workspace separately charged. Source-grid, order-validation, first-C0 and coefficient-stream work remain.

The inherited `evaluation_batches` counter now counts flushes, not query-tree constructions. New counters separate convolution batches and fallback query trees; their sum must equal flushes. Setup trees are explicitly excluded from query-tree counts and charged once per initialized context. Inherited printed labels were changed from trees to batches in this new variant; prior immutable reports retain their original meaning.

## Exact native validation

`validation01/receipt.json` pins rustc,49transitive source inputs and stdout; overflow checks enabled. All native checks pass:

- 31,230 nonsingular convolution queries and2,538singular fallback queries, covering114,093exact output values. Primes7,17,97,257; degrees1,2,3,7,16 where admitted; zero and nonzero polynomials; steps1,2,r-1; every field shift; prefix lengths1andd. Direct Horner is the reference, with fallback additionally checked through the tree evaluator.
- 36integrated interval products across caps2,7,16;694convolution batches and10fallback query trees. These deterministic coverage counts are not workload rates. All outputs equal direct multiplication.
- A real r1009/m56 coefficient stream takes7convolution batches and14fallback query trees, plus its separately charged one-time progression setup. The21flushes preserve the packed schedule.
- Domain and zero/over-limit prefix controls pass. All inherited375request panels,4,820interval checks, coefficient/residue/state controls and public recovery panels pass.
- Public panels:281targets with14,421source calls and617targets with5,929calls, zero unresolved in both, matching reference outputs/routes/statistics. Original4,563-case cohort remains unchanged.

No native compile/assertion or report failure. Early output reads found no files while the original command was still preparing/running; its live process was inspected and awaited, without duplicate execution. No cloud/timing run or fullCargo rerun; historical unrelated missing-fixture/deadline limitation remains.

Replay into a fresh directory:

```sh
python3 research/primefield-companion-followup-20261102/feature-native-progression-values/validate-recovery.py /tmp/progression-values-replay-unique
```

## Sources and next step

KB unavailable: MCP absent and scripts/research-kb status reported no executable. Exact packed helper, coefficient stream and scalar low-product engine inspected. Primary source: Bostan, Gaudry and Schost, https://mathexp.eu/bostan/publications/BoGaSc04.pdf, Figure1, Proposition1, proof and progression scaling on pages3-5. The implementation checks the required nonzero denominators and uses the existing fallback outside those conditions.

Next reduce repeated inverse work or handle overlapping grid shifts directly from cached samples, while keeping exceptional cases exact and charging setup. New SVG/PDF documents this construction; historical and canonical graphs remain unchanged.
