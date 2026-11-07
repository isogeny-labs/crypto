# Lean attacks on two weak points of the ECDLP hardness graph

Date: 2026-11-04. Targets from
[`docs/ecdlp-hardness-corpus/WEAKNESS_TRIAGE.md`](../../docs/ecdlp-hardness-corpus/WEAKNESS_TRIAGE.md)
W2 (relation rank) and W7 (xedni waiting time). Lean project:
[`lean/`](lean/) (Lean v4.34.1, Mathlib v4.34.1, same pins as
`research/jksst_xedni_audit_20261101/lean`).

## Results

| module | theorem | what it settles |
|---|---|---|
| `LeftKernel` | `left_kernel_identity`, `left_kernel_recovers` | Any left-kernel vector `λ` of the relation matrix gives `Σλa + x·Σλb = 0`, so `x = −Σλa/Σλb` when `Σλb ≠ 0`. |
| `LeftKernel` | `exists_left_kernel` | With more relations than factor-base columns a nonzero `λ` exists for **every** matrix: no rank hypothesis. |
| `LeftKernel` | `card_dot_ker_mul`, `card_bad_pairs` | If `λ` is any nonzero-valued function of the relation points only (their logs `a + b·x`), exactly `q^(2n−1)` of the `q^(2n)` hidden pairs `(a, b)` give `Σλb = 0`: failure density exactly `1/q`. |
| `HazardSeparation` | `survival_ge_of_marginals` | Union bound: if each trial `t ≤ N` succeeds with marginal mass `≤ ε` (correlated or adaptive trials allowed), survival after `n` trials is `≥ 1 − nε`. |
| `HazardSeparation` | `truncated_expectation_ge_half_horizon_of_marginals` | With `Nε ≤ 1/2`, `E[min τ N] ≥ N/2`, the conclusion of `JKSSTExpectedTrials.truncated_expectation_ge_half_horizon` **without** its conditional-hazard recurrence. |
| `HazardSeparation` | `marginals_U`, `marginals_do_not_give_hazard`, `wait_U` | Separation: three equally likely outcomes, outcome `k` succeeding at trial `k+1`. Every trial's marginal is `≤ 1/3`, the recurrence `(1−1/3)·s(n) ≤ s(n+1)` fails at `n = 1` (`4/9 > 1/3`), and the wait is still `E[min τ 3] = 2`. |

## Petit–Quisquater Assumption 2 (W1): `SolvingDegreeBound`

| theorem | result |
|---|---|
| `linear_count_of_gb_in_span` | If a Gröbner basis (degree-compatible order) of `⟨F⟩` lies in `span_K F` and `1 ∉ ⟨F⟩`, the `k` ideal elements with distinct leading variables force `k` independent elements of degree ≤ 1 in `span_K F`. |
| `low_add_top_le_card` | `dim(span_K F ∩ deg ≤ 1) + rank(top₂ F) ≤ |F|` (rank–nullity). |
| `solving_degree_ge_three` | If `k + rank(top₂ F) > |F|`, no Gröbner basis lies in `span_K F`: for degree-2 generators (field equations included) the solving degree is ≥ 3. |
| `Toy.toy_solving_degree_ge_three` | Non-vacuity: `F = {x₀²+x₀, x₁²+x₁, x₀x₁+x₀+x₁}` over F₂ (`k = 2`, rank 3, `|F| = 3`). |
| `not_tendsto_of_frequently_gap` | An integer gap ≥ 1 infinitely often is not `o(1)`. |

**What this does to the assumption.** At `m = 2` the first fall degree is 2 for
every `n` (Kosters–Yeo), so the literal reading predicts solving degree 2. With
`n` descended equations, `N` bit variables and `N` field equations, the criterion
says solving degree 2 needs `r₂ ≥ k`. Here `r₂` is the number of independent linear
polynomials in the degree-2 span (Kosters–Yeo: `r₂ ≥ 1`, generically 1), and `k` is
the number of independent linear consequences (`k ≥ N − affdim(Z)` for solution set
`Z`). Any instance with `r₂ < k` therefore has solving degree ≥ 3 = D_ff + 1.
Infinitely many such `n` refute the literal reading.

**Not established here** (`GAP-PQ2-M2-RANK-CENSUS`):

- That `r₂ < k` holds on actual descended `S₃` instances for infinitely many `n`.
  This is a finite rank check per instance, cheap far beyond the `n ≤ 48` Gröbner
  computations.
- The Nullstellensatz step giving `k`.
- That PQ's "maximal degree reached" is the Caminata–Gorla solving degree.
- Kousidis–Wiemers' reading `D_reg = m² − m + 1 + o(1)`, which is 3 at `m = 2` and
  needs solving degree ≥ 4.

## Round 3: the last Lemma 4 cell, and protocol countermodels

| module | theorem | result |
|---|---|---|
| `NagaoLiteralInside` | `fake_fall_to_true_fall`, `lemma4_literal_inside` | Literal `≥` reading, field equations inside, any field and any system: every fake fall is a true fall at the same degree, so `d_F ≤ d'_F`. The fourth (reading × placement) cell is a theorem; the other three are refuted. |
| `ProtocolCountermodels` | `zero_hash_forgery` | Constant-zero challenges accept `(sP, 0, s)` for every key: special soundness does not give Fiat–Shamir security. |
| | `pedersen_alternate_opening` | `H = tG` gives `commit(m, r) = commit(m + t, r − 1)`. |
| | `cofactor_kills_subgroup`, `cofactor_bijective` | In Z/9 the cofactor 3 annihilates the order-3 subgroup; for `r ∤ h` cofactor multiplication is a bijection. |
| | `pairing_decides_ddh` | A nondegenerate pairing decides DDH with one evaluation. |
| | `ambient_blinding_leaves_subgroup`, `subgroup_blinding_bijective` | Ambient blinding leaves the prime subgroup; subgroup blinding is a bijection. |
| | `always_accept_complete_unsound` | Complete and simulatable does not imply sound. |

