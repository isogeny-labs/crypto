# Audit proof obligations and sources

Sources checked: frozen source5759530, complete result1b04550 and its
ACTUAL_H_BASIS_PROOF_A/DERIVATION_A; original C7 cover and degree9 sources
in their inheritance; Milne AV v2(2008), I.7, I.8 Remark8.12, IV.2Lemma2.1,
https://www.jmilne.org/math/CourseNotes/AVc.pdf; Stacks062D Lemma15.31.2,
https://stacks.math.columbia.edu/tag/062D. Primary text read in this
session. KB/Conductor tools unavailable, wrapper enters locked VLT1 and
must not run; exact-source fallback gives no novelty/absence clearance.
[pcc:unverified]

The independent verifier proves exact rank with two witnesses: a nonzero
r×r minor and n−r independent right nullvectors. A standard identity
submatrix on the free coordinates certifies their independence. Bareiss
determinant uses unnormalized cross-products and division by the previous
pivot; it is a different implementation from candidate normalized RREF.
All finite-algebra units are checked as invertible multiplication maps,
which works for both etale fibres and nonreduced jets. [pcc:unverified]

The regular sequence of two coprime conics has the (2,2) Koszul resolution,
so its degree2 and degree6 coordinate spaces have dimension4. Multiplying
by the actual quartic between those spaces has cokernel equal to its
scheme intersection with C. Opposite-divisor units and conic degrees8
make common degree3 sufficient to identify both residual degree3
divisors, hence the claimed rational-function divisor. Rank counts without
these full scheme/unit conditions do not suffice. [pcc:unverified]

Standard reduced prime-to-characteristic E[3] and quotient universal
property then give g=(phi0−phi1−phi3*pi)/3, as stated in the frozen written
proof. Native coefficient identities provide an additional check, not a
new proof of those algebraic-geometry theorems. Fix pi=omega−2 and
barpi=−omega−1, with omega2−omega+5=0. For the genuine cover Gram G,

 N=[[0,−1,1],[3,1,−1],[0,pi,−pi−3]],
 Tminus=[[1,1,0],[−(1+omega),0,−1],[2−omega,0,−1]].

Then beta=B N/3, N Tminus=3I and N dagger G N=9H. Check full matrices,
detH1 and integral adjugate independently in Z[omega]. The geometric
kernel certificate is what makes beta integral; rational matrix identities
alone cannot establish that. No genus3 divisor group is replaced by an
unjustified coordinate group. [pcc:unverified]
