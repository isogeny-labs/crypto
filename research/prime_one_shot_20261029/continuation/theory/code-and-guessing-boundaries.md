# Divisor coefficients as an elliptic evaluation-code problem

2026-10-29. Read-only mathematical continuation for `/root/solver_mathematics`.
No scientific experiment, timing, solver invocation, or repository source change
was performed. The statements below are derivations with proofs, not formal
proof-assistant certificates or certified novelty claims.

The local primary source checked was Petit–Kosters–Messeng, PKC 2016,
`/workspace/audit/PKM2016.txt`, especially sections 3.1–3.2: a compact membership
tower is supplied, while the relation solver and its asymptotic cost remain open.
The selected norm/remainder construction is described in
`crypto/research/prime_one_shot_20261029/README.md`. The coding interpretation below
uses elementary genus-one Riemann–Roch and a directly derived interpolation
inequality; no unread coding paper is represented as a verified source.

## 1. Exact finite-field reformulation

Let E/F_p be nonsingular, O its rational identity, R a nonzero rational target,
and S a set of B distinct rational factor-base points, excluding O and -R.
For the norm chart, also exclude R and later reject supports containing repeated
abscissas or inverse-point pairs. Assume B>m and m>=2.

Put

    D_R=(m+1)O-(-R),  W_R=L(D_R).

The divisor has degree m. Since E has genus one, dim_Fp W_R=m. Its nonzero
elements are functions with their only pole at O of order at most m+1 and with a
zero at -R. Let C_R be evaluation of W_R on the B points of S. Evaluation is
injective: a nonzero function in W_R cannot have B>m additional distinct zeros.
Thus C_R is a length-B, dimension-m code over F_p.

For odd m=2h-1, one can construct the matrix without solving the norm system.
A basis of L((m+1)O) is

    1,x,...,x^h, y,xy,...,x^(h-2)y.

There are m+1 basis elements. Compute their evaluations at -R and take the
m-dimensional kernel of that one linear functional; evaluate a basis of this
kernel at S. This yields a generator matrix G_R of size m by B. Building it uses
O(B poly(m)) field operations after obtaining the rational base. Field operations
have poly(log p) bit cost. With B=exp(O(sqrt(log p))) this matrix construction and
storage are themselves sub-exponential. The hard part is selecting its columns.

This matrix construction is a useful **independent rational model-set oracle**
for tiny development fixtures. It uses actual rational points and linear algebra,
so it does not confuse a geometric coefficient solution over F_pbar with an
F_p decomposition. Exhaustively checking column supports is only a correctness
reference, not a proposed fast algorithm.

### Proposition 1: the m-column predicate is exactly point decomposition

Every m-1 columns of G_R are linearly independent. An m-column submatrix indexed
by distinct P_1,...,P_m is singular iff

    P_1+...+P_m=R.

In the singular case its left kernel is one-dimensional.

Proof. Imposing zeros at a subset T of m-1 points leaves
L(D_R-sum(T)), a divisor space of degree one and hence dimension one. Thus the
m-1 evaluation constraints have rank m-1. For m points the residual divisor has
degree zero. On a genus-one curve its space has dimension one iff that divisor
is principal, and otherwise zero. The Abel–Jacobi class of D_R is R, so the
degree-zero class vanishes exactly when sum(P_i)=R. All statements are over F_p;
the linear equations have rational coefficients. End proof.

The additional norm-chart conditions remain separate: a singular m-column set
can contain P and -P, repeated x values, or a target overlap. Those are legitimate
group relations but lie outside the selected squarefree-abscissa norm chart.

### Corollary: this is an extremal-distance search, not ordinary decoding

Every nonzero codeword of C_R has at most m zero positions, and any prescribed
m-1 positions can be made zero. Therefore

    d_min(C_R) is B-m or B-m+1.

The first value occurs exactly when an m-point decomposition exists in S.
Equivalently, the dual code C_R^perp has minimum distance m or m+1, and a
weight-m dual codeword is precisely a dependence among m columns of G_R.

