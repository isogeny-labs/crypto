# Primary sources and existing declarations

- Diem, *Index calculus with double large prime variation for curves of small
  genus with cyclic class group*, arXiv:math/0606607v1, sections2--3, pp3--10:
  https://arxiv.org/pdf/math/0606607 . Read the graph construction, independent
  alpha/beta draws and target coefficient recovery. Its theorem requires
  genus>=2 and cannot be specialized by setting genus1. Graph recombination
  and the informative coefficient test are classical. [pcc:unverified]
- Diem, *On the discrete logarithm problem in elliptic curves*, Compositio147
  (2011), sections1,2.3/Proposition2.12: https://doi.org/10.1112/S0010437X10005075 .
  Read decomposition/selection and the label-independence proof directly. The
  extension-degree family/subfield base does not supply a prime-field oracle.
  The current work retains the independent-label/selection contracts.
  [pcc:unverified]
- Shoup, *Lower Bounds for Discrete Logarithms and Related Problems* revised
  EUROCRYPT97 paper, p1 generic model, rendered from author's PDF:
  https://www.shoup.net/papers/dlbounds1.pdf . PDF text extraction is corrupt;
  actual page rendering was inspected. Generic encodings do not include our
  coordinate root oracle. This paper is scope context, not a proof of the
  new oracle model. [pcc:unverified]
- Original-map inverse and signed-fibre reasoning: preceding
  torsion-column-transport-v1/DERIVATION.md and Rust lib.rs; exact frozen native
  source/claim remain unchanged. No new map or cofactor validation is claimed.
- Mathlib cfc55fc5: Finset.card_biUnion_le/card_product/sum_le_sum and Matrix
  vector products. CryptoLean current main ff94d4295f includes
  Combinatorics.LargePrimeGraph (singleton peeling), Group.SingleRelation
  (mixed-label fibre/recovery) and Curve.LineIncidence. Read those before the
  proof plan. New generic laws go in a separate owner-floor worktree, not a
  copy in this study. [pcc:unverified]

Retrieval gaps: no loaded KB MCP tools; previously tested wrapper127, no
unchanged retry. ePrint2004/153 and two alternate Gaudry delivery URLs refused;
the accessible Diem primary successor supplies the graph/label assumptions.
No claim that this incidence argument or successor is novel follows from the
KB gap. Literature text and third-party papers are not redistributed here.
[pcc:unverified]
