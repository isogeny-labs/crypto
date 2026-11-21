# B1: branch-and-bound past the Howgrave-Graham bound on S₃ lattices (design, not run)

2026-11-10 · design only, no experiment, no claim. This is record
`MOON-B1-milp-beyond-howgrave-graham` in
`research/prime_moonshot_20261110/ideas-generator.md`. Every number taken from
the repository is `[pcc:unverified]`. Everything else is derived here.

**Verdict.** With one shared reduced basis, B&B visits about 2^{2·w̄·j}
nodes j bits past the HG box, with w̄ ≥ 2 the mean weighted degree of the
monomial set; bit-guessing needs 2^{2j} reductions (derived). Pure B&B loses
in the exponent. The best hybrid keeps bit-guessing's exponent and gains about
(C_LLL/C_node)^{1/w̄}/2 ≈ 3–5× (derived). **Recommendation: do not run the B&B
experiment.** If the derivation is doubted, run the 10-minute norm-slope
pre-check of §6, which can refute it with existing code.

## 1. Setting

- S₃(x₁, x₂, x_R) ≡ 0 (mod p) at 40 bits, with planted roots x₁, x₂ ≤ X, on
  probe C's lattices: `Sym{m}` ((e₁, e₂) = (x₁ + x₂, x₁x₂), bounds (2X, X²),
  α + β ≤ 2m) and `Rect{m,t}` (α ≤ 2m + t, β ≤ 2m).
- After LLL, every row satisfies h_j(x₀) = p^m·q_j with
  |q_j| ≤ K_j := ⌊√dim·‖b_j‖/p^m⌋; below HG, K_j = 0.
- Probe C, `Sym{2}` (dim 15): "HG first two" holds 8/8 at X = 15 and 1/8 at
  X = 27; "≥ 2 vanishing" holds 7/8 at 27 and 48, 2/8 at 84 and 0/8 at 147. An
  LLL takes 3–4 ms (`Rect{2,1}`, dim 30: 19–50 ms) `[pcc:unverified]`.
- So X_HG ≈ 16 in the worst case, and typical roots keep q = 0 for about 1.5
  more bits.

## 2. What is branched on, and the pruning bound

- **Node.** An axis-aligned box [a₁, b₁] × [a₂, b₂] ⊆ [1, X′]² in (x₁, x₂),
  with X′ = X_HG·2^j. The lattice is built and reduced **once**, at bound X′.
- **Bound.** For each of the first r reduced rows (r ∈ {2, 4}, frozen), compose
  h_j with e₁ = x₁ + x₂ and e₂ = x₁x₂ for `Sym`. Enclose its range over the box
  exactly: on the positive orthant every monomial is monotone, so each term's
  minimum and maximum are at the two corners. Positive and negative
  coefficients are summed separately.
  - Prune the box if, for some j, [lo_j, hi_j] contains no multiple of p^m
    (that is, the integer interval for q_j is empty).
  - For `Sym`, also prune boxes with e₁² < 4e₂ everywhere.
- **Branch.** Split the longer side at its midpoint, depth-first.
- **Leaf.** At a unit box, check S₃(x₁, x₂, x_R) ≡ 0 (mod p) exactly, then
  check ±P₁ ± P₂ = R on the curve.
- **q-branching** ((q₁, q₂) ∈ [−K, K]², then solve over ℤ) has the same
  count, (2K + 1)², so it is not a separate arm.

## 3. Why nodes grow like 2^{2w̄j} (derived)

- **Norm slope.** ∏‖b_i*‖ = det L ∝ X′^{Σ wdeg}, so the Gram–Schmidt norms'
  geometric mean grows as X′^{w̄}, w̄ = Σ wdeg / dim (rigorous). Under the
  Gaussian heuristic the first r rows follow, so K_j ≈ 2^{w̄j}.
- **Box count.** A box of side w changes h_j by about ‖b‖·w/X′, so pruning
  needs w ≲ X′/K: about K² boxes (pruning near the origin changes only the
  constant).
- **Values of w̄.** `Sym{m}`: w̄ = 2m (2, 4, 6). `Rect{m,t}`: w̄ = 2m + t/2
  (4.5, 5, 6.5 for the probe C cells).
- **Exponent.** In the record's notation (nodes ≈ 2^{2γj}), pure B&B has
  γ ≈ w̄ ≥ 2. The record predicted γ ∈ [0.95, 1.05]. That prediction assumed
  HG-sized sub-boxes prune, but a shared basis at X′ makes those sub-boxes
  K-fold too coarse.
- **Hybrid.** Bit-guess the outer j − i bits (2^{2(j−i)} LLLs) and run B&B on
  the inner i bits (2^{2w̄i} nodes). The optimum i* = log₂(C_LLL/C_node)/(2w̄)
  gains about (C_LLL/C_node)^{1/w̄}/2. With C_LLL/C_node ≈ 10³–10⁴ (assumed:
  1–5 µs per BigInt node) that is about 2.8× (`Sym{2}`) to 4× (`Rect{2,1}`),
  with the exponent unchanged.

