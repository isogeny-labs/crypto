# A levelwise binomial counterfamily for a common-origin interpolation bridge

2026-10-30 UTC. Independent written verification of the proposed construction.
No implementation, solver, benchmark or timing experiment was performed.
The proof concerns interpolation data from different levelwise polynomials;
it does not establish that the polynomials arise from repacked Cauchy tensor
networks, and it does not refute source LemmaD.5.

## Version and source scope

Before adding this link to the matched-level analysis, its previous version
was preserved as
`/workspace/audit/matched_level_multivariate.pre-binomial-20261030.md`.
Its SHA256 is

    df584de97fdc264813a4b89e8dbd8f910e0a392fa253ab65895030ec966aa4b9

The relevant original source is the pinned Oct25 manuscript
`/workspace/audit/tensor-primary/openai-oct25.txt`, D.4.3-D.4.4. The precision
schedule below is its displayed schedule with a=2u. The generalized binomial
identities used here are derived below. The main matched-level note credits
the original Hermite recovery argument and its independent verification.

## Exact parameters and construction

Let C>=1 be an integer constant. Choose integers M,u,p satisfying

    M>=u>=32,        1<=p<=C*M,        2^u>=4*(C+1).

Put

    a=2u,       L=2^a,       alpha=2^(a+1),
    h=ceil(3M/u),
    D_i=L/2^i,
    p_i=p+8*h*D_i+8*M*(a+1),        0<=i<u,
    d=M*(2L-1).

These conditions are explicit sufficient conditions, not optimal ones. For
example C=1, p=1, and any M>=u>=32 satisfy them.

For N=2^i define the rational polynomial

    B_N(T)=binom(T,N)=T*(T-1)*...*(T-N+1)/N!,
    P_i(T)=1-B_N(T),
    H_i(T)=P_i(T)^(p_i),
    F_i(T1,T2)=H_i(T1).

Each H_i has origin value1. Each F_i consequently has F_i(0,0)=1.
The index i labels a different polynomial, which is the essential point.

The claims proved below are:

1. Every scaled chart H_i(t+alpha*z), for every signed integer t, has
   coefficients in Z_(2), the rationals with odd denominator.
2. If v2(t)=i, every coefficient of that chart is divisible by2^(p_i).
3. deg H_i<=d, hence F_i has bidegree at most(d,d).
4. The descriptions, including their charged rational denominators, have
   polynomial bit length in d and log(N+1).

Thus the entire level-i matched data block of F_i vanishes at its claimed
precision, even though its origin is1. This holds for all local coefficients,
not just the finitely many jets used in Hermite recovery.

## 1. Generalized binomial values at negative integers

For nonnegative integer t, B_m(t) is an integer by the usual binomial
coefficient formula, and it is0 if m>t. For t=-s with s>=1,

    B_m(-s)=(-1)^m*binom(s+m-1,m).

This follows by changing the signs of the m numerator factors. Therefore
B_m(t) is an integer for every signed integer t. The convention B_0(T)=1
will be used in Vandermonde's identity.

Suppose v2(t)=i and N=2^i. Factoring the numerator gives

    B_N(t)=(t/N)*product_(1<=j<N) ((t-j)/j).

The first factor is an odd integer. Every 1<=j<N has v2(j)<i, so
v2(t-j)=v2(j), and each ratio is a two-adic unit. The product is a
two-adic unit. Since B_N(t) is an integer, it is odd. This proof includes
negative t and the case i=0, for which N=1 and the product is empty.

## 2. Coefficientwise local integrality and divisibility

For 1<=k<=N, directly factoring the binomial polynomial gives

    B_k(alpha*z)
      =(alpha/k)*z*product_(1<=j<k) ((alpha/j)*z-1).

Because k<=N=2^i and a>=i, every alpha/j and alpha/k lies in Z_(2).
Furthermore

    v2(alpha/k)=a+1-v2(k)>=a+1-i>=1.

The product has integral coefficients and unit constant term. Consequently
every coefficient of B_k(alpha*z) is divisible by2^(a+1-i) in Z_(2).
Its constant term is0.

The polynomial Vandermonde identity is

    B_N(t+X)=sum_(k=0)^N B_(N-k)(t)*B_k(X).

One derivation is to compare the coefficient of Z^N in
(1+Z)^(t+X)=(1+Z)^t*(1+Z)^X, using formal binomial series in Q[X][[Z]];
only finitely many terms enter the coefficient. Equivalently it holds at
every nonnegative integer X and hence identically as a polynomial in X.
It holds for each signed integer t as well, either by the formal identity
or polynomial continuation in t.

Set X=alpha*z. Each B_(N-k)(t) is an integer by Section1. The k=0 term is
the integer constant B_N(t). All remaining terms have coefficientwise
divisibility at least2^(a+1-i). Hence

    B_N(t+alpha*z) belongs to Z_(2)[z],
    B_N(t+alpha*z)-B_N(t) belongs to 2^(a+1-i)*Z_(2)[z]

for every signed integer t.

