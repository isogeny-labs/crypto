# Literature search and coverage ledger

Search date: 2026-11-01

Purpose: establish the theorem/assumption/bound corpus, not rank papers by citations

## Search method

The search used four evidence layers:

1. repository bibliographies and theorem audits (`papers/ecdlp-*`, the 96-entry
   six-month source register, the scoped-boundaries proof map, the lead atlas, and
   prior literature reviews);
2. exact-title and DOI queries to OpenAlex, Crossref, and HAL;
3. publisher, author, standards-body, and arXiv PDFs for primary reading;
4. source-code traces for claims about the live implementations.

Broad keyword search was used only for discovery because it was noisy (for example,
OpenAlex mixed astronomy papers into an ECDLP query). A claim entered the graph only
after exact identity and scope checking. Direct PDF failures are retained as
`metadata_only` or `secondary_only` in `sources.tsv`; they are not silently treated as
read.

Representative search strings included:

- `elliptic curve discrete logarithm lower bound generic group preprocessing`
- `Pollard rho parallel collision search automorphism negation map`
- `MOV Frey Rueck anomalous trace one supersingular embedding degree`
- `Semaev summation polynomial index calculus Gaudry Diem fixed extension`
- `Weil descent ECDLP first fall degree last fall degree Groebner`
- `prime field ECDLP PKM APS small abscissa`
- `xedni calculus elliptic curve discrete logarithm`
- `isogeny reduction same order endomorphism ring ECDLP`
- `quantum resource estimate elliptic curve discrete logarithm improved circuit`

## Primary texts read in this pass

The exact hashes and citations are machine-readable in `sources.tsv`. Primary text was
read for:

- Shoup's generic-group DLP/CDH bounds;
- Corrigan-Gibbs--Kogan's advice and preprocessing tradeoffs;
- Pohlig--Hellman's prime-power decomposition;
- Smart's trace-one/anomalous attack;
- Gaudry--Hess--Smart Weil descent;
- Diem's extension-field theorem;
- Shor's quantum DLP algorithm;
- van Oorschot--Wiener's parallel collision search;
- FGHR's symmetry and point-decomposition analysis;
- Freeman--Scott--Teske's embedding-degree taxonomy;
- NIST SP 800-186 curve criteria;
- Roetteler et al. (2017), Haener et al. (2020), Gouzien et al. (2023), and
  Haener et al. (September 2026 preprint) for successive quantum-resource layers;
- Jacobson--Koblitz--Silverman--Stein--Teske's primary analysis of xedni.

The GHS, Smart, Shoup, CGK, Diem, Shor, quantum, and xedni conclusions in the graph
are scoped to the precise text read. MOV, Frey--Rueck, Pollard's original paper,
Silverman's original xedni proposal, Gaudry 2009, Joux--Vitse, PKM, FPPR, and several
solver papers remain visibly marked metadata/secondary where primary access failed.

## Coverage by research branch

| Branch | Load-bearing literature located | Corpus conclusion |
|---|---|---|
| Group/order foundations | Hasse bound; subgroup/cofactor definitions; Pohlig--Hellman; NIST criteria | Security is controlled by a large prime subgroup, not field size or total curve order alone |
| Generic lower bounds | Nechaev/Shoup line; Shoup 1997; CGK 2018 | Rigorous birthday lower bounds are oracle-model statements, not unrestricted classical lower bounds |
| Generic upper bounds | BSGS; Pollard rho/lambda; Teske walks; van Oorschot--Wiener; automorphism and negation work | Square-root upper bounds and constants depend on memory, interval width, useful-collision rate, walk quality, parallelism, and quotient handling |
| Preprocessing/many target | Bernstein--Lange; CGK | Advice size, setup work, target count, and target independence must be explicit |
| Pairing transfers | MOV; Frey--Rueck; embedding-degree taxonomy | Pairing usefulness is governed by the minimal target field and its finite-field DLP cost |
| Exceptional curves | Semaev/Satoh--Araki/Smart anomalous attacks; supersingular pairing facts | Trace-one and low-embedding-degree families are excluded, not evidence against well-screened ordinary curves |
| Weil descent to Jacobians | GHS; Jacobson--Menezes--Stein; later hyperelliptic algorithms | Powerful on specified extension-field families, but map kernels, genus, and extension structure are decisive |
| Summation-polynomial IC | Semaev; Gaudry; Diem; Joux--Vitse; FGHR; FPPR; Petit--Quisquater; HKY; Galbraith--Gebregiyorgis | Every exponent rests on distinct yield, dimension, solver-degree, rank, linear-algebra, and descent obligations |
| Prime-field algebraic routes | PKM; APS; McGuire--Mueller; Semaev 2015; repository PKM and solution-count work | No verified end-to-end prime-field sub-rho exponent is established; solver scaling remains live |
| Xedni / lifting | Silverman proposal; JKSST analysis; exact repository function-field density | Historical characteristic-zero xedni is conditionally obstructed by canonical height; the frozen function-field variant has exact density Theta(p^-3) |
| Isogeny transport | Jao--Miller--Venkatesan; Galbraith isogeny work; repository correction history | Random reducibility is scoped by endomorphism ring/GRH; coordinate-generated transport is not covered by a generic same-cost theorem |
| Quantum | Shor; Proos--Zalka line; Roetteler 2017; Haener 2020; Gouzien 2023; 2026 trapped-ion preprint | Asymptotic hardness is gone on a fault-tolerant quantum computer; physical cost is an architecture scenario, not one canonical number |
| Protocol layer | CDH/DDH reductions; gap-DH; Pointcheval--Stern; standards validation | “ECDLP-based” is too coarse: each protocol needs its actual assumption, proof model, reduction loss, and validation boundary |

