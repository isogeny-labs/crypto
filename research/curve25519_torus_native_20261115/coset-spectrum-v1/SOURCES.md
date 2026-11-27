# Sources checked before the construction

- Internal: section-coeff-v1/DERIVATION.md (split/separable membership,
  original rational norm, positive window and literal-degree limits);
  section-compiler-v1/{NEXT,PROTOCOL,PROOF_MAP}.md (frozen native/compiler
  limits); target-circuit-v2/PROTOCOL.md and its recorded original circuit
  counts (the stronger representation comparator).
- David A. Cox, *Stickelberger and the Eigenvalue Theorem*,
  [arXiv2007.12573v2](https://arxiv.org/html/2007.12573v2), section1,
  Theorems1.1/1.2 and formula1.2, read directly. Characteristic polynomials
  retain eigenvalues with local multiplicities, so they do not erase
  nilpotent parts. Section4 also separates residue fields from local length.
- Faugere--Mou, *Sparse FGLM algorithms*,
  [arXiv1304.1238v1](https://arxiv.org/pdf/1304.1238), sections1.2--2.1,
  read directly. D is the dimension of the SOLUTION ideal's quotient;
  shape position, sparse multiplication matrices and their construction
  are algorithmic hypotheses/costs, not supplied by a small candidate ring.
- Faugere--Huot--Joux--Renault--Vitse, *Symmetrized Summation Polynomials*,
  [EUROCRYPT2014, DOI](https://doi.org/10.1007/978-3-642-55220-5_3),
  section3.3, read in the authors' paper transcription hosted at
  [ResearchGate](https://www.researchgate.net/publication/265799241_Symmetrized_Summation_Polynomials_Using_Small_Order_Torsion_Points_to_Speed_Up_Elliptic_Curve_Index_Calculus).
  Symmetric variables and later desymmetrization predate this proposal.
  That paper's extension-field experiments do not establish prime-field
  scaling here. Publisher/IACR/HAL/CiteSeer PDF access failed; the primary
  text transcription, not only search snippets, was read at section3.3.
- Mathlib cfc55fc58714a59dc06e6e79edc6d1d95140d3c2:
  `LinearAlgebra/Matrix/Charpoly/Coeff.lean:charpoly_fin_two`,
  `LinearAlgebra/Matrix/Trace.lean:trace_fin_two`, and existing finite
  matrix/ZMod arithmetic. The proof map names the exact inherited laws.

KB unavailable: no MCP status/search/theorem tools loaded; maintained
scripts/research-kb status exits127 with no executable. The intended four
queries are recorded in PROOF_MAP.md. The local lead atlas is absent and
its connected README fetch failed. No absence-of-prior-work or novelty
clearance claim follows. Initial arXiv HTML1304.1238v2 was404; the PDFv1
was read successfully. The Berkeley Sturmfels PDF was inaccessible;
Cox's directly read primary exposition supplies the needed precise law.

These sources support the algebraic design and identify missing solver
assumptions. They are not signed validation of the implementation, which
has not been built for this proposal.
