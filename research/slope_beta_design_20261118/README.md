# Can any factor-base / decomposition structure reduce the IC-vs-rho slope β? — a design (2026-11-18)

**This is a design, not a result.** Nothing here is measured. It decomposes the
slope β of the one rigorous priced comparison
(`research/ecbench_frobenius_orbit_vs_rho_20261014/`), identifies which lever
each known technique pulls, and preregisters an experiment that would test
whether any candidate structure reduces β, with an honest falsification rule.

- Preregistration (arms, n-span, controls, falsification): [`PREREGISTRATION.md`](PREREGISTRATION.md)
- Sources and repo leads used: [`SOURCES.md`](SOURCES.md)

Class: **engineering of a measurement / research design.** No security claim, no
sub-rho claim. The panel is toy/medium Koblitz curves (log₂ r 11–39), all with
**prime** extension degree n.

---

## 1. What β is made of

### 1.1 The incumbent and its model

The study measures the paired ratio **IC ÷ rho-frob** of the counted unit S
(GAE/√r) across eight Koblitz curves, and fits ln(ratio) against ln r (and
ln n). Its verdict is **constant-only**: β = 0.165 [0.117, 0.213] per ln r with
ln n in the fit (`free_r_n`), β = 0.093 [0.062, 0.123] without
(`free_r`); model value 1/6
(`research/ecbench_frobenius_orbit_vs_rho_20261014/trend/trend-orbit-vs-rho-frob.json`).

The study's own cost model (PROTOCOL.md §1):

- IC (m = 2, `c` orbit columns): **≈ n·c²/2 table additions + r/(2 n² c) trials**,
  minimised at **c\* ∝ (r/n³)^{1/3}**, giving total **∝ r^{2/3} / n**.
- Frobenius rho: **√(π r / 4n) ∝ r^{1/2} n^{-1/2}**.
- Ratio IC ÷ rho-frob **∝ r^{1/6} n^{-1/2}**.

So, exactly:

> **β = d ln(ratio)/d ln r = p_IC − p_rho = 2/3 − 1/2 = 1/6,**

and γ (the n-exponent) = −1/2 in the model. **β is the difference of two r-exponents
and nothing else.** The entire slope is `(IC's r-exponent) − (rho's r-exponent)`.

### 1.2 Where the 2/3 comes from — grounded in the code, not just the model

The measured method is `orbit_ic.rs`, described in its own header as *"a bounded
enumerated **pair-table** method, not an algebraic shortcut"*
(`src/cryptanalysis/ecbench/orbit_ic.rs:1`). Its phases and the counters that
grow with the factor-base size `c` (= `columns`,
`orbit_ic.rs:474-480`):

| phase (code) | work | op-count at optimum c\* ∝ r^{1/3} |
|---|---|---|
| `factor_base` | build c orbit columns | ∝ c ∝ r^{1/3} (sub-dominant) |
| `oracle_setup` (pair table) | nested loop over columns i,j → **c² pairs** (`orbit_ic.rs:333-334`, `pair_visits`) | **∝ c² ∝ r^{2/3}** |
| `target_PDP` / `target_query` | **trials** random targets, one lookup each (`orbit_ic.rs:400-407`, `trials`) | **∝ r/c² · c... ∝ r/c ∝ r^{2/3}** |
| `linear_algebra` | exact RREF rank over c+1 columns (`orbit_ic.rs:111-146`) | ∝ c² entries (`matrix_entries_scanned_uncharged`), but priced near-zero |

The decisive structural fact: **at the balanced optimum c\* ∝ r^{1/3}, the
pair-table build (c²), the relation search (r/c) and the linear algebra (c²) are
all Θ(r^{2/3}) simultaneously.** That is what "balanced" means — you cannot move
the exponent by shifting cost from one of these phases to another, because they
share the exponent at the optimum. The GAE total is dominated by the group
additions of the table build and the walk/search; the linear algebra is counted
but priced as cheap row operations (`full_price.rs`), so it does not drive S.

**Therefore β is a property of the m = 2 meet-in-the-middle (MITM) 2-sum
structure itself**, not of any particular factor base. The 2/3 is the MITM
birthday-balance exponent: you build an N²-entry pair table (N = effective base
size) and search r/N targets, balanced at N = r^{1/3}. This is a *theorem of the
balance*, not a heuristic — and it is why every folding so far has moved only the
constant.

### 1.3 Which lever each known technique pulls (and why it is the constant)

| technique | quantity it changes | effect on p_IC (the 2/3) | why |
|---|---|---|---|
| **Frobenius-orbit folding** (incumbent) | base columns /2n; trials ×n² (study §1) | **none** | c\* still ∝ r^{1/3}; r^{2/3} unchanged. Buys the −1/2 in n and a constant (orbit ÷ negfold 0.51→0.065, study H5). |
| **negation folding** | factor-2 in base | none | constant |
| **subspace / cyclotomic base** | which points are in the base | none | yield is fixed by the `|B|^m/r` identity (`RESEARCH_IC_LEADS_ASSESSMENT_20261018.md:57,61`); still m = 2 MITM → r^{2/3} |
| **stride-renew targets** | removes a fixed walk set-up | none | H2: a constant, interval → 1 as r grows |
| **nearly-sparse / Wiedemann linear algebra** (Joux–Vitse) | the linear-algebra phase | none **here** | linear algebra is not the GAE-dominant phase in this MITM method; moving its constant moves nothing measurable |

**Every lever pulled so far pulls a constant because none of them changes the
MITM pair-table balance.** To move β you must change **p_IC itself** — i.e.
replace the structure whose balance is r^{2/3}.

