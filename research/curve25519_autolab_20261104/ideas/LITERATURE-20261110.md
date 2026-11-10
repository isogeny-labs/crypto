# Literature check, 2026-11-10: sub-birthday single-instance ECDLP over prime fields

Scope: narrow literature check, no repository experiments, no new numbers computed
from repository data. Author: Idea Generator (Sonnet 5.5). Nothing here is a result
or changes hypothesis status. Every number below is tagged [pcc:unverified]: it is
copied from a source I read (or a search snippet, or memory, as labelled), or is my
own arithmetic (labelled), not a pcc-ledger claim.

## 0. Bottom line (two separate answers)

(a) Anything sub-sqrt(r) per single target, no precomputation, no multi-target [pcc:unverified]
amortisation, on a prime-field curve: NOTHING FOUND. The generic lower bound
(Shoup/Nechaev, restated in Bernstein-Lange p.4, which I read) makes it impossible for
any algorithm that does not use curve structure. The non-generic candidates found
(summation-polynomial index calculus, "initial minors") either say in their own text
that they do not beat generic methods (Petit-Kosters-Messeng) or have exponential
demonstrated cost (McGuire-Mueller, O(p log^2 p); minors paper p.3).

(b) Anything below the 0.886*sqrt(r) CONSTANT (generic, still Theta(sqrt(r))):
INCONCLUSIVE, with one literal candidate. 0.886 = sqrt(pi)/2 = 0.8862 [pcc:unverified]
is the standard rho-with-negation mean (non-negating rho = sqrt(pi/2)*sqrt(l) =
1.2533*sqrt(l) [pcc:unverified], Bernstein-Lange p.1/p.15, read; the 1/sqrt(2) for the
+-P orbit is from memory, Wiener-Zuccherato / Gallant-Lambert-Vanstone 1999, not
opened). Published average-case multiplication counts, NON-inverting group, read in
Bernstein-Lange 2012 p.15: rho 1.2533*sqrt(l); interleaved BSGS 1.3333*sqrt(l);
two-grumpy-giants-and-a-baby, EXPERIMENTAL chain weights 1.23046*l^1.5 (l = 65537,
n = 146) and 1.20545*l^1.5 (l = 1000003, n = 558), i.e. average multiplications about
1.230*sqrt(l) and 1.205*sqrt(l) [pcc:unverified]. So a published, memory-heavy
(O(sqrt l), "we have not found a low-memory variant", p.3) generic method is below
rho by about 2 to 4 percent in the non-negating setting, by experiment, not proof.
Bernstein-Lange say the ideas "can be adapted" to the inverting/negating case but
I found no printed negation-case constant in the pages I read. Galbraith-Wang-Zhang
(ePrint 2015/605, abstract read) state that "both the interleaved BSGS and
grumpy-giants algorithms have superior average-case running time" than rho in the
fully optimised (negation + batch inversion) context for small groups/intervals; I
could not open the body (PDF 403), so I do NOT know their constant or whether it is
below 0.886*sqrt(r) for a full-group single target. Naive extrapolation, my own
arithmetic and NOT published: 1.205/sqrt(2) = 0.852 [pcc:unverified]. This is the
one lead the Coordinator should have read in full before concluding anything about
(b).

Searches were web-search based (not an exhaustive database sweep), so "nothing found" [pcc:unverified]
means "nothing surfaced in about 35 queries/fetches", not "proved absent".

## 1. Source-access ledger (be honest about what was opened)

OPENED (page images read):
- Petit, Kosters, Messeng, "Algebraic approaches for the ECDLP over prime fields"
  (PKC 2016), pp. 1-3 of https://christophe.petit.web.ulb.be/files/16PKC_primeECDLP.pdf
  (abstract, intro, contributions, perspectives only; experiments NOT read).
- Bernstein, Lange, "Two grumpy giants and a baby" (ANTS-X, 2012),
  https://antsmath.org/ANTSX/bernstein/paper.pdf, pp. 1-4 and 12-15, 17-19
  (abstract, intro, Section 2 generic model, Section 6 and "Weighing the giants",
  appendix).
- Abdullah, Mahalanobis, "Minors solve the elliptic curve discrete logarithm
  problem", arXiv 2310.04132v1, pp. 1-3.
