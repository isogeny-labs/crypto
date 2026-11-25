# Red-team review: Track 1 "drop the degree-slope route" (frozen at de026cd612)

Date 2026-11-25. Reviewer: Red Team subagent (Claude Opus 5.5). Every statement here is
`[pcc:unverified]`; there is no pcc claim id. This review challenges the work. It does not
decide whether the result stands: the Coordinator and Validator do that and record it.

**Evidence reviewed.** Commit `de026cd612` on `claude/adoring-fermi-ap4ukn` (pushed; working
tree clean for these paths): `THEORY_ATTEMPT.md`, `reach.py`, `raw/reach.jsonl`;
`research/solving_degree_tower_frontier_20261124/README.md`; and the cited sources (list at the
end). Nothing was rerun. Desk arithmetic only (snippet in "Reproduce").

**Conclusion under review.** The dispatch's wording is "the degree-slope route cannot close the
PKM/tower lever; drop it as a closure program". The artifact itself says less (§4): "As a
closure of the PKM lever: no, not on any near horizon." No impossibility argument appears anywhere
in the artifact. Every argument it gives is about difficulty or about what the result would be
worth.

## Verdicts

| item | verdict | one-line reason |
|---|---|---|
| **Overall "drop as a closure program"** | **STANDS-WITH-CAVEAT** | As a near-horizon priority call it holds, and F1 strengthens it. The wording "cannot close" is not argued. |
| (1) target `D_PC ≥ σ0·m·t`, `σ0 > σ*(m) = H⁻¹((m−1)/(2m))`, under A_M1 | STANDS-WITH-CAVEAT | The σ* arithmetic reproduces for m ≥ 5. But the target is a refutation-degree statement, while the cost rule's optimum attempt is a satisfiable solve (F1), so even a proof of T_M1 would not close the lever. |
| (2) degree alone gives no cost bound (extension variables; IPS loses a square) | STANDS-WITH-CAVEAT | The qualitative conclusion holds. For a fixed presentation the real obstruction is A_M1, not Lemma 2 (F5). The IPS figure `2^{0.0225N}` is unsupported, because the tower basis is a character basis in which degree and size decouple (F2). |
| (3) an m = 2 two-free-block theorem closes nothing | STANDS-WITH-CAVEAT | Holds for the decisive m ≥ 5. Wrong at m = 4 under the repo's adopted quadratic-LA bar (0.0417 < 1/16). "The cost rule already excludes m ≤ 4" is false. The transfer from m blocks to 2 is cited to lemmas that do not state it (F3). |
| (4) reach criterion; conjecture `d_F(2,t) ≥ t/4 + 2.75` | STANDS-WITH-CAVEAT | The one-way criterion checks out. "Equivalent (on the data)" overstates it. P3 contradicts the artifact's own `g(5) ∈ {18,19}`. Route (b) duplicates chain_sd T5/§13 (F7, F8). |

## Findings, by severity

Each finding is labelled **wrong** (contradicted by the cited evidence), **unsupported** (no
evidence carries it) or **unpriced** (a cost left out).

### F1 [high, wrong]: the target is in the refutation regime; the attacker's optimum is a satisfiable solve

- **Optimum.** `research/pkm_cost_rules_20261115/README.md` gives
  `E(β) = max(lβ, max(0, 1−(m−1)β) + sβ)`. For `s > l` it is minimised at `β = 1/(m−1)`. There
  the attempt exponent is 0 and `μ = B^m/(m!·r) ≈ B/m!` decompositions per target.
- **The repo's own output agrees.** `out/rules.out` ("m=8 finite exact optimisation") reports
  `attempts: 1.0` at log₂r = 64, 128 and 256. At log₂r = 64, N_best = 104, so
  `μ = 2^{104−64}/8! ≈ 2^{24.7}` solutions per attempt.
- **T_M1 says nothing about those instances.** T_M1 bounds `D_PC` "for a positive fraction of
  targets", and §2 treats only inconsistent instances (f a unit). At `β = 1/(m−1)` the fraction of
  inconsistent targets is about `e^{−B/m!}`, and those targets have no refutation.
- **The bar `(m−1)/2` belongs to the satisfiable regime.**
  - In the refutation regime (`β ≤ 1/m`, `μ < 1`), `E ≥ (1+s)/m`. The bar there is
    `s < m/2 − 1`, with `σ*_ref = H⁻¹(1/2 − 1/m) = 0.0724` at m = 8 (equal to rules.out's
    "one-output ML" column).
  - T_M1 pairs the satisfiable-regime threshold with a refutation-regime quantity.
