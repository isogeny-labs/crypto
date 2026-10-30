# IC lead 5: is there a non-enumerative membership oracle for prime fields? (D4, EXP-ICEX-153c34)

2026-10-30 · desk study, no new measurement · session started from `local_6a4e8577-fbce-4ee7-8353-7d5c574e7e82`
· script `breakeven_beta.py` (closed-form arithmetic, output `breakeven_beta.out`) · nothing pushed

Question (REVIEW.md §5.4 D4, `/Volumes/SSD990-2/ic-regime-review-20261024/REVIEW.md`): the generic-oracle
theorem covers only enumerative oracles; is a non-enumerative oracle for `E(F_p)` a real route, and is
EXP-ICEX-153c34 the experiment that tests it?

## 0. Verdict

**UNDECIDED on current evidence, with the prior on "no gain". No construction that beats enumeration
was found, and none is proved to be enumeration in general.** Three separate statements, each at its own evidence level:

1. **A non-enumerative oracle for prime fields exists as a construction and was already built and
   measured in this repo.** It is the Petit–Kosters–Messeng (PKM) chain presentation, called the "PKM
   tower" here. It escapes the generic-group hypotheses by construction, so the theorem does not
   apply to it. REVIEW.md §5.4 D4 ("nothing in the repo has tried it") is incomplete on this point. §3.2
   shows that "escapes the theorem" is trivially true of any algebraic oracle and is not progress.
2. **EXP-ICEX-153c34 targets the wrong object.** Its backend G1 is a Gröbner computation on the
   vanishing-polynomial ideal `(S, g_V(x_1), …, g_V(x_m))`, `deg g_V = |V|`. For that presentation
   the proved facts are narrow (Lemma A, §4.1): any degree-bounded solver needs working degree
   `≥ |V|`, and XL then has `≥ C(|V|+m, m)` columns. For lex Buchberger, which is what G1 is, nothing
   is proved. The design report assumes a grid floor from its CRT collision (both solvers decide the
   same predicate, which does not make them cost the same) and computes it as 55–229×
   (symmetrised) to 1,041–8,438× (unsymmetrised) the competitiveness threshold `T` on all six v4
   fixtures. G1 has no chain decomposition, so it cannot test the lead, and building it would at best
   re-measure that assumed floor.
3. **For the PKM chain the question is the solving-degree growth of F4-type solvers, and it is open.**
   - XL/Macaulay on the chain is enumeration-sized: columns `2^N = |V|^m`, with `D_XL ≥ d + δ`
     (Prop 2) and equality verified at `m = 2`, `N = 12–26` (repo `ic_next10_20261014/03-pkm-degree`).
   - F4 escapes XL through degree falls. At `m = 4` the break-even needs `D/N ≲ 0.04` (`w = 1`) or
     `≲ 0.017` (`w = 2`) (§5.2, derived). The recorded rows are `D = 6, 7, 7, ≥ 8` at `N = 8, 12, 16, 20`,
     a local slope of at least `1/8`.
   - The cost metric that decides it is `log2(widest matrix)/N`. It is 0.86 at the `m = 4`, `N = 16`
     cell (symmetric arm, in-house `f4_fp`), against an asymptotic break-even of 0.25 (`w = 1`). The cells that would decide the asymptotics
     (`m = 3, N ≥ 21`; `m = 4, N ≥ 24`) are not reachable on this host.

Task category: **undecidable on current evidence** (i.e. undecided, not proved either way). No
prototype was built (§6), which also names the cells that would decide it.

## 1. What the theorem says, and what "enumerative" means

**Source statements (program notes).**
- `RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md` §0–§1 (2026-10-08): every method that treats `E(F_p)`
  as a black-box group plus hashing of coordinates, "rho, kangaroo, BSGS, *and* every index-calculus
  variant whose decomposition step enumerates or table-looks-up factor-base elements", obeys
  `S·T² ≳ n` (Corrigan-Gibbs–Kogan) and `T ≳ √n` (Shoup). Its table lists IC-2, IC-3 and IC-`m` with an
  `(m−1)`-sum table on or above the line.
