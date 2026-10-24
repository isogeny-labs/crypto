# LEAD-18 — Structured elimination for C₄: a resultant or FGLM path on the D₄ quotient instead of the dense Macaulay solve

**Verdict.** closed-by-measurement, for the two paths as the card states them. The card's baseline is stale: the
ideal degree is `D = 512`, not about 85, and the repository already runs the structured step the card proposes (a
sparse-FGLM-class "chains" step, landed 2026-10-14), cutting `C₄` from `2.54·10¹⁰` to `6.9–8.2·10⁷` (310–370×).
Against that current baseline the resultant chain is about 370× worse and sparse FGLM is at parity. One item stays
open: the wall time of msolve's F4 on the plain D₄ system (§6). It could only license a constant-factor port.
**Confidence.** medium. Dimension, phase shares and the 3.0·10¹⁰ resultant arithmetic are measured or exact. msolve's
cost on these systems is unmeasured: msolve is not on PATH here and it reports wall time, not counted multiplications.
**Expected gain if it works.** Against the card's `C₄ = 2.54·10¹⁰`: already realised (310–370×), not by a new
mechanism. Against the current baseline `C₄ = 8.2·10⁷` (p = 1009, F_p multiplications per decomposition): at most
1.2–1.9× by the FGLM-class and pivoting levers below (desk bound), about 1.0–1.4× expected. That moves the model
crossover `n*` by at most `4·log₂ 1.9 ≈ 3.6` bits (from `2^{103.0–103.7}` at `h = 2`). Distance to rho is unchanged:
the k = 4 ecbench row (p = 29, `ic-d4-chains`) sits at `1.39·10⁶×` rho-negation, and `r∞` is untouched.
**Cost of the discriminating test.** About 0.5–1 person-day (install msolve, about 20 lines of shell, no new Rust),
about 40 CPU-minutes, under 1 GB.

## 1. Mechanism (precise)

**The system (m = 4 summands, E over F_{p⁴}, base `{P : Y(P) ∈ F_p}`).** After the 2-torsion coordinate `Y` and the
`D₄` invariants, each target gives four Weil components of degree 4 and `π² − σ₄` in `(σ₁, σ₂, σ₃, σ₄, π)`. Bézout is
`4⁴·2 = 512`, the Macaulay degree is `4·3 + 1 + 1 = 14`, and the cumulative matrix is `13,412 × 11,628`
(`C(19,5) = 11,628`). Substituting `σ₄ = π²` gives four equations in `(σ₁, σ₂, σ₃, π)` of weighted degree 8, weights
`(2, 2, 2, 1)`, weighted Macaulay degree 26, matrix `3,979 × 4,200`. Both forms have `D = 512` (unit test
`dense.solve.dim == 512` in `gaudry_quartic_d4.rs`).

**Correction to the card.** The card says "about 85 solutions", "degree-85 univariate". I found 85 nowhere in the
repository except `LEADS.md` (LEAD-16 and LEAD-18). Two numbers sit near it. `4096/48 = 85.3`, which is the measured
cost ratio (47.5–48.4×) of `gaudry_quartic_d4_20261013` read as a dimension. And 84 is the peak of the plain Hilbert
series `(1+z+z²+z³)⁴(1+z)` (my desk check), i.e. the number of dense columns of the grevlex multiplication matrix
(Faugère–Mou Prop 6.4). The quotient dimension is 512 (8× below `S₄`'s 4,096; the cost fell 48× because the Macaulay
degree also fell from 29 to 14).

**Path (i), F4 → sparse FGLM → univariate roots.** Faugère–Mou (their §3.1.2, Thm 3.2): `D(N₁ + n log D)` field
operations, `N₁` the nonzeros of the multiplication matrix of the smallest variable. That matrix is a generalised
companion matrix: most columns are unit vectors, `k` columns are dense. Krylov cost is `2D·(k·D)`. Desk values
(unverified, scratchpad `lead18/desk.py`): plain form, variable `π`, `k = 84`: `4.4·10⁷`. Weighted form, `k = 44`
(max coefficient of `(1+t²+t⁴+t⁶)³(1+t+…+t⁷)`, which equals the 44 chains the repository measured): `2.3·10⁷`.
Berlekamp–Massey plus Hankel solves add about `2·10⁶`. Dense FGLM is `nD³ = 5.4·10⁸` (`D³ = 1.3·10⁸`).
Root-finding of the degree-512 polynomial (`x^p mod h`, gcd): about `5.5·10⁶` schoolbook at p = 1009. The repository
measures `1.5·10⁶` there.

