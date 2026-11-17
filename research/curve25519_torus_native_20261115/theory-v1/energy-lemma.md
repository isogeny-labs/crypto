# Proposed average four-sum energy bound for shifted torus bases

Status: draft argument; independent mathematical review, precise primary
citations for the map-degree step, formalization and novelty assessment are
pending. Native arithmetic checks are not a proof of the geometric argument.
No new attack exponent or completed Curve25519 attack is asserted.
[pcc:unverified]

## Exact proposed statement

Let p=2^255-19, A=486662, E: v²=u³+A*u²+u, and assume the standard
#E(Fp)=8*ell with ell prime. Work in Fp2=Fp[sqrt(2)] and its norm-one
multiplicative torus T, of order p+1. Let M=60824497*213156431 and H<=T have
order M. Choose g in T with c=g^M satisfying c^d != +/-1 for d=1,...,8.
Let S be the disjoint union of g^j H, j=1,...,4, of size L=4M.
In particular S contains no two elements differing by a sign.
[pcc:unverified]

For sigma in T define S_sigma=sigma*S. Retain sigma only if
sigma^(2M)*c^r != +/-1 for r=2,...,8. Call this set of parameters U.
There are at most 28M excluded parameters, so |U|>=p+1-28M.
For each z in S_sigma use the unique rational Elligator branch on E, then
choose either orientation consistently and define X_z=[8]P_z in G=E[ell](Fp).
The chosen orientation is arbitrary; signed tuples make it immaterial.
[pcc:unverified]

Use signed ordered four-tuples of DISTINCT indices in S, before identifying
possible cofactor aliases. There are W0=16*L*(L-1)*(L-2)*(L-3) such tuples.
Let r_sigma(Q) count tuples with sum of their signed X_z equal to Q, and
C_sigma=sum_{Q in G} r_sigma(Q)^2. The proposed bound is

    average_{sigma in U} C_sigma
      <= 24 W0 + K W0^2/(p+1-28M),    K=2^28.

Native exact arithmetic checks K*W0 < p+1-28M for these parameters. If the
geometric argument below is sound, the average is therefore <25 W0.
This is an average over the base randomizer, not a certificate that the
currently fixed sigma=1 base has small energy. [pcc:unverified]

## 1. Six branch points and disjoint shifted covers

Put q=(A-2)/(A+2). Native checks establish that q is a nonsquare in Fp.
Since p=5 mod 8, choose i in Fp with i²=-1 and r in Fp2 with r²=q.
The even reciprocal polynomial

    B(z)=(z²+1)*(4*(z²+1)²-A²*(z²-1)²)
        =(4-A²)*(z^6+1)+(12+A²)*(z^4+z²)

has six distinct roots +/-i, +/-r, +/-1/r. Their norm values, by opposite
pairs, are -1, -q, -1/q; these three values are distinct. Consequently a
ratio of two branch points has norm one only when that ratio is +/-1.
This is checked on all 36 ratios in the arithmetic record.
[pcc:unverified]

For alpha in S, consider the double cover

    C_alpha: V_alpha² = -2 A B(alpha*sigma)

of the sigma projective line over the algebraic closure of Fp. For distinct
alpha,beta in S their branch sets cannot meet: an intersection would give
alpha/beta as a norm-one ratio of branch points, hence alpha=+/-beta.
This contradicts distinctness and S intersect (-S) being empty.
Any nonempty product of these radicands has a simple zero unique to one
factor, so it is not a square. For k distinct indices the compositum is
therefore a degree 2^k multiquadratic function field with automorphisms
flipping each V_alpha independently. [pcc:unverified]

## 2. Two elliptic maps and a constant collision bound

For z=alpha*sigma and D=z²+1 the individual genus-two cover maps to E by

    u_+(z)=-A*(z+1)²/(2D), v_+=(z+1)*V/(4D²),
    u_-(z)=-A*(z-1)²/(2D), v_-=(z-1)*V/(4D²).

Substitution using V²=-2 A B(z) gives v_+²=u_+³+A*u_+²+u_+ and the same
for the minus branch. Each map has degree two: the map to the u-line has
degree four on the double cover and E has degree two over its u-line.
The maps extend to the smooth projective normalization because E is proper.
On the compositum of k covers each such map has degree 2^k.
[pcc:unverified]

