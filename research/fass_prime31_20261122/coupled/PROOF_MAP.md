# Proof map: first coupled S3 norm

Status: written derivations and exact native checks; no Lean declaration has been built. Lean and Lake executables are unavailable. No theorem is replaced by an axiom or an unchecked proof stub. [pcc:unverified]

| Report obligation | Exact hypotheses / statement | Current support | Remaining formal obligation |
| --- | --- | --- | --- |
| Quadratic norm identity (1) | d >= 1; polynomial identity over the integers in a,b,c; specializes to any commutative coefficient ring | Root derivation in REPORT.md, existing scalar/Sylvester checks | Formal resultant specialization, power-sum recurrence, polynomial identity |
| Logarithmic circuit | Unreduced resultant for the monomial selector z^d-1; constant-size S3 coefficient formulas | Binary two-by-two matrix-power construction | Formal circuit model and gate bound; canonical remainder extraction excluded |
| Cusp identity (2) | d >= 1; polynomial identity in T over the integers | Written binomial derivation; fixed-d6 evaluated certificate; exact bounded column checks | Universal binomial identity and resultant equality for all d |
| Low coefficient columns (3) | p > 2d; quotient by x^d-1 | Direct coefficient extraction from (2) and exact bounded checks | Formal quotient basis and coefficient extraction |
| Generic rank theorem / determinant (4) | p > 2d; first d t-columns of canonical H; determinant (-1)^(d-1) product binom(2d,j) at A=B=0 | Written permutation/sign proof and 20 bounded cusp determinant checks | Quotient norm substitution, nonzero binomial factors, determinant permutation, generic-rank implication |
| Nonsingular generic locus | Also p > 3 for short Weierstrass interpretation | Nonzero product Delta*(4A^3+27B^2) in polynomial domain | Nonempty open intersection over algebraic closure; no universal finite-field point assertion |
| Sampling bound (5) | p > max(3,2d); uniform nonsingular coefficient pairs | Degree <= 2d^2 and polynomial zero count; exactly p singular pairs | Formal parameter-degree bound, finite-field zero count, singular-pair parametrization |
| Top coefficient and (6) | p > 2d; d >= 2 for next coefficient | Product argument, derivative/interpolation proof and bounded native checks | Formal evaluation in squarefree quotient and degree-bound interpolation |
| Rank-one exception | Odd characteristic; d=2; A=-1,B=0 | Explicit S3 substitution and exact F97 native control | Symbolic substitution equality |
| Separation rank = matrix rank | Polynomial over a field, canonical x-basis; finite t-degree | Elementary rank-one decomposition / rank factorization argument | Formal coefficient tensor correspondence |
| Join is nonunit test | L and right message squarefree and split over F_p | Product-of-fields interpretation in REPORT.md | Formal Chinese remainder isomorphism and witness correspondence |

The evaluated certificate is IDC1h099b8aa03eee75f6 with proof:null. It does not discharge any universal Lean obligation in this table. Native point evaluations support the declared fixtures, not a claim of all-characteristic identity proof.

## Formalization entry point

Start with the quadratic norm identity and the cusp binomial identity in a polynomial ring. Then formalize the quotient substitution, first-d-column determinant, and nonvanishing at characteristic p>2d. Only after those dependencies are built should the generic-rank and sampling statements be promoted. Pin the crypto-lean revision, Lean toolchain, dependencies and complete build/#print axioms output at that time.

## Verification classification

- Written mathematical derivation: present and separately reviewed.
- Exact native fixed-fixture checks: present, with source/build/exit records.
- Separate-role reproduction: present, same host and unsigned.
- Evaluated fixed-d6 identity certificate: present, proof:null.
- Kernel-checked formal proof: absent.
- Signed independent execution or mathematical PCC verdict: absent.
- Group relation or complete ECDLP witness: outside this component.