- **No transfer is given from refutation degree to the solving degree of a proper ideal.**
  Restriction maps a degree-D Gröbner basis into the restricted closure. The image need not be a
  Gröbner basis, so deriving 1 from it can need a higher degree (desk sketch, unverified).
- **The same mismatch is in §1.2.** "Closing needs log₂S ≥ (m−1)/(2m)·N" charges a refutation
  size per attempt, but at the optimum no attempt is a refutation.
- **Effect.** The finding strengthens "drop". The closing object is a solving-degree bound for
  proper ideals with up to about `B/m!` roots. The repo has no technique for that: every PC lower
  bound method in the literature table of `lfd_lower_bound_20261111/NOTE.md` §3 is a refutation
  method. But claim (1)'s "exact target" is not the closing target.
- **Cheapest settling check.**
  1. Desk: split T_M1 into two statements.
     - T_ref: `β ≤ 1/m`, `σ0 > H⁻¹(1/2 − 1/m)`.
     - T_sat: proper ideals with `1 ≤ μ ≤ B/m!`, `σ0 > H⁻¹((m−1)/(2m))`.
  2. Local, minutes: run `chainsd` on the proper N8 instance (2 to 1,399 grid zeros at k = 8..12,
     `chain_sd_bounds_20261115/REPORT.md` §8). Measure its solving degree and compare it with LFD
     on the unit instances at the same k.

### F2 [high for the number, medium overall; unsupported]: the IPS trade-off does not apply in the tower basis

- **The quoted form looks right; its use does not.**
  - The form `S ≥ exp(Ω((D−d)²/N))` matches the standard statement of Impagliazzo–Pudlák–Sgall
    1999. It is quoted from memory and was not re-read here. The square loss is reported to be
    tight in the 0/1 basis (Lagarde–Nordström–Sokolov–Swernofsky, ITCS 2020; second-hand).
  - The IPS argument uses the Boolean axioms `x² = x` and restrictions `x := 0` that kill
    monomials.
- **The tower basis is a character basis.** The tower normal-form basis is the group-algebra basis
  of `F_p[(ℤ/2^t)^m]` (chain_sd T1: `Q_k = F_p[(ℤ/N)²]`), in which a product of two monomials is a
  monomial. This is the analogue of the ±1 Fourier basis.
- **In the Fourier basis no degree–size relation holds.**
  - Tseitin formulas need linear PC degree (BGIP 2001, char ≠ 2) but have small Fourier-basis PC
    size.
  - Mouli 2024 (arXiv:2403.03933) shows Boolean-basis and Fourier-basis sizes are incomparable.
    Bonacina–Galesi–Lauria 2022 treat roots-of-unity variables, and Dahiya–Mahajan–Mouli note
    that size–degree methods "rely on terms vanishing when variables are set to zero".
  - These are second-hand, from web-search summaries. arxiv.org and eccc.weizmann.ac.il could not
    be reached from this container (they would have to be allowed in the session's network
    settings), and research-kb is unavailable.
- **What this undercuts.** [pcc:unverified]
  - "Plausible because normal-form monomials are multilinear" uses the wrong criterion.
  - "Certifies at most 2^{0.0225N}" is unsupported: the size a degree bound certifies here may be
    polynomial.
- **The arithmetic itself reproduces.** `σ²/ln2 = 0.02254`; σ needed = 0.5507 (m = 8) and 0.5887
  (m → ∞); the ratio is 19.4–22.2×.
- **Direction.** The finding strengthens claim (2)'s qualitative conclusion. The "≈20× short"
  figure should be withdrawn, not kept as a bound.
- **Cheapest check.** Desk: read Mouli 2024 §1 and Bonacina–Galesi–Lauria 2022, then state
  explicitly that the size–degree trade-off is unavailable in the tower basis.

### F3 [medium, wrong]: claim (3) at m ≤ 4, and the cited transfer

- **The cost rule does not exclude m = 4.** `rules.out` gives the floored thresholds at m = 4:
  s* = 1.0 (quadratic LA; the "correct ML" bar is α* = 0.0417) and s* = 1.5 (linear LA).
  Neither excludes m = 4.
