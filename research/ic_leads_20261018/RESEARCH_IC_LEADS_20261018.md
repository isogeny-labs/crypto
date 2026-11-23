# Index-calculus leads after the closure-slope study: what still has content, and what the prior art says

**Date.** 2026-10-18. **Status.** A search round over the repository's
notes, ledger and the two closure studies, plus one pilot computation
(§4) and one probe in flight (§3, L3). **Class.** Analysis; no `S`, no rho
ratio, no claim about a deployed curve. Every "prior art" verdict below
names the file it rests on; the prior-art sweep was done by a read-only
agent and spot-checked. Figures: `figures/lead_map.svg`,
`figures/coset_indicator_pilot.svg`; PDF: `leads.pdf`.

## 0. The filter every lead has to pass

Five closure arguments are now settled enough in this repository that a
lead which fails one of them is not a lead:

1. **Generic-group bound.** Any oracle that decides decomposability by table
   lookup, enumeration or collision is a generic algorithm with advice; the
   Corrigan-Gibbs–Kogan bound `S·T² ≳ r/|Aut|` applies, and Bernstein–Lange
   preprocessing rho wins the online comparison at equal advice
   (`RESEARCH_IC_LEADS_AND_CONNECTIONS_20261010.md` §1–2; measured positive in
   `research/ic_next10_20261014/09-equal-advice/`, 50–500× faster online, and
   `research/20261016-n83-precomputed-rho-control/`).