---

## 2. What could change p_IC — and the honest prior for each

Two, and only two, families can in principle change the r-exponent:

**(A) An algebraic decomposition that finds the m-sum without enumerating the
base.** The MITM wall is r^{2/3} *because* the per-target decomposition costs
Θ(N) (enumerate one summand) or needs the Θ(N²) pair table. Semaev summation
polynomials + Gröbner/root-finding promise a per-target cost that is
`poly(d_reg)` *independent of N*, so N can grow and m can rise, giving Gaudry's
Õ(q^{2−2/n}). **For q = 2 with n prime — the entire panel — Diem's theorem gives
expected cost e^{O(n (log n)^{1/2})}, i.e. exponential, which does not beat rho**
(Galbraith–Gaudry survey; Karabina). The subexponential claim (Petit–Quisquater)
rests on first-fall-degree ≈ degree-of-regularity, which Shantz–Teske and others
challenge. The repo's own measurements agree: first-fall degree never exceeds 3
to n = 63, but **the (m−2)·n chaining unknowns are what bite**
(`RESEARCH_KOBLITZ_INDEX_CALCULUS.md:384-389`), and where 2 is primitive mod n
there is **no Frobenius-stable subspace at all**
(`RESEARCH_KOBLITZ_INDEX_CALCULUS.md:326-334`; assessment:57).

**(B) A k-tree / generalized-birthday (Wagner) search** that replaces the N²
pair table with a tree of lists of size r^{1/(1+log₂ k)}. This needs
multi-representation freedom (many ways to write the target as a k-sum), which a
single fixed relation target does not have unless the base is large; and it
finds one solution where index calculus needs N independent relations, so the
amortization differs. Genuinely untried in this harness; honest prior is "likely
null, but it is the one combinatorial route that attacks the pair-table balance
directly."

**Within the MITM pair-table family, raising the arity m does *not* move the
exponent** (derived in PREREGISTRATION.md §3): m = 3 still balances to r^{2/3}
(table c² + search r/c). This is why the first preregistered arm is an arity
sweep used as an *instrument-validation control with a known analytic answer
(Δβ = 0)*, against which the algebraic arm (A) is the scientific test.

### Ranked shortlist of candidate slope-movers

1. **Algebraic m = 3 decomposition (Semaev S₄ + root-finding), orbit/subspace
   base.** Mechanism: per-target cost poly(d_reg) not Θ(N) → the only route that
   can break r^{2/3}. Quantity changed: p_IC via the decomposition exponent.
   Predicted sign/size: **Δβ ≥ 0** (no reduction; the solve cost grows with n as
   the (m−2)n unknowns force degree growth). Cheapest discriminating
   measurement: the slope of ln(cost_arm/cost_incumbent) vs ln r on the panel.
2. **Wagner 4-sum / k-tree search.** Mechanism: avoid the N² pair table. Quantity:
   p_IC via the search exponent 1/(1+log₂ k). Predicted: **Δβ ≥ 0 at these sizes**
   (not enough representation freedom; relation-count amortization unfavourable),
   but with the largest residual uncertainty. Measurement: same slope statistic.
3. **Weil-descent / cover transfer (GHS, hyperelliptic).** Mechanism: change the
   *group*, not the base. Quantity: p of the transferred DLP. Predicted: **null
   on this panel** — all n are prime, GHS/cover needs composite or special n.
   (Counts toward T-tiers only via the "transfers to hyperelliptic" clause, and
   only off this panel.)
4. **Symmetry-reduced Gröbner (equivariant F4, hyperoctahedral invariants).**
   Mechanism: shrink the Macaulay matrix. Quantity: the *constant/base* of the
   d_reg cost, and d_reg itself. Predicted: **constant, not slope** — and **empty
   for n ∈ {13, 19, 29, 37}** of the panel where 2 is primitive mod n (no
   invariant subspace). This is a constant-mover masquerading as an
   exponent-mover.
5. **Nearly-sparse linear algebra (Joux–Vitse).** Predicted **null here**: not
   the GAE-critical phase in the MITM method.

**The honest null for all five: most will not move β.** 1 and 2 are the only two
whose mechanism *can* touch p_IC; 3–5 provably cannot on this panel.

---

## 3. Contrarian bottom line

On the evidence, **for cold, prime-n Koblitz curves the rho gap is very likely
exponent-fundamental for this method family, and no known factor-base structure
is likely to reduce β.** The reasons converge from three independent directions:

1. **Structure:** β = p_IC − p_rho = 2/3 − 1/2. The 2/3 is the MITM pair-table
   balance, invariant under every folding and under the arity m. Folding moves
   constants by construction.
2. **Theory:** Diem — index calculus over E(F_{2^n}), n prime, is
   expected-exponential and does not beat rho; the subexponential route needs
   first-fall ≈ d_reg, which is contested.
3. **This repo's own measurements:** first-fall ≤ 3 but the (m−2)n chaining
   unknowns bite; the invariant-subspace base is *empty* exactly at primitive n;
   yield is conserved across equal-size bases, so no base change can change the
   decomposition probability.

**What would change my mind** (and what the preregistered experiment is built to
detect): a decomposition engine whose **per-target cost grows strictly slower
than the factor-base size N** *and* whose effective degree stays bounded as n
grows for some concrete construction — measured as a **resolvably negative slope
of ln(cost_arm/cost_incumbent) vs ln r** across the panel. Absent that, every
new factor base is a constant-mover, and the right scientific framing is to
report each arm's distance to the matched rho (the success criterion), not to
expect a crossing.
