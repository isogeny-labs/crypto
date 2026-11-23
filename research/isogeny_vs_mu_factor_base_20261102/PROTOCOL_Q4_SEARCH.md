# Protocol Q4: searching equations, point representations and isogeny kernels for cheaper factor bases

Status: **frozen before the recorded run** (2026-11-02). This extends
`PROTOCOL.md`, keeping the same seeds, targets, sampling scheme, Buchberger
engine, gates and budgets. The code is `search/`, a copy of `harness/src/main.rs`
(unchanged, as of commit de0b5628) plus the Q4 code; `harness/` is not
touched. Development runs before the freeze are disclosed below.

## Question

At equal |V| = 2^k and m = 2, can a different point representation (per-level
coordinates on the Kummer line), a different equation form, or a different
isogeny kernel reduce the counted Buchberger work of the isogeny-kernel
decomposition systems? And how far does each result remain from the μ_k
reference?

"Better" means fewer counted termops than the native Vélu presentation of
the same V. Coordinate changes keep V, and therefore the yield, fixed; a
kernel change alters V, and its exact yield is reported. This is a constant-factor,
m = 2 component study with no ECDLP or rho claim.

## Structural expectation (stated before the run)

The chain's quotient algebra has S1 density 1 (one term per product)
exactly when, in some coordinates, every step is monomial, u′ = λu². That
requires the ramification points of each step to sit at 0 and ∞, and its
branch points to equal the ramification points of the next step.

For a 2-isogeny x-map with kernel ⟨T⟩:
- the ramification points are x(½T), the halves of T (4-torsion);
- the branch points are the x-coordinates of 2-torsion points of the
  codomain.

The next step ramifies at the halves of its own kernel point, which are again
4-torsion, so they never coincide with these branch points. **Therefore no
per-level Möbius coordinate system makes a Vélu x-chain monomial, so S1
density 1 is unreachable for the Vélu arms.** For ℓ ≥ 3 the x-map has
ramification index at most 2, so a monomial step is impossible altogether.
The search measures how much density and work can still be removed. This
argument is written here and checked empirically: no Vélu
representation in the search may reach density 1. A Lean proof is not part
of this study and is follow-up work.

## Search space

- **Kernels.** Per cell, 40 random auxiliary pairs (E′, G) with ord G = 2^{k+2}.
  The first 8 are the Q1 auxiliaries and the first 2 are the Q2 ones. The
  search also tries up to 4 curves each with j = 0 (y² = x³ + b) and
  j = 1728 (y² = x³ + ax), and reports cells where none exists.
- **Representations.** Each level j < L gets a coordinate u_j = M_j(x_j),
  where M_j is the Möbius map that sends one special point to 0 and another
  to ∞ (P₀ ↦ 0, P∞ ↦ ∞, with scale 1).
  - The special points at level j are: ∞, 0, the kernel abscissa x(T_j), the
    other rational 2-torsion abscissas, the rational ramification points
    x₀ ± √v, and x(G_j).
  - Montgomery x-only, kernel-at-0, Legendre or AGM-style
    normalisations are all points of this space (scaling changes no sparsity
    pattern).
  - The terminal level stays native. Level 0's coordinate is also
    substituted into S₃, with denominators cleared, as in arm A2.
  - A representation is valid only if every point of V stays finite at
    every level.
  - Recipes searched: native; every *uniform* recipe (the same pair of types
    at every level); and a greedy per-level refinement of up to 3 passes,
    starting from the best uniform recipe by S1.
- **Equation form.** The `inverse_var` form adds w_j = 1/(x_j − x₀) and uses
  the equations w_j(x_j − x₀) = 1 and x_{j+1} = x_j + v_j·w_j, giving two
  variables per level.
- **References.**
  - A1 is the μ_k coset arm.
  - D is the Dickson chain t′ = t² − 2 on the traces of ζ with
    ord ζ = 2^{k+2}, terminal 0: the x-only (Kummer) analogue for 𝔾_m.
    D separates "x-only" from "elliptic".

## Cells

These are the Q2 cells for which 2^{k+2} | p − 1 (needed by D): p ∈ {257, 1153,
7681, 12289} and k ∈ {3, 4}.

## Selection (no confirmation data used)

1. **S1 selection (secondary).** For each kernel, take the lowest exact S1
   density over its representations; ties go to the earlier recipe.
   - C_rep is the S1-best representation on kernel K₀ (the Q2 auxiliary 0).
   - C_best is the S1-best (kernel, representation) pair overall.
   - C_ker_nat is the kernel with the lowest native density, in native
     coordinates.
2. **Measured selection (primary).** On kernel K₀, every valid uniform
   recipe, native, `inverse_var` and the greedy S1 recipe are run on the
   *selection split*: targets 8–9 of the Q1 target list, with 2 "dec" and
   2 "nodec" values of R per target, in both orders. C_sel_grevlex and
   C_sel_lex are the representations with the lowest median termops in each
   order; ties are broken by name.

## Confirmation (held out)

Targets 4–7 of the Q1 target list are disjoint from Q2 (targets 0–3) and from
selection. Per target there are 6 "dec" and 6 "nodec" R values, both orders,
for the candidates C_nat (native B on K₀), C_rep, C_sel_grevlex, C_sel_lex,
C_inv, C_ker_nat, C_best, A1 and D. The Q2 gate applies to every system:
the brute-force consistency check, and evaluation at the lifted witness.

## Decision rules

Comparisons are per (cell, order, stratum): 8 cells × 2 strata = 16 per
order.

- **Primary, grevlex.** C_sel_grevlex / C_nat (median termops) ≤ 0.8, with the
  upper bound of the 95% bootstrap CI < 1 (4000 resamples, seeded).
  - **Improvement** if this holds in ≥ 12/16.
  - **No improvement** if it holds in 0/16.
  - Otherwise: inconclusive.
- **Primary, lex.** The same rule for C_sel_lex / C_nat in lex.
- **Secondary.** The same rule for C_rep and C_best (S1-selected) in grevlex.
  The median, over cells, of the per-cell Spearman correlation between S1
  density and median termops is also reported. That measures whether S1 is
  a usable proxy.
- **Distance remaining.** C_sel / A1 per cell, reported as the gap to μ_k,
  not as a gate.
- **Obstruction check.** No valid Vélu representation may have
  S1 density = 1 (see "Structural expectation"). A density-1 Vélu row would
  contradict the argument, and would be reported first.
- **Kernel axis.** Report the spread of native and best S1 density across
  the 40+ kernels, and the exact yield ratio r for each candidate's V on
  the confirmation targets.

## Development runs before the freeze (disclosure)

The search code was smoke-tested on the cells (1153, 3) and (1153, 4) into
a scratch directory, before the selection split was added and while it was
being added.

On (1153, 3) those runs showed:
- the S1-best Vélu representation reached density 1.375 (native 3.27), and
  every kernel reached the same value;
- that representation had about 1.2× the native termops in grevlex and
  about 0.3× in lex;
- D cost about the same as A1;
- S1 density was a weak grevlex proxy.

**The termops-based selection split was added because of that observation,**
so that a representation is chosen on measured cost and confirmed on
held-out targets, rather than chosen by a proxy. The confirmation targets
(4–7) were used in those runs on the two 1153 cells. Those numbers are not
data. The frozen run recomputes every cell, and cell 1153 is flagged as
"seen in development" in the report.

## Execution

The execution path is the same as for `PROTOCOL.md`: locally, through a
director/executor task card, with counted units primary and `wall_us`
unadmitted.

## Amendments

(none)
