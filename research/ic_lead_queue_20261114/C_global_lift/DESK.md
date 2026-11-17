# Track C: global-lift-height leads over prime fields. Desk analysis (2026-11-14)

> **Review status (main session, 2026-11-14).** An independent red-team review (`REVIEW.md`, pinned to this file's
> sha256 `c52892a0…`; this status block is prepended without changing the text below) reads **"holds with corrections"**:
> - The exponent closure holds only in a narrow setting: fixed relation dimension k, fresh uniform masks per trial, lifts
>   of polynomial size over Q or a number field of bounded degree, relations only among the sampled lifted points, and
>   function fields of bounded Euler characteristic.
> - **It is not new.** The same closure was committed on 2026-11-12 and checker-reviewed
>   (`research/prime_field_ic_closure_20261112/finish/lifting/PROOF.md`; `research/notes/index-calculus/PRIME_FIELD_IC_CLOSURE_20261112.md` §5),
>   using Petsche 2006. The one addition here: Masser 1989 Corollary 1 needs no prime of bad reduction.
> - Unsupported and withdrawn: the extension to number fields of polylogarithmic degree; the 160–700-bit thresholds as
>   stated (they assume a height ratio K); "closed" for p-dependent curves in R2; and the claim that the five leads are
>   closed. The leads stay open on branch (ii) (R2, section-finding cost) and residue R3 (adaptive trials); see `REVIEW.md` M1, M3, M4.
> - Disposition: no atlas lead changes on the strength of this note. Every statement is `[pcc:unverified]`.


**Class.** Desk derivation. No run, no curve, no cost row. This note does not
change any lead disposition or the leaderboard. Every number in it is
`[pcc:unverified]`. No pcc claim covers xedni, lifts or heights
(`docs/session-logs/2026-11-11.md`, the "Lifting to characteristic zero"
entry, ran `pcc.py standing`).

**Labels.**
- `[recorded]`: a file in this repository says so (path given).
- `[Lean, recorded]`: a kernel-checked declaration in
  `research/jksst_xedni_audit_20261101/lean/`. I did not rebuild it here
  (no Mathlib cache on this host, 3.4 GB free).
- `[literature, read]`: I read the statement in the primary source today.
- `[literature, recalled]`: a standard result I did not re-read.
- `[derived]`: my own reasoning. Nobody has reviewed it.
- `[heuristic]`: a dimension count, not a proof.

**Verdict.** The question as posed is: does the per-trial probability that a
lift (or a relation built from lifts) exists decay with an exponent `α < 1/2`
in `p`?

- **Closed in the exponent** for every family the five leads specify. The
  closure composes two declarations already kernel-checked in this
  repository with standard height theorems (§3). It gives `α ≥ 1 − o(1)`.
  - For the function-field branch at bounded Euler characteristic it is
    unconditional.
  - For the `Q` and number-field branch it needs Masser's unconditional
    height lower bound, which I read today, and not Lang's conjecture.
- **The closure is asymptotic only.** Its explicit constant is vacuous below
  roughly 160–700 bits of `q`, or more (§3.4). It therefore says nothing
  quantitative at P-256 size.
- **Two residues stay open.** Neither is a lift-probability exponent of a
  specified family (§5):
  - R1: families whose relation dimension grows with `p`. No lead names one.
  - R2: the cost of *finding* a bounded-height section in the pooled regime,
    `T_section`. This needs a general `F_p` Gröbner engine, which the
    repository does not have.

Per item 4 of the task, `PROTOCOL.md` and the native build are therefore
**not produced**. §6 explains why no experiment is needed for the question as
posed, and what would unblock R2.

---

## 0. Sources checked

- **KB.** No `research-kb` MCP server is available in this session, so I
  used text search over `crypto` @ `0c2cc1b408` and the sibling
  `cryptanalysis` checkout.
- **Per-lead protocols and results.**
  `research/lead_sweep_20261110/LEAD-{EXO-global-lift-height,TRF-global-lift-small-height,ICP-char0-lifting-xedni-smoothness,ICP-xedni-function-field-sections,TRF-function-field-xedni-lift}/{PROTOCOL,RESULT}.md`.
