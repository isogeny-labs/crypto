#set page(paper: "a4", margin: 1.6cm)
#set text(size: 8.6pt)
#set par(justify: false)
#show table: set text(size: 6.6pt)
#set table(stroke: 0.4pt + gray, inset: 3pt)
#show raw.where(block: false): r => text(font: "DejaVu Sans Mono", size: 7.2pt, r.text.replace("_", "_\u{200b}").replace("/", "/\u{200b}"))
#show heading.where(level: 1): set text(size: 13pt)
#show heading.where(level: 2): set text(size: 11pt)
#show heading.where(level: 3): set text(size: 9.5pt)
= Factor bases, solving degree and arity in elliptic-curve index calculus: what is explored, what is not
<factor-bases-solving-degree-and-arity-in-elliptic-curve-index-calculus-what-is-explored-what-is-not>
#strong[Date.] 2026-10-24. #strong[Kind.] Desk research over the four
repositories (`crypto` at `d51b908a77`, branch
`feat/koblitz-norm-base-20261023`\; `cryptanalysis`\;
`crypto-autoresearcher`\; today\'s
`ic-regime-review-20261024/REVIEW.md`) plus a three-lane literature
sweep (factor-base constructions; solving-degree reduction; large-arity
decomposition). Nothing new was run. Every repository number is quoted
from the file named beside it; numbers marked #emph[derived here] are
this note\'s arithmetic on quoted numbers and are not measurements. No
security claim about any deployed curve is made or withdrawn.

#strong[The question asked.] Across the factor-base strategies that
lower the solving degree (degree of regularity) or the factor-base size,
there seem to be two trade-off curves: solving degree against
factor-base dimension, and cost against the number of summands
`m = 3 … 9`. What other strategies exist, which have never been tried,
and which tried ones were not exhausted?

#divider()

== 0. Bottom line
<0-bottom-line>
+ #strong[The two curves are real, and the repository has measured one
  half of each.] Solving degree against factor-base dimension `ℓ` is
  measured on binary curves at `m = 3, 4` only (chained `S₃`
  presentation), and on prime-field PKM towers at `m = 2, 3, 4` with
  `N = m·t ≤ 20`. Cost against arity is #emph[derived] at `n = 131` for
  every `m` (the free-oracle floor and the per-target budget), but
  #strong[no algebraic solving degree has been measured at `m ≥ 5` in
  any of the repositories]. The literature has exactly three `m ≥ 5`
  data sets, all at `ℓ ≤ 3` and `n ≤ 29`: Semaev 2015 (chained `S₃`,
  `n ≤ 16`, F4 degree 4 in 2,300 systems),
  Huang--Petit--Shinohara--Takagi 2015 (the same chain, `n = 13–29`,
  `m = 5, 6`, measured `D_reg` #strong[4.3--5.0], 13--34 hours and
  44--102 GB per instance), and Karabina 2015 (two `S₄` links with one
  auxiliary point, `m = 5`, `n ≤ 19`, `D = 7–8`). Nobody has `m ≥ 5`
  with `ℓ ≥ 4`. The `m ≥ 5` half of the arity curve is the largest
  unexplored region in both the program and the field, and the first
  cells are reachable with the repository\'s existing closure
  instrument.
+ #strong[The shape of a single subspace is closed as a lever on the
  degree; presentation and position-dependence are not.] On binary
  curves at prime `n`, every `F₂`-subspace shape sits at random once the
  trace class is conditioned on (the one measured effect was an
  artefact), and the only closure a subspace can have is bounded by
  linear Kneser and already attained by geometric progressions
  (`RESEARCH_FACTOR_BASE_SHAPE_SEARCH.md`). Yield is conserved for every
  set of a given size (`AR:KN-FIND-007`). What moves the degree is how
  the same ideal is presented to the solver: fall-closure/F4 against XL
  (2--3 degrees), the 2-torsion quotient `E/⟨T⟩` (bit-degree 7 → 5 per
  unknown), the symmetric `e_k ∈ V^{(k)}` formulation (unknowns
  `Σ dim V^{(k)}`, 15--40× cheaper on progressions), auxiliary variables
  (PKM towers on prime fields, the chain on binary), and, in the
  literature only, drawing each summand from a different coset or block
  (Galbraith--Gebregiyorgis, Diem): the one base-side change measured to
  lower the F4 degree, never built here.
+ #strong[The deciding measurements are known, specified, and mostly
  unrun.] In order of information per unit cost: (a) a #strong[Δ-sweep
  at fixed `(m, ℓ)`]: every published \"degree 4 flat to `ℓ = 8`\" cell
  sits at overdetermination `Δ = n − mℓ` of 13--76, the repository\'s
  rise past 4 at `ℓ = 7` sits at `Δ = −2, 0`, and no one has held
  `(m, ℓ)` fixed while moving `Δ`\; (b) the
  #strong[Galbraith--Gebregiyorgis `m = 4` formulation] (binary Edwards
  `t`-coordinate, `(Z/4)^{m−1} ⋊ S_m` invariants), the published best
  `m = 4` Gröbner result (`ℓ = 4` in 9--52 s at `n ≤ 53`), one to two
  orders cheaper than the repository\'s chain at the same `(m, ℓ)`, and
  closed in the repository on a different experiment; (c) the first
  #strong[closure degrees at `m = 5, 6`] on the chain at `N ≤ 56`, which
  fit the current engine; (d) the FGHR-symmetrised `m = 4` system, whose
  unknown count `4ℓ − 3` is independent of `n` (the chain\'s is
  `2n + 4ℓ`), never built because `symmetrised_terms` returns `None` for
  `m ≥ 4`\; (e) any `m = 4, ℓ ≥ 4` cell at `Δ ≥ 0` (every measured one
  is at `Δ ≤ −3`); (f) the guess count that holds the closure at degree
  4 past `ℓ = 7` (one cell, 8 draws); (g) a fitted elimination exponent
  `w` on the real Macaulay matrices, since the `m = 4` price moves by
  about 40 bits between `w = 1` and `w = 2.3`. Items (d)--(g) are on the
  2026-10-18 card `backlog/XP-20261018-m4-solving-degree-equivariant/`\;
  (a)--(c) are new here.
+ #strong[A derived what-if shows why the arity curve matters.] Under
  the one bounded-degree observation the repository has (fall closure at
  degree 4, held to `N = 54`), a formulation whose unknown count does
  not grow with `n` would put `m = 5–6` within roughly 4--15 bits of rho
  at `n = 131`, where the chained presentation stays 16--47 bits above
  it across `m = 4–8` (§1.3, derived here, heavily caveated). Whether
  the degree stays bounded on such a formulation is exactly what no one
  has measured, and the published heuristics (Petit--Quisquater\'s
  `D ≈ m² + 1` first fall degree; Semaev\'s bounded-Macaulay-degree
  assumption) disagree about it by the whole exponent.
+ #strong[For prime fields the map is closed to one open line and one
  closed loophole.] The PKM tower is the only presentation with
  sub-`|F|` membership degree, and its F4 degree grows about one degree
  per eight unknowns at `m = 4` (`D ≥ 8` at `N = 20`), with `m = 3`
  dying at 99.6 GiB inside degree 8. The autoresearcher\'s approved,
  unbuilt \"non-enumerative membership oracle\" (`EXP-ICEX-153c34`) is,
  by its own coordinator\'s Chinese-remainder argument, enumeration in a
  changed basis: `F_p[x]/(g_V) ≅ F_p^V`. Only auxiliary variables escape
  that, which is why PKM is the only line left.

#divider()

== 1. The two curves, as measured
<1-the-two-curves-as-measured>
=== 1.1 Solving degree against factor-base dimension `ℓ` (binary, `F₂`-subspace bases)
#label("11-solving-degree-against-factor-base-dimension-ℓ-binary-f₂-subspace-bases")
System: Weil descent of the chained `S₃` system for `m` summands in an
`ℓ`-dimensional random subspace `V ⊂ F_{2^n}`, `N = (m − 2)·n + m·ℓ`
Boolean unknowns, first fall degree 3 everywhere (Kosters--Yeo trace
equation). Three instruments read three different degrees on the same
draws (`RESEARCH_DREG_MEASUREMENT.md` Results 4--14;
`ic_next10_20261014/06-equivariant-f4-m4/README.md`):

#figure(
  align(center)[#table(
    columns: 6,
    align: (right,right,auto,auto,auto,auto,),
    table.header([`m`], [`ℓ`], [plain XL refutation degree], [msolve F4
      degree], [MutantXL fall-closure degree], [cells, surplus],),
    table.hline(),
    [3], [2], [5], [4], [3--4], [`N = 11–18`, `S = −1…+6`],
    [3], [3], [5--6], [4], [4], [`S = −5…0`],
    [3], [4], [6--7], [4], [4], [`S = −5…+1`],
    [3], [5], [#strong[7] exactly at `(10,5)`\; `≥7` at
    `(11,5) (13,5) (15,5)`], [4 at `(10,5)`,
    `(15,5)`], [4], [`S = −5…0`],
    [3], [6], [not run], [not run], [4 (8/8,
    `N = 34, 36`)], [`S = −4, −2`],
    [3], [7], [not run], [not run], [#strong[\> 4] (8/8, `N = 40, 42`);
    degree-5 closure capped at 169 GB; one guessed variable restores 4
    on 6/8 draws], [`S = −2, 0`],
    [4], [1], [5], [3--4], [3--4], [],
    [4], [2], [6], [3--4], [3--4], [`S = −3…0`],
    [4], [3], [`≥7`], [4], [4], [`S = −3, −2`],
    [4], [4], [not measured], [4 (4/4 at `(11,4)`); `(13,4)` capped at 3
    GB], [4 (`N = 42`)], [`S = −5, −3`],
    [4], [5], [not measured], [capped], [4 (8/8 at
    `(11,5) (12,5)`)], [`S = −9, −8`],
    [4], [6], [not measured], [not run], [4 (2/2 at `(15,6)`, `N = 54`,
    25.8 GB)], [`S = −9`],
  )]
  , kind: table
  )