- `RESEARCH_IC_LEADS_AND_CONNECTIONS_20261010.md` Q1: "If it answers 'is R an m-sum of base points' by
  enumeration, pair tables, direct subtraction, or any membership test on group elements, then the whole
  algorithm is a generic-group algorithm with advice."
- `reports/Index calculus leads next steps.md`: the bound covers oracles "built from group operations and
  membership tests" and "explicitly does not cover an S₄ solve with an algebraic restriction on the
  unknowns".

**Primary sources.** Corrigan-Gibbs and Kogan, *The Discrete-Logarithm Problem with Preprocessing*,
EUROCRYPT 2018 (ePrint 2017/1113): for generic-group algorithms with `S` bits of advice, `T` online
queries and success probability `ε` in a prime-order group of size `N`, `S·T² = Ω̃(εN)`, tight up to
logs. (Statement taken from the ePrint abstract via search; the full proof was not read here.) Shoup,
*Lower bounds for discrete logarithms and related problems*, EUROCRYPT 1997: `Ω(√N)` generic queries.

**The three predicates, which the REVIEW's wording blurs.**

| predicate | meaning | cost for the interval base `{x ≤ B}` |
|---|---|---|
| (a) point-in-F | is `x(P) ∈ V`? | O(1), trivially non-enumerative; not the bottleneck |
| (b) sumset membership | is `R ∈ F+…+F` (`m` terms)? | the decomposition oracle: the quantity `T` in the cost model |
| (c) decomposition search | produce `P_1..P_m` with `ΣP_i = R` | same cost as (b) when solutions are rare |

"Membership of an x-coordinate in the factor base without enumerating the base" is (a) and is free for
every base used in the program. What the cost model needs is (b): does the oracle answer it in time
`≪` the number of candidate tuples.

**Definition used below (generic-group-model terms).**
- *Enumerative oracle:* it learns about the group law only through group-operation queries, and any
  computation on labels (including "is this label in F") is free. Every candidate tuple it rules out
  costs at least one group query, so CGK and Shoup apply.
- *Non-enumerative oracle:* it decides (b) by algebraic relations among coordinates, namely `S_{m+1}`
  plus membership equations, examining sub-linearly many candidate tuples. Summation-polynomial
  methods do this.

**Consequence.** Non-enumerative oracles sit outside the theorem for the trivial reason that they are
not generic-group algorithms. No lower bound against them exists in the sources read here. PKM say
(paraphrased) that proving complexity bounds for their algorithms looks very challenging
(PKC 2016, §1, "Perspectives"). Diem (Compositio Math. 147 (2011) 75–104) proves subexponential algorithms for
extension fields of suitable shape; its introduction, as retrieved, says no efficient algorithm is
known for prime fields except for very special equations (I read only the search excerpt).

## 2. What a non-enumerative oracle must achieve (cost target)

**Model** (PKM §3.2; repo `RESEARCH_PKM_TOWER_ORACLE.md` §3.7; REVIEW AUD-4):
`cost ≈ m!·p·T/B^{m−1} + m·B²`, with `T` the per-target cost of (b), `B = |F|` the base size, the sparse
solve at `Θ(m·B²)` and relation yield `≈ B^m/(m!·p)` per random target.

**Yield is fixed; per-query cost is the only lever.**
- The 153c34 design report: attempts per success are backend-independent, because every exact
  backend decides the same predicate.
- `ic_next10_20261014/02-solution-count`: over 172 scored cells, PKM-Kummer, subgroup, Dickson,
  Dickson-torus and small-x bases have solution counts 0.93–1.03× a random base of the same size
  (positive controls read as predicted). Algebraic structure does not change yield.

**Break-even.** Write `T = p^γ`. The optimum is `B = p^{(1+γ)/(m+1)}`, the total is `p^{2(1+γ)/(m+1)}`, and
it beats rho (`p^{1/2}`) iff

    γ < (m − 3)/4,   and at that point B = p^{1/4} exactly.

