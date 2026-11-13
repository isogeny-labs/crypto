# G3 corrective model-transport contract

Frozen before corrected code or unit02, 2026-11-13. The original protocol394e14,
proposal report and all attempt01 evidence remain immutable. This addendum
corrects an unstated coordinate assumption; it does not drop a fixture or
weaken the requirement to preserve the exact historical plane curve.

## Actual failure and newly read source

Native unit01,7f48f6, has original101,all four tests fail before point testing
with `historical curve mismatch`,zero passed/ignored/filtered,all807 selected
hashes unchanged. The original release compile and complete raw tree are
retained. No full-point, group or ideal-product acceptance follows from this
run. Independently, Lean02,d9f9f6,passes the same nine named statements with
12 audited declarations including three repeatAdd helpers,zero trusted.
Those conditional/algebraic statements do not validate the failed model.
[pcc:unverified]

The original builder `research/ghs_cubic_genus3_build_20261012/genus3_build.rs`,
SHA256 a10f0d2d08fa4f82aaf9018f0988efa90d5bdfba3b2a3247763c7abe7b461552,
was additionally read at immutable de0b472. Its Fld::build chooses the first
modulus and first s for which multiplication by gamma=g+s cycles through
all nonzero elements. Its Moore rows use Zech encodings1,2,3,which mean
1,gamma,gamma². The current original Diem core uses the polynomial-basis
elements1,g,g². Freezing the cubic modulus alone therefore does not freeze
the same plane coordinates. This source distinction explains a candidate
cause of the observed mismatch; the actual polynomial identity must pass
before accepting the repair. [pcc:unverified]

The selected historical E0_weierstrass_ext fields are produced from binary
quartic invariants in that builder,not the root-fitted model of the separate
old quartic-IC adapter. The protocol's provenance wording about a retained
root-fitted model is corrected here. The new source short model still uses
the specified first quartic-infinity origin; both directions and independently
counted order remain mandatory. [pcc:unverified]

## Required repair and identity

Recover the original gamma choice deterministically in the frozen K using
the existing field engine and exact multiplicative-order criterion; take
the first s in0..q. Reuse an existing integer factorization routine for
q³-1. This is setup,not an input-point operation. Do not change the cubic
basis or reinterpret u coefficients. No fitted scalar table is permitted.

If W_g are the original core's coordinates,then the historical coordinates
are W_gamma=B_s W_g,where

B_s = [[1,0,0],[s,1,0],[s²,2s,1]],
B_s^-1 = [[1,0,0],[-s,1,0],[s²,-2s,1]].

These are inverse matrices over Fq,as follows by multiplication. Before any
Picard setup,expand the **entire actual historical quartic** at B_s W_g
with the original homogeneous-polynomial engine and require it to equal
a nonzero scalar times the entire core quartic. Do not replace this with
point sampling,matching orders or a j-invariant comparison. Keep the exact
historical target quartic normalized by a scalar and transport the actual
cover by omi_gamma=omi_g B_s^-1. Its rmat and source short model are unchanged.
If any polynomial identity fails,retain the failed attempt and investigate;
do not accept a merely compatible or unnamed isomorphic target.
[pcc:unverified]

The independent FpR6 oracle must construct the Moore rows directly from
gamma=theta+s and its square,using the explicit source-modulus root theta,
then verify them against the transported inverse. It must still check all
completed divisor schemes and all required generating edges/scalars. Add
a faithful cleared matrix-transport Lean lemma separately,retaining the
successful nine-statement Descent module and its original gate evidence.
Original rank,exceptional-point,domain and negative-control criteria stand.

This changes the new adapter's explicit model transport only. The original
Diem/Picard arithmetic bodies,old794 inputs and all six complete consumer
jobs remain unchanged. Unit02 receives a fresh native build directory,
the same four tests and15-minute limit,complete source/command/original
output/exit/finalguard custody,and a separate whole archive. No speed,
solver,global inverse or full geometric formalization is asserted.
[pcc:unverified]
