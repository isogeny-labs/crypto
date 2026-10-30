# Strict degree-four falls against complete retained V3 spaces

The full objective remains n83 Koblitz, general binary and prime, m2--9, with
actual fixed point bases, FFD/LFD evidence and eventual bases that reduce PDP
degree of regularity. This phase preserves every original dataset and selects
the twelve completed m3 V3 spaces (original, paired baseline, spike40;
Koblitz/binary; planted/natural). It changes no base, target or input equation.
The previous goal turn reconciled existing artifacts but added no new algebraic
evidence; this phase must produce new strict-membership evidence.

## Literature and question

Semaev, *New algorithm for the discrete logarithm problem on elliptic curves*,
section4.5, https://arxiv.org/html/1504.01175, gives the characteristic-two
identity deg_F2(X*S3(X,Y,U))<=3 although the admitted multiplication degree is4.
This is an established candidate construction, not a novelty claim. It alone
does not prove nonmembership in the V3 of the full chained system.
Caminata--Gorla, https://arxiv.org/html/2112.05579, Definitions1.5/2.6 and
Theorem2.8, give the strict-fall criterion. Kosters--Yeo,
https://arxiv.org/html/1503.08001, distinguish first falls from solving behavior
and delimit fixed-target trace consequences. No first-fall heuristic, trace
augmentation or global stabilization assumption is used here.

## Exact membership argument

Let f_0,...,f_82 be the unchanged first83 coordinate equations and set
F=sum z^r*f_r in K[v], K=F_2[z]/(z^83+z^45+z^2+z+1). Verify their complete
field-coefficient polynomial against the saved S3(X0,X1,U), including all lower
terms and the curve constant. X0 and X1 use the saved27 selector weights;
U uses the full83-element polynomial basis. The third selector block is not a
multiplier in this particular family. All164 Boolean field equations must exist.

For each L in {X0,X1,U}, construct the ordinary polynomial L*F first. Every
coordinate is a sum of linear multiples of original equations of degree<=3,
so belongs to V4. Reduce repeated variables with v_i^2+v_i. Each correction
has degree at most4, hence its coordinates also belong to V4. Assert that the
result has degree<=3 after exact coefficient cancellation. Retain the ordinary
field polynomial, its multilinear normal form and deterministic source/multiplier
provenance. Expand all83 coordinate polynomials. No cubic is multiplied inside
the V3 engine, and no degree-three claim is inferred from Boolean reduction alone.

Replay the complete saved V2, the immutable V3 prefix and its completed suffix,
including all operations, row bytes, verdicts, cap boundaries and the exhausted
queue. Compare each candidate against this complete V3 using deterministic
highest-pivot reduction. Retain the candidate row, every reducer ID and the
remainder. A nonzero remainder with a pivot absent from V3 proves nonmembership.
Consequently g in V4 intersect R_{<=3} outside V3 proves a strict degree4 fall
and LFD>=4 for that exact saved system. A zero remainder only records membership.

Evaluate all249 candidates per case, even after a witness is found. Allzero
remainders mean no strict witness in this named finite family; they do not prove
V4 complete, LFD<=3, any global upper bound, or equality of base quality. Count
strict candidates, but do not treat them as independent samples. Any inherited
lower bound3 is labeled inherited. Exact LFD and global upper bounds remain null.

## Controls and verification

Use independent tiny Boolean mask arithmetic for exhaustive V3/V4 closures.
The cubic xyz+1 gives a strict degree4 consequence after multiplication by x;
xyz alone gives no strict witness in this multiplier family. Compare the generic
ordinary-product/normal-form construction and dense-row membership detector with
these independent spaces. Test selector/field coefficients, source matching,
quartic rejection, ordinary-degree guards, row/reducer/remainder tampering and
incomplete-parent rejection. Preserve all development failures and control files.
The frozen predecessor source remains byte-identical; a separate native binary
extends its verified replay to return the complete rows without recomputation.

## Execution and preservation

Freeze source, executable, protocol, all12 input/base identities and both parent
archives before production. One native process,8GiB virtual memory,1800 seconds
per producer or fresh replay plus5 seconds termination grace; attempt all12
cases without automatic retry. Flush candidate records and progress after each
candidate. Preserve errors, caps and partial outputs without assigning a degree.
Wall times are diagnostic only. There are no speed or independent-sample claims.

The first EKS inventory request timed out. Its recheck succeeded and showed no
duplicate n83 execution; two other owners' Semaev jobs were active. The preceding
n83 storage probe remains failed with unconfirmed application cause. Retain both
inventory responses and use the recorded existing-host fallback. Never infer
that a timed-out inventory means no jobs exist. No IAM or capacity changes.

Store all new raw data, source, executable, controls, protocol, logs and format
map in S3 and Git/content-addressed storage. Existing parent archives are
referenced and retained, never rewritten. Fresh-process and S3-restored replays
must reconstruct every candidate and reduction, verify both parent trace layers,
and match every output byte. Publish data before the inspected report. Producer
replay remains distinct from independent scientific review.
