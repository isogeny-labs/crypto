# Track 1: the degree-slope lower bound for PKM/tower systems — target, attempt, obstruction

Date 2026-11-24. Desk theory plus about two minutes of local closure computation
(`reach.py`, reusing the wave-2 engine `research/kummer_wave2_20261115/KM3-LQR-CLOSURE/hky_closure.py`).
Every statement is `[pcc:unverified]`; there is no pcc claim id. Status labels:
DESK-PROOF (short argument written here, not machine-checked), MEASURED, CONJECTURE,
CITED (from memory or another note, not re-derived here). research-kb unavailable in
this container; sources read are listed in §6.

## 1. The exact target

### 1.1 What a closing theorem has to show under the repo's cost model

`pkm_cost_rules_20261115` charges a per-attempt cost `B^s` with `B = 2^t`, and the
repo derives `s` from the solving degree `D` through the dense multilinear count
`M(N, D) = Σ_{k≤D} C(N, k)` with `N = m·t`: if `D = σN` then `M ≈ 2^{N·H(σ)} = B^{m·H(σ)}`
and `s = m·H(σ)·w`. The lever fails to reach rho parity (KILL) when `s ≥ (m−1)/2`, i.e.

    H(σ) ≥ (m−1)/(2mw),    σ*(m, w) := H⁻¹((m−1)/(2mw)).

DESK-PROOF (arithmetic) `[pcc:unverified]`. Values (`python3` in this session):

| m | σ*(m,1) | σ*(m,2) | two-block slope 1/(4m) | empirical slope |
|--:|--:|--:|--:|--:|
| 3 | 0.0615 | 0.0246 | 0.0833 | ≈0.125 |
| 4 | 0.0724 | 0.0286 | 0.0625 | ≈0.125 |
| 5 | 0.0794 | 0.0311 | 0.0500 | ≈0.125 |
| 8 | 0.0903 | 0.0350 | 0.0312 | ≈0.125 |
| 16 | 0.0999 | 0.0383 | 0.0156 | ≈0.125 |
| →∞ | 0.1100 | 0.0417 | →0 | ≈0.125 |

`σ*(8,1) = 0.0903` reproduces the repo's own figure ("parity at m = 8 needs σ < 0.0903"),
which checks this reconstruction of the cost model. The empirical slope ≈ 1/8 is the
long-run `dD/dN` on the Kummer ladders (m = 2: one degree per four t-steps, N = 2t;
m = 4 at P1: 1/8–1/6; m = 8, N 8→16: 0.125).

**Target T_M1 (model-conditional).** For every m ≥ 5 there is `σ₀ > σ*(m,1)` with
`D_PC(m, t) ≥ σ₀·m·t − O_m(1)` for the Kummer split-Semaev chain (fixed presentation),
for a positive fraction of targets, for all large t. Together with axiom
`A_M1: per-attempt cost ≥ M(N, D_PC)^w` this closes the lever in the model.

Two consequences of the table, both DESK-PROOF `[pcc:unverified]`:

1. **The bound must be linear in N = m·t, uniformly in m — every block must contribute.**
   Restriction (Lemma M; D6; the 50/50 spec-ladder data) transfers an m = 2 bound
   `D ≥ t/4` to all m, but that is slope `1/(4m)` in N, below σ*(m,1) for every m ≥ 4.
   Since the cost rule already excludes m ≤ 4 from reaching parity, **a two-free-block
   (m = 2) theorem, even if completed, does not close any live cell.** It would be the
   first growing lower bound on `d_F` for any PKM system (a proof-of-technique), not a
   closure.
2. **The margin is thin.** The empirical slope 0.125 exceeds σ*(∞,1) = 0.110 by about
   13%. A closing theorem has to be within ~13% of the true slope at w = 1 (about 3× slack
   at w = 2). No growing lower bound on `d_F` exists in the repo at all (Prop R barrier;
   closure note §4.4).

### 1.2 What "the degree-only route to a cost bound is ruled out" rules out

I could not locate the round-7 source text behind closure note §4.6. Independent
reconstruction, two separate reasons:

- **Degree without a size or model axiom is not a cost bound.** `lfd_lower_bound_20261111`
  Lemma 2 (DESK-PROOF there, checked on 6 instances): every unsatisfiable system has a
  re-presentation with extension variables whose degree is 2, at a size of
  `Σ_{j≤D} C(N,j)` variables. So `d_F` is not an invariant of the instance; only a
  degree-plus-size statement (or the M1 axiom) carries cost. Note that A_M1 is robust to
  this re-presentation: the extended system's M-count at degree 2 is about `M(N,D)²`.