- **Inventory.**
  `/tmp/claude-0/-home-user/354dbe26-6a93-506a-a404-2fc6ef137fdc/scratchpad/LEADS_UNTESTED.md`
  row 6, and `LEADS_UNCATALOGUED.md` rows F15 and F18.
- **Atlas-derived record.**
  `research/prime_field_lead_assessment_20261107/atlas_survey.md` lines 25
  and 91 (item 11). It records:
  - exact `P_lift(p) = (1−2/p+p⁻³)/(2p³)`, `α = 3`, checked on 596,268,874
    slots;
  - "max|coeff|=1 is forced by norm-2/E_8 structure";
  - the viability condition `α < 1/2`.
- **JKSST audit.** `research/jksst_xedni_audit_20261101/{README,PROOF_MAP}.md`
  and the Lean sources `JKSSTSuccessComposition.lean`,
  `JKSSTConditionalHeightCore.lean` and `JKSSTPacking.lean`.
- **Desk notes.**
  - `research/ecdlp_subexp_desk_20261101/README.md`: header, routes table and
    status note, §4 row 6.
  - `research/ecdlp_subexp_ideas_20261110/README.md` §§1–2: the rigidity
    lemma; "multisection = Semaev"; two fibres; Ulmer.
  - `research/prime_ic_leads_20261110/README.md` §4: the equation budget.
  - `research/prime_ic_leads_20261110/jet_exit/README.md`.
  - `RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md` §4.6, "Global lifting
    (xedni-style)".
  - `docs/session-logs/2026-11-11.md`, entries "Lifting to characteristic
    zero" and "Global-lift lead status".
  - `research/lever2_closure_20261031/REPORT.md`, which says nothing about
    lifts or heights.
- **Sibling repository.** `cryptanalysis/research/xedni_lift_optimizer_20261101/`:
  the frozen p=67 comparison is blocked and was not run. Also
  `cryptanalysis/research/xedni_cross_field_20261101/`.
- **`H-XEDN-4a5b50`.** The hypothesis id occurs only inside the truncated
  next-step text of `research/lead_sweep_20261110/leads.json`. No definition
  file exists in `crypto` or in the searched parts of `cryptanalysis`.
