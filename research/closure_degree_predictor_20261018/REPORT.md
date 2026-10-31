# T06: multigraded Hilbert-series predictor of the closure degree, against the 148 committed draws

**Result: negative.** The frozen predictor matches the observed closure degree on
8 of 148 draws (5.4%), the same 8 as the univariate semi-regular control, and
loses to the constant "closure degree is 4" baseline (124 of 148, 83.8%). Its
draw-level Spearman correlation with the observed degree is 0.369 (univariate
control 0.376; cell-mean 0.489 and 0.524), and it does **not** reproduce the
`ell = 7` rise as a step. Class: stage diagnostic (statistical comparison on
committed records). No `S`, no rho ratio, no speed claim; **no improvement over
the index-calculus baseline is claimed**, and the distance to rho is untouched.

- Code: `predictor.py` (rule), `compare.py` (comparison), `plot.py` (layout only).
- Frozen: `predictor.py` SHA-256 `010362995ab9b8205f0d6d2fecb6139cdb226698163b9533091c25feab441393`
  in `FROZEN.sha256`, written before `compare.py` existed; `compare.py` refuses to run on a changed predictor.
- Outputs: `table.md`, `table.csv`, `results.json`, `observed_vs_predicted.{svg,png}`.
- Rerun: `python3 compare.py && python3 plot.py` (pure Python 3; matplotlib only for the plot; SymPy is not installed and not used).
- Data: the 148 native (unsatisfiable, chained) draws of `research/dreg_closure_{mutantxl_20261016,scale_20261016,driver_20261016,specialise_20261017}/runs/*.jsonl`
  (specialised and satisfiable records excluded), 37 cells `(m, ell, n)`, selection identical to
  `ml-cryptanalysis/csd/closure_rows.py` (PR #30). Nothing is sampled; the permutation p-value uses seed 20261018.

## 1. Derivation

**System.** From `build_decomposition_system` (`src/cryptanalysis/koblitz_groebner.rs`): `m` summand blocks
`x_1..x_m` of `ell` bits and `m-2` intermediate blocks `e_1..e_{m-2}` of `n` bits,
$N = m\ell + (m-2)n$ Boolean unknowns, field equations $x^2=x$ imposed (Boolean quotient). Links
$S_3(x_1,x_2,e_1),\ S_3(e_i,x_{i+2},e_{i+1}),\ S_3(e_{m-2},x_m,x(R))$, each $n$ F2-equations, so $(m-1)n$ equations.
With $S_3(a,b,c)=(ab+ac+bc)^2+abc+b_0$ and squaring F2-linear, every link is multilinear in its blocks:
the $(m-2)n$ equations with three variable blocks have top form of multidegree $e_a+e_b+e_c$ (cubic), the $n$ closing
equations ($x(R)$ constant) have top form $e_{m-2}+e_m$ (bilinear). The grading group is $\mathbb Z^g$ with $g=2m-2$
blocks, not $\mathbb Z^m$: the intermediate blocks carry their own degree (the assessment's "$\mathbb Z^m$" is the
summand part only). For $m=2$ it is one bilinear link, $g=2$.

**Hilbert series.** Pass to the associated graded ring of the Boolean quotient, $\mathbb F_2[x]/(x_i^2)$, whose multigraded series is
$R(t)=\prod_b (1+t_b)^{s_b}$ with $s_b$ the block sizes. A generic (semi-regular, Boolean) system with top-form multidegrees
$\delta_j$ has
$$c(t)=\frac{\prod_b (1+t_b)^{s_b}}{\prod_j (1+t^{\delta_j})},\qquad c(d)=\sum_i \cdots\ \text{exact integers},$$
computed with the closed form $[u^i](1+u)^{-k}=(-1)^i\binom{k+i-1}{i}$ per distinct $\delta$ of multiplicity $k$.
Univariate projection ($t_b\mapsto z$) is $(1+z)^N/((1+z^2)^n(1+z^3)^{(m-2)n})$, whose first non-positive coefficient is
the semi-regular $D_{reg}$; this control reproduces all 13 committed values of `RESEARCH_ECC2K130_DESCENT_DEGREE.md` section 1 (`python3 predictor.py`).

**Rule (frozen).** Alive set $A=\{d: c(d')>0\ \forall d'\le d\}$ (all coefficients before a non-positive one, per multidegree
chain); covered set $U=\bigcup_j(\delta_j+\mathbb N^g)$ (multidegrees that receive generator rows);
$$D_{\text{multi}}=1+\max\{|d| : d\in A\cap U\},$$
the first total degree at which every Macaulay block that has rows has at least as many independent rows as monomials,
capped at 14 (reported as 15 = "at least 15"). Design note, fixed before any draw was compared: without $U$, $A$ is
unbounded (monomials in one block, such as $x_1^k$, are touched by no top form), so $U$ was added; the version without it
returns the cap for every cell and was discarded on that evidence alone. The univariate $D_{reg}$ and the constant 4 are the two reference arms.

**FSS bound.** Faugere-Safey El Din-Spaenlehauer: a generic zero-dimensional bilinear system with $n_x+n_y$ equations in
$n_x+n_y$ variables has $d_{reg}\le\min(n_x,n_y)+1$. `fss_bound` returns $\ell+1$ for $m=2$, $n\ge2\ell$ and `None`
otherwise. **It applies to 0 of the 37 cells** (all have $m\ge3$, so cubic links with $(1,1,1)$ top form are present); the
bilinear closing link alone is not a square system ($n$ equations, $n+\ell$ unknowns), so no bound is claimed for it.

**Hit rule.** Observed labels are 3, 4 or ">4" (the scan stops at degree 4). A draw is a hit if the prediction equals the exact
degree, or is $\ge5$ for ">4". Rank correlations use ">4" as 5 (a lower bound).

## 2. Table (observed counts against the three arms)

| cell (m,ell,n) | N | observed (count) | multi | semireg | FSS | hits multi/semireg/const4 |
|:--|--:|:--|--:|--:|:--|:--|
| (3,2,5) | 11 | 3x1, 4x3 | 5 | 5 | n/a | 0/0/3 of 4 |
| (3,2,7) | 13 | 3x1, 4x3 | 5 | 5 | n/a | 0/0/3 of 4 |
| (3,2,8) | 14 | 4x4 | 5 | 5 | n/a | 0/0/4 of 4 |
| (3,2,10) | 16 | 4x4 | 6 | 5 | n/a | 0/0/4 of 4 |
| (3,2,12) | 18 | 3x4 | 6 | 5 | n/a | 0/0/0 of 4 |
| (3,3,4) | 13 | 4x4 | 7 | 5 | n/a | 0/0/4 of 4 |
| (3,3,5) | 14 | 4x4 | 7 | 5 | n/a | 0/0/4 of 4 |
| (3,3,7) | 16 | 4x4 | 7 | 6 | n/a | 0/0/4 of 4 |
| (3,3,9) | 18 | 4x4 | 7 | 6 | n/a | 0/0/4 of 4 |
| (3,4,7) | 19 | 4x4 | 8 | 6 | n/a | 0/0/4 of 4 |
| (3,4,8) | 20 | 4x4 | 8 | 6 | n/a | 0/0/4 of 4 |
| (3,4,11) | 23 | 4x4 | 10 | 7 | n/a | 0/0/4 of 4 |
| (3,4,13) | 25 | 4x4 | 10 | 7 | n/a | 0/0/4 of 4 |
| (3,5,10) | 25 | 4x4 | 10 | 7 | n/a | 0/0/4 of 4 |
| (3,5,11) | 26 | 4x4 | 11 | 7 | n/a | 0/0/4 of 4 |
| (3,5,13) | 28 | 4x4 | 12 | 7 | n/a | 0/0/4 of 4 |
| (3,5,15) | 30 | 4x4 | 12 | 8 | n/a | 0/0/4 of 4 |
| (3,6,16) | 34 | 4x4 | 14 | 8 | n/a | 0/0/4 of 4 |
| (3,6,18) | 36 | 4x4 | >=15 | 9 | n/a | 0/0/4 of 4 |
| (3,6,22) | 40 | 4x4 | >=15 | 9 | n/a | 0/0/4 of 4 |
| (3,6,24) | 42 | 4x4 | >=15 | 9 | n/a | 0/0/4 of 4 |
| (3,7,19) | 40 | >4x4 | >=15 | 9 | n/a | 4/4/0 of 4 |
| (3,7,21) | 42 | >4x4 | >=15 | 9 | n/a | 4/4/0 of 4 |
| (4,1,4) | 12 | 3x1, 4x3 | 6 | 5 | n/a | 0/0/3 of 4 |
| (4,1,5) | 14 | 3x1, 4x3 | 6 | 5 | n/a | 0/0/3 of 4 |
| (4,1,6) | 16 | 3x2, 4x2 | 7 | 5 | n/a | 0/0/2 of 4 |
| (4,2,5) | 18 | 3x2, 4x2 | 10 | 6 | n/a | 0/0/2 of 4 |
| (4,2,6) | 20 | 3x2, 4x2 | 11 | 7 | n/a | 0/0/2 of 4 |
| (4,2,7) | 22 | 3x1, 4x3 | 12 | 7 | n/a | 0/0/3 of 4 |
| (4,2,8) | 24 | 3x1, 4x3 | 13 | 7 | n/a | 0/0/3 of 4 |
| (4,3,9) | 30 | 4x4 | >=15 | 8 | n/a | 0/0/4 of 4 |
| (4,3,10) | 32 | 4x4 | >=15 | 9 | n/a | 0/0/4 of 4 |
| (4,3,12) | 36 | 4x4 | >=15 | 9 | n/a | 0/0/4 of 4 |
| (4,3,14) | 40 | 4x4 | >=15 | 10 | n/a | 0/0/4 of 4 |
| (4,4,13) | 42 | 4x4 | >=15 | 10 | n/a | 0/0/4 of 4 |
| (4,5,11) | 42 | 4x4 | >=15 | 11 | n/a | 0/0/4 of 4 |
| (4,5,12) | 44 | 4x4 | >=15 | 11 | n/a | 0/0/4 of 4 |

## 3. Result

| arm | exact matches | rate | Spearman, draws | Spearman, cell means | cells whose modal draw matches |
|:--|--:|--:|--:|--:|--:|
| multigraded `D_multi` | 8 / 148 | 0.054 | 0.369 (perm. p 5e-5) | 0.489 | 2 / 37 |
| univariate semi-regular `D_reg` | 8 / 148 | 0.054 | 0.376 (perm. p 5e-5) | 0.524 | 2 / 37 |
| constant 4 | 124 / 148 | 0.838 | undefined | undefined | 31 / 37 |

- **The prediction is wrong in level.** Observed closure degree is 3 or 4 in 140 of 148 draws; `D_multi` is 5 to at least 15 in
  every cell, `D_reg` 5 to 11. These are XL-type regularities; the closure (MutantXL) sits at or below the first fall degree + 1
  and does not follow them. The 8 matches are the 8 draws at `ell = 7`, where ">4" needs only a prediction $\ge5$.
- **Rank correlation is positive but is not predictive.** It is carried by two facts, neither the mechanism: the 8 rising draws
  sit at the largest predictions, and the 16 draws with closure 3 sit at small cells. The permutation p-value treats draws as
  independent; they are not (all 37 cells are internally uniform at `ell >= 3`), so the effective unit is the cell, and the
  cell-level correlation has 37 points of which 2 carry the rise.
- **The `ell = 7` rise is not reproduced.** `D_multi` for `m = 3` rises smoothly with `ell` (5-6, 7, 8-10, 10-12, 14 to >=15, >=15 at `ell = 2,..,7`)
  and saturates the cap already at `ell = 6` (n >= 18); the minimum over `ell >= 7` cells (15) equals the maximum over `ell < 7` cells (15). The univariate control
  (5, 5-6, 6-7, 7-8, 8-9, 9 at `ell = 2,..,7`) climbs by about one per `ell` with no step at 7 (min over `ell >= 7`: 9, max below: 11). Neither separates the rising cells, so the prediction cannot
  distinguish the rules `ell >= 7` and `m*ell >= 21` either (they differ only at cells never run; see ml-cryptanalysis PR #30).
- The constant-4 baseline is the right reference for a model that explains only the rise: it is wrong on the 8 rising draws and on the 16 draws at 3.

## 4. What the predictor cannot see

1. **Specialisation of the instance.** The ideal is the generic one with the given multidegrees; the actual system has
   `x(R)` and the subspace basis plugged in, and the draws are unsatisfiable only by selection (yield falls with surplus).
   A specific system can have lower degree (the 16 draws at 3).
2. **Field equations.** $x^2=x$ is modelled only through the top-form ring $\mathbb F_2[x]/(x^2)$ (block sizes cap each
   exponent at 1). Their lower-degree part ($x$ vs $x^2$) is what lets a Boolean row space of low top degree contain 1 and is invisible to a graded count.
3. **The trilinear term and the lower strata.** Only the top form of each link enters. The pair-product part
   $(ab+ac+bc)^2$, whose multidegrees $e_a+e_b$ differ from the trilinear $e_a+e_b+e_c$, produces the degree falls at 3
   (first fall degree 3 throughout) that MutantXL multiplies up; a filtered-not-graded structure that top-form series do not capture.
4. **Mutants.** The closure is not "rows >= monomials": it multiplies row-space falls by monomials to saturation, and
   the committed data show it refutes at degree 4 in cells where XL needs 6 to 7. A count of independent Macaulay rows
   at one degree is the XL notion; a closure predictor needs the fall space, not the generator count.
5. **Observation cut at 4.** ">4" does not say 5 or more; 8 draws, 2 cells, 1 design point carry all of the signal.

## 5. Next step

Replace the generator-count model by a **fall-space model**: compute, over each multidegree block, the dimension of the degree-3
fall space (kernel of the cubic top forms against the pair-product strata) for a generic multilinear link at block sizes
`(ell, ell, n)`, then predict the closure degree as the least $D$ at which the fall-times-monomial products in the covered
blocks span a constant row. That predicts a block-size threshold directly (the assessment's mechanism) and needs a rank
computation per block, which is the same implicit-row / block-Wiedemann machinery as the engine this task was meant to
precede; cost scales with $\ell^2 n$ per block, small for the 37 cells. Test it with the same 148 draws against constant 4,
and at the cell `m = 4, ell = 6` (n = 10, N = 44), the only observed-design-gap cell that separates `ell >= 7` from `m*ell >= 21`.

## 6. Controlled-experiment record

- Claim class: comparative prediction; positive control for the machinery: the univariate arm reproduces the 13 committed `D_reg` values.
- Reference arms: constant 4 (null) and univariate semi-regular, same instruments, same draws.
- Freeze: rule and hash before comparison; one design iteration (adding $U$) was made on the predictor's own output, before comparison, and is disclosed above.
- Not blind: the labels (which cells rise) were read from the committed studies and from PR #30's inventory before the rule was written.
- Not done: no new draws; independent replay not run.

## 7. PDF

`typst` is not installed, and `pandoc` 3.1.3 is installed but has no PDF engine (`pdflatex`, `xelatex`, `lualatex`, `wkhtmltopdf`, `weasyprint`, `tectonic` are all absent), so no PDF of this report was produced. SVG and PNG graphs are present.
