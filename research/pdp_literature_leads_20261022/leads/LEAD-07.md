# LEAD-07 — Crossbred (Vidal–Ionica–Delaplace 2024; 2026 preprocessing) on the Weil-descent degree cells

**Verdict.** constant-factor candidate (the lowest rung, and predicted to be killed by its own kill criterion; see §5 and §6).
**Confidence.** low that it yields any gain, medium that the card's kill criterion (`k > ℓ`) fires. The desk model is a generic semi-regular count, and this repository's F4 reads degree 4 on systems whose semi-regular degree is 7–8, so only the §6 test can close it.
**Expected gain if it works.** Unknown, most likely none. At best a constant on peak memory or per-oracle work at `(m, ℓ) = (4, 6)`. No effect on end-to-end `S`: the m = 4 free-oracle floor of `2^56.46` per relation set (`dreg_closure_mutantxl_20261016/RESULTS.md`) binds regardless of the solver. The gap to the matched rho stays at the committed `2^+7.62` (descent row, `w = 1`, unconverted column touches).
**Cost of the discriminating test.** 3–5 person-days. About 40–60 CPU-hours at 16 GB, plus one optional degree-5 item of about 15 CPU-hours at 7–25 GB.

## 0. Notation warning (card versus repository)

The card writes cells as "(ℓ, m)". The repository's degree-cell tables (`ic_next10_20261014/06-…`, `dreg_closure_driver_20261016`) write `(m, ℓ)` for the "blocked" cells `(4, 4), (4, 5), (4, 6)`. The card's "(4, 6)" is therefore read here as **m = 4, ℓ = 6**, the cell the driver's RESULTS names as the unmeasured separator. The literal reading (ℓ = 4, m = 6) has no measurement anywhere in the repository; its arithmetic is given once in §1. The card's "about 70 GB at degree 5" is the figure at **N = 40** (760,099 columns, 72.2 GB dense), not at `(4, 6)`. At `(4, 6)`, n = 10, it is **191 GB** (§1).

## 1. Mechanism (precise)

**System.** The chained Weil-descended Semaev system for `S_{m+1}` has `N = mℓ + (m−2)n` Boolean unknowns: `mℓ` coordinates of the summands `x_i ∈ V`, and `m−2` intermediate field elements of `n` bits. It has `E = (m−1)n` equations, so surplus `S = E − N = n − mℓ`. I derived this from the committed tables and checked it on 9 cells: `m = 3` (22, 6) gives N = 40; `m = 4` (11, 5) gives N = 42 and S = −9; (11, 4) gives N = 38 and S = −5; (13, 5) gives N = 46. By the survey §3.1, each chained `S_3` is cubic in the bits (the `x₁x₂y` term is trilinear). The last one has `x_R` known, so it is quadratic. I take `n` quadratic and `(m−2)n` cubic equations (my reading, not a committed fact).

**Structure that matters.** Given all `x_i`, every equation is F₂-affine in the intermediate elements. Only `mℓ` of the `N` variables are free; the rest are determined. Fixing one summand's `ℓ` bits makes the last equation linear in `y_{m−2}`, which pins it, and the problem becomes the `m − 1` cell. That is guess-and-solve, not Crossbred.

**Crossbred.** Conventions differ: the paper's `k` is the number of *kept* (non-enumerated) variables and the loop runs over `2^{n−k}` assignments (VDI24 Alg. 1). The repository's `k` and the card's `k` are the number of *enumerated* variables. I use the repository's: `k` enumerated, `κ = N − k` kept.
- **Preprocessing.** Build the Macaulay matrix `Mac_{≤D}` and take the left kernel of its "bad" columns (monomials with kept-degree `> d`). The kernel gives polynomials of kept-degree `≤ d`.
- **Enumeration.** Specialise the `k` variables, `2^k` times, and linearise at degree `d` (`d = 1` in every implementation to date).
- **Admissibility** (VDI24 Def. 7 and Thm. 10, proved for quadratic strongly semi-regular systems). The number of crossbred polynomials must cover the specialised quotient. For `d = 1` this reads `dim I_{≤D} − bad(D, κ) ≥ κ + 1`, with `D < D_reg`. Under semi-regularity `dim I_{≤D} = C(N, ≤D) − Σ_{j≤D}[z^j](1+z)^N/∏(1+z^{d_i})^{E_i}`. I extended this to the mixed quadratic/cubic count. The paper does not prove that extension.
- **Repository module.** `crossbred.rs` implements `E = {0,…,k−1}` and `d = 1`, with a dense `[M_bad | I]` left kernel and a row cap (`max_rows`, default 4,000).

