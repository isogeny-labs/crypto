# LEAD-17 — Closed-form closure dimensions for the descended S₃ chain (the degree law without a 128 GB host)

**Verdict.** unverifiable-from-here. The count closes exactly at degrees 2 and 3
(and for the plain Macaulay rank at degree 4) for `m = 3, 4`, `ℓ ≥ 4`. It does
not determine the saturation degree: the quadric-fall space is not stable from
`D = 3` to `D = 4`, so no `D(n, ℓ)` or `(m, ℓ) = (4, 6)` prediction follows.
The card's "(ℓ, m) = (4, 6)" is read as `m = 4, ℓ = 6`, the cell the driver
study names as the separator of its two surviving hypotheses (`m·ℓ = 24` either
way).

**Confidence.** medium that the degree-≤3 formulas are right (16 of 17
committed `ℓ ≥ 4` cells match exactly or within ±2, plus about 100 draws of my
own); low for any statement about `D`, which needs data the count does not have.

**Expected gain if it works.** None as an improvement (theory and pricing, same
class as LEAD-05). No priced row moves: `m = 3` degree-4 row 2^82.27 (+21.46
bits to rho 2^60.81), `m = 4` degree-4 row 2^68.43 (+7.62), degree-5 rows +24.37
and +10.32, all `w = 1`, unconverted units
(`research/dreg_closure_scale_20261016/RESULTS.md`). Distances only.

**Cost of the discriminating test.** Replay of the formulas on committed records:
under 1 CPU-minute, no new run. New cells (§6): about 40 CPU-minutes, 8–10 GB
(the closure engine's known limit at N = 44). Degree 5 needs a ≥ 70–128 GB host.

## 1. Mechanism (precise)

**The system, from the code** (`koblitz_groebner.rs::build_decomposition_system`,
`koblitz_bench.rs::closure_refutation_scan`). `S₃(a,c,d) = (a+c)²d² + acd + (ac)² + 1`.
Summands `x_i ∈ V` (`dim V = ℓ`, bits `c_i`), `m − 2` intermediate points
`e_j ∈ F_{2^n}` (bits free), `N = mℓ + (m−2)n` Boolean unknowns, `(m−1)n` equations:
`S₃(x₁,x₂,e₁)`, `S₃(e_j,x_{j+2},e_{j+1})`, `S₃(e_{m−2},x_m,x_R)`. Squaring is
`F₂`-linear in the bits, so every equation has degree ≤ 1 per variable block: the
last link is `n` quadrics, every other link `n` cubics. There are no explicit
field equations; the Macaulay matrix is over square-free monomials (Boolean ring).
Rows are `p·t` with `deg t ≤ D − deg p`. "Closure" is MutantXL: every row of
degree < D times every variable, until nothing new falls. `V_D` is its span.

**Closed forms** (`ℓ ≥ 4`, `m = 3, 4`; `C(·)` binomials, `C(N,≤2) = 1+N+C(N,2)`):

| quantity | formula |
|---|---|
| `dim XL₂` | `n` |
| `dim V₂` | `n + N − 1` |
| `dim XL₃` | `n(N + m − 1) − 1` |
| `dim(V₃ ∩ B≤2)` | `mn + (m−1)ℓ` |
| `dim V₃` | `(mn+(m−1)ℓ)(N+1) + (m−2)n − C(N,2) − (2m−1)n − (2m−2)ℓ − [n − dim(V·V)]` |
| `dim XL₄` | `n·C(N,≤2) + (m−2)n(N+1) − C(n+1,2) − (m−1)(n+ℓ)` |

For `m = 3` the `V₃` line reduces to `n(5n−1)/2 + 8nℓ + ℓ(3ℓ−1)/2 − [n − dim(V·V)]`.
`V·V` is the `F₂`-span of pairwise products of `V`; for random `V` its dimension
is `min(n, C(ℓ+1,2))`, so the bracket is `max(0, n − C(ℓ+1,2))`.

**Where each term comes from.**
- *Trace-linear fall (`V₂`).* In the last link, `Q = y²(x_m²+x_R²) + y·x_m x_R + x_m²x_R² + b`.
  With `μ = x_R^{-2}`, `Tr(μQ) = Tr(y) + Tr(x_m) + Tr(b·x_R^{-2})`, because
  `Tr(w) = Tr(w²)` kills the bilinear terms: one linear form `L` (Kosters–Yeo's
  group morphism to `F₂`). `V₂ = n + (N−1)` is the `n` quadrics plus `x_j·L`.
- *One degree-3 dependency.* Extracting the kernel at `(m,n,ℓ) = (3,8,5)` and
  `(4,8,4)` shows it uses only last-link rows times `c_m` and `e_{m−2}` variables;
  no cubic row is involved. I read it as the same trace identity lifted. That is an
  interpretation; the count (1) and location are measured.
- *`3n + 2ℓ` quadrics.* Chain analogue of Solovyev's "exactly `3n` quadrics":
  `n + (N−1)` plus `n − ℓ + 1` new degree-fall quadrics, for every `m`.
- *Lag.* `q(N+1) + (m−2)n` rows minus `C(N,2) + (2m−1)n + (2m−2)ℓ`. The `C(N,2)`
  and `5n` match Solovyev's lag (`5n + C(N,2)`); the extra `4ℓ` (and the
  `(2m−1)n + (2m−2)ℓ` pattern) are **fitted** on `m = 3, 4`.