| arity `m` | `T` must be below | relation to `B = p^{1/4}` | verdict |
|---|---|---|---|
| 2 | never | | no oracle helps (`≥ 3(pT)^{2/3}`) |
| 3 | `O(1)` (ties only) | | no oracle beats rho |
| 4 | `p^{1/4}` | `T < B`: sub-linear in the base | first arity that can matter |
| 5 | `p^{1/2}` | `T < B²` | |
| 6 | `p^{3/4}` | `T < B³` | |

PKM's own statement of the same threshold, for `B = deg L ≈ p^{1/m}`: the algorithm wins when
`T < p^{1/2−1/m}` (PKC 2016 §3.2; it assumes `ω` for the solve, and at `m = 4` it coincides with the
row above). **All precomputation must be charged.** PKM's auxiliary-curve search costs about `p^{1/m}`
trials (their Table 1), which is below the break-even at `m ≥ 4` but not negligible. The 153c34 threshold
(per success) is `T = 13·0.886·√q/(L+27)` C-3 units.

## 3. Candidates

### 3.1 Census

| candidate | escapes the theorem? | per-query cost | status |
|---|---|---|---|
| **G1** (153c34): lex Buchberger on `(S, g_V(x_i))` | yes (algebraic) | XL: `≥ C(B+m, m)` columns; any degree-bounded solver: degree `≥ B` (Lemma A); Buchberger: nothing proved | design report assumes a grid floor (55–8,438× `T`); not decision-relevant (§4.1) |
| **PKM chain, XL** | yes | columns `2^N = B^m`; Prop 2 proves a lower bound, equality verified at `m = 2`, `N = 12–26` | enumeration-sized |
| multiplicative-subgroup base (`x = y^k` with `k = (p−1)/B` gives `μ_B`) | yes | this is PKM instance 1 with one level | folded into the PKM rows; yield covered by the census |
| **PKM chain, F4** | yes | width `2^{H(β)N}` if `D = βN` | **open**; measured `D` and widths in §5 |
| Coppersmith small roots (interval base) | yes | polynomial time only for `B ≤ p^{δ_max}`, `δ_max = 1/(m(m+1)2^{m−2})` | `m = 4`: `1/80` against `1/5` needed; lattice-based, derived in program note §2.2 (Jochemsz–May shape, not re-derived here; Probes B, C measured at `m = 2`) |
| spectral / sparse-Fourier indicator | yes | none known | flat-spectrum falsifier in `Index calculus leads next steps.md` item "Mechanism"; not run here |
| k-sum / Wagner-type | no structure to use | | needs partial-sum constraints compatible with the group law; none exists for `E(F_p)` (argument, not a theorem) |

### 3.2 Why "escapes the theorem" is not the question

PKM is outside CGK/Shoup because it is algebraic. That fact does not constrain its cost in either
direction. The real question, which the repo's own `RESEARCH_PKM_TOWER_ORACLE.md` §3 asks, is whether
the algebra is cheaper than the grid it describes.

The program note `RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md` §2.3 says "any subset `F ⊂ F_p` of size
`B` has membership polynomial of degree exactly `B`, with no restriction-of-scalars trick". That is
true of a single univariate polynomial, and the 153c34 spec inherits it. PKM evades it by adding one
auxiliary variable per level of a chain of low-degree maps, so every equation has low degree
(PKC 2016 Alg. 1, system (4); Amadori–Pintore–Sala, system (6)). §2.3 therefore does not close the PKM
presentation. `ic_next10` item 2 tested exactly that (yield) and found it neutral; item 3 tests the
solver axis.

## 4. The two presentations

### 4.1 Vanishing-polynomial presentation (153c34's G1): degree floor proved, cost floor only for XL