- **Research directories with "lift" or "xedni" in the name.**
  `jksst_xedni_audit_20261101` is the one relevant here. The others use
  "lift" in different senses:
  - binary x-to-curve lifting and twist avoidance: `lift_dense_*`,
    `lift_repulsion_mechanism_20261027`, `oakley_lift_dense_20261027`;
  - cofactor lifts on Curve25519: `curve25519_scale_lifts_20261105`;
  - `Z/p²` lifts: `p2_lift_defect_20261101` ("scoped low-degree bridge
    rejected") and `p2_lift_linear_complexity_20261110` ("closed by decision
    rule 3");
  - `ic_liftcheck_audit_20261027`.

  None of these measures a global small-height lift.
- **Code.** `src/cryptanalysis/canonical_lift.rs` and `cm_canonical_lift.rs`
  are **p-adic** (`Z/p^k` arithmetic, formal logarithm, Smart's attack, and
  Hensel lifts on class-number-one CM curves). They contain no global height,
  Mordell–Weil or section machinery. The only F4 code
  (`f4_tower_generic.rs`, `f4_fp_tower`) works over tower quotient rings,
  with at most 4 free variables. There is no general `F_p` Gröbner engine.
- **Literature, read today.** D. W. Masser, *Counting points of small height
  on elliptic curves*, Bull. SMF 117 (1989) 247–265, abstract, Theorem and
  Corollaries 1–2 (Numdam PDF):
  - Corollary 1: `q(P) ≥ 1/(C₁ w D³ (w + log D)²)` for every non-torsion
    `P ∈ E(K)`, where `[K:k] ≤ D`, `C₁` depends only on `[k:Q]`, and
    `w ≥ 1` bounds the absolute log height of `(1 : g₂ : g₃)`.
  - Corollary 2: `|E(K)_tors| ≤ C₂ √w D (w + log D)`.

---

## 1. Each lead's actual claim

Here `E/F_p` is the target curve, `P` generates a prime-order subgroup of
order `q`, and `Q = [d]P`. "Lift" means one of three things: a global curve
`Ẽ` over `K = Q`, a number field, or `F_p(t)` (an elliptic surface `S → P¹`
with `S_{t₀} = E`); global points or sections on it; and reduction or
specialisation, which is a homomorphism `Ẽ(K) → E(F_p)`.

| lead | the claim, stated precisely |
|:--|:--|
| `LEAD-TRF-global-lift-small-height` | Some explicit lift family over `Q` or `K` (xedni; SNFS, cyclotomic or Solinas number fields; rational 2-torsion lifts) puts lifted points of small canonical height on `Ẽ`. A `Z`-dependency among them reduces to a useful relation in `E(F_p)` with per-trial probability `≥ p^{−α}`, `α < 1/2`. The next step is to compose the selected-lift height bound with the minimal-discriminant height bound. |
| `LEAD-ICP-char0-lifting-xedni-smoothness` | Classical xedni: lift `r ≤ 9` points `R_j = [s_j]P + [t_j]Q` to a plane cubic over `Q` (or use Cheon's 2-isogeny prescribed lifts), and certify an exact dependency (the bounded "H-XEDN" search). A second leg asks whether integer lifts of `x` are smoother than random. The toy found them generic (`R = 1.002 ± 0.014`). |
| `LEAD-EXO-global-lift-height` | Height-compressing global lifts over `Q`, number fields and function fields. Two live branches: (i) the `Q`-lift height and coverage obstruction; (ii) `T_section(d, χ, p)`, the cost to find a bounded-height section through a prescribed fibre point on an elliptic surface at the Gaussian-heuristic degree. The rule: `T ≤ p^{0.45}` on ≥ 3 primes is a signal, and `T > p^{1/2}·poly(log p)` closes the branch (`ecdlp_subexp_desk_20261101` §4 row 6). |
| `LEAD-ICP-xedni-function-field-sections`, `LEAD-TRF-function-field-xedni-lift` | Mordell–Weil sections of `y² = x³ + a(t)x + b(t)` over `F_p(t)` as a relation source. The frozen rational-surface family has `α = 3` exactly. Reopen only for a surface family with a "dimension surplus" and a lift exponent `< 1/2` derived before measuring. |

## 2. What was already closed, before today

1. **Single-point `Q`-lift coverage (toy).** Box-height lifts need
   `B₅₀(p) ∝ p^{1.71}` [recorded, EXO RESULT.md; 6 primes ≤ 23, no CI].
   A single lift carries no discrete-log information by itself.
2. **Toy xedni-style `Q` family.** The dependence rate decays like
   `H^{−1.92}` [recorded, TRF RESULT.md]. This is a slope in the height box,
   not in `p`.
3. **Smoothness of integer lifts of `x`.** Generic [recorded, ICP-char0
   RESULT.md]. There is also a structural point [derived]: a factorisation
   of an integer lift is no homomorphism on `E(F_p)`, so a smooth lift by
   itself yields no group relation.
4. **Frozen function-field family.**
   `P_lift(p) = (1−2/p+p⁻³)/(2p³)`, `α = 3` [recorded, atlas survey line
   25]. The linear-section toy reproduces `α = 3` exactly [recorded,
   TRF-function-field RESULT.md].
5. **Multisections and two-fibre transfers on elliptic surfaces.** These are
   closed on paper [recorded, `ecdlp_subexp_ideas_20261110` §§2.1–2.2]:
   - a curve in `|D|` through `d` prescribed fibre points exists iff the
     points sum to `T_D`, which is Semaev's condition;
   - the specialisation kernels at two fibres are unrelated lattices.
6. **Fixed global curve with known generators, detected by enumerating
   global points.** This is generic [recorded,
   `RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md` §4.6]. Reduction is a
   homomorphism, so the reachable set is the image of a lattice ball under
   `n ↦ Σ nᵢ gᵢ` with unknown logarithms: an `r`-dimensional
   bounded-coefficient DLP.
7. **JKSST.** They obtain a `C₀/p` obstruction conditionally (Lang,
   height/discriminant, random image, repeated trials) [recorded, JKSST
   audit]. Two pieces are kernel-checked:
   - `JKSSTConditionalHeightCore.bounded_relation_of_certified_height_ratio`:
     `k` points with heights `≤ M`, separation `≥ √m`, `M/m ≤ K`, and
     `dim + 1 ≤ k` admit a nonzero relation with
     `|nᵢ| ≤ coefficientCutoff = T·⌈1 + 2k√K⌉^{dim}`, where `T` is the
     torsion/fibre bound;
   - `JKSSTSuccessComposition.ordinary_success_bound_of_group_relation_certificates`:
     if every success certifies a group relation with `|nᵢ| ≤ C` among the
     points `[s_j]P + [t_j]Q`, then
     `#success · q ≤ ((2C+1)^k − 1) · q^{2k}`. In words,
     `Pr[success] ≤ ((2C+1)^k − 1)/q` for uniform `(s, t)`.

   The audit lists the missing bridges (README, "Remaining proof
   obligations" 1–8). Lang's conjecture is item 2.

## 3. What closes the exponent question, and how far it reaches

### 3.1 The composition [derived; components Lean-recorded or literature]

Take any non-adaptive lift family. Each trial takes `k` fresh points
`R_j = [s_j]P + [t_j]Q` with `(s_j, t_j)` uniform, builds one global object,
and succeeds only if the lifts of the `R_j` are dependent. Suppose that,
whenever they are dependent, a relation exists with `|n_j| ≤ C(p)`.
Specialisation is a homomorphism, so success implies `Σ n_j R_j = O` in
`E(F_p)` with `|n_j| ≤ C(p)`. The recorded union bound then gives

    Pr[success per trial] ≤ ((2C(p)+1)^k − 1)/q,   so   α ≥ 1 − k·log_q(2C(p)+1).      (★)

The bound holds at every `p`, not only in the limit. The certifier does not
need to find a *bounded* relation: the recorded height core shows that one
exists whenever the lifted points are dependent, provided the height ratio
is controlled. So (★) applies with `C(p) = coefficientCutoff(T, dim, k, K)`
and `K = h/λ₁`. Here `h` is the largest canonical height of the lifted
points and `λ₁` is the least nonzero canonical height on `Ẽ(K)`.

(★) is the general form of JKSST's last step, `(2C₂)^r/p`. It is also
plausibly the "kernel-of-reduction lemma" that the atlas survey (line 25)
expects to "collapse five leads' revisit clauses into one obstruction"
[derived reading of a recorded sentence].

### 3.2 Inputs per branch

**`Q` and number fields, plane-cubic xedni of every variant.**
- The number of lifted points satisfies `k ≤ 9`, since a plane cubic has 10
  coefficients over any field, and `dim ≤ k − 1 ≤ 8` [recorded: JKSST
  §3.2, via `xedni_lift_optimizer_20261101`].
- The lifted points and `Ẽ` have coordinates and coefficients of size
  `p^{O(1)}`, so their Weil heights are `O(log p)`. By the Weil/canonical
  difference bound (Silverman 1990, Math. Comp. 55;
  `|ĥ − h/2| ≤ c·(h(j) + h(Δ)) + c'`) [literature, recalled], we get
  `h = O(log p)`.
- **Lower bound on `λ₁` without Lang.** Masser's Corollary 1 with `k = Q`,
  `D = 1` and `w = O(log p)` gives `λ₁ ≥ 1/(C₁ w³) = Ω((log p)^{−3})`
  [literature, read]. Hence `K = O((log p)⁴)`.
- Torsion satisfies `T ≤ 16` over `Q` (Mazur) [literature, recalled]. Over a
  number field of degree `n`, Masser's Corollaries 1–2 with `D = n` keep `K`
  and `T` polynomial in `n` and `log p`.
- **Result.** `C(p) = (log p)^{O(1)}`, so `k·log_q(2C+1) = O(log log p/log p)`.
  Hence **`α ≥ 1 − o(1)`** whenever the field degree is `n = p^{o(1)}`. This
  covers SNFS, cyclotomic and Solinas fields of polylogarithmic degree.
- This discharges audit obligation 2 (Lang) in the weak form that (★)
  needs. Obligations 1, 3 and 4 are met by construction:
  - (1) a non-minimal model changes the naive height by
    `O(log|Δ_model|) = O(log p)`;
  - (3) Mordell–Weil gives the lattice;
  - (4) `Ẽ ≡ E (mod p)` gives good reduction.

  Obligation 5 (useful denominator) is not needed for an upper bound. Of
  obligation 6, Hasse and the cofactor bound only replace `q` by `p` up to
  the cofactor. Obligation 7 (adaptive trials) is handled by restricting to
  non-adaptive trials (§3.3). Obligation 8 (full row rank) is not used.

**Function fields, elliptic surfaces of Euler characteristic `χ`.**
- Rank: `r ≤ ρ − 2 ≤ b₂ − 2 = 12χ − 4`. This uses Igusa's `ρ ≤ b₂` in every
  characteristic and Shioda–Tate [literature, recalled]. For rational
  elliptic surfaces (`χ = 1`), `r ≤ 8`.
- The height pairing takes values in `(1/L)Z`, where `L` is bounded in terms
  of the reducible-fibre component groups and hence of `χ` (Shioda's height
  formula) [literature, recalled]. So `λ₁ ≥ 1/L` unconditionally, which is
  Lang's conjecture over function fields in a trivial form.
- A section of degree `≤ d` has height `O(d)`. A forced dependency needs
  `k ≤ r + 1` sections.
- **Result.** At bounded `χ` and `d = poly(log p)`, `C(p) = poly(d)` and
  **`α ≥ 1 − o(1)`, unconditionally**. The frozen family's exact `α = 3`
  and the atlas note "max|coeff| = 1 forced by E_8" are consistent with this
  and stronger.

### 3.3 Two corollaries that bear on the leads' wording [derived]

- **"Dimension surplus" cannot help a bounded family.** Suppose a family had
  more free parameters than conditions and bounded `|C|`. Surfaces carrying
  `r + 1` sections through *arbitrary* prescribed points would then exist
  with probability `Θ(1)`, which contradicts (★). So the naive count fails:
  the forced dependency becomes a condition on the `R_j`. A family meeting
  the function-field reopen clause ("surplus with `α < 1/2`") must have
  `k·log₂(2C+1) ≥ ½·log₂ q`. Its relation dimension or its coefficient
  height must grow with `p`.
- **The bounded H-XEDN search is self-closing.** A search that certifies
  dependencies only with `|n_j| ≤ c_max` among `k ≤ 9` points has, by (★)
  with no height input at all, `Pr ≤ ((2c_max+1)^k − 1)/q` and so `α ≥ 1`
  for fixed `c_max`. Running it cannot show `α < 1/2`.
- **What the toys measured.** The `α̂ = 0.89` of the
  ICP-xedni-function-field toy measures collinearity of integer lifts of a
  triple *planted* collinear mod `p`. That is not a relation among uniform
  targets, so it is not covered by (★) and does not contradict it.
  Likewise the `H^{−1.92}` slope is a slope in the height box, not in `p`.

### 3.4 Limit: the constant is vacuous at deployed sizes [derived]

(★) gives a per-trial probability `≤ q^{−1/2}` only when `|C| ≤ q^{1/2}`.

- **With the recorded Lean cutoff** at `k = 9`, `dim = 8`, `T = 16` and the
  most favourable `K = 1`: `coefficientCutoff = 16·19⁸ ≈ 2^{38}`, so
  `|C| ≈ 2^{351}`. The bound needs `log₂ q ≥ ~700`.
- **With a Cramer–Minkowski bound** [derived, not in Lean]:
  `|n_j| ≤ T·γ₈⁴·K⁴ = 256·K⁴`.
  - At `K = 1`, the threshold is `log₂ q ≥ ~160`.
  - At a Lang-like `K ≈ 10`, it is `log₂ q ≥ ~400`.
  - At Masser's `K = O((log p)⁴)`, with `C₁` effective but not explicit in
    the paper, it is far higher.

So the closure fixes the **exponent**, which is what the question asks. It
gives **no** quantitative statement at `p ≈ 2^{256}`. There, JKSST's
conditional constant and the recorded measurements are the only evidence:
- atlas "Xedni V2 found 0 certified dependencies among 20 bounded pairs";
- the frozen function-field family at `α = 3`.

## 4. The decisive quantity and its threshold

**Threshold [derived].** A trial of cost `poly(log p)` that succeeds with [pcc:unverified]
probability `p^{−α}` costs `p^{α}·poly(log p)` per relation. Rho costs
`√(πN/4) ≈ 0.886·p^{1/2}`.
- A one-relation method (xedni) beats rho asymptotically iff `α < 1/2`.
- An index-calculus use needing `U` relations requires
  `α < 1/2 − log_p U`.
- If a trial costs `p^{τ}`, the condition is `α + τ < 1/2`.

**Two regimes.**

1. **Non-adaptive regime** (xedni and every lead-specified family). The
   quantity is the relation-probability exponent
   `α = −lim d log Pr[success]/d log p`, with threshold `1/2`. By (★),
   `α ≥ 1 − k·log_q(2C(p)+1)`, and this is `≥ 1 − o(1)` for every specified
   family. **Closed in the exponent.**

   The measurable proxy that would reopen the regime is a property of a
   *candidate* family, not of a curve:
   `k·log₂(2C(p)+1) ≥ ½·log₂ q`. This means a relation dimension
   `k ≳ √(log p/log log p)` or coefficient heights `≥ p^{Ω(1/k)}`.

2. **Pooled / adaptive regime** (index calculus with a fixed global object; [pcc:unverified]
   `LEAD-EXO-global-lift-height` branch (ii)). Take a fixed object of rank
   `r` with known generators, let `M(h)` be the number of global points or
   sections of height `≤ h`, and let `T_dec(h)` be the cost to decide
   whether a random target is the reduction of one of them. The cost per
   relation is `(r+1)·T_dec(h)·N/M(h)`. To beat rho we need
   `T_dec(h) < M(h)/((r+1)√N) ≤ √M(h)/(r+1)`. The decisive quantity is

       β(h) = log T_dec(h) / log M(h),   threshold β < 1/2 at the h where M(h) ≈ N.

   This is a search cost against the generic baseline. BSGS on the
   Mordell–Weil ball attains exactly `√M(h)`, and that baseline is generic.
   The point-level `α` of this regime, `1 − log_p M(h)`, is not decisive on
   its own: it drops below `1/2` simply by taking `h` large.

## 5. What is still open

- **R1. Growing relation dimension** (open; no instance). Examples would be
  genus-one models of degree `n` in `P^{n−1}`, surfaces with `χ` growing
  like `√(log p)`, or number fields with `n = p^{Ω(1)}`. None of the five
  leads specifies such a family.
  - The heuristic dimension count makes such families worse, not better:
    for `χ ≥ 2`, forcing a dependency costs about `14χ − 3` conditions
    beyond the parameters [heuristic, derived].
  - Reopen condition, sharpened: exhibit a family with
    `k·log₂(2C+1) ≥ ½·log₂ q` and a derived (not measured) relation
    probability `≥ q^{−1/2}`.
- **R2. `T_section` in the pooled regime** (partly closed, rest open).
  - **Curves over `Q`, fixed or `p`-dependent: closed for the lattice-scan
    decider** [derived]. That decider looks for a lift of naive height `≤ X`
    by scanning the lattice `{(u, z) : u ≡ x_R z (mod p)}`, which has about
    `X²/p` points in the box, so its cost is `T_dec ≈ X²/p`. Meanwhile
    `M(X) = O((log X)^{r/2})`. Hence `β ≫ 1/2`. This is not a lower bound
    for every decider: a decider that works through the generators is the
    generic case of §2 item 6.
  - **Rational elliptic surfaces (`r ≤ 8`): closed against Macaulay-type
    solvers** (F4, XL, Gröbner) [derived].
    - `M` grows like the `E_8` ball: about `65·n⁴` sections of height
      `≤ 2n`, so `√M/(r+1) ≈ 0.9·n²`.
    - The section system at `σ·O = m = n − 1` has `6m + 7` unknowns. Its
      Macaulay matrix already has `C(6m+9, 2) ≈ 18·m²` columns at solving
      degree 2, and about `36·m³` at degree 3. Either exceeds `0.9·n²`.
    - Only a hypothetical solver linear in its input could tie, and only at
      the constant level.
  - **Surfaces of rank ≥ 9 (`χ ≥ 2`, planted or Ulmer-type high rank):
    open.** Here `√M ≈ h^{r/4}` outgrows any fixed polynomial. A `T_section`
    polynomial in `h` on Ulmer-type surfaces of rank about `log p` would give
    a polynomial-time ECDLP on those fibres, so the prior is very low
    [derived]. The question has not been measured.
    - Instrument: a general `F_p` Gröbner engine (absent), plus the
      construction of high-rank surfaces with known generators and their
      height pairing (absent).

## 6. Why no experiment is built now (task item 4)

1. **For the question as posed, the exponent is fixed by an inequality valid
   at every `p`.** That inequality is (★) with
   `C(p) = (log p)^{O(1)}` for every specified family. A ladder measurement
   at 12–40 bits can only return rates at or below a bound that is itself
   `≥ 1` there (vacuous). The measured slope would be pre-asymptotic and
   could not change `α`. The decisive object is the composition proof, not
   a curve run.
2. **A confirmatory run would repeat recorded results.** The frozen
   function-field family (`α = 3`, exact, 596M slots), the toy `Q` families
   and JKSST's own experiments already sit on record. The only quantitative
   gap, the vacuous constant at deployed sizes (§3.4), is closed by a
   sharper explicit constant, which is a proof. A 40-bit ladder cannot
   extrapolate to 256 bits in the very regime where constants dominate.
3. **The open residues are not lift-probability exponents of specified
   families.**
   - R1 has no family to measure. Its reopen test is the derived property
     in §5.
   - R2 is a search-cost question whose instrument does not exist here: the
     repository's F4 is tower-specific with at most 4 free variables, and
     there is no Mordell–Weil height machinery. The existing
     `canonical_lift.rs` and `cm_canonical_lift.rs` are p-adic and do not
     apply.

**What would unblock R2.** Per AGENTS.md "Name the obstruction", this [pcc:unverified]
belongs in a `problems/` record. I did not create one, because this task
limits edits to this directory.
- A general `F_p` F4 for systems of 20–60 variables of degree ≤ 3, with a
  semi-regular random-system control.
- A builder for `χ = 2` elliptic K3 surfaces with planted sections, giving
  the height pairing and `M(h)`.
- The measurement is `T_section(h)` at `h = 4, 6, 8` against `√M(h)/(r+1)`
  and against the random control's `d_reg`.
- Outcomes:
  - solving degree tracking the semi-regular one and cost growing faster
    than `h^{r/4}` → closes R2 at reachable scale;
  - a `d_reg` bounded in `h` below the control → the first structured
    signal.
- Estimated effort: several days of engine work. Run cost: CPU-hours to
  CPU-days.

**Lean (AGENTS.md "Mathematical results should include Lean proofs").** (★)
is a new derived composition, and formalising it is feasible in principle.
The output of `bounded_relation_of_certified_height_ratio` supplies the
`relationCertificate` hypothesis of
`ordinary_success_bound_of_group_relation_certificates`. That composition is
not written or built here: there is no Mathlib on this host, and edits are
limited to this directory.

The missing bridges stay non-Lean:
- Masser's Corollary 1;
- the Weil/canonical difference bound;
- Igusa and Shioda–Tate;
- the instantiation of the Mordell–Weil lattice as the `NormedSpace` `E`.

Proof-map entry to add when it is formalised: "(★) ⇐ `JKSSTConditionalHeightCore.bounded_relation_of_certified_height_ratio` + `JKSSTSuccessComposition.ordinary_success_bound_of_group_relation_certificates`; hypotheses: non-adaptive uniform targets, `k ≤ 9` (or `r+1 ≤ 12χ−3`), `K = h/λ₁` supplied by Masser Cor. 1 (number fields) or Shioda denominators (function fields)."

## 7. What this note does not establish

- It gives no quantitative bound at cryptographic sizes (§3.4). [pcc:unverified]
- It does not cover adaptive or pooled index-calculus use (R2), nor families
  with growing relation dimension (R1).
- It is not a reviewed proof. Silverman, Mazur, Igusa, Shioda–Tate and
  Shioda's height formula are recalled, not re-read; Masser was read.
- It does not change any lead disposition. Under the README pursuit
  invariant the leads stay `open_next_step` until a reviewer relabels them.
- It makes no improvement claim and computes no `S` row.
