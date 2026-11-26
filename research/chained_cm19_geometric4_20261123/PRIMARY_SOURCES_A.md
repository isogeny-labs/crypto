# Primary source and inherited evidence receipt before qualification

KB and Conductor tools unavailable; exact-path fallback only, no novelty
clearance. Locked vault wrappers were not executed. The first Milne
2005aX search hit was opened and rejected: it is Shimura Varieties, not
the claimed finite-field AV source. No conclusion relies on that hit.

1. J.S. Milne, Abelian Varieties v2.00 (2008),
   https://www.jmilne.org/math/CourseNotes/AVc.pdf, I Lemma10.6 (printedp45,
   PDFp50), read exact injectivity/torsion-free Hom statement and proof;
   II1 and III11 are the Frobenius/Weil reference locations. Hypotheses:
   abelian varieties over the base field, auxiliary prime different from
   the characteristic. Applying it over algebraic closure gives actual
   geometric Hom torsion-freeness. The short argument via surjectivity
   of multiplication also establishes the needed no-integer-torsion fact.
2. Stacks Project, Frobenii33.36, https://stacks.math.columbia.edu/tag/0CC6,
   definitions/lemmas33.36.1--8 read: distinguish relative Frobenius over
   the algebraic closure from absolute Frobenius, use the naturality
   square for f and f^(p), and surjectivity of relative Frobenius.
3. Stacks Project, Riemann-Hurwitz53.12,
   https://stacks.math.columbia.edu/tag/0C1B, read the differential divisor
   equality, total degree and tame local multiplicity e-1. Apply to smooth
   projective geometrically connected curves with a separable map;
   over the algebraic closure the residue extensions are trivial, and
   char7 is prime to the permutation-group order24.
4. Stacks Project, Kummer extensions9.24, Lemma9.24.1,
   https://stacks.math.columbia.edu/tag/09I6, read converse for cyclic
   degree n, characteristic prime to n and a primitive nth root in the
   field. Here n=3 over F7bar(E); local ramification/valuation and sum
   of a principal divisor's valuations are supplied explicitly in the
   written proof. The separately corrected radical-subextension lemma
   on the same page is neither needed nor invoked.

Exact inherited sources read completely: actual beta proof
ACTUAL_H_BASIS_PROOF_A.md (05a984/d9f917); actual global L and degree
GLOBAL_SPLIT_TRANSFER_C.md (3c0acb); full prime-field automorphism/quotient
CURRENT_RESULT_B.md (941363). All remain unsigned. Prior mathematical
inputs, genus1 rational-point existence by Weil, canonical adjunction,
quotient descent and Galois closure/inertia facts are explicit standard
bridges and must be named in the written proof; this receipt is not a
claim that a native or Lean leaf proves them. [pcc:unverified]

Additional exact primary locations inspected before freezing: Stacks
Plane curves53.9 https://stacks.math.columbia.edu/tag/0BYA (smooth degree4
genus3); Milne III Theorem11.1, printedp117/PDFp123 (Weil bound for any
complete nonsingular geometrically connected curve, with no rational
point hypothesis); Ravi Vakil, The Rising Sea, July27 2024 public
version https://math.stanford.edu/~vakil/216blog/FOAGjul2724public.pdf,
chapter23 adjunction/canonical plane quartics. Canonical adjunction is
used in its smooth projective hypersurface form, valid in char7, not
a complex-analytic-only statement. [pcc:unverified]