- *Degree-4 syzygies.* `C(n+1,2) = n + C(n,2)` is consistent with `Q(Q+1) = 0`
  (idempotency) and the Koszul pairs of the `n` quadrics; only the count is
  verified. The remainder `(m−1)(n+ℓ) = N + n − ℓ` is fitted.
- *Deviation `n − dim(V·V)`.* Identified on a structured `V = span{1,z,…,z^{ℓ−1}}`
  (`dim V·V = 2ℓ−1`): the deficit is `n − (2ℓ−1)` in all 5 cells tested. For
  random `V` it is the `b(b+1)/2` symmetry defect of
  `research/linearization_reach_20260930/README.md` with `b = ℓ`. It lowers `V₃`:
  structured `V` is worse at degree 3, in the same direction as Kosters–Yeo's
  Semaev-subspace observation (direction only; not a claim).

**Bound the formulation obeys.** HKY Prop. 2.8 / Petit–Quisquater `D_reg ≈ D_ff`
as in LEAD-05: no `n`-independent last-fall bound is proved for subspace bases.
The count gives only a lower bound: `V₃ / C(N,≤3) ≤ 0.33` at every `ℓ ≥ 4` cell
and falls like `1/N`, so `D ≥ 4` there.

## 2. Prior art: literature

- Kosters, Yeo, *Notes on summation polynomials*, arXiv:1503.08001.
  **abstract-only** this session (WebFetch 200); full text read in LEAD-05.
  Claim used: Weil descent of `S₃` in characteristic 2 has first fall degree 2 from
  a group morphism to `F₂` (my `L`).
- Solovyev, `alekssolov/weil-descent-s3-dreg`, `paper/paper.tex` (2026).
  **recalled via LEAD-05** (read-at-source there, not refetched). Claim used: `dim V₃`,
  the `3n` quadrics and the `5n + C(N,2)` lag for the unchained `m = 2` family.
  Unrefereed, single author, AI-assisted (LEAD-05 §2).
- Kousidis, Wiemers, arXiv:1906.05594, and Huang–Kosters–Yang–Yeo,
  arXiv:1505.02532. **recalled via LEAD-05** (read there). Claims used:
  `D_ff ≤ m²−m+1`; last fall degree is bounded only for zero-dimensional radical
  systems.
- Caminata, Gorla, solving degree versus degree of regularity. **recalled**; I did
  not verify the identifier (my guess `arXiv:2202.05175` returned an unrelated
  paper). Not relied on.

## 3. Prior art: these repositories

- `research/linearization_reach_20260930/README.md`: `b(b+1)/2` defect of the
  bilinear core. **Supports**: it is the identity behind the `n − dim(V·V)` term.