**Path (ii), resultant chain.** Eliminate `σ₁, σ₂, σ₃` with `π` as parameter: four forms of degree 4 in four variables,
Macaulay degree 13, matrix `C(16,3) = 560` square. One determinant is `560³/3 = 5.9·10⁷`. Interpolating `R(π)` of
degree 512 needs 513 points: `3.0·10¹⁰` F_p multiplications (dense desk bound; `deg_π R = 512` assumed, from weighted
Bézout). Iterated pairwise resultants blow up as degree 4 → 16 → 256. E13 (`RESEARCH_GLV_INVARIANT_FACTOR_BASES.md`
§8.7) worked at k = 3 because three conics in two unknowns need a 15×15 matrix and give `R(q₃)` of degree 16 = 64/4. At
k = 4 the analogue eliminates three unknowns from four quartics, and the matrix is 560².

**Degree of regularity.** The solving degree is not lowered by either path; both start from the same ideal. The F4
bound is the Macaulay degree: 14 (plain), 26 weighted (the repository tried 22–25; the quotient is 502–511 there).

## 2. Prior art: literature

- Faugère, Mou, *Sparse FGLM algorithms*, J. Symb. Comput. 80 (2017), arXiv:1304.1238,
  `https://arxiv.org/pdf/1304.1238`. **read-at-source** (PDF text-extracted; abstract, §1, §3.1.2, §6.4, Table 2; proofs
  and §3.2, §4 skimmed only). Claim used: shape position costs `O(D(N₁ + n log D))`; `T₁` has as many dense columns as
  the greatest coefficient of `(1+z+…+z^{d−1})ⁿ` (Prop 6.4, under Moreno-Socías); `T₂…T_n` are not needed; in their
  Table 2 the change of order costs about as much as the DRL (F5) step (Random13: 193.5 s vs 146.8 s).
- Berthomieu, Eder, Safey El Din, *msolve: a library for solving polynomial systems*, ISSAC 2021, arXiv:2104.03572,
  `https://arxiv.org/pdf/2104.03572`. **read-at-source** (§3.2, §4.1–4.4; benchmark tables not read). Claims used: F4
  with Gebauer–Möller and sparsest pivots, probabilistic linear algebra, characteristic below 2³¹; the F4 tracer is for
  other primes of the same system (§4.3); the transpose of the multiplication matrix is stored as a generalised
  companion matrix with dense lines (§4.4), the same structure as the repository's chains.
