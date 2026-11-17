# LEAD-05 — Degree-of-regularity growth law for descended S₃ and S₄: Solovyev 2025, Kosters–Yeo, Huang–Kosters–Yeo

**Verdict.** unverifiable-from-here. The S₃ (two-point) import is plausible and
cheap to test (§6); its transfer to the `m = 3, 4` chain routes at `n = 131`
cannot be settled in this container. Either way the lead is a pricing input, not
an improvement candidate.

**Confidence.** medium that the S₃ thresholds 3 → 4 → 5 at square `N = 16, 36`
are right for the `m = 2` family (four independent data sets agree, one cell
conflicts); low that the slope prices the chain routes (the paper's own S₄ data
show no growth, and no repository cell shares its family).

**Expected gain if it works.** None as an improvement (accounting, per AGENTS.md).
If the slope transfers it moves priced rows up (desk check; `w = 1` column
touches, unconverted units; rho = 2^60.81 as quoted in the descent note):
`m = 3` closure-4 row 2^82.27 → 2^114.7–2^129.1; `m = 4` row 2^68.43 →
2^132.1–2^155.6. Distance to rho grows from +21.5 / +7.6 bits to +54–68 / +71–95.
Both rows would sit 68–82 / 120–143 bits below the repository's semi-regular
rows (2^196.95 / 2^275.14): the bracket narrows, no baseline row drops.

**Cost of the discriminating test.** About 1 person-day, 10–15 CPU-hours, ≤ 12 GB
(gate cells `N ≤ 38`, `D ≤ 4`, msolve attempts capped). The decisive cells need
hosts we lack: S₄ at `N = 27` (paper: ≈ 37 GB, days), S₃ at `N ≈ 52`, `D = 6`,
chain `m = 4`, `ℓ = 6` (`N ≥ 54`, ≥ 128 GB by the repository's estimate).

## 1. Mechanism (precise)

**S₃.** `E: y²+xy = x³+a₂x²+a₆` over `F_{2^n}`;
`S₃ = X₀²X₁²+X₀²X₂²+X₁²X₂²+X₀X₁X₂+a₆` (paper §2.1).

**Solovyev's S₃ family (repository's `m = 2`, no auxiliary variable).**
`X₀, X₁ ∈ V` random, `dim V = ⌈n/2⌉`, `X₂ = x(Q)`. Descent gives `n` quadrics in
`N = 2⌈n/2⌉` Boolean variables (`N = n` even, `n+1` odd) in the Boolean ring.
First fall degree 2 (trace form, Kosters–Yeo Cor. 4.11).

**Claimed law.** `D_solve` = least `D` with `V_D ⊇` reduced GB, `V_D` the HKY
Def. 1 degree-capped closure. Generic dimensions: `dim V₃ = (3N−2)n − C(N,2)`;
`dim V₄ = ½[3N(N+1)n − 171n − 9n²]`; degree-5 part
`Q₅ = 3n·C(N,3) − C(3n+1,2)·N − n(79N−660)`. Criterion: `D_solve ≤ D` iff
`Q_D(N,n) > C(N,D)`. The ideal holds exactly `3n` quadrics (`n` equations, `N−1`
multiples of the trace-linear form, `2n−N+1` degree-fall quadrics); closures
behave like `3n` semi-regular Boolean quadrics minus a lag (`5n+C(N,2)`, `87n`,
`n(79N−660)`).

**Thresholds (paper §5.3; my desk check reproduces every printed margin).** Even
`n`: D = 3 up to 16, 4 for 18–36, 5 for 38–52 (52 is a prediction). Odd `n`
(`N = n+1`): D = 4 from 15, 5 from 35, 6 from 51 (margin −0.24 % at `(52,51)`).
The abstract's "5 for 37 ≤ n ≤ 52" ignores this parity split. Semi-regular
thresholds for `3N` quadrics: 20, 37, 56, 76, 97, 119 (reproduced; next 141, 163,
186, 209, 232). Slope: `c(3) = 0.036` asymptotically (one degree per 28
variables), about 1/20 in the computed range; `c(1) ≈ 0.09` for `n` generic quadrics.

**Solovyev's S₄ (unchained).** Resultant of two S₃: `n` equations of Boolean degree
6 in `N = 3⌈n/3⌉` variables, about `d⁶/2` monomials each; first fall degree
`m²−m+1 = 7` (Kousidis–Wiemers). All 20 systems computed (`N = 12–24`, `n/N` from 1
down to 1/3) collapse at `D = 7`: solving degree = first fall degree.