**The (m = 4, ℓ = 6) cells** (counts exact; the "dense" column is `cols²/8`, the convention that reproduces the committed 70 GB at N = 40):

| n | N | E (quad + cubic) | S | C(N, ≤4) | dense | C(N, ≤5) | dense |
|--:|--:|--:|--:|--:|--:|--:|--:|
| 10 | 44 | 30 (10 + 20) | −14 | 149,986 | 2.8 GB | 1,235,994 | **191 GB** |
| 15 | 54 | 45 (15 + 30) | −9 | 342,541 | 14.7 GB | 3,505,051 | 1,536 GB |
| 24 | 72 | 72 (24 + 48) | 0 | 1,091,059 | 149 GB | 15,082,603 | 28,436 GB |

n = 10 is the largest n under the driver's 155k-column degree-4 cap. n = 15 is the smallest n with realistic unsatisfiable draws (item 06: N ≥ 54). n = 24 (`n = mℓ`) is the square regime the ECC2K-130 rows are priced in (`N ≈ E`). The literal (ℓ = 4, m = 6) at n = 10 has N = 64, E = 50, C(64, ≤4) = 679,121 (57.7 GB) and C(64, ≤5) = 8.3M (8.6 TB).

**Degree bound the formulation obeys.** Crossbred's preprocessing is XL-like: it multiplies the original equations and keeps no mutants. Its `D` should therefore follow the XL degree and the semi-regular `D_reg`, not the F4/closure degree of 4 that this repository measures. Check (unverified desk check): at `m = 3`, (10, 5), N = 25, the semi-regular series first goes non-positive at degree 7. That equals the committed XL degree (7), against F4 and closure degree 4.

## 2. Prior art: literature

