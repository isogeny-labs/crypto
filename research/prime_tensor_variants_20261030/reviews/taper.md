# Sharper mode weights and nonlinear Hermite tapers

2026-10-30 UTC. Pure mathematical follow-up by `/root/tensor_taper_math`.
No contraction, ECDLP solve, timing experiment or numerical research computation
was run. No new sub-exponential algorithm or certified novelty is claimed.

## Sources and the exact question

Read the pinned October25 manuscript, Theorem D.1 and all of Appendix D.4,
especially D.4.3-D.4.6 (printed pp59-63), from
`/workspace/audit/tensor-primary/openai-oct25.{pdf,txt}`. PDF SHA256:
`5b1612dc2eebb2cad5b4cbe62d1fed2982f01ab1908c26f70010a651487e8123`.
Also read the existing repository's contraction-cost/precision reviews and
the native Hermite-unit protocol. These already verify the source's linear
taper and weighted-partition upper bound. The present question is whether
two discarded inequalities, or a different taper shape, change its exponent.
This is not a full literature novelty search for the counting arguments below.

At an active call use the source notation

    L=2^a, 0<=i<u<=a, D=L/2^i, A=a+1-i,
    S={-L,...,L-1}, alpha=2^(a+1).

Write r_grp=64 for the source group count. The source has at most6M/D mode variables of each ordinary weight b+1 and
precision p_i=O(MD/u). Its weighted-partition bound is
exp(O(M/sqrt(u))). The product has at most M row factors, so every surviving
mode monomial also obeys sum(beta)<=M, with corresponding packet/group caps.

## 1. The sharper coefficient weight is valid

For a crossing row/tile pair put nu=v2(t-y0+x)<=i. In D.12 the first coefficient
has valuation at least a+1-nu, and the ratio raised to the b-th power has
valuation i+1-nu. Thus the stronger individual coefficient bound is

    a+1-nu+b(i+1-nu) >= A+b.

For unit interpolation parameter zeta and a row residue attaining nu=i,
the first factor has exact valuation A and the ratio exact valuation1.
The common usable mode weight can therefore be increased from b+1 to A+b.
No homomorphic canonical-lift assumption is used in this calculation.

This permits safe constant/finite-parameter pruning. It does not by itself
change the weighted enumeration exponent in the source's parameter regime.

## 2. A binary band survives both the shifted weights and degree caps

The following explicit family shows that those inequalities alone cannot
give a universal exp(O(M log(u)/u)) tuple-count bound.

Use full packets with P packets and M=PL real edges. Choose integral
parameter families where every ordered full-label pair has the same real
edge count, a multiple of L. For a connected realization take K_{N,N}
with M=N^2, assign the K=r_grp^a full labels equally on each side, and
choose powers of two so M/(K^2) is a multiple of L. Every pair-label class
then has exactly M/(K^2) edges: load variance is zero. Its degree N=sqrt(M)
is below the source Delta ceiling for sufficiently large root parameters.
Source-style uniform padding therefore leaves every packet full. Its group marginals are exactly
balanced. At least(1-1/r_grp)P>=P/2 packets have unequal coordinate-i symbols.
Each such packet has T=2^(i+1)=2L/D nonempty column tiles. These integral
families are compatible with the source's large-size range (for example
power-of-two M can be divisible by L*r_grp^(2a)). The graph realization is
only an admissibility witness; it is not a hard-instance runtime claim.

Put U=ceil(sqrt(u)), and assume

    D>=32 A U,  U>=1.

Choose W=floor(D/(16U)). Then W>=D/(32U), W>=A, and consider only modes
whose expansion indices b are W,...,2W-1. Give each selected mode exponent
either0 or1 independently. There are at least

    K=P_cross*T*W >= M/(32U) >= M/(64 sqrt(u))

such binary choices, and at most M/(8U) of them. The shifted weight of every
selected mode is at most A+2W-1<=3W. Consequently every binary subset has
total shifted weight at most

    3 P T W^2 <= 3MD/(128 U^2) <= 3MD/(128u).

This is below the literal source precision, whose8hD term alone is at least
24MD/u. More generally it is below c*MD/u for any fixed c>0 after replacing
16 by a sufficiently large fixed constant in W.

The per-packet degree is at most T W<=L/(8U), so the packet cap L and
the global cap M both hold with substantial slack. Even the natural
target-row support caps do not remove the band: each source tile has D/2
target rows in the residue class nu=i, whereas W<=D/(16U). The modes for a
tile can therefore be assigned to distinct rows in that class. Splitting
by the two target groups only reduces these already bounded counts.

There are at least2^K=exp(Omega(M/sqrt(u))) feasible binary tuples. The
source's upper bound remains exp(O(M/sqrt(u))) since imposing extra
constraints can only decrease its count. Thus the maximal tuple count
allowed by the displayed sharpened constraints has the same exponent class.

The condition D>=32 A ceil(sqrt(u)) holds eventually at every active source
node: a=O(log s), u>=log(s)^(4/5), and D>=2^(a-u)>=2^u under the source's
cumulative a schedule. All floor/divisibility effects can be absorbed by
constant-factor padding or restricting to the infinite integral families.