- msolve README, `https://github.com/algebraic-solving/msolve`. **read-at-source** (via the fetch tool's summary). Claims
  used: input format (variables, characteristic, comma-separated polynomials), `-g 1` leading monomials (gives
  dimension and degree), `-g 2` reduced grevlex basis, `-e`, `-t`, `-v`, and a bare `-f/-o` run over a prime field
  computes a parametrization. `-P` is not documented there (the docs site returned HTTP 503); I use the repository's
  own `-P 1` (below).
- Berthomieu, Neiger, Safey El Din, *Faster change of order algorithm for Gröbner bases under shape and stability
  assumptions*, ISSAC 2022, arXiv:2202.09226. **abstract-only** (search-result summary; not fetched). Claim used: the lex
  basis from a Hermite normal form of a univariate polynomial matrix, `Õ(t^{ω−1}D)`, `t` a sparsity indicator. This is
  the one lead that could replace the repository's `det A(λ)` step (§4).
- Joux, Vitse, *Elliptic curve discrete logarithm problem over small degree extension fields*, J. Cryptology 26 (2013),
  `https://www-fourier.univ-grenoble-alpes.fr/~viva/research/articles/DLFq5.pdf`. **read-at-source** (§2.2). Claims
  used: for Gaudry–Diem the degree is generically `2^{n(n−1)}`, grevlex alone cannot finish, dense FGLM `Õ(nD³)` "sharp"
  and the longest step in their runs; F4 itself about `10¹⁰` multiplications at n = 5.
- Faugère, Gianni, Lazard, Mora, *Efficient computation of zero-dimensional Gröbner bases by change of ordering*, J.
  Symb. Comput. 16 (1993) 329–344. **recalled** (volume and pages confirmed by search snippets only). Claim: `O(nD³)`.
- Cantor, Zassenhaus, *A new algorithm for factoring polynomials over finite fields*, Math. Comp. 36 (1981) 587–592
  (AMS page returned HTTP 403), and von zur Gathen, Gerhard, *Modern Computer Algebra*. **recalled**. Claim: rational
  roots by `gcd(h, x^p − x)` after `log₂ p` modular squarings, `Õ(D log p)` operations with fast multiplication.
- Faugère (F4, 1999), Bardet–Faugère–Salvy (F5 complexity, 2015), Gaudry (2009), Faugère–Gaudry–Huot–Renault
  (J. Cryptol. 2014). **recalled**; only the facts "F4/F5 reduce to Macaulay matrices up to `d_reg`" and "`D_n` symmetry".

## 3. Prior art: these repositories

- `src/cryptanalysis/gaudry_quartic_d4.rs`, `gaudry_quartic.rs` (lines 1613–1960): the solver is a fixed-degree
  Macaulay echelon, then either dense normal forms plus characteristic polynomial of one multiplication matrix, or the
  `chains` step (border normal forms of `k` chain ends, `det A(λ)` of a `k × k` polynomial matrix, scanned on F_p if
  `p ≤ 513`, else interpolated at 513 points, roots by `PolyRing::roots`). **Closes** the card's premise: this is the
  structured path already.
- `research/gaudry_quartic_d4_20261013/README.md`: `C₄ = 2.54·10¹⁰`, quotient 512, 19 of 20 targets close.
  **Supports** (the card's baseline, now superseded).
- `research/gaudry_quartic_d4_cost_20261014/README.md`, `raw/summary.json`, `raw/table.md`: `C₄` `2.547·10¹⁰ →
  6.89·10⁷` (p = 269), `8.21·10⁷` (1009), `8.06·10⁷` (4049); 370× paired; ecbench p = 29: new/rho `1.39·10⁶`;
  crossover model `2^{103}`. Phase means in §4. **Closes** the lead at the baseline; names "F4 with the D₄-weighted
  sugar strategy" and "Magma on the weighted system" as not tried.
- `research/gaudry_quartic_replay_20261013/README.md`: replay saves 2–7 %, `C ∝ D^{3.1}`. **Supports**.
- `research/ic_speed_and_rho_scope_20261013/README.md`: 10× off `C₄` moves `n*` by about `2^{13}`. **Supports** the
  unit used for gains here.
- `RESEARCH_RESIDUAL_WALKS.md` §11.17, §11.19: `C₄ = 1.21·10¹²` generic; `r∞ = 0.518`. **Supports.** §11.18 not read.
- `RESEARCH_GLV_INVARIANT_FACTOR_BASES.md` §8.7 (E13): `R(q₃)` of degree 16, `÷ 33–41` on `S₃`. **Supports** by
  analogy; the k = 4 matrix is 560² (§1).
- `leads/LEAD-02.md`: per-target evaluation of a precomputed eliminant is a regression; "sparse FGLM is the wrong step"
  at D = 512 because the charpoly was 0.6 % of the committed `C₄`. **Supports**, and I qualify it: after the
  repository's own cuts that step is 31 % of `C₄` (§4).
- `groebner_compare/README.md`: a Boolean (GF(2)) harness; "Magma Boolean F4, msolve" are listed as not implemented.
  **Unrelated** to F_p msolve; the card's "msolve bridge" does not exist there.
- `research/gb_cas_comparison_20261011/README.md`, `f4_fp_msolve_parity_20261011/`, `ic_next10_20261014/02-solution-count/scripts/run_msolve.sh`:
  msolve was 12–24× faster than the repository's `f4_fp` on square systems and level on overdetermined ones; the
  scripts give the repository's msolve invocations (`-g 2 -t 1`, `-P 1 -t 1`). **Supports** the instrument, not the
  gain: `f4_fp` is a different engine from the Macaulay solver.
- `research/magma_f4_comparison_20261013/` (`PROTOCOL.md`, `scripts/d4_p269_seed1.magma`, `ours_d4.jsonl`): the only
  committed D₄ system file (plain, five unknowns, p = 269, seed 1, ours 4 solutions, `2.547·10¹⁰`); no Magma output
  exists. **Supports** (instance to reuse).
- `examples/gaudry_quartic_d4.rs` (`--export-magma`, `--weighted --chains --sparsest --replay`), `examples/gb_cas_export.rs`
  (`.ms` writer syntax). No `.ms` exporter for D₄ exists.

## 4. Why it could lower the Gröbner cost or the degree of regularity

It does not lower the degree of regularity. It can only move constants, in two places. Phase means of the current best
pipeline (`--weighted --chains --sparsest --replay`, planted targets, `raw/summary.json`), F_p multiplications:

| p | echelon | `det A(λ)` | border NF | roots | total | first solve (no replay) |
|--:|--:|--:|--:|--:|--:|--:|
| 269 | 4.94·10⁷ (72 %) | 1.30·10⁷ | 6.3·10⁶ | 0 (scan) | 6.89·10⁷ | 1.41·10⁸ |
| 1009 | 4.88·10⁷ (59 %) | 2.52·10⁷ (31 %) | 6.4·10⁶ | 1.5·10⁶ | 8.21·10⁷ | 1.55·10⁸ |
| 4049 | 4.52·10⁷ (56 %) | 2.52·10⁷ | 6.5·10⁶ | 3.5·10⁶ | 8.06·10⁷ | 1.58·10⁸ |

**(a) The FGLM-class step (`det A` + NF + roots, 28–40 % of `C₄`).** Per interpolation point the repository pays
`k³/3 + k·D = 2.8·10⁴ + 2.3·10⁴` for `k = 44`, times 513: `2.6·10⁷`, matching the measured `2.52·10⁷` (desk).
Sparse FGLM's Krylov step is `2D·k·D = 2.3·10⁷`. These are the same `k·D²` object, so Faugère–Mou parity, not a gain; the
repository's scan at `p ≤ 513` (`1.3·10⁷`) is already cheaper than Krylov. A Hermite-form determinant (Berthomieu–Neiger–
Safey El Din, abstract only) is `Õ(k^{ω−1}D)` in principle, but at `k = 44` the polylog factors are of the same size as
the gap. Bound: this step at best halves, so `C₄` falls at most 1.25× (to `~6.6·10⁷`); removing it entirely gives 1.44×.
Assumption: the 44 chains are the repository's measurement; the formula for `k` is Faugère–Mou's under Moreno-Socías.

**(b) The elimination (56–72 %).** Dense work in a degree-by-degree F4 (rank = columns − Hilbert function, desk) peaks at
the exact-degree-14 plain matrix, `3,060` columns, `3,060³/3 ≈ 9.6·10⁹` if dense. The measured sparse count for the whole
cumulative plain matrix with sparsest pivots is `4.2·10⁸`, 23× below that. So the repository already harvests the
sparsity F4 relies on; a Gebauer–Möller / degree-by-degree reorganisation has no obvious order-of-magnitude left. There
is measured slack inside the repository: plain form + sparsest + replay has echelon `2.74·10⁷` (p = 269,
`abl_scr`), against `4.93·10⁷` for the weighted form with replay. So pivot or ordering work on the weighted matrix could
plausibly reach about 1.8× on this phase, 1.4× on `C₄`. That is a pivoting lever, not an F4/FGLM one.
**Combined best case:** `8.2·10⁷ → ~4.3·10⁷` (1.9×), 3.6 bits on `n*`.

## 5. Obstructions

1. **Stale baseline and D.** The ratchet rule makes the baseline the best landed pipeline (`8.2·10⁷`). The card's 3×
   criterion is `≤ 2.7·10⁷` against it.
2. **The structured path is already in the tree.** `chains` is Faugère–Mou's observation (dense columns only; msolve
   §4.4's generalised companion matrix), arrived at independently: normal forms `7.4·10⁸ → 6.3·10⁶` (weighted).
3. **The resultant route is Macaulay again.** `3.0·10¹⁰` (dense desk bound), about 370× above the baseline and equal to
   the committed `C₄`. E13's win came from a 15×15 matrix; the k = 4 matrix is 560².
4. **Root finding is 0–4 % of `C₄`.** Even free, it moves nothing. The card's `Õ(D log p)` is already measured
   (`1.5–3.5·10⁶`).
5. **msolve cannot use the two things that made the repository fast.** It takes no weights that I could find (README and
   paper mention only DRL and `-e` blocks; not confirmed absent), and its F4 tracer re-runs the same system modulo other
   primes (§4.3), not new coefficients on the same support, so it has no replay across targets. Its wall time is not an
   admissible `S` (wall is not admitted below isolation L2) and says nothing in counted F_p multiplications. It can only
   rank engines on one host.
6. **Generic bounds.** The oracle is an algebraic solve plus F_p root finding, not a group-operation or membership oracle
   and not enumeration, so Corrigan-Gibbs–Kogan and Shoup do not bind it; it is a constant-factor lever. The exponent
   (`n^{1/4}` relations, `n^{1/2}` linear algebra), `Θ(1)` cost in `p`, and `r∞ = 0.52` (0.73 vs the negation walk) are
   unchanged.
7. **Degenerate quotients.** About 1 in 20 planted targets (dimension 517 plain) and 1 % of uniform weighted systems (509–511)
   are not 512-dimensional. msolve would show a different degree; they are charged and dropped today.
8. **Small primes.** msolve's documented limit is `p < 2³¹`; no lower bound is stated and p = 269 has not been tried.

## 6. Minimal discriminating test (frozen protocol)

**Question.** On the plain D₄ systems, is msolve's single-thread F4 + sparse FGLM wall time below the repository's best
no-replay solve? Nothing else is open.

**Instances.** `generate_d4_instance(p, seed)`, first planted target: `(p, seed) ∈ {(269, 1), (1009, 1), (1009, 2),
(1009, 3)}`. `(269, 1)` is `research/magma_f4_comparison_20261013/scripts/d4_p269_seed1.magma`; the others are exported with
`target/release/examples/gaudry_quartic_d4 --p 1009 --seed S --export-magma DIR` (one plain solve each, about 30 s on
4 threads). Plain form: unknowns `(s1,s2,s3,s4,pp)` in that order, `pp = π` last (smallest in grevlex, fewest dense columns).

**Conversion to msolve input** (mechanical, no code): `awk '/^F := \[P \|/{f=1;next} /^\];/{f=0} f' X.magma | tr -d ' '`
as the body; file = line 1 `s1,s2,s3,s4,pp`, line 2 the prime, then the body (all but the last polynomial end in `,`
already). Alternatively one `--export-msolve` flag beside `--export-magma` (an edit the main session decides; I made none).

**Instruments.** msolve (Ubuntu package or source; record `msolve -h` header and sha256), `taskset -c <one core>`,
`OMP_NUM_THREADS=1`, 600 s cap per run, 5 interleaved repetitions:
- M1 `msolve -v 2 -t 1 -g 1 -f X.ms -o X.g1.out` (dimension, degree);
- M2 `msolve -v 2 -t 1 -g 2 -f X.ms -o X.g2.out` (grevlex basis, F4 round table);
- M3 `msolve -v 2 -t 1 -P 1 -f X.ms -o X.P1.out` (F4 + sparse FGLM; degree as extracted by `run_msolve.sh`).
Repository arms, same host and core, `RAYON_NUM_THREADS=1`, `--p P --seed S --targets 12 --planted --json OUT`:
R0 default (committed, plain, degree 14; once per instance); R1 `--sparsest`; R2 `--weighted --chains --sparsest` (no replay:
first-solve cost, the like-for-like comparator); R3 R2 plus `--replay` (steady state, targets 1–11).

**Recorded.** Per run: wall ns; repository `total_muls` and phase means; msolve dimension, degree, largest F4 matrix
(rows × columns, from `-v 2`), memory. Check: target 0's `rational_solutions` equals the export's `ours_solutions`
(same RNG stream); a mismatch voids the instance.

**Prediction (before data).** (P1) msolve reports dimension 0, degree 512 on every instance (exact). (P2) M3 wall is
between 0.3× and 5× of R2 wall, median about 1× (low confidence). (P3) M2 is below M3 minus the Krylov share. (P4) R3/R2
mult-count ratio about 0.53 (`8.2/15.5`).

**Kill criterion.** The card's two, made mechanical:
- K1 (desk): neither path is `≥ 3×` below the current baseline at p = 1009. Path (i) `4.9·10⁷ + 2.3·10⁷ + 1.5·10⁶ ≈ 7.4·10⁷`
  (1.1×), path (ii) `3.0·10¹⁰` (0.003×). **K1 fires now**, without a run. Against the card's stale `2.54·10¹⁰` both pass.
- K2: M3 median wall is not below R2 median wall by `≥ 2×` on at least 3 of 4 instances. Fires → close. Does not fire →
  port msolve's degree-by-degree F4 ideas into `solve_system_w_inner` as an option, measured in counted multiplications.
Ratios within 3× are "comparable", not wins (L0 shared host, the convention of `magma_f4_comparison_20261013/PROTOCOL.md`).

**Cost.** 40 CPU-minutes, under 1 GB, 0.5–1 person-day. Stage diagnostic only: no leaderboard or `benchmarks/` row, so no
validation-gate certificate is owed. The §5a 0/0.1/1 GAE band does not apply (solver work is priced by count).

## 7. Ranking inputs

| expected gain | confidence | cost | tier | depends on |
|:--|:--|:--|:--|:--|
| 1.0–1.4× on `C₄` (`8.2·10⁷`), ≤ 1.9× best case, ≤ 3.6 bits on `n*`; card as written: none | medium | 0.5–1 person-day, 40 CPU-min | constant | msolve installable; `gaudry_quartic_d4_cost_20261014` baseline |

**Next improvements named (outside this card).** (1) Pivot/ordering search on the weighted echelon, where plain-form replay
already reads `2.7·10⁷` against `4.9·10⁷` (about 1.4× on `C₄`, 1.9 bits on `n*`). (2) A fast polynomial-matrix determinant
for `det A(λ)` at `p > 513` (31 % of `C₄`; ≤ 1.25×). (3) The M1–M3 test.

## 8. Open questions

- msolve's actual wall time on these systems (unmeasured here: not on PATH, `https://msolve.lip6.fr` returned 503, no
  Magma either).