If v2(t)=i, Section1 says B_N(t) is odd. Therefore the constant term
1-B_N(t) is even, and all nonconstant terms are even by the preceding
display. Thus

    P_i(t+alpha*z) belongs to 2*Z_(2)[z],
    H_i(t+alpha*z) belongs to 2^(p_i)*Z_(2)[z].

Raising a coefficientwise-even polynomial to the p_i-th power supplies
that many powers of2; summing its product terms cannot decrease their
minimum valuation. At arbitrary t the same charts remain integral, even
when this stronger vanishing conclusion does not apply.

For F_i the T2 chart is irrelevant. For any integer t2 and any t1 of
valuation i,

    F_i(t1+alpha*z1,t2+alpha*z2)
        belongs to 2^(p_i)*Z_(2)[z1,z2].

In particular every coefficient at every matched-level pair vanishes
modulo2^(p_i). All charts at all other integer pairs are still integral.

## 3. Explicit degree bound

The degree is exactly N*p_i. Since N*D_i=L and a=2u,

    N*p_i=8hL+N*[p+8M*(2u+1)].

M>=u gives h<=4M/u, while N<=2^(u-1). Using p<=CM therefore yields

    deg H_i
      <= M*L*[32/u+(C+16u+8)/2^(u+1)].

For u>=32,

    32/u<=1,
    (16u+8)/2^(u+1)<=1/8.

The latter follows, for example, from 16u+8<=2^(u-2), which holds at32
and is preserved as u increases because the right side doubles and the
left side grows by16. The assumed 2^u>=4(C+1) also gives

    C/2^(u+1)<1/8.

Consequently

    deg H_i <= (5/4)*M*L < M*(2L-1)=d.

The final strict inequality holds for L>=2. It is vastly satisfied under
the stated parameters. No asymptotic degree term or ceiling was omitted.

The underlying statement is more general: any positive integer precision
schedule pi_i satisfying 2^i*pi_i<=d admits the same counterfamily with
exponent pi_i. The chart proof is unchanged. In particular a fixed-constant
schedule pi_i<=K*M*D_i/u has degree at most K*ML/u, which fits the bound for
sufficiently large u. Changing only the constant in that precision order
does not repair the generic data-only bridge.

## 4. Description and denominator costs

Let D=N*p_i. Write (T)_N=product_(j=0)^(N-1)(T-j). Then

    H_i(T)=[N!-(T)_N]^(p_i)/(N!)^(p_i).

The coefficient l1 norm of (T)_N is at mostN!: multiply the l1 norms
1,2,...,N of the individual factors. Hence the numerator base has norm
at most2N!, and its p_i-th power has norm at most(2N!)^(p_i).
Every numerator coefficient and the common denominator thus have bit
length

    O(p_i*log(2N!))=O(D*log(N+1)).

There are D+1 coefficients. An explicit dense rational description therefore
uses O(D^2*log(N+1)) bits, and the displayed factored description is smaller.
Since D<=d=O(ML), these are polynomial bounds in the source's parameters.
When L is polynomial in its global size, as in D.4, no exponentially long
coefficient advice is being supplied.

The displayed global denominator can have a large power of2; that cost has
not been treated as a unit inverse. The global polynomial belongs to Q[T],
as permitted for the deformation polynomial in D.5. Its local integrality
was established before modular reduction. Within a local chart, the
factored formulas above cancel the known powers of2 and leave odd
denominators, which can be inverted modulo any requested2-power. Exact
polynomial arithmetic and modular local evaluation require polynomially
many bit operations in D and the supplied node/precision bit size. This is
a description-cost observation, not an executed timing claim.

## 5. What this refutes, and what it leaves open

Consider a proposed interpolation bridge given only the modular local jet
data from level-i polynomial F_i at level-i nodes, together with these
promises:

- every F_i has the same unknown origin value;
- each has bidegree at most(d,d);
- every required scaled chart, even every integer-node chart, is integral;
- level-i data are supplied modulo2^(p_i) using the stated schedule.

The constructed family has common origin1 and all its supplied data are0.
The all-zero polynomial family has common origin0 and identical supplied
data. Therefore those data and promises do not determine the origin even
modulo2. No universal interpolation identity based only on them can be
valid. Giving all chart coefficients instead of truncated jets does not
help at these modular precisions.

This is stronger than the elementary nodewise example (1-T/t)^q: the new
polynomials vary only with valuation level, obey the requested bidegree
bound, have coefficientwise-integral charts at all signed integer nodes,
and vanish at the actual tapered precisions, with polynomial-size charged
descriptions.

The counterexample has two important scope limits. First, it does not show
that these F_i are realizable by actual node-adapted Cauchy packet networks.
An invariant or congruent-jets property of that narrower family might
exclude them and supply the desired bridge. Second, an algorithm allowed
to inspect the explicit coefficients of this particular counterfamily can
read its constant term1 immediately. This is not a hardness claim against
code-aware algorithms. It refutes the data-only generic interpolation
step, not every use of additional program structure.

Recovery from one common polynomial remains valid, including the positive
diagonal matched-level lemma in the main note. To combine differently
repacked deformations, one must prove additional compatibility, not merely
their common origin, degree and local integrality.