**Lemma A.** Let `V ⊂ F_p`, `|V| = B`, `g_V = Π_{v∈V}(X − v)`, `S ∈ F_p[x_1..x_m]` non-constant, and
`I = (S, g_V(x_1), …, g_V(x_m))`. Then every certificate `1 = h·S + Σ h_i·g_V(x_i)` has
`max_i deg(h_i·g_V(x_i)) ≥ B`, and the same holds for every certificate of a nonzero linear form in
the solution coordinates. Consequently any Macaulay-type solver (XL, F4, Mutant, closure) whose working
degree is `D < B` sees only `(S)` and decides nothing. For **XL** the matrix at `D = B` has at least
`C(B+m, m) ≥ B^m/m!` columns. That column count does not transfer to F4, whose symbolic preprocessing
builds only the columns its selected pairs touch.

*Proof.* If every `h_i·g_V(x_i)` had degree `< B`, then, as `deg g_V = B`, every `h_i` would be 0, and
`1 = h·S` is impossible for non-constant `S`. The same argument applies to a linear form `ℓ` unless
`S | ℓ`, which fails for `deg S ≥ 2`. The column count is the number of monomials of degree `≤ B` in
`m` variables. ∎

**Scope, by solver class.**
- *XL:* hits the grid-sized floor (`≥ C(B+m, m)` columns).
- *Any degree-bounded solver (XL, F4, Mutant):* needs working degree `≥ B`. This alone gives no
  cost floor for F4.
- *Buchberger (what G1 is, lex):* the lemma shows only that some degree-`≥ B` S-pair is formed. No
  cost floor is proved.
- *The design report's CRT collision:* `F_p[x]/(g_V(x_i)) ≅ F_p^{V^m}`, so every exact solver on
  this ideal decides the same grid predicate (`backend_choice.rationale`; D-3 forbids the CRT
  representation for that reason). That says the predicates coincide, not that the costs do.
- *The 55–8,438× figures* (1,041–8,438× unsymmetrised, 55–229× symmetrised, six v4 fixtures) are the
  design report's computed grid floor, assumed to bind G1 (`design_report.yaml`,
  `ceiling_audit_output.json`; I read the report, not the JSON). The coordinator prior recorded
  before any result is that the obstruction persists under G1.
- A symmetric-ring presentation lowers the XL column count by at most a further `m!`, which does not
  change the exponent.

**Reading.** G1 is not a candidate construction; it is a control whose expected cost is the grid.
It lacks the chain decomposition that makes PKM's membership equations low degree, so it cannot
test the lead.

### 4.2 PKM chain presentation

Construction (PKC 2016 §3.1, §3.3, §3.4, read directly in the PDF).
- Factor base `F = {(x,y) ∈ E(F_p) : L(x) = 0}`, `L = ∘_{j≤n'} L_j` a composition of low-degree maps
  whose roots lie in `F_p`; `|F| ≈ deg L ≈ p^{1/m}`.
- Relation: solve `S_{m+1}(x_{11},…,x_{m1}, X) = 0`, `x_{i,j+1} = L_j(x_{i,j})`, `L_{n'}(x_{i,n'}) = 0`.
- Instance 1 (`p−1` has a smooth factor `∏p_i ≈ p^{1/m}`): `V` a coset of the subgroup of that order,
  `L_j(x) = x^{p_j}`. The authors note NIST P-224 (`p−1` divisible by `2^96`) already meets the
  prerequisite for `m ≥ 3`, `B = 2`.
- Instance 2 (arbitrary `p`): an auxiliary curve `E'/F_p` with a smooth-order subgroup `H`, and `L`
  built from the isogeny with kernel `H` (Vélu).
- Their own conclusions (paraphrased, PKC 2016 §1): the complexity of the algorithms is an open
  problem; the experiments suggest the systems are easier than generic systems of comparable
  parameters, but the authors call the experiment set too limited to conclude anything; and at the
  time of writing every such attack is outperformed by generic algorithms at practically relevant
  parameters.