- `dreg_closure_scale_20261016/{RESULTS,PREREGISTRATION}.md`, `runs/*.jsonl`:
  per-degree `xl_rank`, `closure_rank`, `columns` for 40 unsat draws; flat at 4 to
  N = 42 except `m = 3, ℓ = 7`. **Supports** (data source; the JSONL, not the prose,
  holds the ranks).
- `dreg_closure_driver_20261016/RESULTS.md`: `m = 3 ℓ = 6`, `m = 4 ℓ = 5` flat;
  successor `m = 4, ℓ = 6`. **Supports**; the open cell.
- `dreg_closure_mutantxl_20261016/RESULTS.md`: 92 draws, closure 3–4 vs XL 5–7.
  **Supports** (ℓ ≤ 5 rows, most below the regime of the formulas).
- `src/cryptanalysis/koblitz_groebner.rs` (system builder, Macaulay rows),
  `koblitz_bench.rs` (closure scan, solution counts), `binary_semaev.rs` (S₃).
  Define the system above; no field equations beyond the Boolean ring.
- `notes/ecc2k130/RESEARCH_CHAIN_SPLIT_ORDER.md`, `RESEARCH_ECC2K130_DESCENT_DEGREE.md`
  (grep only): unknown counts `mℓ + (m−2)n`; `N` does not set the closure degree.
  **Supports**.
- `leads/LEAD-05.md`: closure = F4 = last fall = `D_solve`; open question "is the
  chain's quadratic-part dimension a multiple of `n`?". **Answered**: no, `3n + 2ℓ`
  (`mn + (m−1)ℓ`).

## 4. Why it could lower the Gröbner cost or the degree of regularity

It cannot; it is a diagnostic and pricing input. What it settles:

1. **The instrument.** My independent code (Python system builder, C++ closure)
   reproduces the committed `(m,n,ℓ) = (3,11,5)` cell exactly: `XL₃ = 307`,
   `V₃ = 772`, `XL₄ = 4071` (unverified desk check; scripts in the scratchpad,
   not committed).
2. **The comparison, 8 of 17 cells shown** (committed measured / formula; all `m`
   with `ℓ ≥ 4`; `XL₂, V₂` are not recorded, the scans start at degree 3):

   | (m,n,ℓ) | N | eqs | XL₃ | V₃ | XL₄ |
   |---|--:|--:|---|---|---|
   | (3,11,5) | 26 | 22 | 307 / 307 | 772 / 772 | 4071 / 4071 |
   | (3,16,6) | 34 | 32 | 575 / 575 | 1451 / 1451 | 9916 / 9916 |
   | (3,19,7) | 40 | 38 | 797 / 797 | 2027 / 2027 | 16136 / 16136 |
   | (3,21,7) | 42 | 42 | 923 / 923 | 2338 / 2338 | 19600 / 19600 |
   | (3,24,6) | 42 | 48 | 1055 / 1055 | 2627–2628 / 2628 | 22368 / 22368 |
   | (4,11,5) | 42 | 33 | 494 / 494 | 1591 / 1591 | 10776 / 10776 |
   | (4,12,5) | 44 | 36 | 563 / 563 | 1799 / 1799 | 12843 / 12843 |
   | (4,13,4) | 42 | 39 | 584 / 584 | 1798–1799 / 1799 | 12728 / 12728 |

   Over all 17 cells: `XL₃` and `XL₄` exact in 16, `V₃` within ±2 in 16; each
   exception is one draw (see §5). Out-of-sample: the `V₃` formula, fitted on `m = 3`
   `ℓ = 4–7` and `m = 4` `ℓ = 4`, hits `(4,9,6)` = 1380 and `(4,10,6)` = 1578 exactly
   (`f₂ = 54, 58`), the `m = 4, ℓ = 6` cells (desk).
3. **The card's kill rule is not triggered** at degree 3 for `ℓ ≥ 4`: the largest
   miss on a committed cell is 2, at the threshold `n ≈ C(ℓ+1,2)`.

**What the count cannot give: degree 4.** With `q₄ = dim(V₄ ∩ B≤2)` and
`c₄ = dim(V₄ ∩ B≤3)`, unsaturated `m = 3` cells at `D = 4` read (unverified desk
check, high-solution cells `n ≪ 3ℓ`):