Readings the repository itself draws:

- #strong[Which degree is which.] The closure column is the quantity
  every published experiment reports (Magma\'s F4 step degree) and, on
  refuted instances, equals the last fall degree exactly
  (Caminata--Gorla 2023, §5.2). The XL column has no published
  counterpart.
- #strong[The XL fits `⌈ℓ/2⌉ + 4` (`m = 3`) and `ℓ + 4` (`m = 4`) are
  XL\'s, not the ideal\'s.] The closure and F4 read 4 flat across
  `m ∈ {3, 4}`, `ℓ ≤ 6`, `N ≤ 54`. The rise at `m = 3, ℓ = 7` is the
  first bounded-degree failure; whether it is one degree or many is
  unmeasured (Result 12--14; `dreg_closure_m4l6_20261018/RESULTS.md`
  refutes \"`m·ℓ ≥ 21`\" and leaves \"`ℓ = 7`\" as the surviving
  explanation).
- #strong[The degree is set by `(m, ℓ)` and is flat in `n` at fixed `ℓ`]
  (Result 10). The semi-regular reference, which counts unknowns,
  overstates growth in `n`.
- #strong[Every `m = 4`, `ℓ ≥ 4` cell is at surplus `S ≤ −3`]
  (underdetermined), chosen because unsatisfiable draws were affordable
  there. Index calculus runs at `S ≈ 0`. No `m = 4` cell at `S ≥ 0` with
  `ℓ ≥ 4` exists (XP card §3, \"Not measured anywhere\").
- #strong[One structural number predicts the `m = 2` degree exactly]
  (cryptanalysis `experiments/pdp-degree-heuristics/README.md`, 656
  bases, 7 families, 9 fields): with `e = n − dim V − dim V^{(2)}`, the
  closure degree is `D = 2 + [e < −3.5] + [e < −8.5]` (MAE 0.06).
  Geometric progressions minimise `dim V^{(2)} = 2ℓ − 1` and are 15--40×
  cheaper per query where a random base needs one more degree. At
  `m = 3` the direct formulation\'s degree is set by system size, the
  family moves it by at most one degree; in the #strong[symmetric
  formulation] (unknowns are coordinates of `e_k ∈ V^{(k)}`) the unknown
  count #emph[is] `Σ_k dim V^{(k)}` (15 against 19 at `ℓ = 3`\; 21
  against 34 at `ℓ = 4`) and progressions are 500--1,500× cheaper where
  the random base needs one more degree. This is the one place where
  factor-base shape still moves cost, and it is a presentation effect
  (the shape sets the unknown count of the symmetric system), not a
  degree effect.
- #strong[Solver cost slopes per unit of `ℓ`] (cryptanalysis
  `experiments/pdp-scaling/README.md`, Koblitz `n = 17…61`, planted
  instances, fits `log₂ s = a + c·ℓ + d·n`): meet-in-the-middle
  `c = 1.93` (`m = 3`), `1.89` (`m = 4`), `2.48` (`m = 5`, range too
  short for the `2^{3ℓ}` half); WDSat `3.02`\; WDSat with the trace
  clause `2.51`\; CryptoMiniSat `3.95` (`m = 3`), `6.97` (`m = 4`);
  msolve `5.25` (`m = 3`). Every algebraic or SAT engine has a steeper
  slope than enumeration; the trace-clause variant is the only one
  within 0.6 bits of enumeration\'s slope.

=== 1.2 Solving degree against tower depth (prime fields, PKM towers)
<12-solving-degree-against-tower-depth-prime-fields-pkm-towers>
System: `S_{m+1}` plus `t` quadratic tower equations per summand,
`N = m·t` auxiliary unknowns, `|V| = 2^t`
(`docs/ic/factor-base-encyclopedia/README.md` §0.1;
`pkm_m3_n21_msolve_20261018`\; REVIEW §1 item 3):

#figure(
  align(center)[#table(
    columns: 4,
    align: (right,right,auto,auto,),
    table.header([`m`], [`N = m·t`], [F4 degree `D`], [status],),
    table.hline(),
    [2], [2…18], [4, 4, 4, 4, 4, 5, 5, 5, 5], [plateau at 5],
    [2], [20, 22], [6, 6], [rises],
    [3], [9, 12, 15, 18], [6, 6, 7, 7], [],
    [3], [21], [`≥ 8`, run died at 99.6 GiB inside degree 8], [formally
    inconclusive; the review puts the prior on linear growth],
    [4], [8, 12, 16], [6, 7, 7], [],
    [4], [20], [`≥ 8` after three productive degree-8 rounds, 5 GB
    cap], [plateau broken: about one degree per eight unknowns],
  )]
  , kind: table
  )

`D` does not depend on the prime, on `v₂(p − 1)` up to 96 (P-224), or on
Kummer against isogeny against Dickson-torus tower (82 cells, PR
\#1942). A deployed size needs `t ≈ 56–64` at `m = 4` with admissible
degree about 3; measured is 7--8 at `t ≤ 5`.

=== 1.3 Cost against arity `m` at `n = 131` (ECC2K-130), derived
<13-cost-against-arity-m-at-n--131-ecc2k-130-derived>
Repository model (`RESEARCH_ECC2K130_DECOMPOSITION.md` §5;
`RESEARCH_ECC2K130_DESCENT_DEGREE.md` §2): relations `2^ℓ`, targets per
relation `m!·2^{n − (m−1)ℓ}`, linear algebra `m·2^{2ℓ}`, oracle
`C(N, ≤D)^w` per target. Rho reference `2^60.81` with the `⟨−1⟩ × ⟨π⟩`
discount. All `n = 131` rows are extrapolations.

#figure(
  align(center)[#table(
    columns: 6,
    align: (right,right,right,right,right,right,),
    table.header([`m`], [`ℓ` at the floor], [free-oracle
      floor], [per-target oracle budget (ops)], [search space per target
      `2^{(m−1)ℓ}`], [required speed-up over search],),
    table.hline(),
    [2], [43.3], [`2^89.25`], [`2^−27.86`], [`2^43.3`], [`2^−71.2`],
    [3], [33.0], [`2^68.58`], [`2^−6.78`], [`2^65.0`], [`2^−71.8`],
    [4], [26.8], [`2^56.40`], [#strong[`2^+5.71`] (≈ 52
    operations)], [`2^77.9`], [`2^−72.2`],
    [5], [22.8], [`2^48.44`], [`2^+13.94`], [`2^86.5`], [`2^−72.5`],
    [6], [19.9], [`2^42.85`], [`2^+19.77`], [`2^92.5`], [`2^−72.8`],
    [8], [16.1], [`2^35.61`], [`2^+27.35`], [`2^100.5`], [`2^−73.2`],
  )]
  , kind: table
  )

Three facts about this curve that the repository states and that frame
everything below:

- #strong[For any enumerative or table oracle the curve is flat.] The
  product `relations × targets × oracle ≈ m·2^n` is independent of `ℓ`,
  and the required speed-up over the oracle\'s own candidate set is
  `≈ 2^−72` at every `m`. Corrigan-Gibbs--Kogan (`S·T² ≳ r/|Aut|`) and
  the 2025 Dinur--Keller--Marmor non-adaptive bound close every table or
  k-sum design regardless of `m`.
- #strong[The budget per target rises steeply with `m`] (52 operations
  at `m = 4`, `2^14` at `m = 5`, `2^20` at `m = 6`, `2^27` at `m = 8`),
  while the yield falls as `2^{−(m−1)ℓ}`. So the arity curve for an
  #emph[algebraic] oracle is a race between a budget that grows about
  `2^{7(m−4)}` and a system whose size and degree grow with `m`.
- #strong[The presentation decides which way the system grows.] Two
  unknown counts at the floor\'s `ℓ`, #emph[derived here]:

#figure(
  align(center)[#table(
    columns: 6,
    align: (right,right,right,right,auto,right,),
    table.header([`m`], [`ℓ`], [chained `S₃`:
      `N = (m−2)·131 + mℓ`], [FGHR-symmetrised direct `S_{m+1}`:
      `N_B = m(ℓ−1) + 1`], [degree-4 columns `C(N, ≤4)` chained /
      FGHR], [per-target budget (ops)],),
    table.hline(),
    [3], [33], [230], [97], [`2^26.8` / `2^21.8`], [`2^−6.8`],
    [4], [27], [370], [105], [`2^29.5` / `2^22.3`], [`2^5.7`],
    [5], [23], [508], [111], [`2^31.4` / `2^22.6`], [`2^13.9`],
    [6], [20], [644], [115], [`2^32.7` / `2^22.8`], [`2^19.8`],
    [8], [16], [914], [121], [`2^34.8` / `2^23.1`], [`2^27.4`],
  )]
  , kind: table
  )

