# Novelty audit: A11, A1, A4, H1/H2 (2026-10-19)

**Scope.** This closes the audit owed in README §6 for the four designs below
only. It is a literature search, not a proof of absence, and it does not call
any design "new". Status vocabulary: **prior art** (the same construction is in
a source), **partly prior** (a component or the framework is, the specific
combination here was not found), **not found in the sources checked**.

**Method and limits.** WebSearch, "standard" mode, seven queries on 2026-10-19
(one "extended" query on the prime-field line, because the first results were
thin), plus the repository's own
[`docs/ic/factor-base-encyclopedia/README.md`](../../docs/ic/factor-base-encyclopedia/README.md)
(its §0 description of the PKM tower base is the one used below).
**No full text of a cited paper was read.** The results were abstracts,
slides and search snippets. The PKC 2016 chapter's PDF is compressed and could
not be decoded by the fetch tool, Springer redirected to a login, and the
ePrint survey (2015/1022) returned 403. Every "what is the same / what
differs" below is therefore a statement about what the abstracts and the
encyclopedia say, and a full-text reading could change a status. A search
engine returning nothing is not evidence that nothing exists. Searches for
"Kummer tower", "norm of a function" and "Riemann-Roch" together with a
prime-field elliptic-curve factor base returned no source (the search did not
surface any such paper; it was not shown that none exists).

## Sources checked