- Amadori, Pintore and Sala, *On the discrete logarithm problem for prime-field elliptic curves*,
  Finite Fields Appl. 51 (2018) 168–182 (ePrint 2017/609): a variation of Semaev's approach for
  prime fields reducing the Gröbner work to a single basis, claimed to outperform Semaev's original and PKM.
  I could cite only the ePrint abstract page; the PDF returned HTTP 403. They give no comparison with
  generic algorithms on that page.

The repo built this as the "PKM tower": `RESEARCH_PKM_TOWER_ORACLE.md` (design, pre-registered H0/H1),
`ic_next10_20261014/{02,03}` (measured), `pkm_symmetric_lead1_20261026` (symmetric m = 3 and m = 4
cells), and `prime_ecdlp_three_leads_20261029/execution-20261030` on `codex/pkm-oracle-g1-g2-20261030`
(today; its "G1–G6" are that program's PKM goals, not 153c34's backend G1; native correctness and accounting software, "no measured panel, cost improvement or
asymptotic result is supplied").

## 5. Evidence on the PKM chain

### 5.1 Recorded rows (all quoted, none new)

| source | system | rows |
|---|---|---|
| `ic_next10_20261014/03-pkm-degree` (msolve 0.6.5 and `f4_fp_tower`) | `m = 2`, full presentation | XL: `δ = N` exactly for `N = 12–26`, `D_XL = N + 4`, F4 `D = 5` |
| same | `m = 3`, plain chain | `D = 6, 6, 7, 7` over `N = 9–18`; XL `D_XL = 13` at `N = 9`, `≥ 14` at `N = 12` (F4 6) |
| same | `m = 4`, plain chain | `D = 6, 7, 7, ≥ 8` at `N = 8, 12, 16, 20` (the `≥ 8` is msolve at a 5 GB cap) |
| `pkm_symmetric_lead1_20261026` (`f4_fp`, `p = 786433`) | `m = 3` symmetric | `D = 4, 5, 5, 6, 6` at `N = 6, 9, 12, 15, 18`; widest matrix 35, 248, 1,224, 10,532, 33,153 |
| same | `m = 4` symmetric (`S₅` in the `e_k`) | `N = 12`: `D = 8`, width 3,663, 4.0·10⁵ field ops; `N = 16`: `D = 8`, width 13,926, 1.08·10¹⁰ ops, 11.8 s; `N = 20`: aborted at 6 GB. At `N = 12` the plain chain: `D = 7`, width 21,619, 1.10·10¹¹ ops |
| `pkm_m3_n21_msolve_20261018` (main) | `m = 3`, `N = 21` | stopped at 99.6 GiB inside degree 8 (quoted from REVIEW §1 item 3) |
| `research/pkm-m3-specialise-20261024` (`cae06169c9`) | `m = 3` specialised | null per REVIEW §4b: no `H0` lower-bound evidence, unit-ideal status unestablished |

The symmetric `m = 4` `D = 8` is the degree of `S₅(e)` itself, so `D` cannot rank arms there (the
lead-1 README says so). Do not read it as a plateau.

### 5.2 Required slope (derived; interpretation, not measurement)

`breakeven_beta.py` prints the largest `β` with `D ≈ βN` that still breaks even, under the model: width
`Σ_{k≤βN} C(N,k) = 2^{H(β)N}`, per-query cost `width^w`, `N = m·log2 B = (m/4)·log2 p` at the break-even
`B = p^{1/4}`. It ignores polynomial prefactors, so it is an exponent-level necessary condition.

    H(β*) = (m − 3)/(m·w)

| `m` | `w = 1` | `w = 2` | `w = 2.25` |
|---|---|---|---|
| 3 | none (needs `γ < 0`) | none | none |
| 4 | **0.0417** | 0.0171 | 0.0148 |
| 5 | 0.0794 | 0.0311 | 0.0267 |
| 6 | 0.1100 | 0.0417 | 0.0357 |

`w = 1` is "one touch per column"; REVIEW §3 item 1 argues the realistic `w` is 2–2.33.

Two readings, each an interpretation:
- **H1's pre-registered threshold is too loose to matter at `m = 4`.** `RESEARCH_PKM_TOWER_ORACLE.md`
  §5.2 sends a cell to Stage B only if the upper 95% bound on `β` is below 0.10 (and `D_F4` is below
  the null). A `β` of 0.10 gives width `2^{0.47N}`, above the `m = 4` break-even width `2^{0.25N}`
  (`w = 1`). H1 would not by itself imply a win at `m = 4`.
- **The recorded slope is above the requirement.** The plain-chain rows give a local slope of at least
  `1/8` over `N = 12–20`, against 0.042 (`w = 1`) or 0.017 (`w = 2`). Four points with one censored
  bound cannot exclude a plateau, and the `m = 3` rows are consistent with a staircase.

### 5.3 Counted cost against the break-even, same units (derived from the rows above)

| cell | `log2(widest)/N` | break-even `(m−3)/(m·w)` (`w = 1`) | field ops | `p^{1/4}` | grid size `2^N` |
|---|---|---|---|---|---|
| `m = 4`, `N = 12`, symmetric | 0.99 | 0.25 | 4.0·10⁵ | 29.7 | 4,096 |
| `m = 4`, `N = 16`, symmetric | 0.86 | 0.25 | 1.08·10¹⁰ | 29.7 | 65,536 |
| `m = 3`, `N = 18`, symmetric | 0.83 | none | not recorded in the README table | | 262,144 |

`p = 786433`, so `p^{1/4} ≈ 29.7`. At `m = 4`, `N = 16` the oracle's counted cost is about `1.6·10⁵×`
the cost of enumerating the grid. **Caveat:** the field-op counts and widths come from the in-house
`f4_fp`. REVIEW §3 item 4b states that cost measured with that engine says nothing about published
Gröbner solvers (degrees from msolve are unaffected), and widths also depend on F4's pair-selection
strategy. So these rows bound the repo's engine, not the published state of the art. The REVIEW (§4,
quoting `generator_effect_ic_20261019/E3.md`) reports the same pattern against the same engine: it
needs `10³` to `10⁷` more operations than a brute grid on the same PKM systems.