The chained unknown count is dominated by `(m − 2)·131`
intermediate-point coordinates and grows with `m`\; the FGHR count is
independent of `n` and nearly flat in `m`. #strong[A what-if, derived
here and not a measurement.] Hold the fall-closure degree at 4 (measured
to hold on the #emph[chained] system only, to `N = 54`, `m ≤ 4`), take
the repository\'s measured closure exponent `w ≈ 2.2`
(`dreg_closure_driver_20261016`) and its ≈ 8-bit column-touch to
rho-step conversion (REVIEW §1 item 3). Then the per-target oracle costs
`2^51–2^69` rho steps on the chained system and a nearly flat
`2^40–2^43` on the FGHR system, against budgets that rise about 7 bits
per unit of `m`. Excess over rho, in bits:

#figure(
  align(center)[#table(
    columns: 3,
    align: (right,right,right,),
    table.header([`m`], [chained, degree 4 held], [FGHR direct, degree 4
      held],),
    table.hline(),
    [4], [+47], [+31],
    [5], [+35], [+15],
    [6], [+26], [+4],
    [8], [+16], [nominally below rho],
  )]
  , kind: table
  )

The chained `m = 4` row agrees with the review\'s independent +37 to +49
bracket at degree 5. Three caveats bound the rest of the table. (i)
#strong[It assumes a bounded degree at `m ≥ 5`, which nothing has
measured]: the repository\'s degree data stop at `m = 4`. (ii) On the
FGHR direct system the Boolean degree of the generators themselves grows
with `m` (the plain-`x` descent has degree at most `Σ wt(e_i)`: 6, 12,
15--20 at `m = 3, 4, 5`\; cryptanalysis `pdp-scaling`), so a degree-4
Macaulay space cannot contain them at `m ≥ 5` without auxiliary
variables, and the FGHR gain measured at `m = 3` is mostly the change of
curve to `E/⟨T⟩` plus one cover variable
(`lit2026_w8_summation_models_20261011`); the FGHR rows at `m ≥ 5` are
therefore illustrative only. (iii) On the chained system the generator
degree is fixed at 3 for every `m`, so its rows have no such
obstruction: the only unmeasured quantity is the closure degree as `N`
grows past 54. The reading that survives the caveats is this:
#strong[under any bounded-degree assumption the arity curve improves
with `m`, because the budget grows faster than a bounded-degree column
count; so the first closure-degree measurement at `m = 5` is the single
most informative unmeasured point on the curve, and on the chained
system it is well posed and affordable] (`m = 5, ℓ = 3, n = 13` has
`N = 54`, the same size as the `(15, 6)` cell that ran in 25.8 GB).

#strong[What the literature says the optimum `m` is.] Three heuristics
and one theorem, none of which reaches `n = 131` (lane report
`lit_large_arity.md` §A5, §(b)):

#figure(
  align(center)[#table(
    columns: 5,
    align: (auto,auto,auto,auto,auto,),
    table.header([source], [assumption], [optimal `m` at given
      `n`], [claimed crossover with rho], [status],),
    table.hline(),
    [Petit--Quisquater 2012], [`D_reg ≈ D_ff ≤ m² + 1`
    (first-fall-degree assumption)], [`m = 2` for `n ≤ 200`, 3 at 500, 4
    at 1,000--2,000, 9 at 20,000; asymptotically `m ≈ n^{1/3}`,
    `ℓ ≈ n^{2/3}`], [`n ≳ 2,000`], [assumption refuted experimentally
    (Kosters--Yeo, Huang--Kosters--Yeo, Kousidis--Wiemers: `D_reg` rises
    with `n` at fixed `m`\; `S₅` Weil descent reaches
    `D_reg = 13 = m(m−1)+1` at `m = 4`)],
    [Semaev 2015], [chained `S₃`, F4 degree `≤ 4` for every
    `m`], [`m ≈ √(2 ln 2 · n / ln n)`: 6 at `n = 100`, 7 at 150, 8 at
    200, 12 at 500], [`n > 310` (so ECC2K-130 is #strong[not] reachable
    even under his assumption)], [assumption contradicted at
    `(n, m) = (45, 2)` and `(25, 3)` (`D_reg = 5`); HPST 2015 measure
    4.3--5 at `m = 5, 6`],
    [Karabina 2015], [`D_reg ≈ 9` at `m = 5`], [`m = 5`,
    `ℓ = n/5`], [`n ≈ 1,200`], [extrapolation from `n ≤ 19`],
    [Diem 2013 (rigorous)], [none], [`m = min(⌈√log₂ q⌉, ⌊n/2⌋)` with
    each summand in its own direct-summand subspace], [subexponential
    `L(3/4)` for `a·log^{1/3} q ≤ n ≤ b·log q`], [gives nothing for
    `F_{2^n}`, `n` prime],
    [Galbraith--Gebregiyorgis 2014 (practical)], [measured], [\"the best
    we can seem to hope for in practice is `m = 4` and
    `ℓ ≤ 10`\"], [none: `2^{n−40} ≥ 2^{n/2}` trials for
    `n > 100`], [measured to `m = 4, ℓ = 4` (Gröbner, 9--52 s at
    `n ≤ 53`) and `ℓ ≤ 7` (SAT)],
  )]
  , kind: table
  )

Joux--Vitse state the program\'s exact question as an open problem (JoC
2013 §6): with a subspace of dimension `d > 1` and `m < n`, \"the best
values for `d` and `m` involve a trade-off that depends heavily on the
cost of the decomposition test\", and the equations lose the `S_{md}`
symmetry, keeping only `S_m`\; their invariant-ring attempts \"have been
unsuccessful so far\". No published work has resolved it.

=== 1.4 The third axis: surplus (overdetermination)
<14-the-third-axis-surplus-overdetermination>
`RESEARCH_DESCENT_CROSSOVER.md` proves the identity that the
overdetermination `S = n − mℓ` of the descended system and the log of
the expected decomposition count are the same parameter: one bit of
yield per surplus equation. So \"pick `ℓ` a little below `n/m`\"
(Shantz--Teske\'s δ; Joux--Vitse\'s `(k−1)`-point decompositions on
extension fields) buys a cheaper refutation at a yield cost that is
exact. The repository\'s surplus control (Result 6) shows the degree
rising only at `S ≤ −4`\; at `S ≥ −2` every `m = 3` cell reads 6 (XL) /
4 (closure). The DoR campaign (`factor_base_dor_campaign_20261013`, PR
\#1826) measured δ at `m = 3`: shrinking `ℓ` from 3 to 2 at `n = 15`
drops XL `D*` from 5 to 3. At `m = 4` the positive-surplus side of this
axis is unmeasured for `ℓ ≥ 3`.

#divider()

== 2. Factor-base strategies: inventory and verdicts
<2-factor-base-strategies-inventory-and-verdicts>
The full inventory is `docs/ic/factor-base-encyclopedia/README.md`
(A1--A13 prime, B1--B2 extension, C1--C11 binary, D1--D10 Koblitz,
E1--E6 Jacobians). Compressed by what each family was #emph[for]:

#figure(
  align(center)[#table(
    columns: 4,
    align: (auto,auto,auto,auto,),
    table.header([aim], [family], [verdict], [bounding result],),
    table.hline(),
    [lower membership degree (binary)], [`F₂`-subspace, any shape
    (coordinate, random, progression, Frobenius progression, trace-zero,
    Möbius pole, scaled, inversion-closed)], [#strong[closed at
    `m = 2`]: trace class is a free bit; closure `dim V·V ≥ 2ℓ − 1` by
    linear Kneser, attained by progressions; at `m = 3` shape moves `D`
    by 0 at `ℓ ≤ 4` (G1, G3, G5 killed)], [Hou--Leung--Xiang;
    `RESEARCH_FACTOR_BASE_SHAPE_SEARCH.md` §2.2--2.3],
    [], [subfield `F_{2^ℓ}`], [one degree below random, #strong[does not
    exist at prime `n`]\; on `b = 1` the yield collapses (points lie in
    a subgroup)], [],
    [Frobenius-stable set at prime `n`], [invariant
    subspaces], [#strong[empty]: dimensions `{0, 1, 130, 131}` at
    `n = 131`], [`ord₁₃₁(2) = 130`],
    [], [Hamming weight in a normal basis], [rejected: calls grow as
    `|F|^{1.9–2.5}`\; SAT at `n = 23` decides nothing in 180
    s], [`RESEARCH_HAMMING_IDEAL_PDP.md`\; `pdp_followups_20261010`],
    [], [cyclotomic γ-weight `C_w`], [yield-neutral (p = 0.36 against
    the null), 2--12% costlier to build; #strong[solver clause untested]
    (one `n = 11` cell went the wrong
    way)], [`cyclotomic_falsifier_20261015`],
    [], [quasi-subfield polynomials], [barren at `n = 131`\;
    `β > 1/2`], [`RESEARCH_QUASI_SUBFIELD.md`],
    [], [quadratic-form sets `Tr(c x^{2^e+1}) = 0`
    (G4)], [#strong[proposed, never built]], [shape-search §7],
    [raise yield at equal `D`], [τ-twisted ordered chain
    `R = P₁ + τP₂ + τ²P₃` (G2)], [#strong[survives at `n ≤ 9`] (degree
    rise 0); `×m!` yield unread; gate G6 (`n = 31`) never
    run], [`shape_search_g1_g6_20261015`],
    [], [any set geometry (prime)], [#strong[theorem]: mean yield
    depends only on `|F|`\; coverage headroom ≤
    1.58], [`AR:KN-FIND-007`],
    [fewer columns (Koblitz)], [signed Frobenius orbits; `Z[τ]`
    small-norm multipliers; integer multiples; CM
    multipliers], [constants (`|Aut|`, multiplier set size); table route
    only; \"does not help the algebraic route\" (asserted, not
    measured)], [`ic_improve_20261023`\; CGK bound],
    [sub-`|F|` membership degree (prime)], [PKM towers (Kummer, isogeny,
    Dickson torus)], [#strong[the only open prime-field line]\; `D`
    grows ≈ 1 per 8 unknowns at `m = 4`\; needs 256--512 GB for the next
    cells], [encyclopedia §0; REVIEW §1.3],
    [], [one-binomial cosets, Dickson sets, intervals, orbits,
    Eisenstein, bit-box, Möbius images, isogeny-augmented], [refuted or
    constant: behave as random sets of their size], [yield conservation;
    pair-table law `S = 0.75·r^{1/6}`\; Bezout `B ≤ 3d`],
    [], [small-root (Coppersmith) membership], [closed: reach `δ = 1/80`
    against `1/5` needed at
    `m = 4`], [`RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md`],
    [], [non-enumerative Gröbner membership
    (`EXP-ICEX-153c34`)], [approved, unbuilt; coordinator\'s own prior
    0.95 negative: `F_p[x]/(g_V) ≅ F_p^V` (CRT), so it is enumeration in
    a changed basis], [`AR:H-ICEX-6ca21f`],
    [intermediate field], [Gaudry--Diem `x ∈ F_q`\; Joux--Vitse
    `(k−1)`-points; FGHR `Y`-line; `D₄`-symmetrised `S₅` at
    `k = 4`], [the only measured sub-rho exponents live here (and in the
    genus-3 cover); constants 18--850×; stacked `D₄ + 2LP` never
    run], [encyclopedia §3],
  )]
  , kind: table
  )