The round also **withdraws** a claim of the earlier triage. The dimension count bounds the
subspace factor base `|F'|`, not its trace image `|Tr(F')|`, so the trace pull-back escape
from the prime-field closure is open again (`GAP-PF-TRACE-PULLBACK-FACTOR-BASE`).

## Round 4: subspace criterion, counting identity, transparent encoding, Boolean Nullstellensatz

| module | theorem | result |
|---|---|---|
| `SubspaceCriterion` | `linear_count_of_gb_in_subspace`, `no_gb_in_rows_of_low_lt` | The degree-2 criterion generalised to any finite-dimensional subspace: a Gröbner basis inside the span of the Macaulay rows of degree ≤ d supplies at most `dim(rowspace ∩ P_{≤1})` linear leading monomials, so `d_lin` (least `d` with that dimension ≥ k) is a lower bound on the solving degree. This is what the `pq2_rank` census computes. |
| `MeanCount` | `sum_card_reps`, `mean_reps` | Ordered representation counts sum to `|F|^m`: the mean of `HEUR-DECOMPOSITION-PROBABILITY` is a theorem, the shape is the heuristic. |
| `TransparentEncoding` | `dlog_identity_encoding`, `zero_operation_dlog` | With a transparent encoding the logarithm is the encoding: Shoup's Ω(√r) is relative to random encodings, not a statement about every algorithm. |
| `BooleanNullstellensatz` | `mem_fieldIdeal_of_eval_zero`, `linear_mem_of_unique_zero` | Over F₂ with the field equations adjoined: vanishing on all of F₂^N means lying in the field-equation ideal; a unique common zero `z` puts every `X_i − z_i` in the ideal. This is the `|Z| = 1` case of the bridge `k = N − affdim(Z)` used by the census. |

## Round 5: F4 stays inside the degree-D closure

| module | theorems | what it says |
|---|---|---|
| `F4Containment` | `run_subset`, `no_gb_of_run_of_low_lt`, `dclosed_of_basis`, `monomial_mul_mem` | A subspace S is D-closed when `X i * f ∈ S` for every `f ∈ S` with `totalDegree f + 1 ≤ D`. An abstract F4 run (`F4Run`) puts each new basis in the span of earlier bases and their monomial multiples within degree D. Every basis of such a run lies in any D-closed S that contains the generators. With `dim(S ∩ P_{≤1}) < k`, no basis of the run is a Gröbner basis, so the `pq2_rank` closure counts are lower bounds on F4's maximal step degree. Closure of `span B` reduces to a finite check on B. |

The axiom audit for all four theorems shows `propext`, `Classical.choice` and `Quot.sound` only.
Their pcc claims are `PCC-20261107-209a57` and `PCC-20261107-91a01d`, both unsigned [pcc:unverified].
Fidelity limit: the model covers F4 as published, including symbolic preprocessing, the
simplify heuristic, criteria and the final interreduction. That msolve, Magma or FGb fit it
is a reading of their algorithms, not a theorem.

## What changes in the corpus

- **`HEUR-RELATION-RANK` (W2).** The left-kernel method needs no rank
  condition. The finite core is now kernel-checked: existence of `λ` for any
  matrix, recovery, and the exact `1/q` failure count with `λ` depending only on
  the relation points. The heuristic stays load-bearing only where relations
  are reused across targets (right-kernel amortisation, multi-target batches,
  PKM's Δ surplus). Not formalised here: the group-theoretic step "a
  decomposition of `R_i` gives the row `M_i`" (by definition of the factor-base
  logs) and the reduction from a uniform `(a_i, b_i)` to the counting statement
  (the counting theorem is the uniform-measure statement itself).
- **`ASM-XEDNI-HAZARD-RECURRENCE` (W7).** The linear waiting time behind
  `OBSTRUCTION-XEDNI-HEIGHT` follows from per-trial marginal bounds. A
  history-uniform hazard bound is not needed, and marginal bounds do not imply
  it. The open xedni bridge is therefore a **per-trial marginal bound for the
  adaptive trial actually run**, a weaker and more natural premise. This
  strengthens the obstruction; it does not reopen xedni.

The W2 and W7 results are recorded in
[`docs/ecdlp-hardness-corpus/lean-weakness-20261104.json`](../../docs/ecdlp-hardness-corpus/lean-weakness-20261104.json); W1 in
[`lean-pq2-20261104.json`](../../docs/ecdlp-hardness-corpus/lean-pq2-20261104.json); round 3 in
[`lean-round3-20261104.json`](../../docs/ecdlp-hardness-corpus/lean-round3-20261104.json); round 4 (with the censuses) in
[`lean-round4-20261104.json`](../../docs/ecdlp-hardness-corpus/lean-round4-20261104.json).

## Verification

```sh
cd research/ecdlp_weakness_lean_20261104/lean
lake exe cache get && lake build      # what CI's lean-proof-gate runs
lake env lean AxiomAudit.lean         # #print axioms for every theorem above
tools/lean/proof-gate.sh research/ecdlp_weakness_lean_20261104/lean   # from the repo root
```

The local check compiled each module with `lake env lean -o` against the
Mathlib v4.34.1 cache. A full local `lake build` was not possible because the
session's disk allowance could not hold Mathlib's rebuild outputs. CI's
`lean-proof-gate` runs the full build and the transitive axiom audit. No `sorry`,
`native_decide` or user `axiom` occurs in either module.