- **The size–degree trade-off loses a square** (CITED from memory: Impagliazzo–Pudlák–Sgall
  1999, PC refutation size `S ≥ exp(Ω((D−d)²/N))`; applicability to the tower encoding is
  plausible because normal-form monomials are multilinear in the `y`-variables, but it is
  not checked). With `D = σN` this gives `log₂S ≥ c′σ²N`. Closing needs
  `log₂S ≥ (m−1)/(2m)·N`, so σ ≥ 0.55–0.59 even with an optimistic `c′ = 1/ln 2`. The true
  slope ≈ 0.125 caps any degree lower bound, which then certifies at most `S ≈ 2^{0.0225N}`
  against the needed `2^{0.44N}`–`2^{0.5N}` — about 20× short in the exponent.
  DESK-PROOF (arithmetic, conditional on the cited trade-off) `[pcc:unverified]`.

**Restated target.** An *unconditional* closing theorem has to bound refutation **size**
(PC size, or F4 work) by `B^{(m−1)/2}` directly. For a non-local system (lfd note Lemma 5:
generators touch ≈ k − √(2k) variables, so expansion methods do not apply) that is an open
problem in proof complexity, out of reach. The meaningful, attackable target is **T_M1**,
which stays model-conditional on A_M1 (supported by `f4-model` measurements: msolve writes
more matrix entries than the XL column count at m ≥ 3, but unproved).

## 2. Attempt in the smallest case: m = 2, two free blocks

Setting (DERIVED.md D1–D4): `R = F_p[z₁,z₂]/(z₁^K−1, z₂^K−1)`, `K = 2^t`, basis `z^k`,
degree `pc(k₁)+pc(k₂)`. `f = NF(S₃(gz₁, gz₂, x_R))` has support in `{0,1,2}²`; the instance
is inconsistent iff `f` is a unit. `V_c` is the degree-c closure; `d_F` the least c with
`1 ∈ V_c`. Because `f` is a unit, every `v ∈ V_c` is `v = h·f` for a unique multiplier
`h`; write `H_c` for the multiplier space and `R(c)` for the largest popcount in the union
of supports of `H_c` (basis-independent).

### 2.1 Reach criterion. DESK-PROOF `[pcc:unverified]`

    d_F ≥ min{ c : R(c) ≥ popdeg(f⁻¹) }.

*Argument.* If `1 ∈ V_c` then its multiplier is `f⁻¹`, so `supp(f⁻¹) ⊆ ∪ supp(H_c)` and
`R(c) ≥ popdeg(f⁻¹)`. ∎ `popdeg(f⁻¹) = 2t` on every measured draw (D4). This is the support
form of the repo's Conjecture Λ (`λ_f(hf) ∝` top coefficient of `h`): `R(c) < 2t` implies
the top coefficient vanishes on `H_c`, which implies `λ_f` kills `V_c`.

So **a lower bound on `d_F` is equivalent (on the data) to an upper bound on the reach
`R(c)` of the closure multipliers.**

### 2.2 Measured reach. MEASURED `[pcc:unverified]` (`raw/reach.jsonl`, p = 40961)

| t | R(2) | R(3) | R(4) | dim V₄ | d_F |
|--:|--:|--:|--:|--:|--:|
| 2–5 (draw 0) | 0 | 1 | = 2t (1 ∈ V₄) | — | 4 |
| 6 (draws 0, 1) | 0 | 1 | **10** | 605 | 5 |
| 7 (draw 0) | 0 | 1 | **10** | 829 | ≥5 |

R(4) = 10 at t = 6 on both draws and at t = 7: the reach does not depend on t once it is
below 2t. At t = 6 the multipliers at degree 4 already cover popcounts 0…10 out of 12
(3,455 of 4,096 monomials); only the top two popcount layers are missing.

### 2.3 Reach conjectures and what they would give

- **P2 (stabilisation). CONJECTURE.** `R_t(c) = g(c)` independent of t whenever `g(c) < 2t`.
  Support: R(4) = 10 at t = 6, 7; KM-ART-DEGEN (replacing the wrap `y_{t−1}² = 1` by 0 or 5
  leaves `d_F` unchanged on 65/65 cells), which says the closure does not see the wrap.
- **P3 (linear reach). CONJECTURE.** `g(c) ≤ 8c − 22` for c ≥ 4. Data: g(3) = 1, g(4) = 10,
  and the committed `d_F` steps (5 for t = 6…9, ≥ 6 at t = 10) force g(5) ∈ {18, 19}.