## 4. What the §7c bound covers and what B&B could add

- **What §7c covers.** §7c and `smallroot_optimal_reach` bound when a lattice
  from (p, f)^t on a convex monomial set **guarantees** that the root vanishes
  over ℤ: n·δ ≤ (n!·vol Newt f)^{−1/n}. That is 0.3536 for S₃ in two unknowns
  with equal bounds, and 1/2 numerically with unequal bounds
  `[pcc:unverified]`. It is a determinant condition. It says nothing about
  search after reduction, where q_j ≠ 0.
- **What B&B could add.** Only (a) a cheaper constant per guessed cell (one
  interval evaluation instead of one LLL), and (b) use of the typical-root
  slack beyond the worst-case HG box. Probe C already captures (b) with zero
  search: q = 0 holds about 1.5 bits past HG.
- **Why not more.** The same determinant drives B&B's cost: Hadamard makes the
  geometric-mean K grow as (det/p^{m·dim})^{1/dim}, so each bit past the
  threshold multiplies the box count. B&B adds no information the determinant
  lacks; it is bit-guessing with a cheaper constant. For S₅ (degree 8, larger
  w̄) it is worse, and m = 2 never beats √p even with a free oracle (note §2.4)
  `[pcc:unverified]`, so B1 is at most a component result.

## 5. Experiment, if run (frozen rules)

- **Instances.** 40-bit prime-order curves; 64 planted and 64 unplanted
  targets per j, j = 0..8, seeds derived. Planted roots must **not** vanish
  over ℤ or mod 2⁶¹ − 1 (the §7b.3 global-relation lesson).
- **Arms.** (A) pure B&B, `Sym{2}` and `Rect{2,1}`, r ∈ {2, 4}; (H) the hybrid
  at i*; (C) guess-and-LLL, `Rect{2,1}` on f(c₁ + y₁, c₂ + y₂) over the X_HG
  grid with probe C's "≥ 2 vanishing" rule. Each arm is exhaustive over
  [1, X′]², so counts are order-free.
- **Metrics.** Per planted-root recovery: nodes and interval evaluations (A, H);
  LLL count and measured C_LLL (C); wall time; solved fraction.
- **Cap.** 2²⁸ nodes per instance. Censored cells are reported as censored.
  The cap binds at j ≈ 3.5 if γ ≈ 4.
- **Controls.** j = 0 must give 1 node; a random polynomial with the same
  Newton polytope (L4-C); brute-force root sets on 8 instances per j ≤ 6.
- **Kill (record's).** The lower CI of γ ≥ 0.9 on the largest uncensored j
  window → close as "bit-guessing with a cheaper constant". Censoring at
  j ≤ 4 already implies γ ≥ 3.5.
- **Pricing (record).** Nodes priced at 0 / 0.1 / 1 GAE; the LLL in measured
  bit-ops. This is a component reference only; no leaderboard row.

## 6. Cheapest native implementation

- **Pre-check (≈ 10 CPU-min, recommended instead).** After `lll_reduce` in
  `lattice_trial_kind`, print log₂‖b₁‖, log₂‖b₂‖ and log₂ p^m for
  X = 16·2^j, j = 0..8, on `Sym{1,2}` and `Rect{2,1}` (8 trials each). Fit
  the slope of log₂ K against j (predicted 2–4.5). If it is ≥ 1, close B1 on
  paper; if it is < 1, reopen B1 and build the experiment.
- **Full build (≈ 1 day).** Split `lattice_trial_kind` into
  `build_reduced(kind, bound)` plus the existing vanishing check. Reuse
  `s3_coeffs`, `poly_mul`, `poly_eval`, `lll_reduce` and
  `FastCurve::{lift_x, canonical, add, neg}`. Add `taylor_shift` (arm C), a
  corner-monotone `BigInt` `interval_range`, and a depth-first `bnb` with node
  counters. Write JSONL, plus a bench record if anything is timed.

## 7. Prior

- P(γ < 0.9 for pure B&B) ≤ 0.01.
- P(the hybrid's constant gain exceeds 10× at j ≥ 4) ≈ 0.1.
- P(this moves S₅ toward δ = 1/5) < 0.001.

## Sources checked

- `research/prime_moonshot_20261110/ideas-generator.md`, record B1 and summary
  table
- `examples/prime_ecdlp_breakthrough_probe.rs`: `LatticeKind`,
  `lattice_trial_kind`, `s3_coeffs`, `poly_mul`, `poly_eval`, `probe_c`
- `research/prime_fast_bench_20261008/probeC.md`
- `research/smallroot_optimal_reach_20261026/README.md`, §1–§5
- `research/prime_s5_small_root_leads_20261028/README.md`, §7b.3–§7c
- `RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md`, §2.2 and §2.4
- research-kb: **not available** (no MCP tool; `scripts/research-kb` reports
  "no executable found"). No `source_id` was recorded.