2. **No Frobenius-stable subspace at `n = 131, 163`.** Stable `F₂`-subspaces
   are binary cyclic codes; their dimensions are subset sums of the
   2-cyclotomic coset sizes mod `n`, and at `n = 131` (and 163) 2 is a
   primitive root (`RESEARCH_KOBLITZ_INDEX_CALCULUS.md` "Which invariant
   factor bases exist at all"; `RESEARCH_ECC2K130_DECOMPOSITION.md` §6).
3. **Shape closure at `m ≥ 3`.** At `m = 2` the only closure effect is the
   Kneser bound `dim V·V ≥ 2l − 1`, worth under one degree; at `m = 3` every
   shape measured equals random (`RESEARCH_FACTOR_BASE_SHAPE_SEARCH.md` §2.3,
   §6, G1 killed 2026-10-15). The reason is not written down anywhere as a
   statement (prior-art Q7): with a generic target `x_R` or a free
   intermediate, the leading forms are coordinates of `x₁x₂u` on
   `V × V × F_{2^n}`, whose span is the whole field for every `V ≠ 0`, so no
   subspace shape can create a dependency among them. L6 below is to write
   that down.
4. **Admissible-degree inequality.** Per decomposition the budget against
   matched rho is about `2^{n(1/2 − 1/m)}` and a degree-`D` Macaulay-type
   solver costs at least `C(N, ≤D)` column touches, so at `n = 131` the
   admissible degree is 3–5 at `w = 1` and 2 at `w = 2.2`
   (`research/dreg_closure_slope_20261018/PREREGISTRATION.md`; stated earlier
   only as a worked table in `research/pdp_root_finder_requirements_20261012/`
   §6.4, "admissible degree at m = 4: 2, perhaps 3; measured: 7").
5. **Measured degree wall.** The fall-aware closure degree is 4 at
   `m·ℓ ≤ 18` and above 4 at `m·ℓ = 21` (16 anchor cells, 8 of 8 draws), and
   degree 5 at 40 unknowns is beyond a 15 GB host with every engine the
   repository has (`research/dreg_closure_slope_20261018/RESULTS.md`).

Figure `figures/lead_map.svg` places each lead against these five.

## 1. Leads with content (ranked)

| # | lead | why it survives the filter | cheapest falsifier | cost | prior art |
|:--|:--|:--|:--|:--|:--|
| **L1** | **Target-free algebraic relations.** Solve `S₄(x₁, x₂, x₃, x₄) = 0` (or the `S₃` chain with no `x_R`) with every variable in `V`, optionally with Frobenius twists `x_i^{2^{a_i}}` so the relation reads `P₁ + σ^a P₂ + σ^b P₃ + σ^c P₄ = O`. | Argument 3 needs a generic multiplier; here there is none: every leading form lives in `V·V·V·V` (or its twists), which for a progression base of dimension `ℓ < n/4` has dimension `≤ 4ℓ − 3 < n`, so `n − 4ℓ + 3` dependencies exist at the lowest degree. Yield is the price: 4-term relations among `2^ℓ` base points number about `n³·2^{4ℓ−n}/24` with twists, so `2^ℓ/n` relations need `ℓ ≳ n/3 − 7`, outside the no-wrap window `ℓ < n/4`. The question is whether a partial-wrap window (`n/4 ≤ ℓ < n/3`) keeps a fraction of the dependencies. | Build the target-free system at `(n, ℓ) ∈ {(16,4), (20,5), (24,6)}` and the chained target system at equal `m·ℓ`; compare closure degrees with the registered instrument. Kill: no cell at least one degree below the target system. | 1 day; small `N = 4ℓ ≤ 24`, degree 4–5 dense closure fits | **closed 2026-10-18, both presentations** (§3a): chained, F4 degree equal to the target system at (16, 4) with about 10× the columns (`research/target_free_relations_20261018/`); eliminated `S₄`, one degree *higher* than the `S₄` target system on 15 of 15 readings at (13, 3) and (16–19, 4), with no top-form dependency on the progression base at ℓ = 4 (`research/target_free_s4_20261018/`, registered verdict killed). Before: never tried (Q1). |
| **L2** | **Dense-bit-row XL at degree 5 for `m·ℓ = 21`.** Plain XL rows at degree 5 are `42 equations × C(40, ≤2) = 34,482` rows over 760 k columns: 3.3 GB as dense bit rows, M4RI elimination in about an hour. The closure degree is `≤` the XL degree, so if XL refutes at 5 the `m·ℓ = 21` cells resolve to exactly 5 (they are already `> 4`). | Aims at argument 5 directly; needs no new mathematics. | Implement a dense-row, sparse-row-count XL refuter (rows as packed `u64` words over the degree-`D` monomial index; M4RI or Four-Russians elimination), run it on the 8 committed `m·ℓ = 21` draws at degree 5. Positive result resolves the cells; negative gives "XL > 5", consistent with the XL fits (`ℓ + 4`), and nothing on the closure. Odds of a positive: about one in four, from the XL-minus-closure gaps of 1–3 in Results 11–12. | 1–2 days | **not built** (Q5): every engine stores sparse rows with fill-in or a dense full matrix. The existing dense engine's scan to degree 5 was tried and killed twice (§3): its closure products, not the XL rows, exceed the host. |
| **L3** | **Raised-cap sparse closure.** The rewritten sparse closure stores 4-byte colex ranks; its 350 M-entry cap used 1.4 GB of 15 GB. | Argument 5 only. | Rerun degree 5 on `m3 (19, 7)` draw 0 with `--max-entries 2500000000`. **Done, unmeasured:** killed by the memory cgroup at 7.2 GB after about two hours (§3). The engine needs more than this host at degree 5, N = 40. | done | this note |
| **L4** | **4E-class stratification (accounting).** For `a = 0` Koblitz curves `E(F_{2^n}) ≅ Z/4 × Z/r`, so points carry a 2-bit class in `E/4E`; EXP-H conditioned on the 1-bit trace class only. | — | **Closed by a sign argument before any run.** A decomposition is `R = Σ ε_i P_i` with free signs; flipping `ε_i` moves the class sum by `2c_i mod 4`, which is 2 whenever `c_i` is odd. So as soon as one summand lies outside `2E`, both residues of the right parity are reachable and the `4E` class never obstructs a target; it can obstruct only bases contained in `2E` (trace-zero bases), and there only if the base has no class-2 point, which a random trace-zero base has. No class-2 artefact exists in the committed draws. | done | not tried; not needed |
| **L5** | **Coset-indicator degree as a predictor of the closure degree.** Hypothesis: the closure must derive the link's solvability parity `Tr(x₁) + Tr(x₂) + Tr(√b/x₁) + Tr(√b/x₂)`, so its degree is bounded below by `deg_ANF(Tr(1/x)|_V)`, which is cheap to compute at any `ℓ`. | — | **Piloted here, §4: negative as a predictor.** `deg Tr(1/x)|_V ∈ {ℓ − 1, ℓ}` on every base (committed, progression, random), far above the measured closure 4; the closure refutes without deriving the parity. The pilot does close one knob: no subspace shape at prime `n` lowers the parity degree, as Mattarei's theorem on inverse-closed subspaces predicts (only subfields). | done | this note |
| **L6** | **Theorem: shape closure at `m ≥ 3`.** Write the two-paragraph argument of §0.3 as a lemma in the FFD defensive programme: for any `F₂`-subspace `V ≠ 0` and any `x_R ∉ {0}`, the `n` coordinate forms of `(x₁, x₂, u) ↦ x₁x₂u` on `V × V × F_{2^n}` are linearly independent; hence the leading forms of the chained system have full rank and no first fall below degree 3 comes from the shape of `V`. | Converts G1's empirical kill into a statement the programme can cite; `RESEARCH_FALL_DEGREE_BOUNDS.md` §2.2 currently says "nothing beyond d_ff ≥ 2 is proved for m ≥ 3". | Proof check: a functional `λ` vanishing on `V·V·F_{2^n} = F_{2^n}` is zero. The eliminated `S₄` form needs the same for `Σ c_e(x_R)·V^{a}V^{b}V^{c}` with generic `x_R`; state it as a conjecture with the exhaustive check at `n ≤ 13` as evidence. | 1 day of writing, tests exist | **not written** (Q7) |
| **L7** | **Lemma: the admissible degree.** State §0.4 as a lemma with its hypotheses (one touch per column, fold `2n`, `m!` attempts, matched rho `√(πr/8n)`), and tabulate it natively for the ladder rungs, `n = 131, 163, 233`. `dreg_score slope` already prints the table. | Gives every later study a fixed target degree per `(n, m)`; the slope study showed its value. | — | half a day | stated only as prose and a worked table (Q6) |
| **L8** | **K-233 as the algebra-friendly standard Koblitz curve.** `ord_233(2) = 29`: `x^233 − 1` has eight irreducible factors of degree 29 over `F₂`, so Frobenius-stable subspaces of dimension `29k` and `29k + 1` exist (58 ≈ n/4 for `m = 4`), the fold `÷ 2n` applies to the algebraic oracle, and the equivariant F4 blocks are eight `F_{2^29}` blocks instead of one `F_{2^130}` block. Admissible degree: 3 for `m = 4` and 4 for `m = 5, 6` under the `C(N,≤D)^{2.2}` cost with `N = m·ℓ` (the model of the first admissible-degree table in this thread); 7–8 for `m ≥ 4` under the `w = 1` one-touch lower bound with the chained `N` (`dreg_score slope` table). Either way the needed closure degree is far above the measured wall at `m·ℓ = 21 ≪ 233`. | Argument 2 does not apply at `n = 233`; the others do; the degree the route may spend is 3–4 (`cols^{2.2}`) to 7–8 (`w = 1`), against a closure degree already above 4 at `m·ℓ = 21 ≪ 233` and growing. So the lead is a *derived row*, not a plan: the honest distance for the one standard curve where Frobenius structure helps the algebra. | Compute and commit the row (derived); no run. | 2 hours | **not tried** (Q4): no K-233 IC analysis, no `ord_233(2)` anywhere |
| **L9** | **G4, stable non-subspace bases**, `F = {x : Tr(c_j x^{2^{e_j}+1}) = 0}`. | Escapes argument 2 (not a subspace). | **Killed by counting before it is built:** at `n = 131` the stable quadratic conditions are `Tr(x^{2^e+1})`, `e ≤ 65`, so a `2^{44}` base needs 87 conditions, at least 22 cubic, and the decomposition system carries `3n = 393` unknowns instead of `3ℓ = 132`; argument 4 then fails by more than for subspaces. Record this in the shape-search note and close G4. | 1 hour | proposed, never built (Q3) |
| **L10** | **Ladder accounting: retire the online-only rows.** Lead A's control ran positive on 2026-10-14 and 2026-10-16 (preprocessing rho 50–500× faster online at equal advice) but `docs/ic/BOUNDARY_TARGETS.md` l.147 and `PLAN_IC_ACCOUNTING_FIXES_20261007.md` l.111 still list it as scheduled. | Bookkeeping that the success criterion requires ("relabelling and accounting are not improvements"). | Update the two documents and the leaderboard's "retained-setup online" rows to carry the matched preprocessing-rho reference. | 2 hours | run (Q2), not propagated |

Leads considered and dropped in this round, with the argument that drops
them: torsion frames for `a = 0` (the Z/4 frame is `x∘(τ − 1)`, the standard
system on `(τ − 1)R`: argument 1 and the exotic-coordinates note);
structured targets (`x_R` of low θ-degree: probability `2^{−n}` to avoid
wrap); explicit parity hints as generators (dense, `2^{ℓ−1}` terms, and
class-0 targets satisfy the parity anyway); quadratised chained systems
(bilinear, but the multihomogeneous regularity bound is still `Θ(N)`);
Frobenius-conjugate relinearisation (FPPR 2012, in the corpus); genus-2 and
cover transfers for prime `n` (`q^{2−2/g} ≥ q > √r`); prime fields (no
subset of `F_p` has membership degree below its size; Coppersmith reach
1/80 against 1/5 needed; `RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md` §2.3).

## 2. What would change the picture

Only three things move the exponent question, and each has a named test:

- the closure degree at `m·ℓ = 21` and beyond, on a 64 GB host or through
  L2/L3 (argument 5);
- ~~a target-free system whose closure degree sits a degree below the
  target system at equal `m·ℓ` (L1)~~: measured on 2026-10-18 in both
  presentations and closed (§3a);
- a proof that the closure degree of the chained system is `Ω(ℓ)` (the
  defensive theorem), for which L6 is the first lemma and §4's pilot the
  first negative mechanism (the parity route is not the one the closure
  takes).

Everything else in the table is a constant, a row, or bookkeeping, and is
listed because the success criterion counts verified constants.

## 3. Probes: L3 landed, L2 test in flight

- **L3 (raised-cap sparse closure): killed, unmeasured.** `dreg_closure --sparse 5 --max-entries 2500000000` on `m3 (19, 7)` draw 0 was killed by the memory cgroup at 7.2 GB RSS after about two hours (92 CPU-minutes), with the dense probe holding 3.5 GB in the same cgroup; no record was written. With 4-byte ranks the registered 350 M cap was 1.4 GB; the fill-in past it outgrows the entry count. Degree 5 at 40 unknowns needs more than about 7 GB for this engine, consistent with F4's 12.6 GB kill. Records: `research/dreg_closure_slope_20261018/probe_highcap/`.
- **L2 test (dense engine scanning to degree 5): killed twice, unmeasured.** `dreg_closure --to 5 --max-cols 800000` on the same draw: run 1 killed by the cgroup at 9.8 GB RSS, run 2 (alone) aborted on allocation under a 13.5 GB address space after 7 minutes. The dense engine's closure holds every fall product as a 760 k-bit row, so the 34,482 XL rows are not the binding count. The pure dense-row XL pass of L2 (no closure) remains unbuilt; it would decide only whether plain XL refutes at 5. Records: `research/dreg_closure_slope_20261018/probe_highcap/`.

## 3a. L1 measured: target-free relations solve no lower, in either presentation

Two preregistered studies, both on 2026-10-18.

- **Chained** (`research/target_free_relations_20261018/RESULTS.md`): the
  target-free system with intermediate `u`, against the chained `m = 3`
  target system on the same base, at (16, 4). No target-free draw finished
  under the registered 1 GiB cap, so the registered verdict is undecided as
  scored. Every draw is bounded at ≥ 4, the target value. Two raised-cap
  draws, a deviation, finish at 4: equal, with about ten times the
  step-matrix columns. The design error is recorded there: Lemma 1 already
  predicts independent leading forms in this presentation.
- **Eliminated `S₄`** (`research/target_free_s4_20261018/RESULTS.md`): the
  presentation the corollary is about. The target-free system reads
  *higher* than the `S₄` target system on 15 of 15 readings at (13, 3) and
  (16–19, 4), on progression, twisted and random bases (8 against 7 at
  ℓ = 4). The progression base shows no dependency among the Boolean top
  forms (degree 7) at ℓ = 4. The corollary's field-level dependencies do
  not reach the forms F4 works with. Registered verdict: killed. (20, 5)
  does not fit in 12 GB.

L1 is closed. The corollary in `LEMMAS.md` stays true as stated, and it now
carries a note that it bounds the wrong quantity for the solving degree.

## 3b. Round of 2026-10-18 evening: L2 unmeasured, L8 computed, a new oracle lead

- **L2, plain dense XL at degree 5** (`research/dreg_xl_degree5_20261018/RESULTS.md`, preregistered):
  unmeasured on all 8 registered draws at m·ℓ = 21. This note's size estimate was wrong:
  the degree-5 matrix has 218,918 × 619,790 (N = 40) and 279,048 × 778,394 (N = 42)
  entries, 17 and 27 GB dense, against 34,482 rows estimated. Every run stopped at the
  6.3 GB pivot cap with about half and about a quarter of the rows inserted. Controls pass.
  Degree 5 at m·ℓ = 21 now needs a 32 GB host or a non-dense rank test (sparse or
  block Wiedemann).
- **L8, K-233 derived row** (`research/k233_derived_row_20261018/`): ord₂₃₃(2) = 29,
  stable dimensions 29a + c. Degree 4 first fits at (m, ℓ) = (6, 30), m·ℓ = 180, with
  N = mℓ and w = 1; never at w = 2.2, where the largest admissible degree is 3. Derived only.
- **New lead, eliminated S₄ as the target oracle** (`research/s4_oracle_cost_20261018/`,
  preregistered): 5.5× and 6.4× fewer counted F4 word XORs than the chained system at
  (12, 3) and (13, 4), every answer verified; no verdict at (16, 5) (about 1.1× on two
  paired draws); (18, 6) unmeasured. A stage improvement at toy sizes; on the
  leaderboard as a component panel. Next: (16, 5) and (18, 6) with a larger cap.
- **Lean:** Lemmas 1 and 2 are machine-checked in core Lean (`lean/`), with three
  bridges open.

## 4. Pilot: the coset-indicator degree (L5), negative

`examples/coset_indicator_degree` computes the ANF of `x ↦ Tr(x)`,
`x ↦ Tr(1/x)` and their sum on `V` by a Möbius transform over `2^ℓ` points
(exact; `x = 0 ↦ 0`), for every committed anchor and fresh base
(`n = 12–30`, `ℓ = 3–10`) and for progression and random bases at `n = 43`,
`ℓ = 4–20`. Records: `coset_indicator_pilot.jsonl` (80 lines); graph
`figures/coset_indicator_pilot.svg`.

| ℓ | committed bases | progression (`n = 43`) | random (`n = 43`) |
|--:|:--|:--|:--|
| 3 | 1 2 2 3 3 | — | — |
| 4 | 3 3 4 4 4 4 4 4 | 4 | 3 3 |
| 6 | 5 5 5 6 6 6 6 6 | 6 | 5 5 |
| 7 | 6 ×5, 7 ×10 | — | — |
| 8 | 7 ×5, 8 ×2 | 7 | 7 8 |
| 10 | 9 9 10 10 | 10 | 9 10 |
| 12–20 | — | 11, 14, 15, 17, 20 | 12, 13–14, 15, 17, 19 |

`deg Tr(x)|_V = 1` throughout. The indicator degree is `ℓ − 1` or `ℓ` on
every base; the measured closure degree is 4 at `ℓ ≤ 6`. So the closure does
not go through the parity, the indicator does not predict the closure
degree, and no subspace shape at prime `n` lowers the parity degree (the
progression is no better than random). Not preregistered; exploratory; it
makes no claim beyond these 80 records.

## 5. Order of work

L3 (killed, unmeasured) → L2 (unmeasured at 6.3 GB, §3b) → L1 (closed, §3a) →
L6, L7 (Lean, §3b) → L10 → L8 (computed, §3b). L4, L5 and L9 are closed by this round (sign argument,
pilot, count). Next: the S₄ oracle at larger ℓ, and L2 on a larger host.