| (n,ℓ) | N | V₄ / C(N,≤4) | q₃ = 3n+2ℓ | q₄ | V₃ | c₄ |
|---|--:|---|--:|--:|--:|--:|
| (8,6) | 26 | 13165 / 17902 | 36 | 78 | 591 | 1345 |
| (8,7) | 29 | 15430 / 27841 | 38 | 66 | 674 | 1321 |
| (9,7) | 30 | 18597 / 31931 | 41 | 64 | 772 | 1514 |
| (10,7) | 31 | 22662 / 36457 | 44 | 76 | 875 | 1786 |

So `q₄/q₃` is 1.5–2.2 and `c₄/V₃` about 2. Unlike Solovyev's `m = 2` family, the
quadrics are not fixed once `D = 3` is passed: new falls appear at each degree, and
a lagged count of the form `Q₄ = q₃·C(N,2) − lag` has no stable `q`. A proxy
`V₃·N / C(N,4)` does not separate saturating from non-saturating cells: 0.98–1.34
(saturates, `m = 3`), 0.88 (does not, `ℓ = 7`), 0.58–0.68 (saturates, `m = 4`);
at `(4, n=9..12, ℓ=6)` it reads 0.50–0.52. No threshold exists across `m`, so I make
no prediction at `(4, 6)`. Both repository hypotheses ("ℓ = 7", "m·ℓ ≥ 21") remain
open.

**Slope.** `dim V₃ ≈ 2.5n²` (`m = 3`), `≈ 6n²` (`m = 4`) at fixed `ℓ`, against
`C(N,≤3) ~ N³/6`: the cubic closure's share of its ambient space falls like `1/N`.
That is a statement about `D = 3` only; the slope of `D` in `n` is not derived.

## 5. Obstructions

- **Degree 4 is open** (§4): `q₄`, `c₄` are not determined by the degree-3 count, and
  `V₄` itself is draw-dependent (94230, 94230, 92838, 94926 at `(19,7)`).
- **Regime.** The formulas fail for `ℓ ≤ 3` (e.g. `(8,3)`: `f₂ = 90` against 30);
  there the `x_i` space is too small and more falls exist. They were checked for
  `m = 3, 4` only. `m = 5, 6` build timed out in my desk code; extension to the
  literal `(ℓ, m) = (4, 6)` is untested.
- **Outlier draws.** About 4 of 75 of my random draws (and `(3,11,4)` draw 2, `V₃ = 1596`
  against 670; `(3,7,4)` control draws with `XL₃ = 293`) have far larger `V₃` and
  `f₂`. `dim(V·V)` is generic in them, so it is not the cause; unidentified.
- **Fitted, not derived:** the `4ℓ` term, the `(2m−1)n + (2m−2)ℓ` and `(m−1)(n+ℓ)`
  patterns. The trace-parity ±1 (e.g. `V₃ = 2304` or `2305`) is draw noise at that size.
- **Unsat versus sat.** For satisfiable draws `V_D` is bounded by the solution space
  (`(3,11,5)` sat draw: 17884 of 17902), so a generic count is only visible in
  unsaturated cells.
- **Oracle class.** Macaulay/F4 algebraic linear algebra, not group operations plus
  membership tests; the generic bounds neither cover nor forbid it. The
  free-oracle floor of the descent note binds at any degree. Units are column
  touches; no §5a band is computed and no two arms are ranked.

## 6. Minimal discriminating test (frozen protocol)

