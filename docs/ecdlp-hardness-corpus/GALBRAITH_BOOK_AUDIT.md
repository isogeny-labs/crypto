# Galbraith book audit and ECDLP promotion ledger

## Result

The user-supplied PDF is Steven D. Galbraith's *Mathematics of Public Key
Cryptography*, extended and corrected version 2.0, dated 31 October 2018. It is
696 pages and has SHA-256
`ca75ee62b96daa0d46faaa2d4dc28205683455ad3a2a48586b9b39299c25022d`.

The reproducible inventory contains:

- 717 distinct numbered or labeled results: 209 definitions, 188 theorems, 230
  lemmas, 46 corollaries, 3 propositions, 4 conjectures, 8 explicitly labeled
  heuristics, and 29 numbered algorithms;
- 375 numbered section and subsection headings; and
- exact PDF page, chapter, section, result type, and relevance class for every
  row.

The authoritative machine artifact is
[`galbraith-2018-inventory.json`](galbraith-2018-inventory.json). Rebuild it from
the supplied PDF with [`extract-galbraith-inventory.mjs`](extract-galbraith-inventory.mjs).
The main corpus validator checks its digest, counts, unique labels, page ranges,
algorithm sequence, and every result locator used by a promoted claim.

This resolves the earlier undercount. A loose text search returned 809 lines,
but that included later references such as “Theorem 5.1.20 shows …” and repeated
proof labels. The strict, deduplicated inventory has 717 results. Four damaged
headings are explicitly repaired and recorded: Theorems 11.1.12, 21.7.11, and
A.5.5, and Conjecture 19.6.5.

## Two corpus layers

The book inventory and claim graph answer different questions:

1. The inventory answers “what labeled material is in the book, and where?” It
   is exhaustive at the numbered-result and numbered-section level and does not
   copy theorem prose.
2. The claim graph answers “which facts change an ECDLP/CDH/DDH security
   dependency, under what model, and through which edge?” It contains concise
   paraphrases, hypotheses, formulas, evidence classes, and source locators.

Promoting all 717 labels into the dependency graph would incorrectly turn
background algebra, RSA facts, proof lemmas, and protocol definitions into
security dependencies. The inventory preserves completeness while the graph
preserves logical meaning.

## Chapter coverage

Abbreviations in the type column are the first three letters of the result type.

| Chapter | Title | Results | Type counts | Relevance tier |
|---|---|---:|---|---|
| 1 | Introduction | 2 | Def:2 | background |
| 2 | Basic Algorithmic Number Theory | 48 | Def:14, The:8, Lem:20, Alg:5, Cor:1 | mathematical dependency |
| 3 | Hash Functions and MACs | 2 | Def:2 | background |
| 4 | Preliminary Remarks on Algebraic Groups | 1 | Lem:1 | mathematical dependency |
| 5 | Varieties | 66 | Def:28, Pro:3, Lem:13, Cor:7, The:15 | mathematical dependency |
| 6 | Tori, LUC and XTR | 25 | Def:10, Lem:14, Cor:1 | mathematical dependency |
| 7 | Curves and Divisor Class Groups | 54 | Def:16, Lem:24, Cor:6, The:8 | mathematical dependency |
| 8 | Rational Maps on Curves and Divisors | 61 | Lem:28, Def:17, The:12, Cor:4 | mathematical dependency |
| 9 | Elliptic Curves | 70 | Def:15, Cor:7, The:30, Lem:17, Con:1 | mathematical dependency |
| 10 | Hyperelliptic Curves | 49 | Def:14, Lem:20, The:15 | mathematical dependency |
| 11 | Basic Algorithms for Algebraic Groups | 15 | Def:4, Lem:5, Alg:5, The:1 | ECDLP core |
| 12 | Primality Testing and Integer Factorisation using Algebraic Groups | 12 | Def:4, Lem:2, The:4, Alg:2 | mathematical dependency |
| 13 | Basic Discrete Logarithm Algorithms | 21 | Def:6, Lem:6, Alg:3, The:4, Cor:1, Heu:1 | ECDLP core |
| 14 | Factoring and Discrete Logarithms using Pseudorandom Walks | 28 | The:10, Def:2, Lem:3, Alg:6, Heu:7 | ECDLP core |
| 15 | Factoring and Discrete Logarithms in Subexponential Time | 19 | Cor:2, Def:4, Lem:3, The:7, Alg:1, Con:2 | ECDLP core |
| 16 | Lattices | 18 | Def:8, Lem:6, The:4 | mathematical dependency |
| 17 | Lattice Basis Reduction | 27 | Def:4, Lem:11, The:6, Alg:3, Cor:3 | mathematical dependency |
| 18 | Algorithms for the Closest and Shortest Vector Problems | 13 | Lem:5, The:6, Alg:1, Def:1 | mathematical dependency |
| 19 | Coppersmith's Method and Related Applications | 17 | The:9, Lem:1, Def:6, Con:1 | mathematical dependency |
| 20 | The Diffie-Hellman Problem and Cryptographic Applications | 11 | Def:4, Lem:3, The:4 | protocol context |
| 21 | The Diffie-Hellman Problem | 53 | Def:15, Lem:18, Cor:9, The:10, Alg:1 | ECDLP core |
| 22 | Digital Signatures Based on Discrete Logarithms | 6 | Def:4, The:2 | protocol context |
| 23 | Public Key Encryption Based on Discrete Logarithms | 9 | Def:5, The:2, Lem:2 | protocol context |
| 24 | The RSA and Rabin Cryptosystems | 17 | Def:5, Lem:5, The:7 | protocol context |
| 25 | Isogenies of Elliptic Curves | 17 | The:4, Cor:3, Def:6, Lem:3, Alg:1 | ECDLP core |
| 26 | Pairings on Elliptic Curves | 22 | The:7, Def:7, Lem:7, Alg:1 | ECDLP core |
| A | Background Mathematics | 34 | The:13, Lem:13, Def:6, Cor:2 | mathematical dependency |