**Scope.** This is a lower bound for the admissible candidate-index set, not
for nonzero coefficients after cancellation or for all contraction methods.
Graph admissibility does not establish tensor hardness. Actual tensor entries
could factor or cancel, and a different algorithm could exploit that structure.
Additional algebraic relations among the modes, cancellation,
Boolean-source identities, or an instance-sensitive decomposition could
still be exploited. The conclusion is specifically that the additive gap
and degree caps alone cannot justify a better universal enumeration bound.

## 3. A maximum-principle obstruction for arbitrary taper shapes

Retain the source's full-polynomial Hermite interpolation and level support.
Let q_i be any nonnegative integer multiplicities for0<=i<u, with q_i=0
for i>=u. Monotonicity is not assumed. At a node of valuation i, D.10 is
valid independently of the linear taper, and gives

    B_i := -v2(E_t(0))/D_i
         = sum_(j=1..a-i) (j/2^j) (q_i-q_(i+j)).

Define B=max_(0<=i<u) B_i. If q is nonzero, B>0: take its last nonzero
level. At that level all later q vanish, so B_i is positive.

Set f_i=2B*max(u-i,0). For i<u,

    sum_(j=1..a-i) (j/2^j) (f_i-f_(i+j))
      =2B sum_(j=1..a-i) (j/2^j) min(j,u-i) >=B.

The j=1 summand alone proves the final inequality. The coefficient of q_i
in the operator is positive. Backwards induction from i=u-1 therefore gives

    q_i<=2B(u-i).

Indeed all later q are already at most the corresponding f; if q_i>f_i,
the operator value at i would exceed that of f and hence exceed B.

There are exactly L/2^i nonzero grid points of valuation i for i<a. Thus
the total number of retained scalar jets satisfies

    Q=L sum_(i=0..u-1) 2^(-i) q_i <4BLu.

If the source's usual full-polynomial degree-uniqueness condition is kept,

    Q>M(2L-1),

then necessarily

    B > M(2L-1)/(4Lu) >=3M/(8u),  for L>=2.

The source's linear taper attains B=O(M/u). Hence its worst normalized
Hermite value-weight loss is minimax-optimal up to constants in this
fixed-support, full-interpolation template, even among nonmonotone tapers.

## 4. What precision lower bound that statement actually implies

In D.9, independently perturb the constant coefficient of one truncated
quotient germ by2^p_i. This changes its contribution by E_t(0)*2^p_i.
For a positive B_i, robustness under every independent residue lift therefore
requires

    p_i >= p+D_i B_i.

At some active level this normalized loss is Omega(M/u). Under the source's
u<=a/2 type cumulative schedule that level has exponentially large D_i,
and the binary-band construction applies to a saturated admissible mode
alphabet at the corresponding budget. Altering the taper's shape alone
does not remove the source-template bottleneck.

This necessity is for the source's termwise arbitrary-lift recovery protocol.
It is not a lower bound for every possible modular recovery algorithm.
In particular, Q>M(2L-1) is the source's full-polynomial interpolation
condition. A method using additional integral-valued-polynomial structure,
correlated cross-node recovery, fewer jets solely for F(0)mod2^p, a different
node set, or a different packet deformation needs its own analysis and is
not ruled out by this argument. Changing the retained support so i can be
very close to a also invalidates the large-D band premise; its larger jet
multiplicities and finite mode rank must be charged separately.

## 5. Node-specific multiplicities: an ultrametric symmetrization extension

The independent within-session reviewer `/root/theory_review` checked the
level-taper comparison, nearest-support potential argument, averaged S_i
operator and positive-mass barrier below. The final conversion to a total
candidate-mode complexity obstruction has not independently been checked.
This extends the normalized-precision obstruction beyond multiplicities
constant on each valuation level. The statement remains confined to the
same full-polynomial Hermite and arbitrary-independent-lift method. Let
m_t>=0 be an integer for every nonzero t in S, with m_(-L)=0 as in the
source's available group coordinates. Put m_0=0 and c=a+1. Define the
capped valuation v_c(x)=min(v2(x),c), taking v_c(0)=c, and the potential

    V(t)=sum_q m_q v_c(t-q), R(t)=V(t)-V(0).

All residues are interpreted modulo2^c, with S its representatives.
For an active t, i=v2(t), A_i=c-i, direct comparison with E_t gives

    R(t)=-v2(E_t(0))+A_i*m_t.

Perturbing the highest retained quotient coefficient, of index m_t-1,
therefore shows that the source's arbitrary-lift robustness requires

    p_t>=p+R(t)-A_i.                         (T1)

### Positive normalized maxima occur on the support

Suppose m_t=0 and R(t)>0. Choose a support point q0 maximizing
k=v2(t-q0). If k<=i, then no support point has valuation i (two numbers
of valuation i differ by valuation at least i+1). Contributions from
support points of valuation below i to R(t) are zero, and those of valuation
above i are negative. This would give R(t)<=0. Hence k>i and q0 has
the same valuation i as t.