- **Joux, Vitse**, *A crossbred algorithm for solving Boolean polynomial systems*, NuTMiC 2017 (LNCS 10737), ePrint 2017/372, https://eprint.iacr.org/2017/372. **abstract-only.** Takes: the algorithm; Fukuoka Type I solved up to 148 quadratic equations in 74 variables. Full text not read.
- **Vidal, Delaplace, Ionica**, *An analysis of the Crossbred Algorithm for the MQ Problem*, IACR Commun. Cryptol. 1(3), 2024, doi 10.62056/ak86cy7qiu (preprint ePrint 2024/992, https://eprint.iacr.org/2024/992; DBLP lists the preprint title as "The Complexity of the Crossbred Algorithm"), https://cic.iacr.org/p/1/3/36/pdf. **read-at-source, partial** (intro, §2, Alg. 1, Def. 7, Thm. 10, Ex. 6.1–6.3, Table 1; proofs and §7 experiments not read). Takes four things:
  - the admissibility series, which for `d = 1` is the count above;
  - every implementation so far has `D ≤ 5` and `d = 1`, and "as soon as D ≥ 4" the matrices stop fitting in memory;
  - the proofs assume *strong semi-regularity* of a quadratic system;
  - Duarte's complexity (their Eq. 3) is `Σ C(k, d_k)·C(n−k, d′)` linear algebra plus `2^{n−k} × (specialised solve)`.
- **Vidal, Delaplace, Ionica**, *Improved preprocessing for the Crossbred algorithm and application to the MQ problem*, ePrint 2026/360, https://eprint.iacr.org/2026/360. **abstract-only.** Takes: a criterion that trims polynomials when preprocessing *over*-produces for admissible `(D, d, k)`; applied to MQOM. It does not help here, because in the cells above preprocessing *under*-produces (§4). Not confirmed from the full text.
- **Duarte**, *On the Complexity and Admissible Parameters of the Crossbred Algorithm*, ePrint 2023/1664. **search snippet only, not fetched.** Cited by VDI24 for the first bivariate series and the extension to `q > 2`.
- **cryptographic_estimators** Crossbred page (URL in §9). **read-at-source**, interface only, no formulas.
- **Bardet** (semi-regular series `(1+z)^n/(1+z²)^m`), via VDI24. **recalled.**

## 3. Prior art: these repositories

- `/home/user/crypto/src/cryptanalysis/crossbred.rs` (lines 1–200 read: header, `CrossbredParams`, `ExtractStats`). Implements `(D, k, d = 1)`, dense left kernel, `E = {0..k−1}`, `max_rows = 4000`. **Supports** the test, and shows its limit: the row cap must be raised about 25–40×.
- `/home/user/crypto/research/crossbred_20260920/` has no `README.md`; the directory holds `e2e_panel.txt`, `e2e_panel.json`, `oracle_ladder.txt`.
  - `e2e_panel.txt`: at `m = 3`, n = 9 the whole-pipeline panel gives crossbred 11.7 relations/s, groebner 33.2, sat 51.7, **enumerate 19,696** (about 1,700× faster than crossbred). All m = 2 rows are also won by enumerate. Footer: "Θ(2^n) … prices the constant, not the exponent." **Closes** the toy cells.
  - `oracle_ladder.txt`: **filters = 0 at every (D, k) tested.** At `m = 3`, ℓ = 6, v = 27 the working choice is `D = 3, k = 12 = 2ℓ`, and crossbred runs 1.4× faster than the repository's own F4. **Closes** `k ≤ ℓ` there; note `k = 2ℓ`.
- `/home/user/crypto/research/dreg_closure_driver_20261016/RESULTS.md` and `PREREGISTRATION.md`. The closure degree is 4 on 16 of 16 draws (m = 3 ℓ = 6; m = 4 ℓ = 5). Degree 5 at N = 40 is 760k columns, about 70 GB dense; the sparse engine is too slow. `(4, 6)` needs "a larger memory host or a sparse closure." **Supports** the memory premise but not the card's 70 GB figure at `(4, 6)`.
- `/home/user/crypto/research/dreg_closure_mutantxl_20261016/RESULTS.md`. The closure refutes at 3–4 where XL reads 5–7+; closure work is about `cols^2.2`; the m = 4 free-oracle floor is `2^56.46`. **Supports** the baseline Crossbred must beat; **bounds** any gain.
- `/home/user/crypto/research/ic_next10_20261014/06-equivariant-f4-m4/README.md`. msolve F4 reads degree 4 (some 3) through `(m, ℓ) = (4, 4)` (N = 38: 84k × 67k matrix, 1.0–1.5 GB, 386–805 s per draw). It hits the 3 GB cap at N = 42, 46 while still in degree-4 rounds (RSS 1.6–1.8 GB). `(4, 5)` and `(4, 6)` are blocked. **Supports** F4 as the matched baseline; shows the real wall is memory at N ≥ 42, not degree.
- `/home/user/crypto/research/notes/index-calculus/RESEARCH_PDP_SAT_SMT_NAGAO_SURVEY.md` §3. The `m = 2` residual (2ℓ unknowns, n ≈ 1.5N equations) is not cheaper than `2^ℓ` by any known method. Generic MQ exponents give `2^{1.39ℓ}`. "Crossbred measured on these systems found no filters at any (D, k)." **Closes** the m = 2 version; the claim's source measurement is `oracle_ladder.txt`.
- `/home/user/crypto/research/linearization_reach_20260930/README.md`. Guess-and-linearize costs `~2^{n−ℓ−b}` per relation; the gap to `2^61` is at least 56 bits at every arity. **Covers** Crossbred with `k ≥ ℓ`.
- `/home/user/crypto/research/ic_leads_review_20261018/README.md` (grep only). Names the gap: Crossbred never run on the descent cells. No measurement.

## 4. Why it could lower the Gröbner cost or the degree of regularity

It would have to: (i) produce degree-1 polynomials from a Macaulay matrix of degree `D ≤ 4`, whose width is a fixed `C(N, ≤D)`; (ii) do so while enumerating fewer than the `ℓ` bits of one summand; (iii) give a smaller peak than msolve at the same N. Crossbred's one real saving is that its degree `D` can be below the degree needed for a full Gröbner basis, because the last step is linear after specialisation.

Desk checks (**unverified desk check**, scratchpad scripts, not committed; generic semi-regular count, assumption A: strong semi-regularity, extended to mixed degrees 2/3, with every variable treated alike):

- **Smallest admissible k, d = 1, D = 4–5.**

| n | D | k_min (κ) | rows | bad cols | r (naive / sr) | dense `[M_bad\|I]` | enumeration |
|--:|--:|--:|--:|--:|--:|--:|--:|
| 10 | 4 | 39 (5) | 10,810 | 8,215 | 2,595 / 2,540 | 0.03 GB | 2^39 |
| 10 | 5 | 38 (6) | 162,170 | 153,101 | 9,069 / 6,394 | 6.4 GB (25 GB as R × cols) | 2^38 |
| 15 | 5 | 47 (7) | 438,930 | 405,440 | 33,490 / 26,440 | 46 GB (192 GB) | 2^47 |
| 24 | 4 | 64 (8) | 66,600 | 61,978 | 4,622 / 4,322 | 1.07 GB | 2^64 |

  Every admissible choice has `k ≥ N − 8`: about 85–90 % of the variables are enumerated, and `k` is 1.6× to 2.7× the `mℓ = 24` free bits. The per-guess work is trivial (`κ + 1 ≤ 9` columns). A Möbius evaluation of about 16 polynomials costs about `2^44` word ops at n = 10. Direct enumeration of the `x`-bits is `2^24` leaves.
- **k ≤ ℓ = 6.** At n = 10 and n = 15 the specialised system is underdetermined (`κ > E`), so `k ≥ mℓ − n = 14` and `9` respectively. At n = 24, with `k ∈ {6, 12, 18, 24}` and `d ∈ {1, 2, 3}`, there is **no admissible D up to 14**. The kernel is empty at the working degrees: at k = 6, D = 4 the bad columns number 1,088,230 against 66,600 rows. At n = 18, k = 6: the naive criterion first holds at D = 12 (1.8×10^12 columns).
- **What fits 16–64 GB.** Hybrid guessing alone, with no Crossbred: dense degree-5 closure at `N − k = 38` (k = 6 = ℓ from N = 44) is 42.8 GB, at `N − k = 34` (k = 10) 13.7 GB. So `k ≤ ℓ` fits 64 GB through plain guess-and-solve. Crossbred adds nothing to that, because its kernel is empty there.
- **Structured variable choice.** Enumerating `x_m`'s ℓ bits reduces `m = 4` to `m = 3` with `N′ = 3ℓ + n` (n = 24: N′ = 42, 124k degree-4 columns). The driver measured exactly this cell (`m = 3`, (24, 6)) at 7.8 GB dense. Crossbred preprocessing on the full N = 72 system at D = 4 is 1.09M columns, **8.8× wider** than that. So Crossbred preprocessing needs more memory than guess-then-F4, not less.

Assumptions: A; the 2/3-degree extension is mine; "naive" rows ignore dependencies (VDI24: they appear from D ≥ 4), so it overcounts useful rows. The real systems carry degree falls the semi-regular model ignores, so the generic numbers may be pessimistic by an unknown factor.

## 5. Obstructions

- **Generic bound.** Crossbred with `k ≥ ℓ` (enumerated variables covering a summand) is guess-and-linearize, already measured at `~2^{n−ℓ−b}` per relation with `b_max ≈ 2–3` past `ℓ = n/3`. The oracle is built from group operations plus membership tests, so Corrigan-Gibbs–Kogan and Shoup apply to it (cited by the card; not re-derived here). Allowed: a constant. Not allowed: an exponent.
- **Dimension.** Only `mℓ` of the N variables are free. A solver that enumerates more than `mℓ` variables (every admissible generic choice above: 38–64 vs 24) is worse than enumerating the summand bits.
- **Underdetermination.** Specialisation is overdetermined only if `k ≥ mℓ − n`. This is a theorem of counting, not of Crossbred. It forces `k > ℓ` for n < (m−1)ℓ, which includes the driver's n = 10, 15 at ℓ = 6.
- **XL-like degrees.** Without mutants the preprocessing degree follows XL/semi-regular `D_reg` (7 at (3, 5, 10)), where F4 and the closure read 4. No mutant-aware Crossbred appears in the sources I read.
- **Measured negatives.** `oracle_ladder.txt` shows filters = 0 everywhere; `e2e_panel.txt` shows enumerate ahead of crossbred by about 1,700× at (m = 3, n = 9).
- **Not closed by these.** No Crossbred run exists on any Weil-descent degree cell at `m ≥ 3` with `ℓ ≥ 4`, and my model is generic. Hence "constant-factor candidate", not "closed".

## 6. Minimal discriminating test (frozen protocol)

**Instances.** The draws committed by `dreg_ladder` (cell seeds) and exported by `examples/ic_next10_06_chain_export.rs`: `(m, n, ℓ) ∈ {(4, 11, 4), (4, 13, 4), (3, 10, 5), (3, 13, 5), (4, 10, 6)}`. Use 4 unsatisfiable and 4 satisfiable draws per cell where they exist. At `(4, 10, 6)`, S = −14 makes unsatisfiable draws very rare (the driver found none in 243 at a comparable surplus), so use satisfiable draws only and say so. The `(4, 13, 4)` and `(3, 13, 5)` cells may hit the memory cap; record that, never read it as evidence.

**Instrument.** `crossbred.rs` extraction with `max_rows` raised to 200,000 and a record of the variable order of the exporter (x-bits first or y-bits first). Baselines: msolve 0.6.5 as in item 06, and the dense MutantXL closure.

**Record.** For `D ∈ {3, 4}` over all `k = 0…N`, and `D = 5` only at `k ∈ {ℓ, 2ℓ, mℓ, N−8, N−6}`: `macaulay_rows`, `bad_cols`, `kernel_dim`, `filters`, `word_ops`, peak RSS. For each `(D, k)` with `kernel_dim ≥ N − k + 1`, also run the solve and record the verified result and `2^k × per-guess` word ops. For msolve at the same draw, record peak RSS and wall.

**Prediction (frozen before running).**
- P1: at every cell the smallest `k` with `kernel_dim ≥ N − k + 1` (D ≤ 5) is at least `N − 8`.
- P2: `kernel_dim = 0` for every `k ≤ ℓ` at every D tested.
- P3: `filters = 0` at every `(D, k)`.

**Kill.** The lead is closed by measurement if P1 and P2 both hold at all five cells. This is the card's kill criterion, made numeric. The lead survives only if some cell has an admissible `(D ≤ 5, k ≤ ℓ)` whose peak RSS is below msolve's at the same draw. A survivor is then costed in word ops against the guess-then-F4 baseline above, not against rho.

**Cost.** (4, 11, 4) and (3, 10, 5) at `D ≤ 4`: minutes each. `D = 5` at (4, 11, 4): R = 117k, bad 93k, dense 3.1 GB, about `R²(B+R)/64 = 4.5×10^13` word ops, so 5–10 CPU-hours. The (4, 10, 6) `D = 5` item (R = 162k, 6.4 GB, 15–20 CPU-hours) is optional and tagged "needs 16 GB+". Total about 40–60 CPU-hours and 16 GB for the required items.

## 7. Ranking inputs

| expected gain | confidence | cost | tier | depends on |
|---|---|---|---|---|
| unknown, likely 0; at best a constant on peak memory at (4, 6) | low | 3–5 person-days, 40–60 CPU-h | constant | `crossbred.rs` row cap raised; item 06 export; LEAD-12 theory gate for the generic bound |

## 8. Open questions

- Which variables does the exporter place first? `crossbred.rs` fixes `E = {0..k−1}`, so this decides whether a given `k` is an x-bit or y-bit enumeration. I did not read the exporter.
- Does ePrint 2026/360 change the admissibility bound beyond trimming? Abstract only.
- Does the preprocessing degree stay near XL's on these systems? Only §6 can say. A mutant-aware Crossbred is a separate algorithm, not in anything I read.
- Whether the closure degree at `(m, ℓ) = (4, 6)` rises to 5 (the driver's open question) decides whether the 191 GB figure is the relevant one.
- JV17 full text not read; the `D ≤ 5, d = 1` practice is taken from VDI24.

## 9. Sources

Fetched:
- https://eprint.iacr.org/2017/372: HTTP OK via WebFetch, abstract only.
- https://eprint.iacr.org/2024/992: HTTP OK, abstract only.
- https://eprint.iacr.org/2026/360: HTTP OK, abstract only.
- https://cic.iacr.org/p/1/3/36/pdf: PDF saved (595,012 bytes). WebFetch's own extractor failed on the binary; I converted the saved file locally with `pdftotext` and read the sections listed in §2.
- https://cryptographicestimators.readthedocs.io/en/latest/MQEstimator/MQAlgorithms/crossbred.html: HTTP OK, interface only.
- WebSearch: three queries. Returned the HAL record hal-04729683 (not opened), the DBLP listing and the Duarte entry (snippets only). No arXiv or HAL mirror of any of the three papers was found or fetched.

Repository paths read: `/home/user/crypto/research/pdp_literature_leads_20261022/{BRIEF_SONNET.md, LEADS.md}`; `/home/user/crypto/src/cryptanalysis/crossbred.rs` (lines 1–200); `/home/user/crypto/research/crossbred_20260920/{e2e_panel.txt, oracle_ladder.txt}` (the `README.md` named in the task does not exist); `/home/user/crypto/research/dreg_closure_driver_20261016/{RESULTS.md, PREREGISTRATION.md}`; `/home/user/crypto/research/dreg_closure_mutantxl_20261016/RESULTS.md`; `/home/user/crypto/research/ic_next10_20261014/06-equivariant-f4-m4/README.md` (lines 1–200); `/home/user/crypto/research/notes/index-calculus/RESEARCH_PDP_SAT_SMT_NAGAO_SURVEY.md` (§3); `/home/user/crypto/research/linearization_reach_20260930/README.md`; `/home/user/crypto/research/ic_leads_review_20261018/README.md` (grep only).

Desk-check scripts (not committed, unverified): `/tmp/claude-0/-home-user/cf89363d-ad6e-5bef-bde6-195cbd206c93/scratchpad/xb_desk.py` and `xb_desk2.py`.