Results that bound every family at once: yield conservation; sparsity
invariance (every generating set of the membership ideal of a fixed set
has the same Hilbert function, so re-encoding without auxiliary
variables cannot lower `D*`); the pair-table family law; the symmetry
inventory (every fold is a constant or polynomial in `n`); Bezout;
subgroup confinement; \"index calculus is priced by the ambient field,
rho by the subgroup\"; and the band-priced frontier (no IC arm beats rho
at any weight on any measured curve).

#divider()

== 3. Explored but not exhausted
<3-explored-but-not-exhausted>
Ranked by information per unit cost. Each row names the gap, the
evidence that it is a gap, and the cheapest next cell.

+ #strong[FGHR-symmetrised `m = 4` system (arm B)]
  - #emph[Done:] FGHR 2-torsion frame built and measured at `m = 2, 3`
    (`koblitz_symmetrised.rs`\; 30--416× on extension fields; `E/⟨T⟩`
    identity certified)
  - #emph[Gap:] `symmetrised_terms` returns `None` for `m ≥ 4`\; every
    `m = 4` degree is on the chained system with `2n` intermediate
    unknowns. The \"`m = 4` is one degree above `m = 3`\" reading may be
    the chain\'s, not the arity\'s
  - #emph[Next:] build `S₅` in `(w₁..w₄, w_R, s)`, exhaustive gate,
    measure closure degree at `ℓ = 2…5`, `n ≤ 19`
  - #emph[Cost:] 1--2 days code, hours of compute (XP card M1)
+ #strong[`m = 4` at surplus `S ≥ 0`, `ℓ ≥ 4` (arm A)]
  - #emph[Done:] all `m = 4`, `ℓ ≥ 4` cells at `S ≤ −3`
  - #emph[Gap:] the regime index calculus runs in is unmeasured at
    `m = 4`
  - #emph[Next:] `(16, 4)`, `(20, 5)` on the chained system, closure at
    degree 4
  - #emph[Cost:] runnable today; `N ≤ 60`, tens of GB
+ #strong[Guess count that holds degree 4 past `ℓ = 7`]
  - #emph[Done:] Result 14: one fixed variable restores degree 4 on 6/8
    draws at `m = 3, ℓ = 7`\; both branches of one guess refute on 5/8
  - #emph[Gap:] one cell, one `ℓ`\; the hybrid-closure cost curve
    `2^g · cols(4)^w` against `ℓ` is what prices the `ℓ ≥ 7` regime, and
    the whole `m = 4` price swings on it
  - #emph[Next:] `g` against `ℓ` at `m = 3, ℓ = 7, 8` and
    `m = 4, ℓ = 6, 7` with the dense closure
  - #emph[Cost:] 25--70 GB per cell
+ #strong[Fitted elimination exponent `w`]
  - #emph[Done:] `w` assumed at 1 and 2; measured ratios
    `log₂ ops / log₂ cols = 2.18–2.33` at one size
    (`dreg_closure_driver`); `K` measured by Callgrind
  - #emph[Gap:] the `m = 4` price is `+10` bits at `w = 1`, `+46` at
    `w = 2`, `+37…+49` in rho units across `w ∈ [2, 2.33]` (REVIEW F1)
  - #emph[Next:] record rank profile and density per degree block of the
    real matrices; try structured Gaussian / block Wiedemann for `w < 2`
    (REVIEW D1)
  - #emph[Cost:] days, no new memory
+ #strong[`C₃ ⊂ S₄` Faugère--Svartz blocks]
  - #emph[Done:] G2 (Frobenius) equivariant pass built, bit-identical,
    2--3× against `gf2_elim`\; useless in production because random `V`
    is not σ-stable
  - #emph[Gap:] the summand-permutation symmetry applies to every
    system; `S₄ ⊃ C₃` is odd-order, characters in `F₄`\; specified,
    unbuilt
  - #emph[Next:] after \#1: two blocks (trivial over `F₂`, conjugate
    pair over `F₄`), each ≈ ⅓ dimension
  - #emph[Cost:] 2--3 days (XP card M2)
+ #strong[G2 τ-twisted ordered chain]
  - #emph[Done:] survives at `(5,2), (7,3), (9,3)` with rise 0; yield
    rises for random bases
  - #emph[Gap:] `×m!` never read because fields saturate; G6 at `n = 31`
    never run
  - #emph[Next:] `n = 13…19` with the real F4, read yield and degree
    together
  - #emph[Cost:] hours
+ #strong[Cyclotomic base solver clause]
  - #emph[Done:] yield-neutral; build cost +2--12%
  - #emph[Gap:] the only value it could have (lower solving degree of
    the membership system) was never tested above `n = 11`
  - #emph[Next:] F4 on `S₃` + cyclotomic membership at `n = 23`,
    `w = 2, 3`, against ONB Hamming base
  - #emph[Cost:] hours (single-word plug-in limits `n`)
+ #strong[G4 Frobenius-stable quadratic-form sets]
  - #emph[Done:] proposed 2026-10-01
  - #emph[Gap:] never built; the only other Frobenius-stable family with
    an algebraic membership test at prime `n`
  - #emph[Next:] yield census via `ic search`\; F4 on `S₃` + `2k`
    quadratic constraints for `D`
  - #emph[Cost:] 1--2 days
+ #strong[Multiplier closure on the algebraic route]
  - #emph[Done:] `Z[τ]`, `Z[φ]`, integer multiples: 2--20× on the table
    pipeline; \"does not help the algebraic route\" asserted
  - #emph[Gap:] unmeasured. Norm-2 multiplier `τ̄` #emph[is] the
    2-torsion symmetrisation (`x ↦ x + 1/x`); higher norms with
    non-rational kernels add a degree-`N(α)` rational constraint per
    summand, not a symmetry
  - #emph[Next:] one toy cell: `m = 3`, `ℓ = 3`, base `V ∪ τ̄²(V)` with
    the degree-4 `x`-map constraint; compare degree and yield
  - #emph[Cost:] hours; expected negative
+ #strong[Closure degree at `m = 5, 6`]

- #emph[Done:] `S₆` (190,252 monomials) and `m = 5, ℓ = 4` descents
  exist in cryptanalysis (`pdp-scaling`, SAT and MITM only); the
  rotated-`m5/m6` admission note (2026-09-25) demands a canonical corpus
  and a `>64`-variable engine for `n = 131`-shaped systems
- #emph[Gap:] #strong[no algebraic degree at `m ≥ 5` in any
  repository]\; the literature has three toy sets at `ℓ ≤ 3` (Semaev,
  HPST15, Karabina; §5.1) and none at `ℓ ≥ 4`. The first cells need no
  engine work: the chained system at `(m, ℓ, n) = (5, 2, 11)`,
  `(5, 3, 13)`, `(6, 2, 11)` has `N = 43, 54, 56` unknowns, under the
  `u64` cap, and the same size as the `(15, 6)` cell that ran at 25.8 GB