Take two signed ordered four-tuples that are not permutations of the same
signed index multiset. Group equal indices and cancel opposite coefficients.
There remain k<=8 distinct indices and nonzero integer coefficients a_j with
sum |a_j|<=8. For any fixed choice of the plus/minus elliptic map at each
index, the sum F=sum a_j*Phi_j is nonconstant. If it were constant, flipping
V_j would imply [2a_j]Phi_j=O, impossible for a nonconstant elliptic map.
The actual rational branch at a finite sigma may vary, so ALL 2^k choices of
elliptic map must be allowed; square-root signs are already in the cover.
[pcc:unverified]

The main imported geometric degree fact is

    deg(F) <= (sum |a_j| sqrt(deg Phi_j))² <= 64*2^k.

It follows from the parallelogram identity for pullback degrees to an elliptic
curve, positivity, and Cauchy-Schwarz. This step must be independently checked,
including inseparability and constant-map cases. Multiplying by 8 multiplies
the degree by 64. Thus [8]F=O holds at at most 4096*2^k points on the smooth
cover, and hence at most that many distinct base values sigma. Allowing the
2^k choices of elliptic map gives at most 4096*4^k <= 2^28 parameter values.
This counts even non-rational torsion in the fiber and is deliberately loose.
[pcc:unverified]

No height-heuristic, finite-field point-count heuristic or observed solver
behavior is used in this proposed collision argument. Its gaps are the
review/formalization obligations stated above, not a claimed completed proof.
[pcc:unverified]

## 3. Count collisions and transfer planted success

For each tuple exactly 4!=24 ordered tuples have the identical signed index
multiset. All remaining pairs are covered by the constant collision bound.
Summing their indicators over U gives the proposed average-energy inequality.
Cofactor aliases at individual parameter values count as collisions; they
are not assumed absent. Zero target sums are included in C_sigma.
[pcc:unverified]

For any budgeted oracle whose input is only the public target and the base,
let a_sigma(Q) be its probability of returning a CHECKED decomposition.
Set a_sigma(O)=0. Its planted hit rate and uniform-target yield are

    s_sigma = sum_Q r_sigma(Q) a_sigma(Q) / W0,
    y_sigma = sum_Q a_sigma(Q) / ell.

Weighted Cauchy-Schwarz gives

    y_sigma >= s_sigma² W0² / (ell C_sigma).

A sampler must hide planted index/sign labels from the oracle; all setup,
failed attempts and checking work is charged. This inequality does not apply
to an oracle given the generating tuple or its reproducible labels.
[pcc:unverified]

If average_sigma s_sigma >=3/4 and the average-energy bound is <25W0,
then at least half the parameters have s_sigma>=1/2, and at least three
quarters have C_sigma<=100W0. At least one quarter satisfy both. Each such
base has uniform yield at least W0/(400ell). For nonzero uniform targets
the yield is no smaller, since a_sigma(O)=0. This does not assert that the
current oracle meets the planted-hit assumption. [pcc:unverified]

## 4. What would move the exponent

After normalizing cofactor aliases, the number B of factor-base +/-classes
is within a constant factor of L: ceil((L-8)/8)<=B<=L. Under the additional
independent-row/rank assumption, a budget D=soft-O(B^gamma) with constant
planted hit on a constant fraction of randomized bases gives relation work
soft-O(ell*D/B^3). Sparse linear algebra costs soft-O(B²); enumeration,
indexing, cofactor transport, query generation, descent and parameter search
must be charged as well. Balancing only when these other phases are bounded
by those terms gives exponent 2/(5-gamma), for 0<=gamma<1.
[pcc:unverified]

This is a conditional reduction and a research target. Solver gamma<1,
row rank, a growing admissible curve/base family, its density and generation
cost, complete phase costs and independently checked measurements remain open.
The native 454-variable construction does not establish any of them.
[pcc:unverified]

## Evidence and questions for independent review

- Native source: tools/torus-s4-tree, sharing tools/torus-s4 arithmetic.
- Arithmetic record: six roots, 36 norm ratios, exact M/order/coset checks,
  and exact K*W0 < p+1-28M. It validates arithmetic only.
- Known S3 coordinate identity has a native identity certificate and exact
  integer coefficient check; the two elliptic-map identities are to be added.
- Primary source read for the degree step: Milne, Abelian Varieties,
  CourseNotes AV.pdf, section 5 theorem of the cube/square. The pullback-degree
  derivation needs a precise citation and review; no Lean proof is supplied.
- Check most aggressively: branch choice bookkeeping, map degree on the
  multiquadratic cover, trivial-pair count, admissibility count, and the
  passage from raw indexed tuples to normalized prime-group relations.
- Novelty has not been established by a literature search.