- Abdullah, Mahalanobis, Mallick, "A new method for solving the ECDLP",
  J. Groups Complexity Cryptology 12(2), arXiv 2005.05039, pp. 1-2.
- McGuire, Mueller, "A few more index calculus algorithms for the ECDLP" (Caen 2018
  slides/abstract, https://nitaj.users.lmno.cnrs.fr/Caen2018/DanielaMuellerCaen2018.pdf),
  pp. 1-2 (the full paper is ePrint 2017/1262, NOT opened).

OPENED (abstract / landing page only, via fetch tool summary):
- Semaev, ePrint 2015/310 (= arXiv 1504.01175).
- Amadori, Pintore, Sala, ePrint 2017/609.
- Hu, ePrint 2025/015 and ePrint 2024/1923.
- ePrint 2018/399 (abstract). NOTE: this is an analysis of Nesterenko's CTCrypt 2015
  algorithm, not the Kudo et al. paper; I first mislabelled it and corrected here.
- ePrint 2015/1022 (Galbraith-Gaudry survey): abstract only; PDF returned 403.
- ePrint 2015/605 (Galbraith-Wang-Zhang): abstract only; PDF returned 403.
- arXiv 2310.04132 abstract page.

SEARCH SNIPPETS ONLY (not opened; treat as secondary): [pcc:unverified]
- Huang-Kosters-Yeo ePrint 2015/573 (CRYPTO 2015) and Huang-Kosters-Yang-Yeo
  arXiv 1505.02532; Kosters-Yeo arXiv 1503.08001; Kousidis-Wiemers ePrint 2015/1121;
  arXiv 1906.05594 (first fall degree m^2 - m).
- Galbraith-Gaudry conclusion on prime fields (zbMATH-style review in a snippet).
- Kudo, Yokota, Takahashi, Yasuda, CANS 2018 (Springer redirected to login).
- "Complexity bounds on Semaev's naive index calculus method for ECDLP",
  J. Math. Cryptology, Oct 2020 (DOAJ fetch 403).
- Cheon-Hong-Kim, "Accelerating Pollard's rho algorithm on finite fields"
  (Asiacrypt 2008 / J. Cryptology 2012; Asiacrypt PDF fetch 403; keywords plus the
  description in Bernstein-Lange p.2 only).
- Mukhopadhyay-Sarkar ePrint 2021/043 (Montgomery multiplication + tag tracing).
- Bernstein-Lange, "Computing small discrete logarithms faster" ePrint 2012/458.
- Bos-Kleinjung-Lenstra negation-map paper (snippet: speedup about 1.29).
- Utrecht bachelor thesis 2025 and IIT Kanpur thesis (Johari 2025) (secondary).
- arXiv 2409.08784 "Double Index Calculus" (multiplicative group of F_p, not EC).

CITED FROM MEMORY (not opened this session): Shoup EUROCRYPT 1997 and Nechaev 1994
(also described in Bernstein-Lange p.4, which I read); Diem 2011/2013; Gaudry 2009;
Faugere-Gaudry-Huot-Renault and Faugere-Huot-Joux-Renault-Vitse (symmetries, small
torsion, extension fields; the latter cited in McGuire-Mueller refs, read);
Petit-Quisquater 2012; Joux-Vitse; Bernstein-Lange-Schwabe negation map; Yun 2015.

## 2. Findings by question

### 2.1 Sub-birthday single-instance algorithms

| Item | Claim | Status | Changes group ops per target? |
|---|---|---|---|
| Shoup 1997 / Nechaev 1994 (restated in Bernstein-Lange p.4, read) | Generic m-multiplication algorithm succeeds with probability O(m^2/l); the model counts slopes, d <= (m+3)(m+2)/2 finite slopes, about twice that with an inversion oracle | Accepted, proved | Floor for any structure-blind algorithm: no sub-sqrt(r). |
| Generic mean-cost floor, MY OWN integration of the p.4 slope bound (not published, not verified) | P(success after m) <= about m^2/(2l) non-inverting, about m^2/l inverting; mean >= integral of (1 - P) | Own derivation, label [pcc:unverified] | Floors about 0.943*sqrt(l) (non-inverting) and about 0.667*sqrt(l) (inverting) [pcc:unverified]. So 0.886 is not a generic floor; room for constant-factor generic improvement exists in principle. Note the non-inverting floor 0.943 is below the best published 1.205 and the inverting floor 0.667 is below 0.886. Verify before relying on it. |
| Bernstein-Lange 2012 two grumpy giants and a baby (read) | After (1.5+o(1))*sqrt(l) additions (no negation): BSGS 0.5625, truly random walk 0.6753, new method 0.71875 [pcc:unverified]. Average case, experimental: 1.23046 and 1.20545 *sqrt(l) vs rho 1.2533 and interleaved BSGS 1.3333 (p.15). "A 4-giants algorithm ... outperforms this 2-giants algorithm"; speculates a limit of (0.5+o(1))*m^2 slopes | Accepted (ANTS-X); generic, high memory O(sqrt l); the average-case claim is experimental | YES, a constant-factor reduction in group operations per target vs rho (about 2 to 4 percent, non-negating, experimental), but still Theta(sqrt(l)) and memory-bound. This is a literal "fewer ops than rho" hit for the non-negating generic setting. Does NOT show below 0.886*sqrt(r) because 0.886 already includes negation. |
| Galbraith-Wang-Zhang 2015/2017 (abstract only) | Negation map + Montgomery simultaneous inversion applied to BSGS; interleaved BSGS and grumpy giants "have superior average-case running time" than rho in the fully optimised context; for small groups or small intervals | Accepted (journal Adv. Math. Commun. 11(3), 2017) per snippets; constants unread | Possibly lowers group ops by a constant vs rho-with-negation. UNRESOLVED whether the constant is below 0.886*sqrt(r) for a full-group, single-target, large-r instance (the abstract frames it for small groups/intervals). Read the body before any claim. |
| Abdullah-Mahalanobis(-Mallick) "initial minors" (2020/2021, 2023) (read) | Reduce ECDLP to finding a zero minor of a matrix built from curve points; Conjecture A: set of initial minors of sub-exponential size; solved groups up to order 2^50 [pcc:unverified] (2020 abstract) | Unclear / unsupported: conjecture "based on experimental data" (2020 p.2); no independent verification or rebuttal found. 2023 p.3 text: to use Theorem 2.1 one must find points summing to O, "There is no way to predict which points might do that. So in practice the complexity is worse than solving the discrete logarithm problem by exhaustive search." | No evidence of fewer group ops. Each trial needs scalar multiples n_i*P. 2023 abstract hedges ("it seems ... could provide a potent attack"); intro asserts "a successful attack in the making" with no complexity data. |
| McGuire-Mueller (Caen 2018 slides, read; paper ePrint 2017/1262 not opened) | Summation-polynomial index calculus for prime fields without Groebner bases; evaluation of S_m in O(log^2 p) steps for m << p; stated total complexity O(p log^2 p) [pcc:unverified]; Magma experiments "faster than other prime field algorithms" | Preprint/talk; claim is relative to earlier index calculus only | No: O(p log^2 p) is worse than generic sqrt(p) (here p is the field size, about the group order). This is the closest thing to a "cheap decomposition test" (cheap S_m evaluation) but the number of tests needed (about p) kills it. |
| Hu, ePrint 2025/015 (abstract only) | "New method" for prime-field multiplicative DLP via index calculus; success probability "increased by nearly 2 times" [pcc:unverified] | Unclear, weakly supported (abstract only; targets F_p DLP, not EC) | No: not an EC group-operation result. |
| Hu, ePrint 2024/1923 (abstract only) | "Pseudo-index calculus" for prime-field ECDLP; concludes no subexponential method is known | Unclear; the author hedges | No. |
| Nesterenko CTCrypt 2015, analysis in ePrint 2018/399 (abstract only) | Average complexity "not less than that of Pollard's rho" | Accepted as a negative result (abstract-level) | No. |

Bernstein-Lange 2012 "Computing small discrete logarithms faster" (snippet:
1.77*l^(1/3) online, 1.24*l^(2/3) precomputation [pcc:unverified]) is
precomputation-based, excluded by the question's own condition.

### 2.2 Semaev summation polynomials over prime fields, decomposition test

- Semaev 2004 (ePrint 2004/031, snippet): introduced summation polynomials; [pcc:unverified]
  Petit-Kosters-Messeng (read, p.2): "Semaev did not provide an algorithm to compute
  relations".
- Petit-Kosters-Messeng 2016 (read pp.1-3): factor bases F = {(x,y) : L(x)=0} with L
  a composition of small-degree rational maps (needs p-1 with a large smooth divisor,
  or an isogeny / auxiliary curve with smooth subgroup); relations by Groebner bases.
  Own words: "Our algorithms are only practical for small parameters at the moment
  and their asymptotic analysis is limited by our understanding of Groebner basis
  algorithms"; "The complexity of our algorithms remains an open problem"; "At the
  moment all attacks are outperformed by generic discrete logarithm algorithms for
  practically relevant parameters". Accepted as a framework, not an improvement over
  rho. NIST P-224 falls in their first framework (p-1 smoothness); that says nothing
  about Curve25519.
- Amadori-Pintore-Sala ePrint 2017/609 (abstract): outperforms Semaev's original and
  the Petit et al. algorithm via one Groebner basis computation instead of several;
  no claim against rho.
- McGuire-Mueller: see 2.1 (cheap S_m evaluation, but O(p log^2 p) total).
- Kudo et al. CANS 2018 (snippet): accelerates that line and proves "a limitation of
  the acceleration, in both theory and practice".
- J. Math. Cryptol. 2020 (snippet): naive Semaev index calculus cannot beat generic
  methods; covers only the naive variant.
- Galbraith-Gaudry, Des. Codes Cryptogr. 78 (2016), ePrint 2015/1022 (abstract
  read; conclusion via secondary review snippet): prime-field curves remain
  unaffected; obstacle is the lack of an efficient decomposition algorithm.
- Symmetries / small torsion (Faugere et al., EUROCRYPT 2014; cited in
  McGuire-Mueller refs, read): extension fields. Cofactor-8 Montgomery curves have
  rational 2-power torsion, but I found no paper using it for a prime-field
  decomposition.

Conclusion for 2.2: the named obstruction is the decomposition step (relation search
over F_p costs about the budget it saves). No published cheap decomposition test with
a competitive total cost over F_p was found.

### 2.3 Semaev 2015 and its rebuttals

- Semaev, ePrint 2015/310 (abstract read): binary fields only; heuristic
  2^(c*sqrt(n ln n)), c about 1.69 [pcc:unverified], under a "first fall degree"
  assumption (regularity degree <= 4) with Magma experiments; claims to beat rho on
  some binary FIPS curves. Nothing claimed for prime fields. Status: heuristic.
- Huang-Kosters-Yeo, CRYPTO 2015 (ePrint 2015/573, snippet): last fall degree;
  computational evidence "raises doubt on the validity of the first fall degree
  assumption" in characteristic 2. Kosters-Yeo (arXiv 1503.08001, snippet):
  first fall degree 2 for Weil descent of the 3rd summation polynomial, "raising
  doubt". Kousidis-Wiemers (ePrint 2015/1121) and arXiv 1906.05594: first fall
  degree bounds, m^2 - m [pcc:unverified] (snippets). No single paper found that
  formally refutes Semaev's heuristic: status is "unproved, doubted, no practical
  break; characteristic 2 only". Irrelevant to prime fields (no Weil descent).

### 2.4 Tag tracing and batched steps that lower group ops per rho step

- Tag tracing, Cheon-Hong-Kim (Asiacrypt 2008, J. Cryptol. 2012): multiplicative
  group of F_p. Bernstein-Lange p.2 (read): looks ahead one rho step in F_p* and
  combines two multiplications into one at a memory cost. No elliptic-curve
  analogue found in any source; Bernstein-Lange stress that multiplication counts are
  blind to such optimisations. Mukhopadhyay-Sarkar (ePrint 2021/043) still F_p*.
  Effect: work per step in F_p* only. I could not open the original (403).
- Batch affine inversion (Montgomery trick), negation map: Bos-Kleinjung-Lenstra
  (snippet: speedup only about 1.29 [pcc:unverified] due to fruitless cycles),
  Bernstein-Lange-Schwabe (memory), Galbraith-Wang-Zhang (snippet). These change
  field-operation cost per addition and the orbit size; walk steps per target stay
  Theta(sqrt(r)). The negation 1/sqrt(2) is already in the 0.886 reference.
- Grumpy giants: see 2.1 (constant-factor group-op reduction, generic, high memory).
- No batched-step or look-ahead method found that reduces EC group operations per rho
  step below one point addition.

### 2.5 2023-2026 preprints claiming prime-field EC index-calculus gains

- Abdullah-Mahalanobis arXiv 2310.04132 (2023): see 2.1; unclear, not shown [pcc:unverified]
  peer reviewed by my evidence, no independent confirmation or rebuttal found; the
  paper's own text concedes the naive relation search is worse than exhaustive
  search. Group sizes were not stated in the pages I read.
- Hu ePrint 2024/1923, 2025/015: see 2.1.
- No 2023-2026 preprint with a verified prime-field EC improvement turned up.
  A 2025 Utrecht bachelor thesis and a 2025 IIT Kanpur thesis (secondary snippets)
  repeat that index calculus has not carried over to prime-field curves. No published
  rebuttal of the minors preprint found; absence of a rebuttal is not evidence of
  correctness.
- Quantum resource-estimate preprints (arXiv 2604.02311) surfaced: out of scope.

## 3. Bearing on the program (labelled interpretation, not a result)

1. Group-operation count per target: the only literature levers that move it are
   (a) the generic constant (negation/orbit, Bernstein-Lange anti-collision, memory
   heavy BSGS-like hybrids), bounded below by the generic model at about 0.667 to
   0.943 of sqrt(r) per my own reading [pcc:unverified], and (b) non-generic
   relations, for which no competitive prime-field decomposition is published.
2. Tag tracing, batching, Montgomery-form tricks change native/field cost, not the
   group-operation count; tag tracing has no EC form found.
3. Cofactor-8 Montgomery curves have no extra automorphism beyond negation, so the
   automorphism-orbit lever sqrt(pi*r/(2|Aut|)) gives only the 1/sqrt(2) already in
   the reference (formula from memory; not opened).
4. Any proposal built on summation polynomials over F_p must name the Lean
   obligations it creates (decomposition-test soundness; per-decomposition cost
   bound); the literature offers no theorem to cite for either.
5. A grumpy-giants-with-negation proposal would be a generic, memory-heavy,
   constant-factor idea. Its cost band must price memory and the Bernstein-Lange
   "overhead" caveat (p.2 of their paper), and its closest baseline is
   Galbraith-Wang-Zhang, which I have not read in full.

## 4. Sources checked in this repository (Grep/Glob only; research-kb MCP not available)

- /home/user/crypto/research/summation_polynomials_20260914/ (README.md, RESULTS.md,
  contract.json, run-01/, confirmation-01/, harnesses/summation_polynomial_bench.rs):
  existing summation-polynomial benchmarks on toy primes (p67, p271 file names).
  Hits for "Semaev|Petit|McGuire|Amadori" only; zero hits for "tag tracing|grumpy|Cheon" in RESULTS.md.
- /home/user/crypto/research/curve25519_cofactor_shared_20261102/ (README.md,
  PROTOCOL.md): `pkm-semaev:lifts=2` shared F4 system on the k = 20 Curve25519
  miniature. Not read beyond the title and test lines.
- /home/user/crypto/research/rho_batchinv_20261022/ and rho_parity_20260915/:
  matched on rho/batch-inversion terms in logs; zero `.md` hits for "tag tracing|grumpy".
- A first repo-wide Grep of /home/user/crypto/research timed out at 40 files; the
  result is incomplete. The sibling crypto-autoresearcher lead atlas
  (`knowledge/leads/ecdlp/`) was NOT found at /home/user/crypto-autoresearcher or via
  /home/user/*/knowledge/leads/ecdlp/, so I did not check it. The Coordinator should
  check it, since this file's "not found" claims have not been cross-checked against
  it.

## 5. Not done (for the Coordinator to decide)

- research-kb MCP queries (`kb_status`, `search_knowledge`, `search_theorems`).
- Forward-citation sweep (Google Scholar / Semantic Scholar) of Petit et al. 2016,
  Semaev 2015, Bernstein-Lange 2012 and Galbraith-Wang-Zhang for 2023-2026 hits.
- ePrint date-range sweep for 2023-2026.
- Reading the body of Galbraith-Wang-Zhang (ePrint 2015/605), the Petit et al.
  experiments section, McGuire-Mueller ePrint 2017/1262, and the original Cheon-Hong-
  Kim paper.
- Research visuals (report, graph, PDF) and pcc claim ids: none produced.