- **Consequence.** P2 + P3 + §2.1 give `d_F(2, t) ≥ (2t + 22)/8 = t/4 + 2.75`, the first
  growing lower bound on `d_F` for any PKM system, matching the measured one-degree-per-four-t
  law. By §1.1 it does not close any live cell.

### 2.4 Where the proof attempt stops

Multiplying a single multiplier by a tower variable `y_{i,l} = z_i^{2^l}` raises the popcount
of its support by at most 1, so reach grows only through multiplication steps, and each step
needs the current product to have degree ≤ c − 1. The number of steps is not bounded by c:
falls come from **carry transport**. Adding `s ∈ {0,1,2}` (the support of `f`) to an exponent
whose low bits end in a run of r ones drops its popcount by r − 1, so high-popcount
multipliers can have low-degree products once the remaining top forms cancel in a linear
combination (D3: `q = y₁₀y₁₁`, the first fall). R(4) = 10 means ten such steps fit under the
degree-3 budget.

A proof of P3 needs an invariant that bounds the **length of cancellation-sustained carry
chains per unit of degree budget**. Two candidate routes, neither carried through:
(a) a potential on multipliers equal to popcount minus a carry-run credit, shown
non-increasing under closure steps at fixed degree budget; (b) the infinite-tower ring
`F_p[ℕ²]` (no wrap) with the same closure, where P2 would be a theorem if every degree-c
closure computation embeds without touching exponents ≥ K. Route (b) is the cheaper next
step: it turns P2 into a finite, t-free computation, after which P3 is a statement about
one fixed infinite closure.

## 3. Ledger

| item | status |
|---|---|
| Cost-model target T_M1 and thresholds σ*(m,w) | DESK-PROOF (arithmetic) |
| Two-block (m = 2) bound cannot close any live cell; slope must be uniform in m | DESK-PROOF |
| Closing margin ≈ 13% (0.125 vs 0.110 at w = 1) | DESK-PROOF from MEASURED slopes |
| Degree→size via IPS is ≈20× short in the exponent | DESK-PROOF, conditional on CITED trade-off |
| Unconditional (size) target is out of reach | assessment |
| Reach criterion `d_F ≥ min{c : R(c) ≥ popdeg f⁻¹}` (m = 2) | DESK-PROOF |
| R(4) = 10 at t = 6 (2 draws), t = 7 | MEASURED |
| P2 stabilisation, P3 linear reach, g(5) ∈ {18,19} | CONJECTURE |
| A growing lower bound on `d_F` for any m | not established |

## 4. Is the track worth continuing?

As a **closure** of the PKM lever: no, not on any near horizon. The closing theorem must be
a nearly tight (within ~13%) linear-in-N degree bound uniform in m, under an unproved cost
axiom; the unconditional size version is an open proof-complexity problem.

As **mathematics**: one cheap, well-posed step is worth taking. Implement route (b), the
no-wrap closure in `F_p[ℕ²]`, compute g(5) and g(6) directly (no t = 10 run needed), and
test P2/P3. If g grows with slope ≈ 8, P3 becomes a concrete combinatorial lemma about carry
chains in one infinite closure, which is the first realistic path to any growing lower bound
on `d_F`.

## 5. Reproduce

```
cd research/degree_slope_two_block_20261124
OMP_NUM_THREADS=1 python3 reach.py 40961 0 2 3 4 5 6     # ~75 s
CMAX=4 python3 reach.py 40961 1 6 ; CMAX=4 python3 reach.py 40961 0 7   # ~30 s
```

## 6. Sources read

`research/prime_leads_chase_20261113/L1-pkm/THEORY.md` (Lemma M, Prop R, corollary);
`research/prime_fall_degree_20261027/DERIVED.md` (D1–D6, D10);
`research/notes/index-calculus/PRIME_FIELD_IC_CLOSURE_20261112.md` §4, §6;
`research/prime_field_ic_closure_20261112/f4-model/README.md` (models M1/M2, Lemma FF);
`research/lfd_lower_bound_20261111/NOTE.md` (Lemmas 1–5, literature table);
`research/kummer_wave2_20261115/KM3-LQR-CLOSURE/{RESULTS.md, hky_closure.py, run_cells.py}`;
`research/kummer_wave2_20261115/KM-POPCODE-STEPS/RESULTS.md`;
`research/pkm_cost_rules_20261115/README.md` (via this session's synthesis).
research-kb and the lead atlas: unavailable in this container.