- #emph[Next:] `dreg_closure` on those three cells, four unsatisfiable
  draws each (`chained_solution_count` already generalises in `m`)
- #emph[Cost:] hours to a day on a 32 GB host; the multiword engine (XP
  card M3) only for `ℓ ≥ 4`

#block[
#set enum(numbering: "1.", start: 11)
+ #strong[Last fall degree (Huang--Kosters--Yang--Yeo) and Crossbred on
  the degree cells]
]

- #emph[Done:] Crossbred tried only on the `m = 2` toy oracle at
  `n = 7, 9`\; LFD never instrumented
- #emph[Gap:] order-independent degree instruments (REVIEW D3)
- #emph[Next:] run on the committed 92 draws
- #emph[Cost:] hours

#block[
#set enum(numbering: "1.", start: 12)
+ #strong[Memory-light degree bound via GF(2) block Wiedemann on the
  Macaulay matrix]
]

- #emph[Done:] block Wiedemann exists over `Z/rZ` only
- #emph[Gap:] would give the plain-XL degree (2--3 degrees pessimistic)
  for cells past `(4, 6)` without renting memory (REVIEW D2)
- #emph[Next:] port `koblitz_sparse_la.rs` to GF(2) consistency test
- #emph[Cost:] days

#block[
#set enum(numbering: "1.", start: 13)
+ #strong[Stacked `D₄ + 2LP` at `k = 4`]
]

- #emph[Done:] `D₄` 18.2× constant; 2LP lowers the LA exponent 0.495 →
  0.420 but costs 2.31× whole-solve; sparse LA null blamed on the
  completion rule
- #emph[Gap:] the only unrun combination in the one regime with a
  measured sub-rho exponent
- #emph[Next:] ecbench end to end with `sparse-cado` and a full-rank
  stop rule (REVIEW M3)
- #emph[Cost:] one week

#block[
#set enum(numbering: "1.", start: 14)
+ #strong[PKM `m = 3` at `N = 21`]
]

- #emph[Done:] died at 99.6 GiB inside degree 8; `m = 4` plateau already
  broken
- #emph[Gap:] formally inconclusive; the review\'s cheap check: finish
  target 1 unspecialised to the end of degree 7 and count solutions (F3)
- #emph[Next:] one msolve run on a ≤ 128 GB host
- #emph[Cost:] hours

#block[
#set enum(numbering: "1.", start: 15)
+ #strong[Published-IC reference arm]
]

- #emph[Done:] every \"× faster\" is against rho or the program\'s own
  producer; the in-house F4 is `10³–10⁷` operations off a brute grid on
  PKM systems (`generator_effect_ic_20261019/E3.md`)
- #emph[Gap:] no FGHR+msolve+WDSat baseline at matched `(q, n, m)`
  exists in the repos
- #emph[Next:] protocol F2 of the review
- #emph[Cost:] 2--3 days to specify, a week to run

#block[
#set enum(numbering: "1.", start: 16)
+ #strong[Galbraith--Gebregiyorgis `m = 4` formulation (closed on the
  wrong experiment)]
]

- #emph[Done:] the encyclopedia\'s C8 verdict \"GG buys variables, not
  degree\" rests on GG14\'s #emph[special subspace] at `m = 3` in a
  symmetric-`S₄` encoding at `n = 9, 15`
  (`factor_base_dor_campaign_20261013` F6). The 4-torsion was pursued
  only as the `π − 1` transport on `K₀`
  (`RESEARCH_EXOTIC_COORDINATES.md` §10--11, negative: a quarter the
  yield at equal cost)
- #emph[Gap:] GG14\'s #strong[Experiment 2] is a different object:
  binary Edwards model, coordinate `t = x + y`, factor base `t ∈ V`,
  invariants under `(Z/4)^{m−1} ⋊ S_m` (`e₁, s_j = Π(t_i² + t_i)`),
  Magma F4, #strong[`m = 4, ℓ = 4` in 9--52 s at `n = 17…53`, degree 4],
  `ℓ ≤ 7` with SAT. The repository\'s best `m = 4, ℓ = 4` is msolve on
  the chain at `(11, 4)`: 386--805 s, and `(13, 4)` capped at 3 GB. The
  published formulation is one to two orders cheaper at the same
  `(m, ℓ)` and larger `n`, and ECC2K-130\'s `E(F₂) ≅ Z/4` is exactly the
  torsion it needs
- #emph[Next:] build GG14 Exp. 2 natively (binary Edwards form of `K₀`,
  `t`-subspace base, 4-torsion invariants), gate against enumeration,
  measure closure degree and cost beside the chain on the same `(n, ℓ)`
- #emph[Cost:] 2--3 days

#block[
#set enum(numbering: "1.", start: 17)
+ #strong[Symmetrised (`e_k`) presentation above the `ℓ = 6` crossover]
]

- #emph[Done:] `RESEARCH_DEGREE_REDUCTION.md` L2: symmetrisation is the
  wider presentation below `ℓ = 6`\; the instrument reached `ℓ ≤ 3`\;
  \"needs a solver this repo does not have\"; withdrawn
- #emph[Gap:] HPST 2013 measured exactly that crossover from the other
  side: at `n = 53` their symmetric-plus-nonsymmetric variable system (≈
  `(m²ℓ + mn)/2` variables, `D_reg` 3--4 against FPPR\'s 6--7) is
  #strong[12× faster at `ℓ = 5` and 85× at `ℓ = 6`] (6.68 s vs 80.8 s;
  34.8 s vs 2,967 s). GG14 Exp. 2 reaches `ℓ = 8` at `m = 3` in 52 s.
  cryptanalysis has the `e_k ∈ V^{(k)}` formulation at `m = 2, 3` only
- #emph[Next:] run the `e_k` formulation (and the HPST mixed one) at
  `m = 3`, `ℓ = 6…8`, `n = 29…53` with msolve, beside the chain
- #emph[Cost:] days

#block[
#set enum(numbering: "1.", start: 18)
+ #strong[Chain topology and auxiliary count]
]

- #emph[Done:] the binary builder is a linear chain (`m − 2` free
  intermediates); a `tree` mode exists only in the prime-field PKM code
  and was never run there either (\"`m = 5` and `tree` were not run\",
  `RESEARCH_PKM_TOWER_ORACLE.md`)
- #emph[Gap:] HPST 2015 report the balanced split
  `{S₃(x₁,x₂,w₁), S₃(w₁,X,w₂), S₃(w₂,x₃,x₄)}` \"better balanced\" than
  the linear chain at `m = 4`\; Karabina\'s one-auxiliary split (`S₄·S₄`
  at `m = 5`, `5ℓ + n` unknowns against the chain\'s `5ℓ + 3n`) trades
  unknowns for degree (`D = 7–8`). The axis \"number of auxiliary points
  `k ∈ {0 … m − 2}`\" has only its two endpoints measured anywhere
- #emph[Next:] add `tree` and `k`-auxiliary modes to
  `build_decomposition_system`\; measure closure degree against `k` at
  `m = 4, 5`, toy `n`
- #emph[Cost:] 1 day

#block[
#set enum(numbering: "1.", start: 19)
+ #strong[Symmetry-broken (disjoint) factor bases at large `m`]
]

- #emph[Done:] G2 (τ-twisted ordered chain) is the Frobenius special
  case; survives at `n ≤ 9`\; `×m!` never read
- #emph[Gap:] GG14 Exp. 3 (affine translates `V + v_i`): `D_reg` often 3
  instead of 4, gain `(m − 1)!`\; Gebregiyorgis thesis §3.7 (disjoint
  block subspaces): success probability `2^{mℓ}/2^n` with #strong[no
  `m!`]\; Diem 2013 (direct-sum decomposition, rigorous); APS 2018
  (partitioned base); PKM tower code has a `disjoint` symmetry mode. The
  `m!` in the program\'s targets-per-relation term is `2^9.5` at `m = 6`
  and `2^15.3` at `m = 8`, against `m×` more relations and `m²×` linear
  algebra (`2^5.2` at `m = 6`), so the lever grows with `m` while the
  linear algebra it costs shrinks in share
- #emph[Next:] add a `disjoint` mode to the binary builder; read yield
  and degree together at `m = 4, 5`
- #emph[Cost:] 1 day

#block[
#set enum(numbering: "1.", start: 20)
+ #strong[Δ-sweep at fixed `(m, ℓ)`]
]

- #emph[Done:] surplus control Result 6 at `m = 3`: degree rises at
  `S ≤ −4`, flat 6 (XL) / 4 (closure) at `S ∈ [−2, +1]`\; the `ℓ = 7`
  rise is at `S = −2, 0`
- #emph[Gap:] every published \"degree 4 flat to `ℓ = 8`\" cell is at
  `Δ = 13–76` (§5.2); no one has the same `(m, ℓ)` across `Δ`. If the
  closure degree is a function of `Δ`, the `ℓ`-laws are artefacts of the
  `Δ ≈ 0` sampling
- #emph[Next:] `m = 3, ℓ = 7` at `n = 21, 25, 29, 37` (Δ = 0 … 16) with
  the closure; same at `m = 4, ℓ = 5`
- #emph[Cost:] hours per cell at `N ≤ 64` (`n ≤ 29` at `m = 3`); the
  engine port for larger `n`