| key | source | what was found there (from the abstract or snippet) |
|:--|:--|:--|
| Semaev 2004 | I. Semaev, *Summation polynomials and the discrete logarithm problem on elliptic curves*, ePrint 2004/031 | summation polynomials `S_m`; factor base defined by a bound on x-coordinates |
| Semaev 2015 | I. Semaev, *New algorithm for the discrete logarithm problem on elliptic curves*, ePrint 2015/310, arXiv 1504.01175 | zeroes of summation polynomials as a cubic Boolean system, binary fields; the introduction says no improvement for prime fields |
| Gaudry 2009 | P. Gaudry, *Index calculus for abelian varieties of small dimension and the elliptic curve discrete logarithm problem*, J. Symb. Comput. 44 (2009); ePrint 2004/073 | decomposition over a factor base of points with x in the base field, Weil restriction, extension fields |
| Diem 2011 | C. Diem, *On the discrete logarithm problem in elliptic curves*, Compositio Math. 147 (2011) | index calculus over extension fields |
| Nagao 2010 | K. Nagao, *Decomposition attack for the Jacobian of a hyperelliptic curve over an extension field*, ANTS-IX, LNCS 6197 (2010); ePrint 2007/112 | the decomposition test and the factors are obtained from a polynomial system through Riemann-Roch spaces, for hyperelliptic Jacobians, with (g, n) = (1, 3), (2, 2), (3, 2) computed |
| Joux–Vitse 2013 | A. Joux, V. Vitse, *Elliptic curve discrete logarithm problem over small degree extension fields*, J. Cryptology 26 (2013); ePrint 2010/157 | decomposition over `F_{p^n}` with a variant of F4; relations on `E(F_{p^5})`; Nagao-style reformulation where no summation polynomial exists (per Vitse's ECC 2011 slides) |
| FPPR 2012 | J.-C. Faugère, L. Perret, C. Petit, G. Renault, *Improving the complexity of index calculus algorithms in elliptic curves over binary fields*, Eurocrypt 2012; ePrint 2012/146 | point decomposition as one multivariate polynomial system solved by Gröbner bases, binary fields |
| PQ 2012 | C. Petit, J.-J. Quisquater, *On polynomial systems arising from a Weil descent*, Asiacrypt 2012 | heuristic analysis of the same systems; for prime fields, a factor base defined by a large-degree rational map broken into a chain of low-degree constraints (per a search summary) |
| HKY 2015 | M.-D. Huang, M. Kosters, S. L. Yeo, *Last fall degree, HFE, and Weil descent attacks on ECDLP*, CRYPTO 2015; ePrint 2015/573 | last fall degree, independent of monomial order; doubts on the first-fall-degree assumption in characteristic 2 |
| PKM 2016 | C. Petit, M. Kosters, A. Messeng, *Algebraic approaches for the elliptic curve discrete logarithm problem over prime fields*, PKC 2016, LNCS 9614 | summation polynomials plus a decomposition map `L` rewritten as low-degree equations, solved by Gröbner bases; the repository's encyclopedia describes the base as `V = L⁻¹(c)`, `L = L_t ∘ … ∘ L_1`, with membership in `V` written as `t` low-degree equations in auxiliary variables |
| APS 2018 | A. Amadori, F. Pintore, M. Sala, *On the discrete logarithm problem for prime-field elliptic curves*, Finite Fields Appl. 51 (2018); ePrint 2017/609 | one relation instead of several among factor-base points; fewer Gröbner computations; claimed to improve on PKM |
| Pintore | F. Pintore et al., ePrint 2017/1262 (a Gröbner-free index calculus building on APS) | no Gröbner basis computation |
| KW 2019 | Kousidis, Wiemers, arXiv 1906.05594 | first-fall-degree bound for Weil-descent systems from Semaev polynomials |

## A11: norm-chain / Riemann–Roch-function presentation of a Kummer fibre

- **Closest prior work.** Nagao 2010 and Joux–Vitse 2013 (decomposition as a
  Riemann–Roch-space system); PKM 2016 and PQ 2012 (membership in a base
  `V = L⁻¹(c)` as a chain of low-degree constraints).
- **Same.** Writing the decomposition of a target as the vanishing of a
  function from a Riemann–Roch space, with coefficients as unknowns, is
  Nagao's reformulation. The base here, `g·μ_{2^t}` (iterated squaring), is
  PKM's tower with `L` the squaring map, which is also this study's A0.
  A chain of low-degree steps for membership is PKM/PQ.
- **Differs.** A11 applies the function presentation over a prime field
  (Nagao and Joux–Vitse work over extension fields, via Weil restriction) and
  expresses membership of the `m` roots as divisibility `G(gX) ∣ X^{2^t} − 1`
  by `t` squaring steps modulo `G`, with `G` a norm factor of the function
  (`pencil_norm_factor`, `quartic_norm_factor`, `chain_sound` in Lean). That
  combination was not found in the sources checked. Measurement qualifies it:
  its shape null N11 equals it on D (README §5, F2), and its κ is above A0's
  (Amendment 4).
- **Status: partly prior** (the function presentation and the tower base are
  each in the sources; the norm-divisibility chain over a prime-field Kummer
  fibre was not found in the sources checked).

## A1: exponent-interval bases through bit products

- **Closest prior work.** PKM 2016 (a base of size `2^t` written with `t`
  auxiliary variables); the repository's own encyclopedia for the base
  families. Semaev 2004 for the x-coordinate-bound base, which is a different
  kind of set.
- **Same.** A1z has the same set as A0 (`g·μ_{2^t}`); only the encoding
  differs, and `exp_interval_identity` proves the encoding correct. Writing a
  `t`-bit exponent as boolean variables with `b_j² = b_j` is elementary
  binary expansion.
- **Differs.** A1g's set `{h·g^k : k < 2^t}` has no smoothness requirement on
  `ord(g)`, where a tower needs a `2^t`-th root structure. No source checked
  uses an exponent interval as the factor base of a prime-field ECDLP system.
- **Status: not found in the sources checked** for the interval base with
  bit products as a prime-field factor base; the encoding itself is
  elementary, and A1z reproduces a set already in PKM. It measured
  cost-unfavourable (κ 6.050 and 5.809 at `m = 2`).

## A4: parallel (sumset) towers

- **Closest prior work.** PKM 2016 (single towers); Gaudry 2009 and
  Diem 2011 (factor bases of points, with decompositions into sums).
- **Same.** Each summand's set is built from PKM-style Kummer towers, and the
  relation search is the same summation-polynomial setting.
- **Differs.** The base is a sumset `{u + v}` with `u` and `v` in two towers
  of depths `⌈t/2⌉` and `⌊t/2⌋` and `x = u + v`. No source checked builds the
  base as a sumset of two towers.
- **Status: partly prior** (towers are PKM; the sumset construction was not
  found in the sources checked). It measured cost-unfavourable (κ 4.902 at
  `m = 2`, 8.028 at `m = 3`).

## H1 and H2: the ablation hybrids

- **Closest prior work.** The same as A11 for H1's curve relation and as
  PKM 2016 for H2's summation polynomials and towers.
- **Same.** H1 uses A11's coefficient equations with Kummer towers on the
  roots; H2 uses A0's `S₃` summation with A11's squaring chain. Both are
  recombinations of components from the two designs above, built to ablate
  A11 (Amendment 4, C4), not as proposals.
- **Differs.** Only in how the components are combined.
- **Status: partly prior** (both consist of components of A0 and A11; the
  combinations were not found in the sources checked). Both measured
  cost-unfavourable (H1 κ 3.535 and 7.660; H2 κ 3.923 and 9.158).

## Summary

| design | status | closest prior work |
|:--|:--|:--|
| A11 | partly prior | Nagao 2010; Joux–Vitse 2013; PKM 2016; PQ 2012 |
| A1 (A1z, A1g) | not found in the sources checked (encoding elementary; A1z's set is PKM's) | PKM 2016 |
| A4 | partly prior | PKM 2016; Gaudry 2009; Diem 2011 |
| H1, H2 | partly prior (recombinations of A0 and A11) | PKM 2016; Nagao 2010 |

Not covered: designs 2, 3, 5–10, 12–16 and K1–K8. The reading of the papers
themselves (full texts) remains owed before any of these statuses is quoted
outside this repository. Related work on the cost question that this study
measures (HKY 2015, KW 2019, APS 2018) was found by the same search and not
compared with these systems' κ.