**The repository's `m = 3, 4`.** Semaev-2015 chain of `m−1` S₃ equations with `m−2`
auxiliary `F_{2^n}` points: `N = mℓ + (m−2)n`, cubic plus quadratic blocks. At
`n = 131`: `m = 3, ℓ = 40` gives `N = 251`; `m = 4, ℓ = 33` gives `N = 394`.
**Neither Solovyev family is the chain.** His S₃ is the repository's `m = 2`; his
S₄ is unchained with `N = 3ℓ`.

**Where the degree bound would come from.** HKY Prop. 2.8 (solve in
`V_{F,max(d_F,s)}`), Petit–Quisquater `D_reg ≈ D_ff`, Kousidis–Wiemers
`D_ff ≤ m²−m+1`. No `n`-independent last-fall bound is proved for the subspace
factor base (HKYY §6; repository `RESEARCH_FALL_DEGREE_BOUNDS.md` §2.3).

## 2. Prior art: literature

- **A. Solovyev**, repo `alekssolov/weil-descent-s3-dreg`, `paper/paper.tex`
  (September 2026; code MIT, text CC BY 4.0). **read-at-source** (raw files, 200).
  Claims: §1 above. Provenance: single independent author (Omsk), no venue, 2 commits
  (24 Sep, 5 Oct 2026), release `v1.0.0` flagged pre-release (14 Oct), 0 stars; the
  acknowledgements say an AI assistant (Claude) was used for programming, analysis and
  drafting. The card's "2025" is wrong (the files say 2026). The release asset URL
  was 404; I read `main` (`de71443`, which the release tags).