#block[
#set enum(numbering: "1.", start: 21)
+ #strong[Trace relation added explicitly at degree 1]
]

- #emph[Done:] the Kosters--Yeo fall is observed at degree 3 on every
  draw; the trace-zero base pre-filter never fires on odd-order targets
- #emph[Gap:] whether part of XL\'s growth is re-deriving a relation
  that is linear on solutions (lane §5.2); one flag in
  `build_decomposition_system`
- #emph[Next:] re-run the committed 92 draws with the trace row appended
- #emph[Cost:] hours

== 4. Not explored at all
<4-not-explored-at-all>
From the repository analysis (literature-derived additions follow in
§5):

- #strong[The `m ≥ 5` half of the arity curve for algebraic oracles], in
  any formulation (§1.3). The budget per target at `m = 5–8` is
  `2^14–2^27` operations, 8--22 bits more generous than at `m = 4`\; the
  repositories have nothing there, and the literature has three toy data
  sets at `ℓ ≤ 3` (§5.1), none at the `ℓ ≥ 4` where the trade-off lives.
- #strong[A formulation whose unknown count is independent of `n` at
  `m ≥ 4`.] The FGHR direct system has `N_B = m(ℓ − 1) + 1`\; the
  FPPR/WDSat symmetric system has `Σ_{k ≤ m} dim V^{(k)}` (≈ `m(m+1)ℓ/2`
  on a progression, also `n`-independent). Both avoid the `(m − 2)·n`
  intermediate coordinates that make every measured `m = 4` cell cost
  what it does. The symmetric formulation exists in cryptanalysis at
  `m = 2, 3` (`pdp-degree-heuristics` §6) and nowhere at `m ≥ 4`.
- #strong[The surplus axis at `m = 4`, `ℓ ≥ 3`, `S > 0`] (the
  Shantz--Teske / Joux--Vitse lever on binary curves at the arity that
  matters).
- #strong[A degree instrument that is not the dense closure] (block
  Wiedemann over GF(2); last fall degree), so that cells past `(4, 6)`
  can be bounded without a 256 GB host.
- #strong[The N1 group-algebra lift] (`F₂[Z/263]` redundant
  representation; exact-cancellation census at `n = 23`) and the
  #strong[G4 quadratic-form sets]: both have one-afternoon falsifiers
  written in `docs/ic/RESEARCH_IC_NOVEL_DIRECTIONS_20261007.md` and
  `RESEARCH_FACTOR_BASE_SHAPE_SEARCH.md` §7 and neither has been run.
- #strong[Position-dependent bases in any form] (affine cosets,
  coordinate blocks, Diem\'s mixed dimensions): the only shape lever the
  literature measured to #emph[lower] the degree (GG14, degree 3 against
  4 at large `n`), and the only one whose value grows with `m` (§3 row
  19, §5.3). The repository\'s G2 is its Frobenius special case and was
  never read for yield.
- #strong[Diem\'s exact-surplus device]: `δ = dm − n` summands from a
  subspace one dimension smaller, so that the condition count equals `n`
  exactly. The repository tunes surplus only in steps of `ℓ`\; at
  `m = 4, ℓ = 33`, `n = 131` the chain has `S = −1`, and Diem\'s device
  would make it 0 at no yield cost beyond the exact one bit.
- #strong[The Galbraith--Gebregiyorgis `m = 4` formulation itself] (§3
  row 16): the repository has the ingredients (2-torsion frame, `Z/4` on
  `K₀`) but never the binary-Edwards `t`-coordinate system with
  `(Z/4)^{m−1} ⋊ S_m` invariants, which is what made `m = 4, ℓ = 4` cost
  seconds in 2014.

Things that look unexplored but are closed by a theorem or an identity,
so that they are not re-proposed:

- Any cleverer #emph[set] of the same size for a table oracle (yield
  conservation; pair-table law; CGK; DKM non-adaptive bound).
- Any re-encoding of a fixed set\'s membership without auxiliary
  variables (sparsity invariance, Proposition 1 of
  `RESEARCH_INDEX_CALCULUS_FACTOR_BASE.md`).
- Moving along the isogeny class to lower the degree (`a₆` enters below
  the leading form at every `m`\; `d_reg` is constant on the class,
  proved).
- Subspace shape at prime `n` (trace class + Kneser).
- Gröbner membership over `F_p` for an explicit set `V` (CRT: it is
  enumeration in a changed basis). This covers the
  Amadori--Pintore--Sala random-point base too (§5.3).
- Two-level bases with intermediates restricted to a larger subspace
  `W`: the yield cost per intermediate is `2^{dim W − n}`, which the
  surplus identity makes exact; it cannot pay (#emph[derived here]).
- F4 trace reuse (Joux--Vitse F4Remake) for Boolean systems: success
  probability `≈ 0.29^{steps}` over `F₂` by the authors\' own analysis
  (§5.2).
- Frobenius-equivariant blocks on a single decomposition system: the
  ideal is not Frobenius-stable for a random target (§5.2).

#divider()

== 5. What the literature adds
<5-what-the-literature-adds>
Three lane reports sit beside this note: `lit_factor_bases.md`,
`lit_degree_reduction.md`, `lit_large_arity.md`. Each has a data table
with URLs. What they add to the repository\'s own record:

=== 5.1 Large arity and the `m`--`ℓ` trade-off (`lit_large_arity.md`)
<51-large-arity-and-the-mℓ-trade-off-lit_large_aritymd>
- #strong[Largest summation polynomial ever computed:] FHJRV (Eurocrypt
  2014) `P_{ϕ,8}` in characteristic 2 with one 2-torsion point, 470,369
  monomials, 40.5 CPU-hours by sparse interpolation of a resultant of
  `P₆` and `P₄`\; `P₇` has 2,247 terms (383 s), `P₆` has 50 (plain `S₆`:
  638 in the elementary symmetric variables; plain `S₇, S₈` never
  finished). The per-variable degree is still `2^{m−1}`\; the gain is
  sparsity and an `m`-fold smaller base. The only decomposition
  experiment with it is `m = 5` (`P_{ϕ,6}`) over `F_{(2^31)^5}` at
  `ℓ = 1`: 10.28 s per test. #strong[No `m ≥ 6` decomposition exists
  anywhere.] The \"`S₉`\" claim in one 2019 paper is unsourced.
- #strong[`m ≥ 5` on binary curves:] Semaev 2015 (`n ≤ 16`, F4 degree
  4), HPST 2015 (`n = 13–29`, `ℓ = 2, 3`, `D_reg` 4.33--5.00, 13--34 h,
  44--102 GB; \"`D_reg` slightly larger than 4 when `m = 5, 6`, so FFDA
  must probably be relaxed\"), Karabina 2015 (`m = 5`, `n ≤ 19`,
  `D = 7–8` with one auxiliary point). HPST15\'s rise from 4.0 at
  `ℓ = 2` to 4.3--5.0 at `ℓ = 3` is the only published hint of a `D(ℓ)`
  dependence at `m ≥ 5`\; the repository\'s closure instrument has never
  been pointed at it.
- #strong[Plain Weil descent of `S_{m+1}` without symmetrisation is
  hopeless in `m`:] Kousidis--Wiemers measure
  `D_ff = D_reg = 7 = m(m−1)+1` at `m = 3` and #strong[13] at `m = 4`.
  That is why every large-`m` formulation chains or symmetrises.