- Whether msolve has any weighted-order option; absence is not confirmed.
- Whether the Moreno-Socías-based count `k = 84` holds for the plain D₄ ideal (the weighted `k = 44` agrees with the
  measured 44, an encouraging desk match, not a proof).
- `-P` semantics are taken from the repository's use, not from msolve's docs.
- BNS 2022 read at abstract level only; its constants at `t = 44` are unknown.
- The degenerate-quotient cause (dimension 509–517) remains uncharacterised (repository open item).

## 9. Sources

URLs fetched: `https://arxiv.org/pdf/1304.1238` (200, PDF saved, `pdftotext` extracted; the fetch tool's summariser could
not decode it, so I read the text myself); `https://arxiv.org/pdf/2104.03572` (200, `curl`, extracted);
`https://www-fourier.univ-grenoble-alpes.fr/~viva/research/articles/DLFq5.pdf` (200, `curl`, extracted);
`https://github.com/algebraic-solving/msolve` and `https://raw.githubusercontent.com/algebraic-solving/msolve/master/README.md` (200,
summaries); `https://msolve.lip6.fr` (HTTP 503); `https://www.ams.org/journals/mcom/1981-36-154/S0025-5718-1981-0606517-5/`
(HTTP 403). Three WebSearch queries (msolve paper, BNS 2022, FGLM 1993): snippets only. This is a search-scope statement,
not proof that no other prior art exists.

