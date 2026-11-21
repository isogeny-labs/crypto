# Cached grid samples remove singular progression fallback

T-1859, 2026-11-07. Producer derivation and deterministic native checks; unsigned evidence [pcc:unverified].

## Construction

The prior progression setup computed H(0)..H(d), formed Lagrange weights and discarded the sample values. The new evaluator retains that already-computed vector. It normalizes the first point to a=x0/s and walks the requested cyclic indices a,a+1,... modulo r, for1<=c<=d<r.

Indices in0..d are copied from the sample vector. A contiguous outside-grid segment is evaluated by the existing nonsingular convolution kernel. The kernel accepts normalized a internally, avoiding redundant normalization in the segmented path. It remains available through the old optional evaluator as a control. No special-case approximate values or division by zero is introduced.

Every eligible arithmetic-progression batch now returns exact values for every canonical shift. Mixed point batches and singleton dispatcher batches still use the original tree evaluator. Point-owner routing, source calls, request caps and recovery logic are unchanged. Counters distinguish progression batches, actual convolution calls, cached copies, cache-only batches and fallback query trees.

## Segment lemma and nonzero denominators

Let C={0,...,d} be the cached arc and U={d+1,...,r-1} its complement in the cyclic scalar field. The requested arc has c<=d points and wraps at most once. It cannot contain two disconnected U segments: that would require traversing the entire cached arc of d+1points between them, already longer than the request. Hence there is at most one U segment, with at most two cached segments.

For an outside segment starting at b and having t points, b>=d+1 and b+t-1<=r-1. The convolution denominator interval is b-d,...,b+t-1, a subset of1..r-1. It therefore has no zero. The existing Lagrange-convolution identity applies without a fallback. Cached values equal H at their indices by setup. Concatenating the segments preserves order and gives all requested H(a+k), including wrap.

The implementation asserts at most one convolution and treats an unexpected None from the normalized kernel as Arithmetic. Its index increments stay<=r before reduction, avoiding overflow under the existing native field bound. Empty, over-degree and noncanonical requests fail. Prime r and matching polynomial-engine modulus remain caller-certified preconditions.

The old evaluator rejects a shift exactly when the requested progression intersects the cached grid. Thus the former singular cases are now either entirely cached or a cached/outside mixture. No query tree is needed for any valid progression shift. This statement does not remove the dispatcher tree path for irregular or singleton batches.

## Work and storage

Retaining the setup samples adds exactly d+1persistent field elements (plus Vec metadata) per initialized context; it adds no sample evaluations or inversions during setup. The same setup tree, extra Horner sample, inverse step and inverse factorial remain charged.

A cache-only query performs one normalization field multiplication, index work and O(c)copies. It performs no query polynomial multiplication, scalar inversion or query-tree construction. A mixed or outside-only query performs at most one convolution and one scalar inversion, plus O(c)copies/index work. Its polynomial/scalar bound remains O(M(d)+d), with the native O(logr) exponentiation cost for the one inversion separately charged. The extra sample vector and output are O(d); kernel workspace remains separate.

The first constraint, source grid, coefficient generation and mode-order validation remain unchanged. Cold setup costs are not amortized away. No timing crossover, universal total-arithmetic dominance or workload-prevalence claim is made.

## Deterministic checks

`validation01/receipt.json` pins rustc and50transitive source inputs, with overflow checks enabled. `validation01/check.stdout` records:

- 50,652 total-shift queries and167,769exact values compared with independent direct Horner. Primes7,17,97,257; degrees1,2,3,7,16 where admitted; zero/nonzero polynomials; steps1,2,r-1; every field shift; lengths1,ceil(d/2),d.
- 3,780queries rejected by the old optional evaluator are answered exactly:2,214cache-only and1,566mixed. There are15,579copied values and981wrapped queries. Every query asserts its exact copied count and zero/one convolution count.
- Integrated request control exercises a former singular cache-only batch and an irregular fallback; interval products still equal direct multiplication. Preflight/domain controls pass.
- The inherited36product panel now has700progression batches and4fallback query trees; these are deterministic coverage counts, not workload rates. The previous immutable panel had694nonsingular convolution batches and10query trees. Progression-batch count is not actual convolution count after this change.
- The real r1009/m56 coefficient stream has no cached overlap in this panel:7progression batches,7convolutions,0cached values and14query trees. This control is unchanged at its working scope.
- All inherited request, interval, coefficient, residue and sequential-state controls pass. Public panels remain281recoveries/14,421source calls and617recoveries/5,929calls, zero unresolved and exact reference routes/statistics.

No native compile/assertion or report failure. Known scalars and root searches remain checker-only. Original4,563-case cohort and prospective trace protocol unchanged. No cloud/timing run or fullCargo rerun; historical unrelated missing-fixture/deadline limitation remains.

Replay into a fresh output directory:

```sh
python3 research/primefield-companion-followup-20261102/feature-native-cached-shifts/validate-recovery.py /tmp/cached-shifts-replay-unique
```

## Sources and next construction

KB unavailable: MCP absent, repository wrapper reports no executable. Inspected exact progression evaluator and packed dispatcher, and Bostan-Gaudry-Schost2004 Proposition1 and its denominator conditions, https://mathexp.eu/bostan/publications/BoGaSc04.pdf. The convolution itself is the established formula; the cyclic segment lemma explains how the retained samples cover its exceptional shifts. No novelty claim.

Next reduce inversions across independent progression batches or retain reusable evaluation context across coefficient streams under an explicit shared-memory cap. Setup and ownership must stay charged. This report's SVG/PDF illustrates the new route; historical and canonical graphs remain unchanged.