- #strong[Shantz--Teske 2013:] at `m = 2`, shrinking `ℓ` below `n/2`
  (\"Delta method\", `Δ ≈ 2⌊(n−15)/6⌋`) pays more than the `2^{m−1}`
  yield it costs; `m = 2` beats `m = 3` until `n ≈ 290` (linear algebra)
  and in relation gathering until `n > 600`\; the hybrid method gives
  less than Delta and does not stack with it. This is the repository\'s
  surplus identity (§1.4) measured from the other side.
- #strong[Generic `m`-sum:] only meet-in-the-middle applies
  (`T = N^{⌈m/2⌉}`, `M = N^{⌊m/2⌋}`); Wagner, Schroeppel--Shamir,
  Howgrave-Graham--Joux, dissection and the Nikolić--Sasaki/Dinur
  trade-offs all need a quotient or an order, which a prime-order curve
  group lacks. No generic-group lower bound for `k`-sum is published;
  the program\'s CGK and Dinur--Keller--Marmor arguments are the
  sharpest available. With MITM the product law becomes
  `m!·2^{n − (m−2−⌈m/2⌉)ℓ}`: flat for `m ≤ 5`, improving by `ℓ` bits per
  two units of `m` beyond that at memory `2^{⌊m/2⌋ℓ}`, still `2^{114}`
  at `m = 8, ℓ = 16` on ECC2K-130.
- #strong[\"Split completely\" transplants exist and are closed:]
  quasi-subfield polynomials (Huang--Kosters--Petit--Yeo--Yun 2020) are
  the elliptic-curve version of the function-field-sieve trick;
  Euler--Petit and McGuire--Sheekey rule out linearized families
  (`β ≥ 3/4` against `β ≤ 0.1` needed). No Bluher-polynomial transplant
  to point decomposition was found.
- #strong[Courtois 2016] claims a memory-free `2^{n/3}` oracle for
  `m = 2, ℓ = n/2` by guessing a small multiple `K·R` and a common
  divisor; unverified, and the repository measured only his linearised
  half (`2^{91}` at `n = 131`).

=== 5.2 Solving-degree reduction (`lit_degree_reduction.md`)
<52-solving-degree-reduction-lit_degree_reductionmd>
- #strong[Vocabulary mapping that reframes the repository\'s data.]
  Every experimental paper (Petit--Quisquater 2012, Shantz--Teske 2013,
  HPST 2013/2015, Galbraith--Gebregiyorgis 2014, Semaev 2015, Karabina
  2015, Kousidis--Wiemers 2019) reports Magma\'s F4 #emph[step degree],
  which re-multiplies degree falls: it is the repository\'s
  #strong[MutantXL closure degree], not its plain-XL degree, and it
  equals Caminata--Gorla\'s solving degree `sd` (JSC 2023, Def. 1.1).
  Proved: `sd = max{d_lf, max GB degree}`, so on a refuted instance with
  field equations #strong[the closure degree is exactly the last fall
  degree]\; `sd ≤ d_reg + 1` (Salizzoni 2023); plain XL `≤ 2·d_reg − 2`
  (Semaev--Tenti 2021). Consequences: (i) the repository\'s XL laws
  `⌈ℓ/2⌉ + 4` and `ℓ + 4` are #strong[new]: nobody has published a
  plain-XL refutation degree on these systems; (ii) its closure data (4
  flat to `m·ℓ ≤ 24`) agree with the literature\'s \"`D_reg = 4`\" at
  `m = 3…6`\; (iii) Kousidis--Wiemers\' `D_ff = 7` (`m = 3`) and `13`
  (`m = 4`) are for the plain Weil descent of `S_{m+1}`, whose
  generators have degree `m(m−1)+1`\; the repository\'s \"first fall
  degree 3 everywhere\" is for the chain, whose generators have degree
  \3. Not a conflict; worth writing up as such.
- #strong[Every published \"flat at 4\" sits at large
  overdetermination.] GG14: `n = 37–97`, `ℓ ≤ 8`, so
  `Δ = n − mℓ = 13–76`\; HPST13: `n ≤ 53`, `mℓ ≤ 18`\; Semaev\'s own
  remark: the degree exceeds 4 as soon as the system is
  under-determined. The repository\'s rise past 4 at `m = 3, ℓ = 7` is
  at `Δ = −2, 0`, a regime no paper sampled; its surplus control (Result
  6) ran `Δ ∈ [−5, +1]` only. #strong[A Δ-sweep at fixed `(m, ℓ)` is the
  cheapest discriminating experiment]: if the degree is a function of
  `Δ` rather than of `ℓ`, the `⌈ℓ/2⌉ + 4` law is an artefact of `Δ ≈ 0`,
  and the cost model\'s yield identity (one bit per surplus equation)
  prices the trade exactly. Gaggero--Gorla\'s threshold (`n > C(N, D)`
  equations makes degree `D` generic) and Semaev--Tenti\'s
  \"sufficiently overdetermined\" theorem give the curve to compare
  against.
- #strong[The only rigorous growth statement is linear in `ℓ` and
  `n`-free.] Derived by the lane (not in any paper; hypotheses to
  check): encode `x_i ∈ V` upstairs with the linearised polynomial `L_V`
  and apply Huang--Kosters--Yang--Yeo 2018 Thm 1.1 to
  `{S_{m+1}, L_V(x_1), …, L_V(x_m)}`\; the last fall degree of the
  descended system is at most about `2m²·ℓ + O(m)`, independent of `n`.
  Loose (≈ 90 against measured 4--7 at `m = 3, ℓ = 5`) but it fixes the
  #emph[shape]: flat in `n` is plausible and partly proved; #strong[flat
  in `ℓ` has no supporting theorem and contrary data at `m = 2`] (step
  degree 3 for `ℓ ≤ 8`, 4 for `9–18`, 5 for `19–24`, Kosters--Yeo and
  Kousidis--Wiemers, up to 208 GB). No lower bound exists for
  Weil-descent systems; the only one in the area is Yokoyama et al. 2020
  for the naive prime-field method.
- #strong[Closed by the literature, so not worth building:] F4Remake /
  trace reuse (G1 of the novel-directions note) has success probability
  `≈ 0.29^{steps}` over `F₂` (Joux--Vitse\'s own `c(q)` analysis),
  measured gains of 2--3.4× only over `F_p`\; Faugère--Svartz blocks
  cannot act on a single decomposition system because the Weil-descent
  ideal is Frobenius-stable only when `V` is stable #emph[and]
  `x_R ∈ F₂`, which fails for every random target (the `C₃ ⊂ S₄`
  summand-permutation block of §3 row 5 is unaffected).
- #strong[Constant-factor items not in the repository:] adding the
  Kosters--Yeo trace relation explicitly at degree 1 to the XL matrix
  (does part of the `⌈ℓ/2⌉ + 4` growth come from XL re-deriving a
  relation that is linear on solutions?); Macaulay-submatrix \"degree
  smoothing\" (Jaques--Ran--Samardjiska--Seitner, ePrint 2026/408), a
  fractional-degree XL worth a few bits where the `C(N, D)` jump between
  consecutive degrees is large; a guess count calibrated against the
  Gaggero--Gorla threshold `n > C(N − g, D)` instead of the degenerate
  hybrid sweep.
- #strong[Nothing from 2022--2026 attacks these systems directly.]
  Adjacent: Caminata--Gorla 2023, Salizzoni 2023, Gaggero--Gorla 2023/24
  (theory), Vidal--Ionica--Delaplace 2024 and ePrint 2026/360
  (Crossbred), ePrint 2024/1923 (a prime-field index-calculus
  implementation).

=== 5.3 Factor-base constructions (`lit_factor_bases.md`)
<53-factor-base-constructions-lit_factor_basesmd>
28 constructions tabulated with URLs; those not in the repository\'s
inventory, and what each is worth here:

- #strong[Position-dependent (disjoint) bases]: GG14 affine cosets
  `F_i = V + v_i` (measured, `m = 3`, `ℓ = 5–7`, `n = 37–97`, degree
  often 3 against 4); Gebregiyorgis coordinate blocks (proposed);
  Nagao\'s \"disjoint factor base\" (ePrint 2015/984, claims success
  probability ≈ 1 under the first-fall assumption); #strong[Diem II\'s
  mixed-dimension pair] `U ⊃ U′` with `dim U′ = dim U − 1` and exactly
  `δ = dm − n` summands from `U′`, which is the device behind his
  rigorous `e^{O(log(q^n)^{3/4})}` and is the only published
  construction that tunes the surplus to zero #emph[exactly] rather than
  in steps of `ℓ`. §3 row 19 and §1.4. Accounting note (#emph[derived
  here]): at equal column count a disjoint design has lower yield than
  one subspace of the same size by about `e^m/√(2πm)`\; its gain is at
  equal #emph[system size per summand], where it removes the `m!` for
  `m×` columns, so it favours large `m` where linear algebra is cheap.
- #strong[Amadori--Pintore--Sala \"dynamic\" base (prime fields)]: the
  base is the random points `R_i = u_iP + v_iQ` themselves, one relation
  from `S_m` with each `X_i` constrained by `Π_j(X_i − x(R_j))`, no
  linear algebra; the only prime-field construction with end-to-end DLP
  timings (22-bit `p`, `m = 3, 4, 5`: 268--388 s). #strong[Closed by the
  repository\'s own argument]: `F_p[X]/(Π(X − x_j))` is CRT-isomorphic
  to functions on the grid, so the Gröbner computation is enumeration of
  `s^m ≈ p` grid points in a changed basis (encyclopedia §7 item 2,
  `AR:H-ICEX-6ca21f`); Yokoyama et al. 2020 prove the matching lower
  bound for this \"naive\" structure. APS\'s crossover claims are
  against PKM, not against rho.
- #strong[Frobenius-stable bases on subfield curves]
  (Galbraith--Granger--Merz--Petit, SAC 2020): one decomposition system
  per Frobenius orbit, `n²` on linear algebra, Frobenius as a second
  symmetry-breaking source. This is the repository\'s
  signed-Frobenius-orbit and barrel formulation, re-derived without the
  paper (`AR:DEC-20260916-3c0cf5`); its full text, the most useful
  blocked document in the sweep, would show which of GGMP\'s several
  Frobenius-stable bases they found cheapest. #strong[Tsakou--Ionica
  2021] generalise to any endomorphism `φ` with `φ^k = ±1`: factor base
  `{P : Tr_φ(x(P)) ∈ F_q}` or the norm, imposed as `n − 1` linear
  conditions in a normal basis while keeping `S_{m+1}` in `x`.
  Extension-field and GLS/GLV curves only; theoretical for elliptic
  curves. The binary-prime-`n` analogue is the repository\'s
  `Z[τ]`-multiplier closure, which the lane confirms is Koblitz-only.