Repository paths read: `research/pdp_literature_leads_20261022/BRIEF_SONNET.md`, `LEADS.md` (lines 1–50, 519–531, 574–607,
644–675), `leads/LEAD-02.md`; `src/cryptanalysis/gaudry_quartic_d4.rs` (all), `gaudry_quartic.rs` (1613–1960 and
outline); `examples/gaudry_quartic_d4.rs` (1–260), `examples/gb_cas_export.rs` (110–200);
`research/gaudry_quartic_d4_20261013/README.md`; `research/gaudry_quartic_d4_cost_20261014/README.md`, `raw/summary.json`,
`raw/table.md`; `research/gaudry_quartic_replay_20261013/README.md`; `research/ic_speed_and_rho_scope_20261013/README.md`;
`research/notes/index-calculus/RESEARCH_RESIDUAL_WALKS.md` (2365–2472, 2521–2570);
`research/notes/index-calculus/RESEARCH_GLV_INVARIANT_FACTOR_BASES.md` (1148–1229); `groebner_compare/README.md`;
`research/magma_f4_comparison_20261013/` (`PROTOCOL.md`, `scripts/d4_p269_seed1.magma`, `ours_d4.jsonl`, `bundles/index.json`);
`research/gb_cas_comparison_20261011/README.md`; `research/f4_fp_msolve_parity_20261011/` (`README.md`, `run.sh`);
`research/pkm_m3_n21_msolve_20261018/README.md`; `research/ic_next10_20261014/02-solution-count/scripts/run_msolve.sh`;
`docs/ic/LEADERBOARD.md` (grep: no k = 4 row).

Desk checks (unverified, not measurements, not committed):
`/tmp/claude-0/-home-user/cf89363d-ad6e-5bef-bde6-195cbd206c93/scratchpad/lead18/desk.py` (Hilbert series: plain sum 512, top
degree 13, peak 84; weighted sum 512, top degree 25, peak 44; exact-degree monomial counts 3,060 and 560; Krylov, resultant
and root-finding arithmetic in §1 and §4).