I do not compare the width with a break-even width at `N = 16`: with `p = 786433` this cell has
`B = 2^4 = 16 ≈ p^{0.2}`, not the break-even `B = p^{1/4}`, so `2^{0.25N}` is the wrong yardstick at this
size. The direct comparison is the counted cost against `p^{1/4} ≈ 29.7`, and at these sizes constants
dominate, so the table shows the distance at the measured sizes and is not an extrapolation.

The width exponent, not `D`, is the right metric to carry to larger cells. It is also what `H(β)` converts to. The symmetric
arm's widths are lower than a plain arm's by up to `m!`; the table uses recorded widths and does not
credit that factor back in either direction.

## 6. Why no prototype

- G1 would at best re-measure the grid floor the design report assumes (55–8,438× the threshold); Lemma A
  proves only the degree floor for degree-bounded solvers and the column floor for XL.
- The PKM chain is already built and measured with msolve and `f4_fp` (§5.1); a new toy-field cell would
  duplicate it.
- The decisive cells (`m = 3`, `N ≥ 21`; `m = 4`, `N ≥ 24`) need 256–512 GB (REVIEW L2) and cannot run on
  this host (35 GiB free disk, load high). The task's Step 3 condition ("only if Step 2 gives a sound
  construction") is not met.

**What would decide the verdict.**
1. `m = 4`, symmetric, `N = 20, 24, 28`: record `log2(width)/N` and field-op counts. The break-even
   on that quantity is `(m−3)/(m·w)`, i.e. 0.25 at `w = 1` and 0.125 at `w = 2`.
2. `m = 3`, `N = 21`: `D = 6` or 7 separates "bounded" from "staircase" (lead-1 README). It cannot
   change the verdict by itself, since `m = 3` cannot beat rho at any `D` (§2).
