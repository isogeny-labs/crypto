# Paired projected adaptive transport

For the saved monomial source over caller-certified prime p, write p-1=q*m,
q=2^v2(p-1), m odd. This development implementation accepts q<=4. Its projected
transport uses each inherited monomial exponent e'=(m*e) mod (p-1).
Consequently the direct monomial is W^m. The coordinate-difference product is
D^m, the stored anchor square is (W(P)^m)^2, and each homogeneous recurrence
step represents the m-th power of its full-value counterpart. Common scaling
of a pair remains harmless. The -P anchor retains its sign because m is odd;
O, P and 2P anchor/reset behavior is inherited unchanged. [pcc:unverified]

The source computes the same geometry residual pair [G_N,G_D], raises both
coordinates to m, and multiplies by the projected monomial pair. Its calibration
is projected in the same way. On the inherited sign domain, calibrated signs
are unchanged. Unlike the algebra-only predecessor, this implementation performs
these operations inside the real adaptive source and its geometry tail.
The word-size/domain restrictions are inherited; no extension field is used.
[pcc:unverified]

The public paired wrapper contains both the projected and full-value sources.
Every evaluated initial/extension row must pass the original full-ratio sign
predicate, agree on sign and point, and satisfy the homogeneous power relation
against the full-value ratio. Skipped-sign steps advance both states without
evaluating either residual. The wrapper exposes no unchecked sign-only route.
This doubles parts of the development work; it is not a production speed claim.
[pcc:unverified]

We retain all geometry lanes, division-value nonzero checks, phase construction,
initialization limits, overflow guards and rejected-point behavior. In particular,
a zero projected exponent does not justify deleting a division-value or
coordinate-difference domain check. This implementation therefore does not yet
realize all factor-elimination opportunities in the preceding derivation.

Next: separate the remaining domain checks from eliminated factors, establish
which are implied by the existing subgroup assumptions, and investigate a
projected geometry residual. Removing the full-value shadow requires a sign-domain
proof or an independently sufficient validator. Complete-cost accounting must
include projection exponentiation and all residual/setup work.

Sources checked: feature-native-sign-projection/DERIVATION.md and its reviewed
Sutherland MIT18.782 Lecture3 sections3.1-3.2; predecessor adaptive monomial
source/transport/extra files and split-window tests. Standard cyclic-group facts
are reused; no novelty claim. KB unavailable in the current tools; the repository
wrapper was previously checked and lacked its executable. This is a direct
integration of the preceding construction, not a new literature survey.