- **Under the repo's adopted bar, the m = 2 theorem closes m = 4.**
  - The two-block slope is 1/16 = 0.0625 > 0.0417. Then `s = 4·H(1/16) = 1.349 ≥ 1`, so
    E ≥ 1/2 under A_M1 with quadratic LA.
  - At m = 4 with quadratic LA, the optimum of the below-parity region [pcc:unverified] is at `β = 1/(5−s) ≤ 1/4`. That is
    the refutation regime (μ < 1), so a refutation-degree theorem does apply there, unlike in F1.
- **Under linear LA (the artifact's bar, (m−1)/2).**
  - m = 4 is not closed (1.349 < 1.5).
  - m = 3 is closed by the artifact's own table (0.0833 > 0.0615; `3·H(1/12) = 1.24 ≥ 1`).
    However, the below-parity region at m = 3 (`β ∈ (1/3, 1/2)`) is satisfiable, so F1 applies there.
- **The decisive cells are unaffected.** For m ≥ 5, `1/(4m) ≤ 0.05 < 0.0794`.
- **The transfer from m blocks to 2 is cited to lemmas that do not state it.**
  - Lemma M (`L1-pkm/THEORY.md`) is monotonicity in t.
  - D6 (`DERIVED.md`) specialises all m blocks to one grid point.
  - Keeping two free blocks needs a partial specialisation with the `m−2` partial sums `u_k` set
    to field values. These are rational iff `R ± ΣC_i` is an F_p-point. It also needs an m = 2
    bound that holds for those derived targets.
  - The argument is plausible but not written.
- **Cheapest check.** Desk:
  - fix the LA charge used in closure statements (linear is the lower bound; quadratic is the
    best known);
  - write the half-page partial-specialisation lemma on the D6 pattern.

### F4 [medium, unsupported]: the "empirical slope ≈ 0.125" column and the "13% margin"

- **Certified slopes in the repo** (rules.py `cert`, closure note §4.2):

  | m | slope | range |
  |---|---|---|
  | 4 | 0.25 | t 1→3 |
  | 5 | 0.20 | t 1→2 |
  | 6 | 0.167 | — |
  | 8 | 0.125 | t 1→2; at t = 1, `|V| = 2 < m`, which closure note §4.3 flags as atypical |

  The m = 8 P1 mixed ladder gives an OLS slope of 0.0 over N 8→11 (rules.out).
- **What is measured long-run.** A long-run slope near 1/8 is measured only at m = 2 and at m = 4
  on P1 (1/8–1/6). Nothing is measured at m ≥ 9, so the "≈0.125" entries at m = 3, 5, 16 and ∞
  are assumptions.
- **The margin depends on m.** At m = 8 it is `0.125/0.0903 = 1.38`, i.e. 38%. The 13% figure is
  the m → ∞ limit and presumes the slope does not depend on m.
- **If the slope falls with m, T_M1 is false.** If the long-run law is "dD/dt constant" (as at
  m = 2) rather than "dD/dN constant", then σ ≈ c/m and T_M1 is false at large m, not merely hard.
  The lever would then be open in the model. "Drop" would still hold, for a different reason, and
  the README's "strongly leaning KILL" would lose its large-m support.
- **Cheapest check.** Relabel the column "assumed". The cheapest long-run point at m > 2 is the
  decisive m = 4, N = 20 cell named in closure note §4.5.

### F5 [medium, overclaim]: T_M1 with A_M1 closes one presentation, not "the lever"

- **What Lemma 2 does and does not do.**
  - `lfd_lower_bound_20261111/NOTE.md` Lemma 2 is written for the Boolean descent over F_2. Its
    transfer to the tower normal-form basis over F_p looks routine, since multiplying by a
    variable maps a monomial to a monomial, but it is not written.
  - The note itself (§1 item 1, §9) says Lemma 2 defeats only bounds stated "for any
    presentation".
  - So Lemma 2 does not weaken T_M1, which is stated for a fixed presentation. The artifact
    correctly notes that A_M1 survives the re-presentation: `M(N', 2) ≈ M(N, D)²`.
- **What follows is a scope limit.** T_M1 covers only the Kummer split-Semaev chain. Each of the
  following needs its own theorem, or a degree–size statement that holds across presentations:
  - the symmetric tower (one degree lower; README);
  - Dickson;
  - isogeny towers;
  - auxiliary-variable presentations.
- **For a fixed presentation, the step from degree to cost fails because of A_M1, not because of
  extension variables.** The dispatch's summary of claim (2) attributes it to the wrong cause.
- **Cheapest check.** Reword T_M1's conclusion to "closes the Kummer split-Semaev presentation in
  the model", and list the presentations the lever admits.

### F6 [medium, unsupported]: A_M1's cited support comes from other presentations

- **The cited support does not cover towers.** §1.2 cites `f4-model` ("msolve writes more matrix
  entries than the XL column count at m ≥ 3"). f4-model measured univariate, interval, graph and
  bits bases, and says of towers "Theorem C territory. M1 adds nothing without hypothesis G". Its
  "M1" also charges `rank Mac_D` at `D ≥ D_ff`, not `M(N, D_PC)^w`.
- **The tower evidence points both ways.**
  - For it (closure algorithm, m = 2): `chain_sd` §13.6 shows 1 enters the closure at 84–99.7% of
    `C(2k, ≤D)` columns for k ≤ 9.
  - Against it: rules.out reads `s_W = 1.90 < 3.5` from msolve widths at m = 8. At m = 6 and 7,
    `s_W` = 4.37 and 5.72, above their bars.
- **How much rests on A_M1.** At slope 0.125, the closure needs an A_M1 exponent w of at least:

  | m | minimum w |
  |---|---|
  | 8 | 0.805 |
  | 16 | 0.862 |
  | → ∞ | 0.920 |

  At slope 0.1 it needs w ≥ 0.933 at m = 8, and w > 1 as m → ∞.
- **Cheapest check.** Desk read of committed logs: compare msolve nnz with `M(N, D)` on the
  existing m = 4..8 tower cells (`pkm_m8_runs_20261115`, `ic_next10_20261014/03-pkm-degree`).

### F7 [medium, stale grounding]: route (b) and "the first growing lower bound"

- **Route (b) already exists in the repo.** The no-wrap ring `F_p[ℕ²]` is covered by
  `chain_sd_bounds_20261115/REPORT.md`:
  - T5: containment, Lean-checked;
  - Observation H-nowrap: equality in 6 cells;
  - §13: Theorems A and A′ in Lean, Missing Lemma Π, and `Π(3) = 6`, `Π(4) ≥ 10`.
- **chain_sd predicts the route fails.** Its §13.4 counting heuristic predicts `Π(w) ~ w·log w`,
  in which case this route cannot give a linear bound.
- **The source was not read.** THEORY_ATTEMPT §6 does not list chain_sd, although SYNTHESIS.md in
  the same commit cites it.
- **"The first growing lower bound on d_F for any PKM system" is false as written.**
  - `DERIVED.md` D5 shows `d_F ≥ 2^t` for the naive PKM presentation.
  - Lane-P asserts `LFD ≥ ⌊(k−10)/14⌋ + 1` (quoted in chain_sd §7.3; not re-derived).
  - It should read "for the tower presentation, by proof".
- **Cheapest check.** Cite chain_sd. Before spending effort on route (b), run the SYNTHESIS
  character-design test with the existing `chainsd` engine.

### F8 [low, internal inconsistency]: P3 and the reach "equivalence"

- **P3 contradicts the artifact's own allowed range.** P3, `g(c) ≤ 8c − 22`, gives `g(5) ≤ 18`,
  but §2.3 allows `g(5) = 19`.
  - The lane-P law `⌊(t+14)/4⌋`, together with the unproved converse of the criterion, gives
    `g(c) ∈ {8c−22, 8c−21}`.
  - The weaker P3′, `g(c) ≤ 8c − 21`, gives `d_F ≥ (2t+21)/8` with the same slope.
- **The upper end of `{18, 19}` uses the converse.** It assumes `R(c) ≥ popdeg ⇒ 1 ∈ V_c`, which
  §2.1 does not show; the criterion runs one way. "Equivalent (on the data)" rests on three
  non-trivial cells, all at c = 4.
- **The one-way criterion checks out.** `1 ∈ V_c` implies that the multiplier `f⁻¹` lies in
  `H_c`, so `supp(f⁻¹) ⊆ ∪ supp(H_c)`. The union of supports does not depend on the basis.
- **It adds something small.** At c = 4, t = 6 it separates 1 from `V_c`, which is the first depth
  where chain_sd §13.8's top-residue invariants fail.
- **Consistency of the conjecture.** `t/4 + 2.75 ≤ ⌊(t+14)/4⌋` holds for t = 2..29 (desk check).
- **Cheapest checks.**
  - P2: compute `R_8(4)` and `R_9(4)` with `reach.py` (minutes).
  - P3: compute `R_10(5)`. Chain_sd already ran the `(10, 5)` closure (rank 18,569, about 1 h on
    4 cores).

### F9 [low, wrong]: "only the top two popcount layers are missing" (§2.2)

- **t = 6, c = 4** (`raw/reach.jsonl`): 641 monomials are missing from the union of multiplier
  supports. 628 of them lie in popcount layers 4–10; in layer 8 alone, 180 of 495 are missing.
- **t = 7:** coverage is 6,378 of 16,384, and 9,991 of the 10,006 missing monomials lie below the
  top two layers.
- **Fix:** correct the sentence.

### F10 [low, unsupported]: d_F for t = 6, draw 1

The §2.2 table gives `d_F = 5` for t = 6, draws 0 and 1. For draw 1, `raw/reach.jsonl` stops at
c = 4 (CMAX=4), so `d_F` was not measured. The instances do differ: `make_instance` seeds on idx,
so the identical histograms across the two draws are a property of the support pattern, not a
duplicated draw.

### F11 [low, overclaim across artifacts]

- **"Cannot close" is not argued.** See the header.
- **SYNTHESIS.md contradicts claim (3)** (same commit `de026cd612`, lines ~121–124). It says that
  if Missing Lemma Π holds, "the degree-slope lower bound closes, and the PKM/tower lever is
  cleanly KILLed". Π is an m = 2 statement, which claim (3) (holds for m ≥ 5) says closes no
  decisive cell.
- **SYNTHESIS.md also overreads chain_sd.** Its line "Π false ⇒ LFD is Ω(k/log k)" goes further
  than chain_sd §13.4, which leaves linearity to reachability.
- **The tower README overclaims.** Its line "degree-slope theorem … closes the lever … permanent"
  leaves out A_M1, the fixed presentation (F5) and the regime (F1).

## Could anything other than a size bound close the lever?

The dispatch asked whether such a route exists, since one would make "drop" too strong.

| route | what it buys | does it avoid the hard part? |
|---|---|---|
| Degree + a proved width or rank-at-entry bound for closure-type algorithms (XL, MutantXL, HKY closure) | Replaces the A_M1 axiom by a theorem for a named class of algorithms. XL's cost is at least its column count by construction. chain_sd §13.6 suggests the bound holds at m = 2. Not considered in THEORY_ATTEMPT. | No. It still needs the degree theorem: uniform in m, satisfiable regime (F1). |
| Hilbert function of the quotient (`dim R/I = μ`) | Cost ≥ μ ≥ 1 (output size), i.e. s ≥ 1 at the optimum. | Gives no degree bound: a Gröbner basis of low degree can have many standard monomials. Far too weak. |
| Counting, dual and cofactor forms (chain_sd §7.3, §13) | Methods for proving the degree bound. | They are not alternatives to it. |
| Unconditional size bound | The only route that would not depend on a model. | Open (non-local system, lfd Lemma 5), and degree cannot reach it in a character basis (F2). |

So every closure route goes through a hard bound that the repo has no technique for. That
supports "drop as a near-horizon closure program". It does not support "cannot close".

## Pricing and scope checks

- **Root extraction is unpriced.** The cost model leaves out root extraction for the `≈ B/m!`
  roots of each attempt. That cost is at least `μ·polylog`, consistent with s ≥ 1; it does not
  change any conclusion here.
- **No band is needed.** The rho reference is the exponent 1/2. Neither artifact ranks measured
  arms, so the §5a band does not apply.
- **The m = 83 rule and ICV1 curve naming are not engaged.** No ECC2K-130 evidence is involved.
- **The m = 2 instances use a random field x_R** (`ns_degree.make_instance`) rather than a
  subgroup-point abscissa. That is acceptable for a degree desk check. They agree with chain_sd's
  admissible family on `dim V_3 = 2t+1` and `dim V_4 = 605, 829`, which supports universality.

## What reproduces

- **σ*(m, 1)** = 0.0794, 0.0842, 0.0876, 0.0903, 0.0924, 0.0941, 0.0999 at m = 5, 6, 7, 8, 9, 10,
  16, matching `rules.out` to 4 decimals. The asymptote is `H⁻¹(1/2) = 0.11003`.
- **σ*(m, 2)** = 0.0350 at m = 8.
- **m = 3 and 4.** The artifact's 0.0615 and 0.0724 are the linear-LA bars. The repo's adopted
  value at m = 4 is 0.0417 (F3).
- **Monomial-count model.** The full-columns variant gives a slightly lower bar (0.0871 at
  m = 8), so the multilinear count is the conservative choice for a closure.

## Settling checks, cheapest first

1. Desk: restate T_M1 per regime (F1). Choose the LA charge (F3). Relabel the slope column (F4).
   Restrict T_M1's scope to one presentation (F5). Fix the text in F8–F10.
2. Desk: read Mouli 2024 and Bonacina–Galesi–Lauria 2022. Drop the IPS figure (F2).
3. Desk read of committed logs: compare msolve nnz with `M(N, D)` on the m = 4..8 tower cells
   (F6).
4. Local, minutes: `reach.py` at t = 8, 9 with c = 4 (P2).
5. Local, minutes: solving degree of proper m = 2 instances against LFD of unit instances at the
   same k (F1).
6. Local, about 1 h: `R_10(5)` (P3, F8).
7. Owner-gated: the m = 4, N = 20 cell (F4).

## Reproduce (desk arithmetic of this review)

```python
from math import log2, log, sqrt
H = lambda x: -x*log2(x) - (1-x)*log2(1-x)
def Hinv(y, lo=1e-15, hi=0.5):
    for _ in range(200):
        mid = (lo + hi) / 2; lo, hi = (mid, hi) if H(mid) < y else (lo, mid)
    return lo
# floored thresholds: lin LA s* = (m-1)/2 (m>=3); quad LA s* = min(m-3, (m-1)/2)
print(Hinv(7/16), Hinv(1/2), Hinv(1/4), Hinv(3/8))  # 0.0903 0.1100 0.0417 0.0724
print(4*H(1/16), 3*H(1/12))                          # 1.349 (>=1 quad m=4), 1.241 (>=1 lin m=3)
print(0.125**2/log(2), sqrt(7/16*log(2)))            # 0.02254, 0.5507
print([round((m-1)/(2*m)/H(0.125), 3) for m in (8, 16, 10**6)])  # w needed: 0.805 0.862 0.920
```

## Sources checked

- **Frozen evidence:**
  - `research/degree_slope_two_block_20261124/{THEORY_ATTEMPT.md, reach.py, raw/reach.jsonl}`;
  - `research/solving_degree_tower_frontier_20261124/README.md`;
  - `research/new_relation_paradigm_20261124/SYNTHESIS.md` (lines 15–30, 85–130).
- **Cited sources:**
  - `research/pkm_cost_rules_20261115/{README.md, PRESET.txt, scripts/rules.py, out/rules.out}`;
  - `research/prime_leads_chase_20261113/L1-pkm/THEORY.md`;
  - `research/notes/index-calculus/PRIME_FIELD_IC_CLOSURE_20261112.md` §3.5, §4.1–4.6;
  - `research/lfd_lower_bound_20261111/NOTE.md` §1, §3, §5 (Lemmas 1–5), §9;
  - `research/chain_sd_bounds_20261115/REPORT.md` (all sections);
  - `research/prime_fall_degree_20261027/DERIVED.md` D4–D6, D8;
  - `research/prime_field_ic_closure_20261112/f4-model/README.md` §0, §4;
  - `research/kummer_m3_ns_degree_20261115/ns_degree.py` (`make_instance`).
- **Not available:**
  - research-kb: `scripts/research-kb` reports no executable;
  - arxiv.org and eccc.weizmann.ac.il: not reachable from this container;
  - the round-7 source behind closure note §4.6, which states only "supporting evidence covers
    m = 2 and 3".
- **Literature, second-hand from web search; to be checked against the papers:**
  - Impagliazzo–Pudlák–Sgall 1999;
  - Lagarde–Nordström–Sokolov–Swernofsky, ITCS 2020;
  - Sokolov, STOC 2020;
  - Bonacina–Galesi–Lauria 2022;
  - Mouli 2024 (arXiv:2403.03933);
  - BGIP 2001.