- **M4GB fix**: pull request **#14 of `cr-marcstevens/m4gb`** ("Fix Gebauer-Moeller
  F-criterion in update(): equal-lcm pairs discarded each other"), opened 18 Sep 2026
  by `alekssolov` (the preprint's author), merged 25 Sep 2026 by the maintainer.
  **abstract-only** (PR page via WebFetch summary); the corrected criterion is in
  master's `src/m4gb.hpp` lines 1682–1708 (**read-at-source**). Claim: the old code
  inflated the reported step degree by one on part of the S₃ family (4 vs Magma's 3 at
  `n = 16`); the basis was correct.
- **Kosters–Yeo**, *Notes on summation polynomials*, arXiv:1503.08001 (v3).
  **read-at-source**. Magma `D_reg` (random `⌈n/2⌉` subspace): 3, 3, 4, 4, 4, ≥5 at
  `n = 12, 16, 17, 20, 30, 40` (`n = 40` exceeded 38 GB). §5: Semaev's non-random
  subspace gave 5 at `n = 45` (126 GB) and 4 at `n = 40`; `n = 25, m = 3, t = 3`
  "also seem[s]" 5, unfinished at 111 GB. The first-fall assumption applied to the
  splitting trick would solve an NP-complete problem in polynomial time.
- **Kousidis–Wiemers**, J. Math. Cryptol. 13 (2019), arXiv:1906.05594.
  **read-at-source**. `D_ff ≤ m²−m+1` for `S_{m+1}` (Thm 3.2). Table 1 (Magma): `m = 2`,
  `n = 34–48`: 4, 4, 4, then 5 from `n = 37`; `m = 3`, `n = 13–21` (`N = 15–21`):
  `D_ff = D_reg = 7`; `m = 4`, `n = 13–16`: 13. Solovyev does not cite it.
- **Huang–Kosters–Yeo**, CRYPTO 2015, ePrint 2015/573. **abstract-only** (abstract
  page 200; PDF 403). Its §5.2 table reaches me via Kosters–Yeo and Solovyev only.
- **Huang–Kosters–Yang–Yeo**, JSC 87 (2018), arXiv:1505.02532. **read-at-source**
  (abstract, Thm 1.1, §6). An `n`-independent last-fall bound for zero-dimensional
  radical systems; summation systems are not zero-dimensional without the field
  equations, so it does not apply.
- **Petit–Quisquater** (ASIACRYPT 2012; `D_ff ≤ m²+1`, assumption `D_reg = D_ff+o(1)`)
  and **Bardet–Faugère–Salvy(–Yang)** (semi-regular series): **recalled**; I verified
  the series numerically.

## 3. Prior art: these repositories

- `research/dreg_closure_scale_20261016/{RESULTS,PREREGISTRATION}.md` (no `README.md`
  exists in these four directories). Closure refutes 40 unsatisfiable draws at 4 for
  `N = 26–42`, except `m = 3, ℓ = 7` (`N = 40, 42`), `> 4` on 8/8; degree 5 at
  `N = 40` is 760k columns, unmeasured. **Supports** the law's one visible step
  (4 → > 4 near `N = 37–40`), not its family.
- `dreg_closure_driver_20261016/RESULTS.md`. `N` does not set the degree (`ℓ = 6`
  refutes at 4 at `N = 40, 42`; `ℓ = 7` does not). **Qualifies**: the chain is not a
  function of `(N, equations)` alone.
- `dreg_closure_mutantxl_20261016/RESULTS.md`. Closure 3–4 on 92 draws where XL needs
  5–7; soundness control 0/345; work ≈ columns^2.2. **Supports** §4 item 1.
- `dreg_ell_grid_20260925/RESULTS.md`, `notes/index-calculus/RESEARCH_DREG_MEASUREMENT.md`.
  XL degrees 5, 6, 6–7, 7; first fall 3. Result 9: semi-regular `D_reg` matches XL;
  Result 11: that growth "is XL's". Result 1's `m = 2` rows are XL with `ℓ ≪ n/2`.
  **Supports** the XL/closure split; the `m = 2` rows are **unrelated**.
- `notes/index-calculus/RESEARCH_FALL_DEGREE_BOUNDS.md`,
  `fall_degree_bounds_20261008/{RESULTS,TABLE}.md`. Sage `lfd.sage` measures the HKY
  last fall degree by mutant closure on **exactly Solovyev's family** (`m = 2`, random
  `V`, `n' = n/2`): `d_F = 3` on 37 of 38 random-base draws at `n = 8–16` (one
  unsatisfiable draw at `n = 10` reads 2), `4` on all 10 draws at `n = 18, 20, 22`.
  **Supports** the 16|18 boundary with independent code. Its E2 control reads the
  Semaev system one degree below the `n`-quadric semi-regular reference ("not yet
  identified"); the paper's extra quadrics explain such a gap.
- `ic_next10_20261014/{README.md, 06-equivariant-f4-m4/README.md}`. msolve F4 max step
  degree 4 (some 3) through `(11, 4)` at `m = 4` and `(15, 5)` at `m = 3`, where XL
  reads ≥ 7. Against the closure study's table the two agree draw for draw on every
  draw where both exist (36 `m = 4`, 12 `m = 3`; my reading of two tables, not a
  rerun). **Supports** §4 item 1.
- `notes/ecc2k130/RESEARCH_ECC2K130_DESCENT_DEGREE.md` (lines 1–150) and
  `descent_degree_cost-output-20261016-scale.txt`: the cost model I replicate; its
  verdict (free-oracle floor 2^68.58 at `m = 3` vs rho 2^60.81; `m = 4` needs an
  oracle below 2^5.71 per target) is degree-independent.
- `examples/dreg_ladder.rs` (line 104: `assert!(m >= 3)`), `dreg_closure.rs`,
  `dreg_sweep.rs`, `assumption1_closure.rs`: the closure driver cannot generate
  Solovyev's `m = 2` cells without new code.
- `crypto-autoresearcher/research/dreg-linear-law/FINDING_v2.md` (read), `VALIDATION.md`
  and `ledger/evidence/EV-SEMBIN-07960c.yaml` (skimmed; first-fall median 2 at toy
  `n`). FINDING_v2 prices the chain at the semi-regular XL level (`c* = 0.2375` per `n`,
  `d_reg(24) = 10` at `N = 48`) and says the true degree is "≥ this". Closure and F4
  read 4 at `N = 42–44` on the repository's chains and Solovyev reads 4–5 up to
  `N = 52`, so that premise fails at the closure/F4 level: its 2^1194 is an XL-level
  number. All sources say linear; the slope depends on the instrument.
  **Closes** it as an F4-level price.
- `RESEARCH_EXOTIC_COORDINATES.md` §8: an unchained symmetrised S₄ arm (Artin–Schreier
  frame, Boolean degree 4): a different system. **Unrelated.**

## 4. Why it could lower the Gröbner cost or the degree of regularity

It cannot lower either. What it offers:

1. **Instrument reconciliation (settled from existing data).** Solovyev's `D_solve` is
   the degree-capped closure with iterated falls: the repository's MutantXL closure
   (for unsatisfiable systems, least `D` with `1 ∈ V_D`) and HKY's `V_{F,c}`, in the
   same Boolean ring (columns `C(N, ≤D)`). It is **not** the XL/Macaulay degree: the
   paper shows `R₄ = 2994 < P₄ = 4995` at `(20,10)`, and the repository reads XL 5–7
   where the closure reads 4. A correct F4's top step degree is `≥ D_solve` (his
   Lemma 1) and equals it on all data; msolve equals the closure on the repository's
   draws; the paper reports last fall degree `= D_solve` on every instance. So closure
   = F4 = last fall = `D_solve`, XL above. The first fall degree (2, 3 on the chain, 7
   for S₄) bounds nothing above.
