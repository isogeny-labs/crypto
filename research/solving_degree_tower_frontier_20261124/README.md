# Solving-degree / tower-presentation frontier: OPEN, strongly leaning KILL

Date 2026-11-24. Desk synthesis, nothing run. All figures `[pcc:unverified]`,
quoted from the named records. research-kb unavailable in-container; grounding by
direct `research/` + `docs/` reads plus a codebase survey agent. Follow-on to the
cost-per-relation negative (PR #5147), which identified the solving degree of the
algebraic presentation as the repo's own remaining open door (tower /
auxiliary-variable presentations escape the Bézout bound `AR:KN-OPEN-020`,
degree ≥ B/3).

## The question and the bar

Can the per-attempt **solving-degree cost exponent `s`** (per-attempt charge
`B^s`, `B = r^β` the base size) stay below the rho-parity bar
**`s < (m−1)/2`** (m≥5; `s < m/2−1` for one output per call) on a **valid
on-curve prime-order** instance, as arity `m` and tower depth `t` grow?
`research/pkm_cost_rules_20261115` derives this bar (the "10-31 rule"); thresholds
`s*`: 1.5 (m=4), 2.0 (m=5), 2.5 (m=6), 3.0 (m=7), 3.5 (m=8).

## Verdict: OPEN, strongly leaning KILL

Neither a proof nor a valid-instrument measurement settles the bar. Every piece of
suggestive evidence leans **against** reaching rho parity, but none closes the question.

### What is measured (the frontier)

- **Solving degree `D` / last-fall degree `d_F` grows as a slow staircase in `t`**
  (`prime_fall_degree_20261027`, `pkm_m8_runs_20261115`, `ic_next10_20261014/03-pkm-degree`):
  e.g. m=8 `D=7` for N=8..11 then `8` at N=12 (σ=dD/dN≈0.2); m=2 `d_F=4→6` over
  t=2..13. First-fall degree is a constant 4 (Kummer) / 5 (isogeny). The symmetric
  tower shifts the staircase down by **one** (`D=4,5,5,6` vs `5,6,6,…`) and shrinks
  the matrix (1,224 vs 12,314 cols at m=3,N=12) but does **not** remove the growth.
  `d_F^Dickson = d_F^Kummer` exactly across all tested cells.
- **No study measures `s` directly.** The KILL values below are *derived* from the
  slope σ via a dense-multilinear **lower-bound** column-count model, not a measured
  solver cost (`pkm_cost_rules_20261115/out/rules.out`):

  | cell | σ | derived s | bar (m−1)/2 | verdict |
  |---|--:|--:|--:|---|
  | m=4, N 4→12 | 0.25 | 3.25 | 1.5 | KILL |
  | m=6, N 6→12 | 0.167 | 3.90 | 2.5 | KILL |
  | m=8, N 8→16 | 0.125 | 4.35 | 3.5 | KILL |

  Resulting IC exponent at m=8: `E_corr = 0.621` vs rho `0.5` (the earlier `0.43`
  was **withdrawn** 2026-11-18; it used the too-generous m−3 closure rule). Parity
  at m=8 would need σ < 0.0903; measured slopes are 0.125–0.2.
- **The only genuine end-to-end cost-exponent fits** (F4 field-ops vs |V|):
  `curve25519_exponent_study_20261108` gives `s≈2.77–2.81` at m=2 (k=20..24), m=3/4
  **censored** (3.5 GB / 1200 s); `pkm_odd_tower_f4_20261114` fits F4 ~ `S^3.1..3.6`
  (superlinear, vs enumeration `S^1.0`). The chase (`prime_pkm_aux_chase_20261119`)
  fits whole-positive-call `γ̂≈3.88` (m=2) → `7.67` (m=3). All above their bars.
- **Whole-solve rho comparison** (the one valid pair-table IC, m=2,
  `pkm_ic_factor_base_20261119`): 114–1,308× rho on 20/24/28-bit prime-order curves.

### Why it is OPEN despite all that

1. **No degree-slope lower-bound theorem.** `PRIME_FIELD_IC_CLOSURE_20261112` §4.4 states that no lower bound on the F4 / polynomial-calculus degree
   that grows with N is established — "the theorem that would close PKM rigorously." A proof is blocked by the
   **one-free-block barrier** (`prime_leads_chase_20261113/L1-pkm/THEORY.md`, Prop R):
   a single free block has `t`-independent degree ≤ max(c₁(m), 2m), so any
   restrict-to-one-block lower bound is `O_m(1)` and cannot show growth in `t`; a
   growing bound needs **two free blocks jointly**, which is unproved. The XL bound
   `Ω(N/(m log N))` is weak and covers XL only.
2. **No valid-instrument measurement.** Every tower solving-degree row is in the
   **refutation regime** with a **uniform-field `x_R`** (not a subgroup-point
   abscissa) and **root extraction unconfirmed** (`prime_one_shot_20261029`;
   `pkm_m8_runs_20261115/PRESET.txt`). The nearest-to-valid studies
   (`curve25519_exponent_study`, `pkm_priced_20261115`) still use random `x_R` and
   confirm no extraction.
3. **The decisive cells are compute-blocked.** m≥8 at N≥13 and m=10 at N≥12 were
   never reached (the m=3,N=21 degree-8 block alone needs ≈317 GB); the reachable
   cells (m≤4) already lean KILL.

## Correction to this session's interim framing

An interim note in this session said the lever was "measured-killed at scale." That
**overstated** it: the KILL values are derived from a lower-bound cost model, the
genuine cost fits stop at m≤4, the decisive m≥8 cells are out of compute reach, and
no measurement is on a valid on-curve instrument. The accurate verdict is **OPEN,
strongly leaning KILL** — the evidence is against reaching rho parity, but it is neither a
proof nor a valid-instrument measurement.

## The two resolution paths and their blockers

| path | what it would settle | blocker | value |
|---|---|---|---|
| **Degree-slope lower-bound theorem** `d_F(m,t) ≥ c·t`, c large enough that the IC exponent α=c/m exceeds the rho cutoff | **closes** the lever (proves it can't reach rho parity) | the one-free-block barrier — needs a *two-free-block* joint lower bound (Prop R), unproved | high; permanent |
| **Corrected on-curve measurement** at m≥8 | would give a valid-instrument data point at the decisive arity | ~317 GB / long wall per cell (EKS, budget-gated); reachable m≤4 only re-confirms lean-KILL | low at reachable scale; decisive scale is compute-blocked |

## Recommendation

The corrected-on-curve measurement at **reachable** scale (m≤4) — the ACTIVE.md
item this note opened — is **low value**: it only re-confirms the lean-KILL the
existing (invalid-instrument) m≤4 rows already show, and the decisive m≥8 cells are
compute-blocked. The **high-value** open problem is the **degree-slope lower-bound
theorem via the two-free-block route** — the repo's own "missing theorem." It is a
hard Lean/algebra problem, not a run.

Strategically, this is the third consecutive IC lever (yield → cost-per-attempt →
solving-degree/tower) to land on the same place: the available levers lean against
reaching rho parity, and the residual open questions are hard theorems or compute-blocked
measurements. Incremental lever-tuning will not close the exponent gap; closing it
needs either a new relation paradigm or the degree-slope lower-bound proof. That is
the honest distance remaining.
