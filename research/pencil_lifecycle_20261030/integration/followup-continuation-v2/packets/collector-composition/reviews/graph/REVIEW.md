# Independent signed graph algebra review

Reviewer /root/monic_review. 2026-11-01. Prospective source-first plan recorded before GraphAlgebra body access. Source three_leads.rs find/constraint/anchor/edge, lines 1558-1712, and Ring arithmetic lines 1491-1530.

## Findings

Nine main algebra declarations match the scoped source operations. Potential evaluation is node=s*root+a+b*k. Compression composes this affine map with the parent's map; substitution in compression_sound is legitimate and does not assume a post-compression forest. Edge reduction subtracts both signed endpoint offsets, matching native subtraction. Union forward solves the reduced edge by multiplying by cu, requiring cu^2=1. Union backward retains s=-cu*cv and applies -s to offsets, exactly matching the opposite-size source branch. Both sign premises imply the new union sign is involutive. Reversal is an involution under its stated hypothesis.

Fixed-child propagation is correct: child=x+y*k and child=s*parent+a+b*k imply parent=s*(x-a)+s*(y-b)*k. This transports the meaning of a known anchor. The theorem does not prove the mutation or native Some/None policy of anchor. Source anchor, when already fixed, compares two known affine expressions and submits their difference to constraint; storing a first anchor does not itself determine k. Same-root cycles and final division are expressly excluded by the producer report, so their absence is not a defect in the scoped nine-lemma claim.

No mathematical or sign-orientation defect found. The abstract CommRing formulation is sufficient for these algebra identities; it correctly avoids unjustified field inverse assumptions. Native correspondence remains partial: signed(sign,a) interprets any sign unequal to 1 as negation, so i8 values must actually be +/-1 for the ring-sign reading. Scalar correspondence for graph nodes is an external premise and is not created by these identities.

## Fresh verification

GraphAlgebra, GraphControls and GraphReadback rebuilt in an isolated snapshot. Nine main declarations and nine producer controls passed; separate readback returned 18 declarations. Six independent controls also passed and were read back: all four union sign combinations with both orientations; positive fixed-child transport; wrong anchor-offset sign rejection; non-involutive sign reversal rejection; a consistent anchor-difference equation; and equal-anchor zero-coefficient case. All 24 passing axiom readbacks use only propext and Quot.sound. No fresh failures occurred. Producer failures remain preserved in their original packet; this review does not alter them.

## Missing native obligations

- Parent forest acyclicity, valid indices, root self-parent and root-identity potential invariants; find recursion termination and path-compression mutation preservation.
- Closure of i8 signs to +/-1, all modular offsets canonical, r prime>2, and node-value interpretation.
- Native Ring::add requires addition headroom (e.g. 2r-2<2^64); constructor primality alone supplies no such bound. Ring::neg also needs canonical input to avoid underflow. Ring::mul uses u128; its canonical output and modular interpretation must be composed separately.
- Size/operation/cycle counter headroom and memory safety across node creation/merging.
- Fixed-root metadata policy after union, root swap, and path compression; stale nonroot anchors must not be read as authoritative roots.
- Same-root positive/negative-cycle branches, inverse of +/-2, constraint's b=0 consistency assertion and b!=0 inversion/returned answer soundness.
- Actual compiled Rust equivalence and complete matched execution costs.

These are separate tasks rather than inferred correctness from local identities. No scientific execution occurred.

## Workflow

Separate reviewer, prospective local source-derived plan, independent sign checks, fresh module replay and axiom checks completed. No sibling reports read. Plan not precommitted; stable producer files not published immutably at dispatch. Imported local helper binaries and cached Mathlib remain a trust boundary; no full dependency rebuild or separate kernel checker. Multi-reviewer independence gate and coordinator publication are separate. Model fidelity faithful_scoped; native mutable graph correctness partial/not established; no official transition.

Primary references: native source and producer modules/report, with the affine signed-potential invariant independently derived from source before proof-body inspection. Standard ring substitution underlies these identities; no novelty claim.
