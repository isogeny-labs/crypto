# Normalized translation of a complete subgroup product

Status: producer derivation, not independently verified. [pcc:unverified]
Task T-1783, 2026-11-07. This is a construction/search record, not an experiment.

## Domain and exact identity

Let E/F_p be nonsingular y²=x³+ax+b, p>3 prime. Let H=<P> have odd prime
order r, r!=p; set h=(r-1)/2. All H points are F_p-rational. Define the monic
kernel polynomial K(X)=product_{j=1}^h (X-x([j]P)) and F(Q)=K(x(Q)).
For every nonzero S in H let f_{r,-S} be the Miller function with divisor
r[-S]-r[O], normalized to leading coefficient 1 at O in parameter t=-x/y.
Then as a rational-function identity on E,

    F(Q+S)/F(Q) = c_S / f_{r,-S}(Q),
    c_S = 2 y(S) K'(x(S)).                              (1)

Proof: x-x(T) has divisor [T]+[-T]-2[O], with distinct zeros since r is odd.
Thus div(F)=sum_{T in H}[T]-r[O]. Translation by S permutes H, so
 div(F(Q+S)/F(Q))=r[O]-r[-S].
The ratio times f is consequently a nonzero constant. At O,
x=t^-2+O(t²), y=-t^-3+O(t), F=t^{-(r-1)}(1+O(t)).
The invariant differential dx/(2y) has expansion (1+O(t))dt. Its translation
invariance gives x(Q+S)=x(S)+2y(S)t+O(t²), hence
F(Q+S)=2y(S)K'(x(S))t+O(t²). Multiplying the ratio by the normalized
f=t^-r(1+O(t)) proves (1). K' is nonzero at these simple roots; y(S)!=0.
All constants are in F_p. No characteristic change or extension field is used.

Normalization is essential. A line quotient convention may produce f_tilde=d*f
for nonzero d. In that convention the numerator is c_S*d. An implementation
must track leading coefficients; an arbitrary Miller value cannot be inserted
into (1) unchanged.

## An anchor-free ratio of ratios

For A,B outside H, (1) implies

    F(A+S)F(B) / (F(A)F(B+S)) = f_{r,-S}(B)/f_{r,-S}(A). (2)

The normalization cancels too, provided the same Miller function is used.
For q dividing p-1, pi(z)=z^((p-1)/q), applying pi to (1) or (2) preserves
multiplication. In particular q=2 or4 uses the existing projected image.
Equation (2) needs two Miller evaluations with O(log r) addition-chain steps,
plus the final projections. It does not materialize K or any orbit list.
This is a derived arithmetic bound for this ratio, not a measured runtime.
Use generic rational-function evaluation or handle removable intermediate
line zeros explicitly: A,B outside H avoid zeros/poles of the final f, but
not necessarily intermediate line factors in a naive Miller program.

Equation (2) is the positive construction in this round. It replaces a
specific four-product cross ratio; it does not supply an absolute F value.

## Exact small-field check, by substitution

Take E/F_5: y²=x³+1 and P=(0,1), of order3. Then H={O,(0,1),(0,4)},
K(X)=X, c_P=2, and f_{3,-P}=-(y+1), normalized since y=-t^-3+O(t).
For the three points outside H:

| Q | Q+P | F(Q+P)/F(Q) | f(Q) | c_P/f(Q) |
|---|---|---|---|---|
| (2,2) | (2,3) | 1 | 2 | 1 |
| (2,3) | (4,0) | 2 | 1 | 2 |
| (4,0) | (2,2) | 3 | 4 | 3 |

These entries are exact hand-derived residues modulo5, not native-run output.
For A=(2,2), B=(2,3), both sides of (2) equal3. This catches the sign of c
and the normalization of f. It is a consistency example, not a general proof.

## Why this cannot yet replace the source

Every nonidentity subgroup target Q has F(Q)=0, and O is a pole. When H=E(F_p)
there is no rational A or B outside H at all. One cannot plug subgroup inputs
into the displayed fractions and interpret 0/0 as an ordinary field quotient.
The rational identity still has a regularized meaning away from O,-S, but
extracting local leading terms is a new interface. Values such as K'(x(Q))
then enter; they have not been made cheaply available by naming them.

The current geometry residual uses partial products and resultants. It is
not the complete K product: replacing it by (2) requires a divisor equality
and a domain/normalization bridge for that exact residual. None is assumed.
The linear shifted-character constructor's h-request bound is unchanged.

## Gate for a sparse multiplicative shortcut

Suppose a proposed rational identity is

    F = c * H0^q * product_{i=1}^t g_i^{e_i},            (3)

where q>1 divides p-1, e_i are integers, H0 is a rational function, c!=0,
and each g_i has divisor support on at most s geometric points. At every
T in H\{O}, ord_T(F)=1. Reducing divisors modulo q shows that T must lie
in the support of at least one g_i. Consequently s*t >= r-1.
This elementary support-union proof permits arbitrarily large e_i and arbitrary
H0. It does not assume low polynomial degree or bound a general circuit.

A complete Miller primitive has at most3 support points, even after translation;
a line has at most4 including O (at most3 away from O when unshifted).
Thus a representation using only translated Miller primitives in (3) needs
at least ceil((r-1)/3) primitives. Sparse multiplicity compression is different
from compression of many distinct simple zeros. Equation (1) escapes the gate:
its target divisor is a translated quotient with only two support points.

This gate applies to rational q-th-power equivalence, NOT to equality of
projected values on a finite input set. For example X^p-X+1 is identically1
on F_p, yet has p simple geometric roots (derivative=-1), so is not a q-th
power in the rational function field for q>1. Finite-set identities, additive
circuits, maps with large fibers, and implicit norm primitives remain open.
Divisor divisibility alone is also not sufficient for a rational q-th root;
no converse to the support test is asserted.

## Next implementation target

Implement a normalized Miller evaluator and a local-leading-term interface,
with an independent divisor-and-series reference. First establish (2) on a
fixed prime-field fixture with rational points outside H, then test its
regularized form on H. Separately compute the exact divisor of the existing
partial residual and check whether paired source queries cancel its support.
A cancellation would give a concrete cheap update target; failure would retain
the partial-support problem. Do not run a speed panel before that bridge.

## Sources checked and scope of search

KB unavailable: no loaded kb_status/search_knowledge tools; scripts/research-kb
status reports no executable. Fallback reviewed existing projected-residual,
resultant-class, half-gcd and linear-constructor derivations in this study.
Their next steps identify explicit geometry and nonlinear construction costs.

Read Victor S. Miller, Short Programs for functions on Curves (1986), Algorithm1,
https://crypto.stanford.edu/miller/miller.pdf . Its size parameter sums logarithms
of multiplicities over the supplied divisor support. It does not automatically
compress a divisor with r distinct simple points. Read Ben Lynn's Miller
recurrence and line formulas,
https://crypto.stanford.edu/pbc/notes/elliptic/tate.html . Used only rational
functions and divisors; no pairing transfer or destination DLP extractor.
Miller here means Victor S. Miller's elliptic-function algorithm, distinct from
the J.C.P. Miller power-series recurrence in feature-native-miller-builder.
The identity and gate above are elementary derived specializations; no novelty
priority claim. No Lean artifact, independent review, experiment or scalar
recovery is claimed. Original4563-case cohort and prior evidence untouched.