**T1 (no new run, mechanical).** Read every `runs/*.jsonl` of the three closure
studies; for records with `ℓ ≥ 4` and `m ∈ {3,4}` compute the table of §1 and compare
`xl_rank` at degrees 3 and 4 and `closure_rank` at degree 3 (non-refuted records).
PASS: `XL₃`, `XL₄` equal in ≥ 95 % of records and `V₃` within ±2 in ≥ 90 %.
KILL (the card's rule): a cell with `ℓ ≥ 4` whose `V₃` misses by more than ±2 in
≥ 50 % of its draws. Prediction: PASS (already done by me, 16 of 17 cells).

**T2 (existing binary, new cells).** `dreg_ladder` + `dreg_closure --satisfiable
--to 4 --max-cols 155000` on `m = 4, (n, ℓ) = (10, 6)`, `N = 44`, 149,986 columns, 4
satisfiable draws, plus `dreg_closure --to 4` on `m = 3` satisfiable draws at
`(19,7)`. Record `xl_rank`, `closure_rank` at degrees 3 and 4, `columns`,
`solutions`. Predictions: `XL₃ = 469`, `V₃ = 1578`, `XL₄ = 10707` at `(4,10,6)`
(formulas; the first two verified in desk). The degree-4 outcome is a
**measurement of the separator**, not a prediction: "ℓ = 7" says `closure_rank ≥
columns − solutions`; "m·ℓ ≥ 21" says below. Caveat: `N − 3n = 14` gives about
2^14 solutions, so this reads collapse onto the solution space (a solving
degree), not unsat refutation. Cost about 40 CPU-minutes, 8–10 GB, plus
solution counting at `2^{(m−1)ℓ+n+m−2} = 2^30`.

**T3 (needs a 5-line patch to the scan output).** Print row counts by leading-monomial
degree at `D = 4` on `m = 3` `(19,7)`, `(21,7)` and `(22,6)` unsat draws (about 150–250 s,
5.8 GB each, from the scale study). This gives `q₄, c₄` where the question lives
(`S ≈ 0`); fit a lagged count to them. The patch is repository work and is not done here.

## 7. Ranking inputs

| expected gain | confidence | cost | tier | depends on |
|---|---|---|---|---|
| none as improvement; fixes a pricing input (exact `V₃`, `XL₃`, `XL₄`) | medium (degree ≤ 3), low (`D` law) | T1 < 1 CPU-min; T2 ≈ 40 CPU-min; T3 ≈ 1 CPU-h plus a patch | neither (diagnostic; accounting) | T1 none; T3 and degree 5 need a patch and a ≥ 128 GB host; LEAD-05 reads the same rows |

## 8. Open questions

- What fixes `q₄`, `c₄` (and `q_D` in general) at `S ≈ 0`? This is the missing identity.
- Why do ≈ 5 % of draws have much larger `V₃`? Not `dim(V·V)`; not tested: dependence on `x_R`.
- Do the fitted `(2m−1)n + (2m−2)ℓ` and `(m−1)(n+ℓ)` patterns hold for `m ≥ 5`?
- Is the `D = 3` dependency exactly the lifted trace identity?
- Unread this session: the full Solovyev text and the HKY §5.2 (relied on via LEAD-05).
  I ran no repository binary and no git; every number from my scripts is an
  unverified desk check, not a measurement.

## 9. Sources

**Fetched.** `https://arxiv.org/abs/1503.08001` 200 (abstract); `https://arxiv.org/abs/2202.05175`
200, unrelated paper (my recalled identifier was wrong). Others via LEAD-05 (not refetched).

**Repository paths read** (under `/home/user/crypto`): `research/pdp_literature_leads_20261022/{BRIEF_SONNET.md,LEADS.md (Standing constraints, LEAD-17),leads/LEAD-05.md}`;
`research/linearization_reach_20260930/README.md`;
`research/dreg_closure_{scale,driver,mutantxl}_20261016/{RESULTS.md,PREREGISTRATION.md (scale),runs/*.jsonl,score-output.txt (scale)}`;
`research/notes/ecc2k130/{RESEARCH_CHAIN_SPLIT_ORDER.md,RESEARCH_ECC2K130_DESCENT_DEGREE.md}` (grep);
`src/cryptanalysis/{binary_semaev.rs (header),koblitz_groebner.rs (builder, Macaulay rows),koblitz_bench.rs (closure scan, solution counts)}`.
Desk scripts, not committed, in the scratchpad
`/tmp/claude-0/-home-user/cf89363d-ad6e-5bef-bde6-195cbd206c93/scratchpad/`: `chain.py`, `clos.cpp`,
`sweep3.py`, `xl4.py`, `fit.py`, `struct.py`, `dep3.py`, `dump.py`.