This is a concrete one-unit minimum-distance gap. Merely obtaining a code
description, its rank, or its designed distance does not decide the gap. Standard
unique-decoding guarantees address errors of weight about half the minimum
distance. The desired primal codeword has weight B-m, nearly B, while the dual
form asks for weight m when the usual half-distance guarantee is about m/2.

This equivalence is not a theorem that every coordinate-aware decoding method
is exponential. It identifies the exact new decoding interface that would have
to be supplied; it does not establish a complexity lower bound for it.

## 2. The simplest constructive interpolation recurrence collapses to group addition

### Proposition 2: m-1 guessed zeros have one determined completion

Choose distinct T={P_1,...,P_(m-1)} in S. The one-dimensional space
L(D_R-sum(T)) determines one projective function. Its remaining zero, counting
divisors and allowing O in a lower-pole degeneration, is

    Z_T=R-sum(P_i).

Indeed a nonzero section has divisor

    (-R)+sum(T)+Z_T-(m+1)O,

with cancellations at O understood. Abel–Jacobi gives the displayed formula.
If Z_T lies in S outside T, this gives m distinct rational zeros. Otherwise it
does not. Recovering the monic A+C y representative is extra interpolation work,
not a different way of choosing the completion.

Thus the algorithm “guess m-1 zeros, interpolate the RR function, factor its
remaining norm factor, and test the last point” is exactly subtraction plus a
base-membership query. It has a valid native implementation but no new
non-generic relation-search advantage. This is a rigorous identification of
that particular algorithm, not an exclusion of implicit coefficient solving.

### Proposition 3: an m-2-zero pencil is a two-summand oracle

After imposing zeros at m-2 fixed points, the residual line bundle has degree
two and dimension two. Its projective pencil is the usual degree-two map from
E to P^1. If

    S_T=R-sum(T),

its fibres are pairs P,Q with P+Q=S_T, including the tangent/ramified case
2P=S_T. The induced involution is P -> S_T-P.

Proof. The residual zeros of any section have degree two and fixed Abel–Jacobi
class S_T. Conversely every effective divisor P+Q in that class is the zero
divisor of a section of the residual bundle. End proof.

Consequently “solve the two-dimensional coefficient pencil” is precisely
two-point decomposition of S_T, with the same exceptional cases. A dedicated
non-generic two-point solver could improve that subroutine, but guessing almost
all m factors first is still expensive, as quantified next.

## 3. Exact guessing probability and its bit-length consequence

Fix a target and base. Let V_R be the number of accepted m-element supports in
S (include whatever chart restrictions are actually required). Pick a uniformly
random g-element subset T of S, and grant a **free complete oracle** deciding
whether T extends to an accepted m-support.

The number of successful T is at most

    V_R * binom(m,g).

Thus success probability is at most

    V_R * binom(m,g) / binom(B,g).

This is an elementary union bound over supports. For g=m-1 it is an equality:
two accepted m-supports cannot share an m-1 subset, because their remaining
point would be the unique Z_T of Proposition 2. Hence

    Pr[successful (m-1)-guess] = m V_R / binom(B,m-1).

In the usual balanced random-like regime binom(B,m) approximately r, with
V_R of order one for a successful target, this is approximately B/r. The
expected number of uniform trials is approximately r/B, even if interpolation,
factoring, and verification are free. For n=log_2 r, m approximately sqrt(n),
and log_2 B=n/m+O(log m), that is

    2^(n-O(sqrt(n)+log n)),

which is exponential in n. Constructing the rational base in sub-exponential
time does not change this trial count.

More generally take g=c m for fixed 0<c<1, B much larger than m^2, and
V_R=exp(o(n)). Stirling estimates give

    log_2 [binom(B,g)/binom(m,g)] = c n + o(n).

So uniform guessing of a constant fraction of factor blocks requires
exp(c n-o(n)) trials under that relation-count regime, even with the residual
oracle free. This rules out one common shallow-looking implementation as a
sub-exponential method.

