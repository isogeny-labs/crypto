# Quotient-query proof map

Tracking: [crypto PR #4966](https://github.com/isogeny-labs/crypto/pull/4966).
Written source: [THEORY.md](THEORY.md). All new mathematical standing below
is unverified in the repository PCC sense. [pcc:unverified]

| Statement | Exact hypotheses and evidence | Formal declaration / status |
| --- | --- | --- |
| Certificate decomposition, Proposition 1 | Monic split squarefree F,G; coprime monic block product G; monic D; both divisibility identities and Bezout identity modulo each block. Written CRT/fibrewise proof in THEORY sections 1-2. | No Lean declaration; not kernel checked |
| Exact support count, equation (7) | Same hypotheses; monic D retains its x-degree in every rational fibre and divides squarefree F. Integer sum of block-degree products counts affine grid points. | No Lean declaration; not a signed-row or relation-rank count |
| Reduced lex bases per positive block, Proposition 2 | Coefficients reduced modulo monic q_i; D_i monic of positive x-degree; lex x>t. Coprime leading monomials and reduced tails. | No Lean declaration; no global-basis or original-system solving-degree claim |
| Radicality of finite-grid ideal | F,G split squarefree; CRT gives a product of fields; quotient by H selects components. | No Lean declaration; does not cover a nonreduced unreduced selector chain |
| Dynamic Euclid production and termination | Squarefree G makes every proper gcd split coprime; inverse only after unit test; each split reduces modulus degrees; each Euclidean step reduces x-degree. | Written derivation; native certificate implementation/replay owned by coordinator |
| Norm identity (10) | Every commutative coefficient ring; formal monic-first resultant convention, including quadratic degree drops. | Inherited written result from selector/REPORT.md; no new formal status |
| Fast dynamic half-gcd cost (12) | Dahan et al. arithmetic-time framework, fast multiplication and properly organized splitting, one squarefree coefficient modulus. | Literature result, not a complexity certification of the new schoolbook implementation |
| Selector-target support lower bound and equality classification | [CANCELLATION_PREDICTION.md](CANCELLATION_PREDICTION.md): p>3, nonsingular curve, d divides p-1, full left selector F=x^d-1, rational target Q=(X,Y) with X in the selector and Y nonzero, and actual squarefree rational right G of degree e. The vertical x=X subset and S3(x,X,t)=0 subset force at least 2e-1 affine support points; equality forces one fibre of size one at x(2Q) and all other fibres of size two. | Written derivation, independently source-reviewed; no Lean declaration, signed-usefulness claim, or application of the lower bound to 2-torsion targets |
| Signed-relation routing | Rational lifts, all required selector/subgroup members, complete collision/sign recovery, original group replay and separate infinity/identity branches. | Interface obligations; no theorem that every unfiltered grid point is a signed relation |

The capability check found no Lean or Lake binary. Formalization would first
instantiate the existing polynomial quotient/CRT and Groebner structures in
Mathlib or crypto-lean under the repository lean-instance-gate, then build and
audit the actual modules. No weaker substitute, placeholder proof or native
evaluation is presented as a proof of these universal statements.

Finite-field certificate checks can establish that specific returned
identities hold for specific input polynomials. Their source binding,
independent arithmetic, mutation controls and execution status belong in the
component validation record. They do not change the universal proof status
in this table.