- #strong[Gorla--Massierer trace-zero hyperplane section] (`n = 3, 5`):
  a #emph[geometric] base (the `F_q`-points of
  `V_n ∩ {x_0 = … = x_{n−3} = 0}` in Weil-restriction coordinates) for
  the DLP in the trace-zero subgroup `T_n`\; a full 36-bit `T_3` DLP in
  957 s. Not on the list, but it attacks a different group (`T_n`, not
  `E(F_p)` or `E(F_{2^n})`), and the repository\'s A10 experiments show
  why trace-zero sets do not transfer to the prime-field target:
  relations land in the wrong subgroup.
- #strong[Kosters--Yeo\'s measured base preference]: at `m = 2` the
  \"`deg < k`\" window kept degree 4 at `n = 40` where random subspaces
  grew. The repository\'s shape search explains most such
  coordinate-versus-random gaps as the trace class (§2) and the residual
  as Kneser closure, so this is covered.
- #strong[Sieving]: no sieve exists for the summation-polynomial
  decomposition on an elliptic curve itself; every published sieve
  (Joux--Vitse 2012, Sarkar--Singh 2014/15, Vitse--Wallet 2015) acts on
  a Jacobian through a one-parameter family of functions and a
  univariate splitting test. The only route to a sieve is a cover to a
  Jacobian, which the repository has (E2, E6).
- #strong[Largest measured instances on record], for calibration of the
  reference arm (§3 row 15): WDSat `m = 3` at `(n, ℓ) = (89, 11)`:
  85,834 s satisfiable, 175,718 s refuted, under 20 MB; Gröbner on the
  same model stops at `ℓ = 7` (35 GB), `ℓ = 8` exceeds 200 GB; `m = 2`
  at `(41, 20)`: 4.2 s. Extension fields: FHJRV 10.28 s per
  decomposition on the `F_{2^155}` Oakley curve; Joux--Vitse 8.4 s on
  `F_{p^5}` at 32-bit `p`. Measured relation probability at `n = 24`
  (Trimoska Table 7.7) runs about 2× the `2^{mℓ}/(m!·2^n)` heuristic,
  which the repository\'s exact census did not reproduce
  (`pdp_followups_20261010` item 6).
- #strong[2022--2026]: no new elliptic-curve factor base. Only Hu 2024
  (a critique) and quantum-annealing reruns of Semaev\'s interval base
  at 8 and 16 bits.

#divider()

== 6. Recommended order
<6-recommended-order>
+ #strong[Δ-sweep] at `m = 3, ℓ = 7` over `n = 21…29` and `m = 4, ℓ = 5`
  (\#20), with the existing closure instrument. Hours. It decides
  whether the degree laws are laws in `ℓ` or artefacts of `Δ ≈ 0`, which
  changes how every other cell is read.
+ #strong[Closure degrees at `m = 5, 6`] on the chain at `N ≤ 56`
  (\#10), same instrument, same session. The first points anyone would
  have on the `m ≥ 5` half of the arity curve at the program\'s own
  degree measure; compare with HPST15\'s F4 step degrees 4.3--5.0.
+ #strong[Build GG14 Experiment 2] (\#16) and run it beside the chain at
  `m = 4, ℓ = 3, 4`. Two to three days. It is the published best `m = 4`
  formulation and the repository has no arm for it.
+ Build the FGHR `m = 4` system (\#1) and the `e_k` presentation above
  `ℓ = 6` (\#17); measure beside the chain on the same draws. These
  decide whether \"one degree above `m = 3`\" belongs to the arity or to
  the chain.
+ Run the `m = 4`, `Δ ≥ 0`, `ℓ ≥ 4` chained cells (\#2).
+ Measure `w` and the rank/density profile on the real degree-4 matrices
  (\#4); re-price the `m = 4` table in rho units with the fitted `w`.
+ The guess-count curve at `ℓ = 7, 8` (\#3), on a 25--70 GB host.
+ Chain topology (\#18) and disjoint bases (\#19) as one-day builder
  additions, read together with items 1--2.
+ The one-afternoon falsifiers that have sat unrun since 2026-10-01/07:
  G2 at `n = 13–19`, G4, N1, the cyclotomic solver clause, the explicit
  trace row (\#6--8, \#21).
+ The reference arm (\#15), because the program\'s stated goal since
  2026-10-22 is to beat published index calculus at matched `(q, n, m)`
  and no such baseline exists.

#divider()

== 7. What this note does not establish
<7-what-this-note-does-not-establish>
It does not show any factor base or presentation beating rho or
published index calculus. The §1.3 what-if is arithmetic on quoted
numbers under an assumption (bounded closure degree on a formulation
that does not yet exist) that the literature\'s own heuristics
contradict; it is there to show where the arity curve\'s leverage sits,
not to price an attack. Every `n = 131` figure is an extrapolation from
`n ≤ 19`.

#divider()

== 8. Files, build, and graphs checked
<8-files-build-and-graphs-checked>
- `README.md`: this note. `lit_factor_bases.md`,
  `lit_degree_reduction.md`, `lit_large_arity.md`: the three literature
  lane reports with their data tables and URLs (web-only; eprint PDFs
  were behind a bot wall and the entries marked abstract-level say so).
- `figures/plot.py` lays out the recorded and derived values of §1 into
  `figures/degree_vs_ell.svg` (repository instruments beside the
  published F4 step degrees) and `figures/arity_curve.svg` (the derived
  arity curve at `n = 131`), with 150 dpi PNG renders. It computes
  nothing new; every value is cited in the script header.
- `report.typ` and `report.pdf`: built by `sh build.sh` (Pandoc to
  Typst, then Typst), with the two figures embedded.
- #strong[Canonical graphs checked and left unchanged], because no
  operation count, verdict or coverage moved:
  `docs/index-calculus-scoreboard.html`,
  `docs/ic/progress-timeline.json`, `docs/ic/leaderboard.json`,
  `docs/ic/BOUNDARY_TARGETS.md`,
  `docs/ic/factor-base-encyclopedia/figures/`. The encyclopedia\'s C8
  entry and the degree-reduction note\'s L2 verdict are the two records
  this note says were closed on a different experiment than the one the
  literature measured (§3 rows 16--17); correcting them is a separate
  change once the proposed cells have run.
- Conductor could not be reached from this session (the control plane at
  `127.0.0.1:8491` refused the connection under the sandbox), so no task
  claim was recorded for this path.
- #strong[Lead atlas] (`crypto-autoresearcher/knowledge/leads/ecdlp/`,
  reviewed 2026-10-24, 147 leads; matched with
  `tools/lead_atlas.py --match` after the note was written). The rows of
  §3 map onto it as follows, so that they advance a lead\'s
  `next_action` rather than file duplicates. #emph[Advance:]
  `LEAD-ICE-descent-closure-degree-at-scale` (open, blocked on a ≥ 250
  GB host for the `(3, 19, 7)` degree-5 cell) gains the Δ-sweep (\#20)
  and the `m = 5, 6` cells (\#10), both inside the current engine and
  host class; `LEAD-ICE-chained-semaev-dreg-vs-semiregular` and
  `LEAD-ICE-linearization-excess-fb-prior` gain the vocabulary mapping
  of §5.2; `LEAD-ICE-hybrid-guess-determine-l7` is \#3;
  `LEAD-ICE-semaev-yield-model-coset-typing` (coset-typed bases, open)
  is \#19 with the published GG14 Exp. 3 and Diem II data attached;
  `LEAD-ICB-tau-horner-chain` is \#6. #emph[Reopen with a stated
  reason:] `LEAD-EXO-curve-model-representation` is closed on the
  finding that the model-native solving degree is invariant across
  Weierstrass, Edwards and other models at `m = 2`\; \#16 is not a model
  change but the tuple-level `(Z/4)^{m−1} ⋊ S_m` invariants GG14
  measured at `m = 4`, which that closure did not test.
  `LEAD-ICB-decomposition-encoding-and-sat` is closed on the measured
  per-attempt slope `2^{αℓ}` of SAT and enumeration encodings; \#17
  concerns the Gröbner cost of the symmetric presentation at fixed
  `ℓ = 6–8`, which HPST 2013 and GG14 measured and the closure\'s slope
  data do not cover. `LEAD-ICE-semaev-chain-cost-model` is closed under
  Semaev\'s cost model (`t* = m*` at `n = 250–600`); \#18 asks for the
  closure #emph[degree] against the auxiliary count, which that model
  assumes rather than measures. Related known results: `KR-IC-fbdb61`
  (last fall and solving degree), `KR-IC-955fd6` (symmetries and
  torsion), `KR-IC-8b9daa` (Karabina), `KR-IC-f5c584` (trace morphism).

= Figures

#figure(image("figures/degree_vs_ell.svg", width: 100%), caption: [Refutation degree against factor-base dimension. Left: the repository's three instruments on the chained S3 system (plain XL, MutantXL closure, msolve F4), n <= 15, four draws per cell; hollow labels mark lower bounds. Right: published Magma F4 maximum step degrees, the same quantity as the closure, all measured at overdetermination Delta = n - m ell of 13 to 76. Sources in figures/plot.py.])

#figure(image("figures/arity_curve.svg", width: 85%), caption: [Cost against arity at n = 131 (ECC2K-130). Free-oracle floor and per-target budget from RESEARCH_ECC2K130_DECOMPOSITION.md section 5.3; the what-if rows hold a degree-4 closure, w = 2.2 and an 8-bit column-touch to rho-step conversion, and are derived, not measured (section 1.3). The shaded region has no algebraic degree measurement in any repository.])