The 175 `ecdlp_core` results are all retained in the inventory. The 495
`mathematical_dependency` results include the algebraic geometry, curve,
finite-field, smoothness, and lattice machinery on which the attack chapters
depend. The 43 `protocol_context` results separate a mathematical problem from
the exact assumption a protocol theorem uses.

## Load-bearing result crosswalk

The following table gives the main book-to-graph promotion decisions. “Existing”
means the first corpus already had a claim, usually from a primary paper;
“promoted” means this audit added a distinct missing node. The book remains a
primary-read corroborating source, not a substitute for the original papers.

| Book locator | Security content | Graph disposition |
|---|---|---|
| Theorem 13.2.5, p. 272 | Pohlig-Hellman cost when the order is smooth | Existing `ALG-POHLIG-HELLMAN` |
| Theorem 13.3.1, p. 273 | BSGS square-root time and storage | Existing `ALG-BSGS` |
| Theorem 13.4.5 and Corollary 13.4.7, pp. 277-278 | Generic DLP success and query lower bounds | Existing `LB-SHOUP-DLP` |
| Definition 13.5.1 and Lemma 13.5.6, pp. 278-279 | Representation/product DLP and generic lower bound | Promoted `DEF-MULTIDIMENSIONAL-DLP`, `LB-GENERIC-REPRESENTATION` |
| Definition 13.6.1 and Theorem 13.6.9, pp. 279-281 | Low-Hamming-weight DLP and its time-memory bound | Promoted `DEF-LOW-HAMMING-DLP`, `ALG-COPPERSMITH-LOW-HAMMING` |
| Heuristic 13.8.4, p. 283 | Wagner four-sum cube-root heuristic | Promoted `HEUR-WAGNER-FOUR-SUM` |
| Heuristics 14.2.9, 14.2.10, and 14.2.15, pp. 291-295 | Random-walk and collision constants | Existing `HEUR-RHO-RANDOM-MAPPING`, `ALG-POLLARD-RHO` |
| Theorem 14.2.23, p. 296 | Rigorous rho bound for random three-partitions | Promoted `THEOREM-RHO-THREE-PARTITION` |
| Theorem 14.3.2, pp. 298-300 | Distributed distinguished-point work/storage bound | Existing `ALG-PARALLEL-COLLISION` |
| Theorem 14.4.3, pp. 300-301 | Equivalence-class/automorphism speedup and representative cost | Existing `ALG-RHO-AUTOMORPHISM`, `HEUR-AUTOMORPHISM-SPEEDUP` |
| Theorem 14.5.5, pp. 306-307 | Heuristic kangaroo interval bound | Existing `ALG-KANGAROO-INTERVAL` |
| Theorem 14.7.5, pp. 317-318 | Two-dimensional Gaudry-Schost constant | Promoted `ALG-GAUDRY-SCHOST-TWO-DIMENSIONAL` |
| Definition 15.1.5, p. 324 | L-notation and subexponential regimes | Promoted `DEF-SUBEXPONENTIAL-L` |
| Theorem 15.6.1, p. 344 | Hyperelliptic divisor smoothness in a fixed-q, growing-genus regime | Inventory only; does not directly bound prime-field ECDLP |
| Theorem 15.8.2, pp. 347-348 | Summation-polynomial zero condition and degrees | Existing `THEOREM-SEMAEV-SUMMATION`, `THEOREM-SEMAEV-DEGREE` |
| Theorem 15.8.4, p. 350 | Diem's extension-field parameter window | Existing `ATTACK-DIEM-EXTENSION`; scope cross-checked |
| Section 15.9.3, pp. 351-352 | General-ECDLP index-calculus discussion and Xedni calculus | Existing index-calculus and Xedni branch; no theorem of a general subexponential attack |
| Corollary 21.1.18, p. 450 | Fixed-CDH/CDH equivalence under the chapter's reductions | Promoted fixed-base granularity in `DEF-FIXED-CDH` |
| Theorem 21.2.1, pp. 450-451 | Generic CDH success bound O(m²/r) | Existing `LB-SHOUP-CDH` |
| Lemma 21.3.1 and Theorem 21.3.8, pp. 451-454 | CDH random self-reduction and success amplification | Promoted two explicit nodes |
| Theorem 21.4.6, pp. 455-456 | den Boer reduction governed by factors of r-1 | Promoted `RED-DEN-BOER-FIXED-CDH` |
| Theorem 21.4.10 and Corollary 21.4.14, pp. 458-461 | Maurer auxiliary-curve reduction | Promoted `RED-MAURER-AUXILIARY-CURVE` |
| Theorems 21.5.1 and 21.5.6, pp. 462-464 | Static-DH auxiliary-power tradeoff | Promoted `ALG-STATIC-DH-AUXILIARY-POWERS` |
| Theorem 21.6.3, p. 466 | Perfect least-significant-bit oracle recovers DLP | Promoted `RED-DLP-LEAST-SIGNIFICANT-BIT` |
| Section 25.6, pp. 570-571 | DLP transport through an efficient isogeny and conductor-gap caveat | Promoted `RED-ISOGENY-DLP-TRANSPORT` |
| Theorems 26.2.3 and 26.3.3, pp. 574-577 | Pairing bilinearity and nondegeneracy | Promoted pairing foundation node |
| Theorems 26.4.1 and 26.4.2, p. 584 | MOV/FR field of definition and supersingular small embedding degree | Existing pairing-transfer branch |
| Lemmas 26.4.3 and 26.4.5, pp. 584-585 | Anomalous-curve reduction to differentials | Existing `ATTACK-SMART-TRACE-ONE` family branch |
| Section 26.5.2, p. 586 | Pairings solve co-DDH; ordinary DDH needs a suitable second direction | Promoted `RED-PAIRING-CO-DDH` |
| Section 26.6, pp. 586-587 | Pairing-friendly curves need hard source and target DLPs | Promoted `FACT-PAIRING-FRIENDLY-TWO-DLPS` |