2. **Where the repository's cells sit.** The law has `D ≤ 5` for all `N ≤ 52`; every
   chain cell has `N ≤ 44`. "Flat at 4" cannot separate constant from slope 1/20. The
   one step (4 → > 4 at `m = 3, ℓ = 7`, `N = 40`) lands near the law's 4 → 5
   (`N = 37–38`) but is no test: the chain also reads 4 at `N = 40–44` for `ℓ = 6`
   and `m = 4`.
3. **Pricing arithmetic (unverified desk check).** My script reproduces ten committed
   rows of the descent cost model to the printed digit (`m = 3, 4`; `D = 4, 5, 7`;
   `w = 1, 2`). Model: targets `2^{ℓ+max(0, n−mℓ+log₂m!)}`, oracle `C(N,≤D)^w`, linear
   algebra `2^{2ℓ+log₂m}`, `n = 131`. Widths: `C(251,≤4) = 2^27.3`,
   `C(251,≤12) = 2^66.5`, `C(251,≤15) = 2^78.8`; `C(394,≤4) = 2^29.9`,
   `C(394,≤5) = 2^36.2`, `C(394,≤17) = 2^97.8`, `C(394,≤22) = 2^119.0`;
   `C(132,≤7) = 2^36.9`, `C(132,≤9) = 2^44.6`.

   | `n = 131`, `w = 1` | `m = 3`: ℓ, N, D, total | `m = 4`: ℓ, N, D, total |
   |---|---|---|
   | constant 4 (repository; `m = 3` withdrawn at ℓ = 7) | 40, 251, 4, 2^82.27 | 33, 394, 4, 2^68.43 |
   | constant 5 (repository) | 41, 254, 5, 2^85.18 | 34, 398, 5, 2^71.13 |
   | law, asymptotic slope (D = 4 + (N−36)/28) | 43, 260, 12, 2^114.7 | 34, 398, 17, 2^132.1 |
   | law, slope 1/20, or semi-regular(3N) × 0.93–0.97 | 42–45, 257–266, 15–16, 2^128.9–129.1 | 33–34, 394–398, 21–22, 2^151.5–155.6 |
   | repository semi-regular `D_reg(N)` | 26, 209, 28, 2^196.95 | 27, 370, 55, 2^275.14 |

   "Law" rows apply the S₃ square-system `D(N)` to the chain's `N` (assumption A1,
   not derived). At `w = 2` (closure work ≈ columns^2.2) every row is higher. Solovyev's
   own `m = 2` at `n = 131`: `N = 132`, `D = 9` (10 at the 0.93 end), `2^44.6` columns
   per call against `2^66` relations: exponential, as he says.
4. **A formulation signal.** At `ℓ = 6` the chain reads 4 (`N = 36–42`) where direct S₄
   reads 7 (`N = 18`): the chain buys degree with variables. If chain `D` grows while
   direct S₄ stays at 7, direct S₄ (`N = 3ℓ`) prices 2^86.79 (`ℓ = 42`, `w = 1`),
   between the constant-4/5 and law rows. That price is not tenable: each descended
   equation has about `44⁶/2 = 3.6·10⁹` terms (his density), so one system is
   ≈ 4.8·10¹¹ terms (≈ 2^38.8) *per target* (`x(Q)` enters the coefficients), more than
   the oracle width it credits. Direct S₄ is no improvement lead either.

## 5. Obstructions

- **Family mismatch (decisive).** The law is for `m = 2`, quadratic, `N ≈ n`. His S₄
  data show no growth (flat 7 to `N = 24`; Magma 7 to `N = 21`), so his own data refute
  the first-fall assumption only for `m = 2`, which Kousidis–Wiemers call pathological
  and Petit–Quisquater's `m ≈ n^{1/3}` regime does not use. The only `m = 3` growth
  evidence is the repository's `ℓ = 7` rise. The "3n quadrics" multiplier has no chain
  analogue until the chain's quadratic-part dimension is measured.
