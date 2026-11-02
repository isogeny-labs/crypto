# Protocol: isogeny-kernel vs μ_k factor bases for m = 2 point decomposition

Status: **frozen before data** (2026-11-02). Any later change is an amendment
appended at the end with its reason and date. Data taken before an amendment
stays in the record.

## Origin

A user experiment (pure Python and sympy, lex order, toy primes) compared two
PKM-style factor bases V ⊂ F_p of equal size for m = 2 point decomposition
on E/F_p:

- **μ_k** (Petit-style): V is the fibre of a chain of squarings, x_{j+1} = x_j².
- **Isogeny kernel**: V is the set of x-coordinates of ⟨G⟩ ⊂ E′(F_p) with
  ord G = 2^e, presented as a chain of Vélu 2-isogeny x-maps.

It reported three things:

1. The isogeny systems were much slower: about 3 s against 0.06 s at
   p = 257, and about 15 s against 0.18 s at p = 1153.
2. The isogeny base decomposed more targets: 7/8 against 3/8 at p = 1153,
   and 3/10 against 1/10 at p = 257.
3. Even multiples of G hit an affine pole, so V was "really just the odd
   multiples", at a cost of one extra variable.

Prior work: Petit–Kosters–Messeng (PKC 2016) introduced rational-map tower
factor bases, catalogued here as `LEAD-ICP-pkm-tower-algebraic-oracle`. The
lead review `research/prime_ic_lead_review_20261031/` puts the
exponent-relevant evidence at m ≥ 4 and treats m = 2 as exponent-irrelevant.
`LEAD-ICP-isogeny-transported-relation-yield` records yield conservation
under isogeny transport. **This study is a component-level, m = 2 study of
how the factor base is presented. It makes no ECDLP cost claim and has no
rho row.**

## Questions and pre-registered hypotheses

**Q1. Yield.** At equal |V|, does the isogeny-kernel V decompose more
targets than a uniformly random V of the same size?

- Pre-registered expectation: no (H1₀).

**Q2. Cost of presentation.** Is the higher Gröbner cost of the isogeny
systems caused by how the multiplication of the chain's quotient algebra
looks? Or is it specific to the isogeny set V?

Mechanism being tested. With the natural variable order (level 0 largest),
every chain step in every arm has leading monomial x_j², so every chain is
already a Gröbner basis, and Buchberger's first criterion applies equally to
all arms. (An earlier explanation given to the user said the Vélu step
creates mixed leading terms. That is false under this order, and this
protocol replaces it.) The quotient algebra A = F_p[x_0..x_{L-1}]/(chain)
has the squarefree monomial basis in every arm. For μ_k, A is a twisted
group algebra: every product of two basis monomials is a scalar times one
basis monomial. For Vélu chains it is not.

- **H2 (presentation).** The cost tracks the density of A's structure
  constants, so a Möbius change of coordinates of μ_k that makes A dense
  (arm A2) costs about as much as the isogeny arm.

**Q3. Terminal condition.** Is the user's construction (terminal at the
2-torsion image, e − 1 steps) radical, and does a terminal at the order-4
image give |V| = 2^k with only k steps?

## Arms

Here p ≡ 1 mod 2^{k+1}, and |V| = 2^k in every arm.