## What is proved, assumed, or heuristic

The book itself makes several separations that are easy to lose in a flat list:

- Shoup-style bounds apply to generic encodings and oracle operations. Wagner's
  four-sum example beats a birthday scale precisely because visible XOR bits
  expose non-generic representation structure.
- The familiar Pollard-rho leading constant is heuristic. The rigorous
  three-partition theorem has an asymptotic square-root bound with logarithmic
  overhead and does not certify a fixed implementation's constant.
- Distributed rho changes wall-clock time under processor and communication
  hypotheses; it does not change birthday-order total work.
- The den Boer and Maurer results are reductions with arithmetic or auxiliary-
  curve hypotheses. They are not unconditional proofs that CDH and DLP are
  equivalent for every elliptic-curve family.
- Diem's theorem is for an extension-field parameter regime. It cannot be
  substituted into a fixed prime-field curve by setting the extension degree to
  one.
- Isogeny transport needs an efficiently computable map of degree coprime to the
  subgroup order. Equal cardinality of two curves does not by itself provide the
  reduction.
- A pairing makes co-DDH easy in a natural two-source setting. DDH inside one
  subgroup additionally needs a nontrivial self-pairing or a distortion map; it
  is not automatic for every ordinary curve.

## “Zeno calculus” terminology

The PDF contains no occurrence of “Zeno calculus.” It does discuss **Xedni
calculus**—“index” backwards—in Section 15.9.3. The graph already separates Xedni
from index calculus and records its canonical-height obstruction, finite formal
core, function-field density result, and unresolved selected-lift/adaptivity
bridges. The complete section is indexed even though the discussion is not a
numbered theorem.

## Remaining depth work

This pass closes book-level coverage at the labeled-result and section level and
adds 19 missing load-bearing claims. It also adds `RT-REPRESENTATION-FILTER`, which
turns Wagner's key escape mechanism into a falsifiable ECDLP research test: find a
coordinate-visible predicate whose qualifying pairs can be enumerated at the required
density in subquadratic time, or prove that the candidate is still generically
simulable. A deeper claim-by-claim formalization of all
495 mathematical-dependency results would be a separate algebraic-foundations
corpus: useful for proof assistants and citation tracing, but not necessary to
decide the current ECDLP attack dependencies. The inventory provides the stable
queue for that work without losing a single labeled result.