3. A last-fall-degree bound (Huang–Kosters–Yeo) on the chain presentation, or a proof that `δ_Z(ℓ/S̄)`
   structure forces `D ≥ cN` for F4 as well. This is the only route to a negative theorem;
   `RESEARCH_PKM_TOWER_ORACLE.md` §3.5 names it as the open question.

## 7. Recommendations

- **Do not build 153c34's G1.** Mark the experiment as "predicted at floor, retarget" and, if the
  autoresearcher wants an algebraic arm, bind it to the PKM chain presentation, with the metric
  `log2(width)/N` and a pre-registered break-even of `(m−3)/(m·w)` rather than a ratio exponent.
- **Correct REVIEW §5.4 D4.** The oracle was tried (PKM tower, Stage A, symmetric lead 1) and the result
  is "open, prior H0", not "untried".
- **Tighten PKM H1.** Replace the 0.10 upper-bound rule by the `m`-dependent `β*` of §5.2, or state that
  H1 at `m = 4` is necessary but not sufficient.

## 8. Provenance and limits

Read directly: REVIEW.md; 153c34 `specification.yaml` (first 120 lines) and `design_report.yaml`;
`RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md` (commit `44cc0bc359`, §0–§2.4); `RESEARCH_PKM_TOWER_ORACLE.md`
(`origin/main`, §0, §3); `ic_next10_20261014/{02,03}` READMEs; `pkm_symmetric_lead1_20261026/README.md`
(`origin/claude/pkm-symmetric-lead1`, `ea987db0cb`); `codex/pkm-oracle-g1-g2-20261030` README; PKM PKC 2016
PDF pages 1–10.

Not verified: Corrigan-Gibbs–Kogan and Shoup proofs (abstract-level only); Diem's text (search excerpt);
Amadori–Pintore–Sala full text (HTTP 403); the Jochemsz–May `δ_max` derivation (taken from the program
note, whose Probes B and C measured `m = 2`); `ceiling_audit_output.json` (numbers taken from the design
report); the `E3.md` F4-vs-grid factor (quoted from the REVIEW); PKM PDF pages 11 onward. The host load
was high, so no wall-clock number in this note is used as evidence.

## Sources

- Petit, Kosters, Messeng, *Algebraic approaches for the Elliptic Curve Discrete Logarithm Problem over
  prime fields*, PKC 2016, DOI 10.1007/978-3-662-49387-8_1 (PDF: https://christophe.petit.web.ulb.be/files/16PKC_primeECDLP.pdf).
- Amadori, Pintore, Sala, *On the discrete logarithm problem for prime-field elliptic curves*, Finite
  Fields Appl. 51 (2018), ePrint 2017/609 (https://eprint.iacr.org/2017/609).
- Corrigan-Gibbs, Kogan, *The Discrete-Logarithm Problem with Preprocessing*, EUROCRYPT 2018, ePrint 2017/1113
  (https://eprint.iacr.org/2017/1113).
- Shoup, *Lower bounds for discrete logarithms and related problems*, EUROCRYPT 1997.
- Diem, *On the discrete logarithm problem in elliptic curves*, Compositio Math. 147 (2011) 75–104;
  *… II*, Algebra & Number Theory 7(6) (2013) 1281–1323.
- Semaev, *Summation polynomials and the discrete logarithm problem on elliptic curves*, ePrint 2004/031;
  Semaev, ePrint 2015/310 (binary-field focus).
- Repo: `RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md`; `RESEARCH_IC_LEADS_AND_CONNECTIONS_20261010.md`;
  `reports/Index calculus leads next steps.md`; `research/notes/index-calculus/RESEARCH_PKM_TOWER_ORACLE.md`;
  `research/ic_next10_20261014/{02-solution-count,03-pkm-degree}`; `research/pkm_symmetric_lead1_20261026`;
  autoresearcher `experiments/EXP-ICEX-153c34/specification.yaml` and
  `coordination/goals/GOAL-ICEX-001/batches/BATCH-454f85/tasks/TASK-20261001-768475/design_report.yaml`.
