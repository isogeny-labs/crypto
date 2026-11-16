# G3 ramification coverage correction

Frozen on 2026-11-13 before changing the fibre test or running unit03. The
original protocol394e14, model addendum6a3cfe and failed unit02b8e858 remain
unchanged. This addendum corrects the expectation that each complete cubic-field
source group contains a ramification image. It adds geometric checks without
replacing any primary fixture or reducing any complete-input comparison.

## Observation and sources

Unit02 matches all 148 q=5 source fibres against the independent Fp6 point and
jet oracle, then fails `ramified > 0`. It never reaches the q=7 or q=11 fibre
loops; the other three tests pass. This is a failed complete-unit attempt.
[pcc:unverified, b8e858]

The frozen all-nine historical-case panel6b2a53 was run using the original
`isogeny_algos` FpR3 field, polynomial and square-root routines. Native diagnostic
69e6a0 exits0 with all810 selected hashes unchanged. For the primary q=5 case,
the two u1 roots are [2,2,0] and [2,2,1]. The corresponding u0*u2 values are
[3,1,1] and [0,1,4], with norms3 and2 in F5; both have Euler value -1 in K.
Consequently neither ordinate lies in K. Both acquire square roots in Fp6.
The primary q=7 and q=11 cases each have four K-rational ramification images.
[pcc:unverified, 69e6a0]

The underlying cover is the same Ciani degree-two map from the original
`src/cryptanalysis/diem_quartic.rs`, and the local oracle is the existing
plane-quartic point/jet construction. Additional primary code read for this
correction: `isogeny_algos/src/series.rs`, specifically the original truncated
series inverse; no duplicate field, polynomial or inverse-series engine is
introduced. Earlier literature and source receipts remain in
`evidence/genus3-primary-01/sources.json`. KB remains unavailable in this runtime
as recorded there; this correction makes no novelty or performance claim.

## Revised coverage contract

1. Retain the original three primary fixtures, all four substantive tests,
   every source point, every complete generating edge and every odd-subgroup
   scalar. Retain all ideal-product, origin, gauge, auxiliary-reducer,
   coordinate, group and strict-input controls. No ignored or filtered test.
2. Replace the false per-field positive rational counter with exact expected
   counts q=5:0, q=7:4, q=11:4, checked by the complete original source loops.
3. Independently find both u1 roots over Fp6 in each case and both nonzero
   ordinates y=+-sqrt(u0*u2): four geometric branch images per case. Check u0,
   u2 and the derivative of u1 are nonzero, and the image is on the actual
   completed elliptic model. Check its K-rationality counts are exactly0/4/4.
4. At each image build its cover point (X,Y,Z)=(0,y/u0,1), transport through
   the independently constructed historical Moore matrix, and require the
   exact historical plane equation to vanish. Construct a length-three smooth
   local jet using the existing oracle. Transform the jet back to Ciani
   coordinates using the actual inverse Moore matrix and evaluate the original
   cover t=s1/s0 using the original series inverse.
5. Require t(0)=t0, t's linear coefficient zero and quadratic coefficient
   nonzero; require X's constant coefficient zero and linear coefficient
   nonzero. Require L=Y-(y/u0)Z and Q=X^2 to vanish modulo the square of the
   local parameter, with Q's quadratic coefficient nonzero. The fibre
   therefore retains its doubled point rather than only its reduced support.
6. Print actual geometric branch counts and jet witnesses in the native log.
   These extra geometric checks do not extend the public runtime input domain
   from E(K) to E(Fp6).

Fresh strict all-target Clippy and an unfiltered four-test release unit03 run
with a fresh external cache remain required. Preserve each original terminal,
all source guards, raw members, whole archive and failure if a check rejects.
Local correctness build times are not performance measurements.