Scope matters. This is a bound for **uniform/support-blind guesses** and the
stated count V_R, not for target-dependent coordinate predicates correlated
with a true support. A structure-aware predictor is a different missing
algorithm. Guessing only O(1) complete factor blocks costs B^O(1), which is
sub-exponential; a fixed-size root-block backdoor followed by a genuinely cheap
large-arity residual remains an open possibility. Guessing an arbitrary full
F_p coefficient costs p choices in the absence of a smaller proven domain and
is not interchangeable with guessing a factor-base element.

## 4. Why standard list-interpolation guarantees do not supply the missing solver

The desired nonzero function agrees with the all-zero received word at only
m of the B positions. Standard multiplicity interpolation with a pole budget L
and multiplicity s gives the familiar dimension-count scale

    agreement > sqrt(B m),

up to lower-order genus/multiplicity terms. At B>m, the desired agreement m lies
strictly below that sufficient threshold; at B exponentially larger than m,
the gap is enormous. This is failure of that guarantee, not a universal lower
bound for all list-decoding algorithms.

The scale follows directly from Riemann–Roch. Write
Q(P,Z)=sum_j q_j(P) Z^j, with q_j a section of a line bundle of degree L-jm.
The number of coefficient parameters is about L^2/(2m). Multiplicity-s
conditions at B evaluation points give at most B s(s+1)/2 linear constraints.
The usual dimension-count existence certificate therefore needs roughly
L/s >= sqrt(Bm). The zero-count root-forcing step needs s*agreement > L.
Combining the two yields the displayed sufficient agreement condition.

For the zero received word there is an even more transparent degeneration:
Q=Z^s satisfies all interpolation conditions but has only the zero function as
a root. To infer a nonzero root through the ordinary zero-count argument, strip
the maximal Z^j factor. If j<s and q_j is the first nonzero coefficient, then
q_j must vanish to order at least s-j at all B points. Therefore

    L-jm >= B(s-j).

But forcing a candidate with only m agreements to be a root of Q/Z^j would need

    L-jm < m(s-j).

These inequalities are incompatible when B>m. If j>=s, stripping the Z factor
leaves no useful multiplicity information from agreement with zero. A specially
constructed Q might contain the desired function as a factor for some other
reason, but finding such a Q is then another algorithmic problem; the ordinary
agreement/root-count mechanism has not found it.

The dual high-rate code has a Johnson-style sufficient error radius
B-sqrt(B(B-m)), approximately m/2 when B is much larger than m^2. The desired
weight-m codeword is again outside that ordinary guarantee.

## 5. A useful next interface, and what has not been achieved

The matrix G_R is small enough to build at the proposed sub-exponential scale.
A dedicated algorithm that finds an accepted singular m-column set in time
poly(B,n)*exp(o(n)) with all m dependence itself exp(o(n)) would be a legitimate
non-generic decomposition route. The evaluation coordinates carry the curve
equation and tower structure, so a generic-group bound alone does not exclude it.

However, none of the following supplies that algorithm:

- Enumerating all binom(B,m) supports: this is about r at balanced parameters.
- Enumerating binom(B,m-1) interpolations: Proposition 2 is generic completion.
- Searching a two-dimensional pencil after m-2 guesses: Proposition 3 and the
  guessing bound preserve exponential search cost.
- Constructing all Plucker coordinates/minors: there are binom(B,m) of them.
- Calling ordinary unique/list decoding on zero and assuming it finds a minimum
  nonzero codeword: its stated guarantee does not cover this extremal regime.
- Sampling uniform coefficients in projective W_R: there are approximately
  p^(m-1) coefficient lines but only V_R successful supports, so it is worse.

The precise constructive outcome of this note is an exact rational linear-
algebra comparator and three scoped obstruction lemmas. It has not produced a
new fast relation solver, a useful rank-producing relation supply, an
asymptotic field family, or a sub-exponential ECDLP algorithm. It also does not
show that such an algorithm is impossible. Its practical value is to ensure
that a coefficient/interpolation proposal does not silently become ordinary
subset enumeration or invoke an inapplicable decoding guarantee.