For any support point q at distance valuation below k, ultrametricity gives
v_c(q0-q)=v_c(t-q). For the remaining support points it gives
v_c(q0-q)>=k=v_c(t-q); the q0 term is strictly larger because its self
valuation is c>k. Thus V(q0)>V(t), with D_i unchanged. Consequently,
on every valuation level, a positive maximum of R(t)/D_i is attained at
an active point. Inactive-point constraints can be included without
increasing that positive maximum.

### Averaging cannot improve the augmented precision bound

The odd units modulo2^c act transitively on each valuation level and
preserve v_c(t-q). Average m over this group. The resulting rational
nonnegative multiplicities are level-constant q_i, preserve the total
jet count Q, and do not increase any level's maximum of R/D_i. No claim
that averaged rational multiplicities are executable jets is needed: they
are used only in a linear lower-bound argument.

For this averaged profile, the normalized augmented loss is

    S_i=R_i/D_i
       =(2-1/D_i)q_i
         -sum_(j=1..a-i) (j/2^j) q_(i+j),  0<=i<a,

with q_a=0. This follows from D.10 because
sum_(j=1..a-i) j/2^j=2-(a-i+2)/D_i.

Let S_i^+=max(S_i,0) and f_i=sum_(k=i..a-1) S_k^+. The coefficient of
S_i^+ in the displayed operator applied to f is2-1/D_i>=3/2. Every
other coefficient is nonnegative: the full forward weight sum is
2-(a-i+2)/D_i, smaller than the diagonal coefficient by(a-i+1)/D_i.
Thus the operator on f is at least S_i. Backwards comparison proves

    q_i<=sum_(k=i..a-1) S_k^+.

It follows that

    Q/L=sum_i 2^(-i)q_i <2 sum_i S_i^+.

Retaining Q>M(2L-1) therefore forces

    sum_i S_i^+ > M(1-1/(2L)) >=3M/4.        (T2)

In particular some level's augmented normalized loss is at least3M/(4a).
If support is restricted to i<u, the earlier3M/(8u) bound also applies
after symmetrization. Formula(T1) differs from the augmented loss only by
A_i<=a+1, which is negligible in the source's large-M regimes.

### The finite-mode geometry prevents moving all the loss to high levels

The following gives a candidate-enumeration corollary even if the taper's
support is allowed throughout0,...,a-1. Use the same saturated full-packet
families as section2 and take a sufficiently large, with M growing faster
than any fixed power of a, as happens at active source nodes.

Split levels into D_i>=a^2 and D_i<a^2. If the first class carries at least
half the positive mass in(T2), one of its at most a levels has
S_i>=c0 M/a. By(T1), some active node of that level needs precision at
least c1 MD_i/a. The band construction, with W=c2 D_i/sqrt(a), is valid
because D_i>=a^2 dominates A_i sqrt(a). It yields
exp(Omega(M/sqrt(a))) candidate tuples, satisfying the packet degree caps.

Otherwise the small-D levels carry at least half the mass. There are at
most2log2(a) such levels, so one has S_i>=c3 M/log(a), giving precision
at least c4 MD_i/log(a) at an active node. Write v=log(a).

* If D_i is larger than a sufficient constant times A_i sqrt(v), use the
  same band with W=c5 D_i/sqrt(v). It yields exp(Omega(M/sqrt(v))) tuples.
* Otherwise D_i=O(v^(3/2)), since A_i=1+log2(D_i)<=1+2log2(a).
  The b=0 mode alone supplies Omega(M/D_i) independent candidate indices.
  Select a fixed set of at most the smaller of that number and the precision
  divided by A_i. Its size is at least Omega(M/v^(3/2)), using D_i>=A_i.
  Every binary subset respects the precision and row/packet degree caps.

Both small-D subcases give at least exp(Omega(M/sqrt(a))) tuples for
sufficiently large a, since powers of log(a) are o(sqrt(a)). Hence merely
moving the required precision to high valuation levels does not escape the
same worst candidate-count class. This argument includes the improved gap
A_i+b and uses b=0 only where a large-weight band is unavailable.

This corollary concerns the candidate set surviving the listed valuation,
degree and row-support inequalities. It does not establish that all those
coefficients are nonzero modulo the chosen precision or that an optimized
algorithm must enumerate them. It rules out an exponent improvement based
only on a different full-Hermite taper and these same pruning inequalities.
Breaking the recovery contract or exploiting actual coefficient structure
is still outside its scope.

## 6. Research consequence

Two plausible-looking local changes are now quantitatively resolved within
their exact scope: the additive valuation/degree refinements preserve the
worst candidate tuple exponent, and nonlinear taper shapes cannot uniformly
improve the normalized Hermite loss order in the source's interpolation
template. This is not an impossibility theorem for a faster contraction
algorithm or for ECDLP.

Independent review confirmed the exact taper/potential algebra described
above, not a general contraction lower bound. No strengthened contraction
theorem or new ECDLP algorithm was obtained.
A genuine improvement needs additional structure beyond these two inequalities,
such as a proved coefficient-cancellation/compression invariant, a different
deformation, or a modular recovery method that avoids the termwise independent
lift/full-interpolation contract. Those are explicit open obligations, not
assumed advantages or achieved algorithmic steps.