## Important literature located but not yet promoted to load-bearing nodes

These works are relevant for later expansion, but this pass did not promote a precise
claim without primary verification or because the current graph already captures the
branch through a stronger/source-read theorem:

- Nechaev's generic DLP bound (historical predecessor to Shoup);
- Satoh--Araki and Semaev's independent anomalous-curve attacks (Smart is the
  primary-read representative here);
- Wiener--Zuccherato, Gallant--Lambert--Vanstone, Duursma--Gaudry--Morain, and
  Bernstein--Lange--Schwabe on automorphism/negation constants;
- the original Gaudry 2009 paper and Joux--Vitse 2013 small-extension refinement;
- Faugere--Perret--Petit--Renault, Petit--Quisquater, Huang--Kosters--Yeo,
  Galbraith--Gebregiyorgis, PKM, APS, Semaev 2015, and McGuire--Mueller on
  polynomial-system variants;
- Gaudry--Thome--Theriault--Diem and Diem--Thome on large-prime and genus-three
  index calculus;
- Galbraith--Granger--Merz--Petit on subfield curves;
- Proos--Zalka and later binary-field quantum-circuit papers;
- 2024--2026 quantum preprints beyond the September 2026 trapped-ion paper. These
  are discovery candidates, not assumed improvements, until their circuits and
  architecture assumptions are checked.

## Negative and non-implication checks

The search explicitly looked for common overclaims:

- A Shoup bound does **not** cover coordinate inspection, pairings, polynomial
  systems, or a free Frobenius oracle.
- A fast decomposition oracle is not an end-to-end ECDLP algorithm.
- First-fall degree is not automatically the maximum Groebner solving degree.
- A fixed-extension asymptotic does not extend to the prime-field case by setting
  `n=1`.
- A measured `ecbench.bound/v1` record is an upper measurement for one method and
  domain, never a mathematical lower bound on ECDLP.
- Small embedding degree alone is not the full MOV cost; the target finite-field DLP
  must be priced.
- Isogeny equivalence of group orders is not a free same-cost transport theorem for
  coordinate-aware factor bases.
- Xedni is not another name for Semaev index calculus; it is a global lifting attack
  with a different, height-based obstruction.
- Logical quantum gates/qubits are not physical qubits or elapsed time; physical
  estimates inherit architecture and error assumptions.
- ECDLP hardness does not automatically imply CDH or DDH hardness, nor does an
  invalid-curve key leak solve ECDLP on valid inputs.

## Search limitations and next searches

This is a comprehensive load-bearing map, not a claim that every ECDLP paper ever
published is represented. Paywalled/blocked primary texts are identified above. The
next highest-value literature work is:

1. primary verification of the original MOV/Frey--Rueck and Gaudry/Joux--Vitse
   theorem statements;
2. a modern multi-target/nonuniform DLP survey tied to the exact CGK experiment;
3. primary protocol proofs for Schnorr/EdDSA, ECDSA, ECDH/HPKE, and pairing
   protocols;
4. a reproducible comparison of 2020--2026 quantum circuits under one logical and
   physical resource schema;
5. formalization or independent review of the repository's generic-table,
   yield-conservation, xedni-density, and isogeny-oracle arguments.