- **Reach of the fit.** Formulas fitted at `N ≤ 24`, verified to `N ≈ 40`; `Q₅` has two
  fitted constants and was checked only at `n − N/2 ≤ 3`. The 52|53 boundary
  extrapolates from `n/N ≤ 0.78` to 1 at −0.24 % margin; `D = 6` is bracketed 66–76.
  "Slope 1/20" rests on one measured gap (16 → 36) and one predicted (36 → 52); beyond
  it the model is semi-regular `3N` with an assumed lag ratio 0.93–0.97.
- **A conflicting cell.** Kousidis–Wiemers' Magma reads `D_reg = 4` at `n = 35`
  (`N = 36`, 10 repetitions, memory like `n = 36`); Solovyev reads 5 by closure and
  M4GB. By his Lemma 1 a correct F4 reading of 4 contradicts `D_solve = 5`. Of 15
  `m = 2` rows in Kousidis–Wiemers, 14 agree. Unresolved from here.
- **Provenance.** Unrefereed, single author, AI-assisted, pre-release; PR author = paper
  author (the maintainer's merge checks the criterion, not the degree claims).
- **Oracle class.** A Macaulay/F4 oracle is algebraic linear algebra, not group
  operations plus membership tests: Corrigan-Gibbs–Kogan/Shoup and the structured
  generic-group bound neither cover nor forbid it. What binds at `n = 131` is the
  free-oracle floor, which holds at any degree; the law can only raise the rows.
- **Units.** Column touches are not group operations; `K` is unmeasured. No §5a band is
  computed and no two arms are ranked; only the distance is reported.

## 6. Minimal discriminating test (frozen protocol)

**Purpose.** Gate the import on its own family and settle `n = 35`. A pass licenses the
`Q_D` criterion for S₃ `m = 2` at `N ≤ 40`; it says nothing about chains or `D ≥ 6`.

**Instruments.** (a) Solovyev's `wdclose.c` (`./wdclose input.txt D`) with
`python3 tools/wd_export_c.py N n D [seed]`, from `github.com/alekssolov/weil-descent-s3-dreg`
at commit `de71443`, `gcc -O3 -march=native -fopenmp`, in a scratch directory outside
the repository (external code; record commit and binary SHA-256). (b) msolve 0.6.5
(`-t 1 -g 2 -v 2`, DRL, characteristic 2, field equations appended, as in
`ic_next10_20261014/06-equivariant-f4-m4`) on the `*.in` files in `data/m4gb_runs/`
(headers record N, n, seed, #solutions), converted to msolve format. A
repository-native second instrument needs a new `m = 2` example; it is not part of
this frozen test.

**Cells** (criterion margin `Q_D − C(N,D)` in parentheses). Record `dim V_D`,
`|B_{≤D}|`, #solutions; collapse iff `dim V_D = |B_{≤D}| − #sol`.

| id | `(N, n)` | `D` | predicted | msolve `D_F4` |
|---|---|---|---|---|
| B1 | (16,16) | 3 | collapse (+8) | 3 |
| B2 | (16,15) | 3 | no collapse (−35) | 4 |
| B3 | (18,17) | 3 | no collapse (−136) | 4 |
| B4 | (34,32) | 4 | collapse (+40) | 4 |
| B5 | (34,31) | 4 | no collapse (−1271) | 5 (attempt) |
| B6 | (36,36) | 4 | collapse (+117) | 4 |
| **B7** | (36,35) | 4 | **no collapse (−1365)**; Magma says 4 | 5 (attempt) |
| B8 | (38,37) | 4 | no collapse (−5217) | 5 (attempt) |
| F1 | (20,11) | 3, 4 | `dim V₃ = 448`, `dim V₄ = 5445` | – |
| F2 | (26,15) | 3, 4 | 815, 13,500 | – |
| F3 | (30,17) | 3, 4 | 1,061, 20,961 | – |

**Rules.** msolve on B5, B7, B8 is capped at 12 GB and 3 h; a kill is "unmeasured",
never evidence. **PASS:** 8/8 flags and 3/3 formula points exact at both levels.
**KILL (the card's rule):** ≥ 2 of F1–F3 differ, or ≥ 2 flags differ (boundary
displaced by more than one in `n`). **SPLIT:** exactly one deviation; if it is B7
(the `D = 4` closure collapses at `(36,35)`), Kousidis–Wiemers stands and the odd-`n`
parity claim and the M4GB `n = 35` result are wrong.

**Cost.** Closures: ≤ 83k columns, minutes each (≤ 2 CPU-h). msolve B1–B4, B6: ≤ 1 h
each; B5, B7, B8 capped. Total ≤ 15 CPU-h, ≤ 12 GB, one mechanical-tier day. Beyond
this container: chain `dim(V₃ ∩ B_{≤2})` per cell (the "3n" analogue), `m = 4, ℓ = 6`,
S₄ at `N = 27`.

## 7. Ranking inputs

| expected gain | confidence | cost | tier | depends on |
|---|---|---|---|---|
| none as improvement; priced `m = 3` / `m = 4` rows move +32…+47 / +64…+87 bits up if the slope transfers (distance to rho, unconverted) | medium (S₃ import), low (transfer) | ≈ 1 day, ≤ 15 CPU-h for the gate; ≥ 48–128 GB hosts for the decisive cells | neither: calibration of existing rows (accounting) | nothing for the gate; a ≥ 128 GB host for chain `D = 5, 6` and S₄ `N = 27`; any lead that prices the descent route reads these rows |

## 8. Open questions

- Is Magma's `D_reg = 4` at `n = 35` an instrument artefact, or does the closure
  collapse? (B7.)
- Does S₄'s collapse at `D = 7` persist beyond `N = 24`? His `N = 27` run needs ≈ 37 GB.
- Is the chain's quadratic-part dimension a multiple of `n`? Unmeasured; without it
  there is no chain analogue of `Q_D`.
- Does msolve's `D_F4` equal `D_solve` at `D = 5` on his family? The identity is shown
  only on the repository's chains at `D ≤ 4`.
- Unread: HKY §5.2 (ePrint 403), the release PDF (404), the PR diff beyond a summary.
  I ran nothing; every number from my scripts is an unverified desk check, not a
  measurement. The release lists two assets but names one, so I cannot say whether
  it differs from `main`'s paper.

## 9. Sources

**Fetched (HTTP).** `https://github.com/alekssolov/weil-descent-s3-dreg` 200, `…/releases/tag/v1.0.0`
200, `…/releases` 200, `…/commits/main` 200 (all WebFetch summaries);
`raw.githubusercontent.com/alekssolov/weil-descent-s3-dreg/main/{README.md,
README_UPDATE_2026-10.md, paper/paper.tex, paper/paper.pdf, m4gb/M4GB_ISSUE.md}` 200;
`…/releases/download/v1.0.0/paper_v1.0-preprint.pdf` 404;
`https://github.com/cr-marcstevens/m4gb/pull/14` 200 via WebFetch (summary), 403 via curl
and the GitHub tools (repository not enabled for the session; not added);
`raw.githubusercontent.com/cr-marcstevens/m4gb/master/src/m4gb.hpp` 200;
`https://arxiv.org/abs/{1503.08001,1906.05594,1505.02532}` and
`https://arxiv.org/pdf/{same}` 200; `https://eprint.iacr.org/2015/573` 200 (abstract),
`…/2015/573.pdf` 403 (curl and WebFetch);
`https://link.springer.com/chapter/10.1007/978-3-662-47989-6_28` 303 to
`idp.springer.com` (not followed).

**Repository paths read.** Under `/home/user/crypto/research/`:
`pdp_literature_leads_20261022/{BRIEF_SONNET.md, LEADS.md}`;
`dreg_closure_scale_20261016/{RESULTS,PREREGISTRATION}.md`;
`dreg_closure_driver_20261016/RESULTS.md`; `dreg_closure_mutantxl_20261016/RESULTS.md`;
`dreg_ell_grid_20260925/RESULTS.md`;
`notes/index-calculus/{RESEARCH_DREG_MEASUREMENT,RESEARCH_FALL_DEGREE_BOUNDS}.md`;
`notes/index-calculus/RESEARCH_EXOTIC_COORDINATES.md` (§8 skim);
`fall_degree_bounds_20261008/{RESULTS,TABLE}.md`;
`ic_next10_20261014/{README.md, 06-equivariant-f4-m4/README.md}`;
`notes/ecc2k130/{RESEARCH_ECC2K130_DESCENT_DEGREE.md (lines 1–150),
descent_degree_cost-output-20261016-scale.txt}`;
`/home/user/crypto/examples/{dreg_ladder,dreg_closure,dreg_sweep,assumption1_closure}.rs`
(headers); `/home/user/crypto-autoresearcher/research/dreg-linear-law/{FINDING_v2,VALIDATION}.md`,
`ledger/evidence/EV-SEMBIN-07960c.yaml` (skim). Desk-check scripts (not committed) are in
the session scratchpad.
