# LEAD-11 — McGuire–Mueller Gröbner-free algorithms (ePrint 2017/1262): full-text verification and classification

**Verdict.** unverifiable-from-here. The full text was not obtained (§9). Provisional classification from second-hand evidence and counting, not a finding: Algorithm 2 coincides in mechanism with the repository's enumeration oracles, Algorithm 1 is the same oracle with an algebraic evaluation step, and neither is an exponent candidate.
**Confidence.** low for anything the paper says (no primary text; the mechanism is relayed by one earlier repository session's note and by an inconsistent search summary); medium that the classification survives the text, because it rests on a counting argument that does not depend on the paper's details (§4, §5).
**Expected gain if it works.** None. The relayed Algorithm 2 cost, `O(p)`, is a worse exponent than the current prime baseline (Table B, `m = 2`, `r^{2/3}`). Unverified desk check: about 48× (`log₂ r = 20`) to 119× (`23.4`) above the Table B IC rows and about 400× to 1,600× above matched rho in S.
**Cost of the discriminating test.** About 0.5 person-day and under 1 CPU-hour, no memory, once a host that can download the (CC BY) PDF is available (§6).

## 1. Mechanism (precise)

**What is known from the source.** The ePrint abstract page (read, §9) says: an index calculus algorithm for the ECDLP that uses a technique for fast evaluation of summation polynomials and, unlike all other summation-polynomial algorithms, involves no Gröbner basis computation; a second algorithm with neither Gröbner bases nor summation polynomials; a complexity estimate and extensive computational data. History: received 2017-12-31, second revision 2018-08-08 ("Presentation improved and Lemma 5.10 added"), CC BY 4.0. Nothing else from the paper was read.

**What is relayed (second-hand, not verified by me).** `papers/ecdlp-index-calculus-program/LITERATURE.md:51-67` records a reading of the Caen 2018 slides on 2026-10-15: factor base `F = {a_i P + b_i Q}` (after Amadori–Pintore–Sala); `S_m` evaluated by iterated resultants with `S₃` in `O(log² p)`; and "another variant": choose `m−1` points of `F` and test whether their sum lies in `F`, complexity `O(p)` for `s ≥ (m−2) log₂ p`. A search summary today repeated "`O(p)` for `m ≪ s` and `s ≥ (m−2)log₂ p`" and "still exponential, better than other Gröbner-based prime-field algorithms"; a later search denied finding it. The meaning of `s` is not defined in either record.

**The mathematics the two relayed algorithms must live in** (prime field `F_p`, prime order `N ≈ p`, base `F` of `B = |F|` points, signs folded). Decomposing a target `R` into `m` base points is `S_{m+1}(x₁,…,x_m,x_R) = 0` with `x_i ∈ x(F)`.

- `m = 3`: `S₄`, degree 4 in each of 4 variables, 3 unknowns, one equation over `F_p`. The grid `x(F)³` has `B³` points and about `B³/p` solutions, so `B ≈ p^{1/3}` (`2^8` at `p ≈ 2^24`).
- `m = 4`: `S₅`, degree 8 per variable, 4 unknowns.
- There is no Weil descent in a prime field, hence no `n`-dependent degree of regularity. A random `F` has no algebraic description either: putting it in a Gröbner system needs field equations `∏_{f∈F}(x − f)`, of degree `B`. That is why Gröbner bases do not help for a random base, and why avoiding them costs nothing in solving degree and everything in search.
- Evaluation without symbolic `S_m`: `S_{m+1}(x₀,…,x_m) = Res_X(S_{k+2}(x₀..x_k, X), S_{m−k+1}(X, x_{k+1}..x_m))`. For `m = 3` this is the resultant of two quadratics in `X`. Unverified desk count: about 19 field multiplications per numeric `S₄` evaluation. This is the same numeric-resultant device `line_oracle.rs` already uses.

**Counting for the relayed Algorithm 2** (`m−1` base points summed, membership tested). A random signed sum lies in `±F` with probability about `2B/N`. About `B` relations are needed, so about `N/2` tuples regardless of `B` and `m`, at one group addition per tuple (Gray-code enumeration). That is the slides' `O(p)`, and `S ≈ B` memory. With a table of pair sums (`S = B²` keys) and one probe per fresh point, success per trial is about `B²/N`, so `T ≈ N/B` and `S·T² ≈ N²`: at `B = N^{1/3}` that is `r^{2/3}`, the repository's Table B `m = 2` baseline.

## 2. Prior art: literature

- McGuire, Mueller, *A New Index Calculus Algorithm for the ECDLP and Summation Polynomial Evaluation*, IACR ePrint 2017/1262 (rev. 2018-08-08), https://eprint.iacr.org/2017/1262. **abstract-only.** Claim taken: the two algorithms exist and avoid Gröbner bases; nothing about their cost.
- Mueller, *A few more index calculus algorithms for the ECDLP*, Caen 2018 (20–22 June), slides `…/Caen2018/Caen2018Slides/Caen2018Mueller.pdf` and paper `…/DanielaMuellerCaen2018.pdf` on `nitaj.users.lmno.cnrs.fr`. **recalled (second-hand via the repository note above; the server returned 504 today).** Claim taken, unverified: Algorithm 2 is sum of `m−1` base points plus membership, `O(p)`.
- Mueller, UCD algebra and number theory seminar, Nov 2015, "Summation polynomials and the elliptic curve discrete logarithm problem". **abstract-only via a search-result summary** (the page returned 403): the talk adapts the method to prime fields.
- McGuire, Mueller, *Some Results on Linearized Trinomials that Split Completely*, arXiv:1905.11755v3 (2019). **read-at-source** (PDF, §6). Claims taken: the authors measure ECDLP progress against generic `q^{n/2}`; trinomial quasi-subfield polynomials with `n = d²−d+1` "will not beat generic algorithms"; Remark 6.1 conjectures a four-term variant at `q = 2`, `m = 4`, `d ≈ n/5` reaching `q^{0.4n}` for `n > 500`, conditional on the Huang–Kosters–Petit–Yeo–Yun complexity formula (which I did not check); `S_m` "have only been computed for `m ≤ 9`". It does not cite 2017/1262 in its references. Affiliation UCD; IRC postgraduate scholarship, so a PhD student around 2019. No thesis found.
- Semaev, ePrint 2004/031, definition of `S_{m+1}`. **recalled** (restated at source in 1905.11755 §6.2).
- Amadori–Pintore–Sala, *On the DLP for prime-field elliptic curves*, FFA 51 (2018), ePrint 2017/609. **recalled.** Instance-derived base, one Gröbner call.
- Petit–Kosters–Messeng, PKC 2016. **recalled.** Their own conclusion, as recorded in `crypto-autoresearcher/inputs/lit_search_20260724.md:74`: all attacks are outperformed by generic algorithms at relevant sizes.
- Corrigan-Gibbs–Kogan (EUROCRYPT 2018), `S·T² = Ω̃(εN)`; Corrigan-Gibbs–Henzinger–Wu ePrint 2026/384 (SGGM); Dinur–Keller–Marmor arXiv 2505.00894. **recalled / second-hand** through `research/lit2026_w3_ggm_audit_20261011/README.md`, which read the SGGM PDF.
- Citers (Semantic Scholar API, metadata only): Yokoyama–Yasuda–Takahashi(–Kogure), JMC 2020; Meneghetti–Sala–Taufer 2019/2020 ("none of them seem to significantly reduce the problem complexity"); Schneider–Auer–Wagner, TCHES 2025. None describes the algorithms.
- Hu, ePrint 2024/1923 (EasyChair 15613). **abstract-only.** No mention of McGuire–Mueller; concludes no subexponential prime-field method is known.

## 3. Prior art: these repositories

- `crypto-autoresearcher/knowledge/literature/KN-LIT-6966ea.md`: abstract-only note; supports nothing beyond the abstract (corrections in §8).
- `crypto/papers/ecdlp-index-calculus-program/LITERATURE.md:49-67`, `sections/rootfinding.tex:42-43`, `sections/theory.tex:98-101`, `sections/openproblems.tex:37`: the manuscript already states that the two-summand root-finding oracle is the second McGuire–Mueller algorithm "in mechanism". Supports the provisional classification; the claim is only as good as the 2026-10-15 reading.
- `crypto/src/cryptanalysis/ic_framework/rootfind.rs` header: `rootfind-mitm` (pair table, `2A` probes a target) and `rootfind-pairs` (memoryless `U ∓ P_j`, membership). Charged per group addition; lookups counted, not charged. Supports: Algorithm 2 is `rootfind-pairs` / `mitm`.
- `crypto/research/pdp_root_finder_20261012/README.md`: every root-finder slope equals its floor (G3 fails, class engineering). `rf-pairs` within 0.91–1.18× of `group-scalar`; PS-0's algebraic pairs-and-solve (quartic plus `gcd`, no Gröbner basis) 8.7 ms against `rf-pairs` 0.2 ms at (23, 8), about 43×. Closes the Gröbner-free-by-evaluation gain as already taken, and shows the algebraic evaluation route is the slower one.
- `crypto/src/cryptanalysis/prime_fast.rs` header, item 4: relation collection "without summation polynomials", `S₃` root `X = x(R ± F_i)`; `S₄` roots matched against a table `x(F_j ± F_k)`. Supports: Algorithm 2 and its table variant exist natively.
- `crypto/src/cryptanalysis/line_oracle.rs` header: `S₃` restricted to a line over `F_{p^k}`, nine evaluations plus interpolation, `4 × 4` Sylvester resultant, `O(log p)` a target. The one place a Gröbner-free numeric-resultant oracle beats enumeration, because the base has algebraic structure. Distinguishes: a random prime-field base has none.
- `crypto/src/cryptanalysis/line_s4_oracle.rs` header: three summands on a Frobenius line by Macaulay plus eigenvalues, cost independent of `p`. Same lesson; unrelated to a random prime-field base.
- `crypto/docs/ic/LEADERBOARD.md`, Table B: prime rows at `log₂ r = 10–23.4`, IC 8.4–22.8 GAE per `√r`, 5.5–13.5× matched rho (all "unvalidated (pre-gate)").
- `crypto/research/ecdlp_leads_20261018/README.md` L4 and `research/notes/ecc2k130/RESEARCH_ECC2K130_ROUTE_TARGETS.md:1102-1106`: McGuire–Mueller listed as an unchecked prime-field algorithm; the latter quotes the authors as saying theirs "are worse than … Pollard-Rho". Provenance of that quote is not recorded.
- `crypto/research/lit2026_w3_ggm_audit_20261011/README.md`: the mapping of factor-base oracles onto the SGGM `δ`, marked as interpretation, not a theorem of the paper.
- `crypto/research/ic_next10_20261014/08-quasi-subfield/README.md:53`: the "McGuire–Mueller trinomial bound" is the 2019 arXiv paper, a different paper from 2017/1262; consistent with `KN-LIT-4fe9d2.md:52-54`.

## 4. Why it could lower the Gröbner cost or the degree of regularity

It does not lower either. It removes the Gröbner step by replacing solving with search, which is what the repository's root-finders already did (stage-level 10⁵–10¹² below F4 counted, end-to-end S moved by at most 1.26×). The quantitative position of each relayed algorithm:

- **Algorithm 2, memoryless:** `T ≈ N/2` group additions, `S ≈ B`. Assumption: success probability `2B/N` per tuple, i.e. a uniform-landing base; KN-FIND-007 (yield conservation) says base geometry does not change it.
- **Algorithm 2 with a pair table:** the repository's `mitm`; `T ≈ N/B`, `S = B²`, `S·T² ≈ N²`. No exponent below the table floor was measured (G3 fails).
- **Algorithm 1:** per candidate, numeric `S_{m+1}` evaluation costs tens of multiplications (about 19 for `S₄`, unverified desk count), against about 2.5–10 for the group-law candidate in `rf-mitm` / `rf-pairs`. If it enumerates `F^m` it pays an extra factor `≈ B`; if it enumerates `F^{m−1}` and solves the last variable (quartic plus `gcd`) it is PS-0's mechanism, measured about 43× slower than `rf-pairs` in the binary analogue. Under AGENTS.md §5a the field multiplications must be priced at 0.1 and 1 GAE; at price 0 an algebraic-evaluation arm would look free, which is the artefact the band exists to remove.
- **Unverified desk check, not a measurement:** `T = N/2` against matched rho gives S about 512 GAE per `√r` at `log₂ r = 20` (rho 1.29, IC baseline 10.7) and about 1,663 at 23.4 (rho 1.04, IC baseline 14.0). Distance to rho: `2^{10}` at `log₂ r = 20`, growing as `r^{1/2}`.

## 5. Obstructions

- **Corrigan-Gibbs–Kogan** `S·T² ≳ εN` covers every oracle built from group operations and membership tests. Algorithm 2 is one, so it is covered. The bound does not rule it out; it sits far above the line (`S·T² ≈ B·N²/4`) and the line forbids only what would beat about `√(N/S)`.
- **Structured GGM** (CGHW 2026/384): the base and its `j`-sums are the constrained labels, `δ ≈ Σ_{j≤m} C(B+j−1, j)/N ≈ B^m/(m!·N)` (interpretation, not a theorem for EC, as the audit marks it). Beating rho needs `δ ≲ N^{-1/2}`, i.e. `B ≲ N^{1/(2m)}`, a base too small to yield relations at the same rate. Covered.
- **Algorithm 1** is covered if its decomposition step is evaluation of an equation at grid points plus membership. If the paper contains a step that is neither a group operation nor a membership or equality test and skips candidate tuples, the generic bounds do not apply to it and the classification fails (kill criterion K1, §6).
- **Dominated by the baseline.** Relayed `O(p)` exceeds the `r^{2/3}` of the Table B `m = 2` prime rows; no improvement over the current baseline is possible from Algorithm 2 as relayed.
- **Closed items touched:** KN-FIND-007 (yield conservation), root-finder G3 failure, "enumeration in disguise" (standing constraints).

## 6. Minimal discriminating test (frozen protocol)

Instances: none native. This is a text-extraction plus arithmetic test; the repository oracles it maps onto are already measured (`research/pdp_root_finder_20261012/`).

**Step 0, acquire.** From a host that passes the Cloudflare check (a person with a browser, or a runner with network access), download `https://eprint.iacr.org/2017/1262.pdf` (CC BY 4.0), the two earlier versions from `https://eprint.iacr.org/archive/versions/2017/1262`, and the two Caen files. Record sha256 and page count. If Step 0 fails, record the failing URL and stop: verdict stays unverifiable.

**Step 1, extract** with `pdftotext -layout`, no judgement. For each of the 14 fields below, paste the verbatim block (40 lines around the first hit of the pattern) or write `absent`:
1. `Algorithm 1`; 2. `Algorithm 2`; 3. every `Theorem|Lemma|Proposition|Corollary` with a complexity; 4. `O(` and `Õ(` formulas; 5. the definition of `s` and of the factor base; 6. field sizes and `m` in every table; 7. timings and what they are compared with; 8. `Pollard|rho|generic`; 9. `Gröbner`; 10. `resultant`; 11. `Lemma 5.10`; 12. `conclusion|future`; 13. cost unit (group additions, field operations, bit operations); 14. any step that is not a group addition, a table lookup, a membership test or a numeric evaluation.

**Step 2, arithmetic.** Evaluate the paper's stated cost formula at `p ∈ {2^16, 2^20, 2^24, 2^30}`, `m ∈ {3, 4}` in the paper's unit; divide by the Table B baseline S and the matched rho S of `docs/ic/LEADERBOARD.md`, at 0, 0.1 and 1 GAE per native operation (§5a).

**Prediction.** Fields 1 and 2 reduce to group-law enumeration or numeric evaluation at grid points; field 4 is `O(p)`; field 14 is `absent`. My prior, unverified: 0.85.

**Kill criteria.** K1: field 14 is `absent` for both algorithms. Then the lead is closed as already covered by `rf-pairs`, `rf-mitm`, `subtract`, `pairs3`, `mitm3`, `mitm4`, and KN-LIT-6966ea is corrected (§8). K2: the stated exponent is at least 2/3 in `log_r`. Then no improvement over the baseline is possible. Promote only if field 14 is present and the paper's pseudocode skips candidate tuples with a stated exponent below 2/3; the next step is then an `ic-autolab` arm under the matched-rho protocol, outside this lead.

**Cost.** 0.1 person-day to acquire, 0.3 to extract, under 1 CPU-hour; no memory.

## 7. Ranking inputs

expected gain: none (at best a constant; worse than baseline as relayed) | confidence: low on content, medium on class | cost: about 0.5 person-day | tier: constant at best, otherwise closed | depends on: access to the PDF; LEAD-12 (generic-bound gate).

## 8. Open questions

- The paper's own cost statement, unit, and the definition of `s` (the relayed condition `s ≥ (m−2)log₂ p` is dimensionally unclear to me: read as `|F| ≥ p^{m−2}` it would make the base the whole group).
- Whether Algorithm 1 enumerates `F^m`, `F^{m−1}` plus a root-solve, or something smarter; and what Lemma 5.10 (added in the last revision) proves.
- What the "extensive data" are: field sizes, timings, and what they compare against (the relayed "worse than rho" is unsourced).
- Whether Mueller's UCD PhD thesis (about 2019–2020) exists online and contains the algorithms. Not found by search; the Mathematics Genealogy search listed "O'Hara, Daniela, UCD 2020" under McGuire, identity with Mueller unconfirmed, and the entry pages returned 404.
- Why the 2019 arXiv paper, which does ECDLP analysis, omits 2017/1262 from its references.
- **Corrections and additions for `KN-LIT-6966ea`.** (a) Set `url` to https://eprint.iacr.org/2017/1262; add revision date 2018-08-08, 2 revisions, license CC BY 4.0, keywords, UCD affiliation (from arXiv:1905.11755), and the name variant "Daniel Mueller" used by Semantic Scholar. (b) Add the Caen 2018 talk, the Nov 2015 UCD seminar title, and the follow-up arXiv:1905.11755. (c) "Not sub-rho at cryptographic size" and "reported still worse than rho" are unsourced: cite where they come from (`RESEARCH_ECC2K130_ROUTE_TARGETS.md:1105`, `lit_search_20260724.md:77-78`) and keep them marked relayed. (d) Add the relayed mechanism with its provenance (2026-10-15 slide reading) and the classification "Algorithm 2 = group-law enumeration oracle, generic line". (e) Keep "Not verified here", and add that the full text was still unread on 2026-10-22 with the list of failed routes.

## 9. Sources

Fetched, with outcome (2026-10-22 unless stated; WebFetch unless stated):

- `https://eprint.iacr.org/2017/1262.pdf`: 403 three times; curl: HTTP/2 403, `cf-mitigated: challenge`, `server: cloudflare`. A guessed `/archive/2017/1262/…pdf` path: same challenge. Not worked around.
- `https://ia.cr/2017/1262`: 302 to the abstract page. `https://eprint.iacr.org/2017/1262`: 200, normal record page (abstract paraphrased by the fetcher; verbatim quoting refused by the 125-character limit).
- Caen slides and paper (`nitaj.users.lmno.cnrs.fr/Caen2018/…`): WebFetch timeout at 60 s (slides twice, paper three times); curl: 40 s timeout, then HTTP/2 504 (nginx) at 08:17 and 08:18 UTC. `Caen2018/Programme.htm`: timeout.
- UCD seminar `…/021115algebra`: 403.
- Semantic Scholar page: 403. API `graph/v1/paper/bcb0ee08…`: 200 (year 2018, 14 references, 4 citations; abstract and references elided by the publisher); `/citations`: 200; `/references`: 200, null data; one repeat: 429.
- DBLP search: Anubis access denied. CORE search: 403. `researchrepository.ucd.ie` search: 405. Google Scholar: 200, one metadata result. ResearchGate search: 403.
- `web.archive.org` (three URLs): refused by the fetch tool; not worked around.
- EasyChair 15613 (Hu): 200, abstract only. `doi.org/10.1515/jmc-2019-0029` and `degruyterbrill.com` (Yokoyama et al.): 302, then 405. `cs.toronto.edu/~cvs/dlog`: 200, no entry.
- `mathgenealogy.org/id.php?id=134484` and `id=11943`: 404. `people.ucd.ie/gary.mcguire/publications`: 200, no entries. `maths.ucd.ie/pdftables/seminars/1045.pdf`: 200, unrelated seminar.
- `https://arxiv.org/abs/1905.11755`: 200. `https://arxiv.org/pdf/1905.11755`: 200, text extracted with `pdftotext` and read.
- About ten WebSearch queries. Their summaries are model-written and inconsistent across calls; used only as pointers, never as sources.

Repository paths read: `research/pdp_literature_leads_20261022/BRIEF_SONNET.md`, `LEADS.md` (standing constraints, LEAD-11, LEAD-12), `papers.csv` (grep); `src/cryptanalysis/ic_framework/rootfind.rs`, `line_oracle.rs`, `line_s4_oracle.rs`, `prime_fast.rs` (headers); `research/pdp_root_finder_20261012/README.md`; `docs/ic/LEADERBOARD.md` (head); `papers/ecdlp-index-calculus-program/LITERATURE.md`, `sections/rootfinding.tex`, `sections/theory.tex`, `sections/openproblems.tex`; `research/lit2026_w3_ggm_audit_20261011/README.md`; `research/ecdlp_leads_20261018/README.md`; `research/notes/ecc2k130/RESEARCH_ECC2K130_ROUTE_TARGETS.md`; `research/elliptic_snfs_survey_20261016/README.md`; `research/ic_next10_20261014/08-quasi-subfield/README.md`; under `crypto-autoresearcher`: `knowledge/literature/KN-LIT-6966ea.md`, `KN-LIT-4fe9d2.md`, `inputs/lit_search_20260724.md`, `research/sheriff/SHF-20261017-e4d7f6/scripts/readers/papers/pdftext/ic_program.txt`. Scratch (not committed): `/tmp/claude-0/-home-user/cf89363d-ad6e-5bef-bde6-195cbd206c93/scratchpad/deskcheck_mm.py` (the unverified desk check of §4).