| arm | V | chain (L steps) | terminal |
|---|---|---|---|
| A0 | μ_{2^k} | x_{j+1} = x_j², L = k | x_k = 1 |
| A1 | ζ·μ_{2^k} = {x : x^{2^k} = −1} | x_{j+1} = x_j², L = k | x_k = −1 |
| A2 | the same set as A1, in coordinate y = (x−α)/(x−1) | y_{j+1} = (y_j² − α)/(2y_j − α − 1), L = k | y_k = (1+α)/2 |
| B | x(odd multiples of G), ord G = 2^{k+2} on an auxiliary E′ | Vélu x-maps x + v/(x − x₀), L = k | x(image of G), of order 4 |
| B2 | the same set as B (user's construction) | Vélu x-maps, L = k + 1 | x(image of G), of order 2 |
| R | 2^k uniform distinct elements of F_p^* (null; Q1 only) | none | none |
| PC | {x(jP₀) : j = 1..2^k} (positive control; Q1 only) | none | none |

A2 is conjugate to the squaring map. It has the same pole shape as a Vélu
step (quadratic over linear, affine pole, ∞ ↦ ∞ unramified). In A2 the
summation polynomial is also substituted, with x_i = (y_i − α)/(y_i − 1),
and denominators cleared. That confound is reported, not removed.

## Instruments

All are native Rust, `harness/` (no dependencies). One binary and one
engine are used for every arm.

- **Q1.** Exact enumeration. F = {P ∈ E(F_p) : x(P) ∈ V} and
  D = {P₁ + P₂ : P₁, P₂ ∈ F} \ {O}.
  - r = |D| / E₀, with E₀ = N·(1 − exp(−|F|(|F|+1)/(2N))) and
    N = #E(F_p) − 1. This normalises for |F|, the number of abscissas that
    actually lift to E.
  - p_dec = |D|/N is also reported.
- **Q2.** Buchberger over F_p (normal selection strategy, Buchberger's two
  criteria, full reduction, early stop on 1 ∈ I), in grevlex (primary) and
  lex (the user's order).
  - Variables are x_{0,0} > x_{1,0} > x_{0,1} > … (party i, level j,
    index 2j + i). The system is S₃(x_{0,0}, x_{1,0}, x_R) plus both
    parties' chains.
  - **Primary counted unit:** `termops`, one per coefficient
    multiply-subtract against a reductor term and per S-polynomial term.
  - Also recorded: S-pairs reduced, pairs dropped by criterion 1 and by
    criterion 2, zero reductions, basis size, maximum leading degree.
  - `wall_us` is secondary and unadmitted (see Execution).
  - Budget: 5·10⁷ termops (grevlex) and 2·10⁷ (lex). Exceeding it marks the
    row censored, and censored rows count as ≥ budget in medians.
- **S1.** For each chain, the number of nonzero structure constants of A
  over all 2^L × 2^L products of squarefree basis monomials.
- **Q3.** Backward enumeration of the level-0 fibre of the terminal over
  F_p, with multiplicities (a double root counts 2). The fibre is compared
  with V, and radical means distinct = 2^L.

## Gates (a failure invalidates the cell, and is reported)

- **Q2 correctness.** A consistent system must be consistent by brute force
  over V × V. The returned basis and the input equations must vanish at the
  chain lift of a brute-force witness. An inconsistent system must have no
  witness. Any failure is reported, and the cell is excluded.
- **Q3.** The fibre must equal V for A0, A1, A2 (image set), B and B2.
- **Q1 positive control.** The median r of PC must be below 0.5 × the
  median r of R in every cell, otherwise the detector is invalid for that
  cell.
- **Q1 A/A.** R is split into two halves (instances 0–3 and 4–7), and the
  bootstrap CI of their median difference is reported as the noise floor.

## Workload and seeds

- Master seed 20261102. Every sub-seed is derived from the master and a
  label: FNV-1a(label), then splitmix64.
- **Q1:** p ∈ {257, 1153, 7681, 12289, 40961} and k ∈ {3, 4, 5}, wherever
  2^{k+1} | p − 1. Per cell: 16 target curves, 8 auxiliary (E′, G), 8 random
  V and 1 PC per target. That gives 128 B values and 128 R values per cell.
- **Q2:** p ∈ {257, 1153, 7681, 12289} and k ∈ {3, 4}. Per cell: the first
  4 targets of the Q1 list, 2 auxiliaries (shared by B and B2) and 2 values
  of α. Per (target, arm instance): 6 targets R drawn uniformly from D (the
  "dec" stratum) and 6 drawn uniformly from E(F_p) \ D, rejected unless they
  have no x-level witness (the "nodec" stratum). Each system runs in both
  orders.
- **Q3 / S1:** the Q2 cells and arm instances.

## Decision rules (fixed now)

**Q1.**
- Effect: Δ = median r_B − median r_R per cell, with a 95% bootstrap CI
  (4000 resamples, seeded).
- **Yield excess** if in ≥ 2 cells the CI is above 0 and Δ/median r_R ≥ 5%,
  and no cell has a CI below 0.
- **Null** (H1₀ retained) if every cell's CI contains 0 or |Δ|/median r_R
  < 5%.
- Otherwise: inconclusive.
- The user's reported ratio, about 2.3×, would be a Δ/r of about +130%.

**Q2.** Ratios are median termops, arm / A1, per (cell, order, stratum).
In grevlex there are 8 cells × 2 strata = 16 comparisons.
- **H2 supported** if in ≥ 12/16, A2/A1 ≥ 2 and B/A1 ≥ 2 and
  A2/B ∈ [0.5, 2].
- **H2 falsified** (the cost is specific to V) if in ≥ 12/16, B/A1 ≥ 2 and
  A2/A1 < 1.25.
- Otherwise: inconclusive.
- **Replication of the user's slowdown:** B/A0 ≥ 2 in lex, in ≥ 12/16.
- The success/failure confound is reported as dec/nodec median termops per
  arm.
- **S1 prediction:** A0 and A1 have exactly one nonzero per product;
  A2 and B have more than two per product.

**Q3 prediction:**
- B is radical, with distinct = 2^k = |V| in k steps.
- B2 is non-radical, with distinct = 2^k and total multiplicity 2^{k+1} in
  k + 1 steps. So the user's extra variable is caused by the terminal,
  and it also makes the ideal non-reduced.

## Execution

- **Repository default: Iso Lab on EKS Spot (AGENTS.md §9a).** It is not
  reachable from this session:
  - no AWS identity (`aws` is absent and no profile is configured);
  - no EKS MCP;
  - the GitHub-identity route (`isolab-experiment.yml`) only builds and runs
    an `ecbench.spec/v1` workload from a fixed sparse checkout, so it cannot
    run this harness.

  That is reported as an integration gap, not worked around.
- Every primary quantity is a deterministic counted unit (exact counts,
  termops, structure constants). These are identical on any host, and they
  are what the decision rules use. The run is executed in the session
  container through the director/executor task card. `wall_us` is recorded
  as unadmitted secondary data (shared cloud container, no isolation tier
  earned) and is used for no decision.
- A replay on any other host must reproduce every counted column byte for
  byte. That is the replay check for this study.

## Stop conditions

- Run each question once, to completion. No reruns with changed seeds or
  budgets.
- A harness crash is reported with its output. Any fix is an amendment, and
  the run is then repeated in full.

## Development runs before the freeze (disclosure)

The protocol text above was written before any Q2 output existed. Before
the freeze commit, development smoke runs went to a scratch directory to
check the harness and calibrate run time:

- Q3/S1 in full.
- Q2 for (p, k) = (257, 3), all four targets.
- Q2 for (1153, 4), target 0.

The pair-selection queue was then changed from a linear scan to an ordered
set. The counted columns of target 0 at (257, 3) were byte-identical before
and after the change. No threshold, budget, seed or arm was changed after
those runs, and none of their outputs is used as data. The frozen run
recomputes every cell.

## Analysis

`isomu_fb <out> analyze` reads the run's CSVs and writes `q2_cells.csv`
and `verdicts.json` by the decision rules above. It is frozen with this
protocol. Figures and the report only lay out recorded values.

## Amendments

(none)
