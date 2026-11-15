# What counts: faster than rho, smaller exponent, subexponential, and transfers

Owner rule, 2026-11-15. This file defines the long-term claims of the
index-calculus program precisely: what "beats rho" means at one size, what an
exponent claim needs, what a subexponential claim needs, which parameter sizes
support which claim, and how a transfer to another group (a hyperelliptic or
other Jacobian, a finite field, an isogenous curve) is accepted. It sits on top
of AGENTS.md "Success criterion" (every verified improvement over the baseline
is a success), "The rule: boundary, table, ratio" (§1 to §8b), "IC measurement
rules", `docs/VALIDATION_GATE.md` and `docs/bounds/README.md`, and changes none
of them. Where this file and those disagree, they win and this file is fixed.

The success criterion does not change: progress is iterative, and every tier
below is reported as the **distance remaining** to the next one, never as a
pass/fail gate on the work.

## 1. The instance and the unit

An instance is `(E / F_q, P, Q)`, named by its ICV1 slug (AGENTS.md §11):

- `q = p^m`; `#E(F_q) = h · r` with `r` prime, `h` the cofactor; `P` of
  order `r`; `Q ∈ ⟨P⟩` a target not seen before the online clock starts.
- **Size** is `log2 r`, written `b`. Field size, extension degree `m`,
  subgroup size and the number of polynomial variables are kept separate
  (§8b): matching bit counts is not matching instances.
- **Cost** is counted in group-addition equivalents on `E` (GAE), every
  phase priced, read across the §5a band (0, 0.1, 1 GAE per unpriced native
  operation) and at the calibrated weight `w_cal`. The headline unit is
  `S = total GAE / √r` (§2).
- **One target**, cold (IC measurement rules): the primary cost covers
  everything from instance to verified `[k]P = Q`, including all
  precomputation. A cost amortized over `t` targets is a separate row that
  names `t` and is never the headline.
- **The rho reference** is the best generic algorithm on the same instance:
  Pollard rho with the negation map and every automorphism of `E` that the
  instance admits (order `γ`), measured on the same host, in the same unit,
  with its unpriced work priced the same way (§5a). Its expected cost is
  about `√(π r / (2γ))` additions; a measured rho is required wherever it can
  be run (section 3 for sizes where it cannot).

## 2. The four claim tiers

Each tier is a different statement, with its own evidence. A result at one
tier is never reported as a higher tier.

### T0. Improvement over the IC baseline

AGENTS.md "Success criterion": lower verified cost than the frozen baseline
on the same instances, at component or pipeline scope, classified by §3 and
written as a §3a cost chain. Reported with its ratio to matched rho as the
distance remaining.

### T1. Faster than rho at a stated size

The IC pipeline costs less than matched rho on the same instance at size `b`.

Required, all of:

1. Cold, one-target, end-to-end cost for both arms, every phase priced, on at
   least 32 independent targets of that instance (or a panel of instances of
   the same family and size), each answer verified `[k]P = Q` and replayed.
2. The ratio `C_IC / C_rho`, with a paired 95% confidence interval, lies
   wholly below 1 at every weight of the §5a band **and** at `w = 0`,
   `w_cal`, `10·w_cal`. Inside the band but not at `w_cal`, or the reverse,
   it is reported as unresolved.
3. The validation gate passes: replay certificate, validator report, matched
   rho with the band, named by the evidence's `VALIDATION.json`.
4. The claim names the size, the family and the instance. It says nothing
   about other sizes.

A T1 result at small size is real and reported as such (the IC positive
control, `research/ic_positive_control_20261112/`, is the precedent: an IC
expected to win at `p = 65,537` had to be shown with `w_cal`).

### T2. Smaller exponent than rho

The pipeline's cost grows more slowly than rho's along a declared family path.
This is still exponential in `b`; it is a statement about the exponent.

Required, all of T1's evidence rules at each size, plus:

1. **A declared family path** before any data: how `q`, `m`, the factor-base
   size and the method's parameters grow with `b` (for example fixed `m`,
   growing `p`; or fixed `p`, growing `m`; or both coupled by a stated rule).
   A fit across sizes that changes the path is not a fit.
2. **Sizes**: at least six sizes on the path, spanning at least 24 bits of
   `b`, the largest at `b ≥ 48`. Fewer sizes or a narrower span give a
   constant-level result only (`docs/bounds/README.md` §2: an exponent needs
   four sizes at minimum; this tier asks for more because it is the claim
   the program is built on).
3. **The fit**: `ln C = ln K + α · ln r` by the bounds fit (`ecbench bound
   fit`), with the 95% interval of `α` wholly below the rho arm's interval
   fitted on the same sizes (rho's declared `α = 1/2`). Every phase is fitted
   separately; the dominant phase at each size is named; no phase is left
   unpriced at any size (§5).
4. **A derived prediction**: the method's own heuristic or proved cost
   (for example `Õ(q^{2-2/n})` for summation-polynomial IC over `F_{q^n}`
   with fixed `n`) is written next to the fit, with its assumptions, and the
   fit is compared with it.
5. **A preregistered held-out size**: after the fit is frozen, predict the
   cost at a size at least 8 bits above the largest fitted size, with an
   interval; then measure it. A measurement outside the predicted interval
   falsifies the exponent claim at that path.
6. **Independent review**: `red-team-breakthrough` and
   `validator-breakthrough` on the frozen evidence before the claim is cited.

### T3. Subexponential

The cost is `L_N[a, c] = exp((c + o(1)) (ln N)^a (ln ln N)^{1-a})` with
`0 < a < 1`, for `N` the group size (or `q^m`, stated), along a family.

No measurement at feasible sizes can establish T3 on its own: over any finite
range, an `L_N[a, c]` curve and an `r^α` curve can both fit. T3 therefore
needs an argument, not only data. Required, all of:

1. **The family and the parametrisation** as functions of `N`: field, degree,
   genus, factor-base size, decomposition degree. Subexponential claims are
   about families (Diem's subexponential result for `E / F_{q^n}` holds when
   `n` and `log q` grow together in a stated way; a fixed-`n` family is not
   subexponential).
2. **The complexity statement** with `a` and `c`: either a proof, or a
   heuristic analysis whose every heuristic is named and stated as a
   hypothesis (for example a degree-of-regularity or first-fall-degree
   assumption, a smoothness-probability assumption, a rank assumption on the
   relation matrix). A heuristic analysis is reported as heuristic, never as
   a theorem.
3. **Each heuristic tested** at every measured size by its own experiment
   (`dreg-triage` for degree-of-regularity forecasts; relation yield against
   the predicted smoothness probability; matrix rank). A heuristic that fails
   at a measured size withdraws the claim at that size and above until it is
   repaired. Known precedent: first-fall-degree arguments for binary curves
   have been disputed in the literature; such an argument needs explicit
   per-size evidence here.
4. **Model selection on data**: at least eight sizes on the path; fit both
   `L_N[a, c]` (with `a` from the analysis) and `K · r^α`; report both, the
   held-out prediction of each at a size at least 8 bits above the largest
   fitted size, and which one the held-out measurement falsifies.
5. **Lean**: the complexity theorem, or its conditional form with the
   heuristics as explicit hypotheses, formalized where feasible
   (`lean-instance-gate`; AGENTS.md "Mathematical results should include
   Lean proofs"). Partial formalization is reported as partial.
6. **Independent review** as in T2.

## 3. Parameter sizes and what each supports

| band | `b = log2 r` | supports | notes |
| --- | --- | --- | --- |
| toy | `b < 32` | T0; debugging; positive and negative controls | never cited as evidence of scaling |
| small | `32 ≤ b < 64` | T0, T1, the lower sizes of T2/T3 fits | rho is measured, not modelled |
| medium | `64 ≤ b < 100` | T1, upper sizes of T2/T3 fits and held-out sizes | rho may be infeasible to run: see below |
| challenge | the exact challenge instance (ECC2K-130: `E_0` over `GF(2^131)`, `#E = 4r`, `b ≈ 129`) | target-specific claims only | needs its own evidence; no transfer from smaller sizes |

- **Fidelity gates still apply.** Binary Koblitz work toward ECC2K-130 passes
  the `m = 83` gate (§8a) before any claim of surviving scale; `m = 31, 51,
  53` are exploratory and their structural differences are disclosed (§8b).
  Prime-field results are labelled as prime-field: no subexponential
  algorithm for general prime-field curves is known, so a claim there gets
  T3 scrutiny even when stated as T2.
- **Where rho cannot be run** (typically `b ≥ 64` within the session's
  budget), the reference is the sealed rho bound for that domain
  (`docs/bounds/FRONTIER.md`), extrapolated at its declared `α = 1/2` with
  the **low end** of its constant's 95% interval, so the comparison errs in
  rho's favour. The IC side is always measured, never extrapolated, for a T1
  claim.
- **Extrapolation** of the IC side is allowed only as a labelled prediction
  with the exponents it rests on (§6), and is the held-out test of T2/T3, not
  a result.

## 4. Transfers

A transfer moves the problem to another group where it is cheaper, solves it
there, and brings the logarithm back. It is a valid route to every tier, and a
transfer that solves faster than rho on `E` counts exactly like a direct IC
win. Examples, each with its known complexity to be checked against the
literature before citing (`research-kb`):

| transfer | applies when | cost on the target group |
| --- | --- | --- |
| Weil descent / GHS to a Jacobian of genus `g` over a subfield | `E` defined over `F_{q^n}` with a suitable cover; depends on the exact field and curve | IC on the Jacobian: `Õ(q^{2-2/g})` for fixed small `g` (double large primes); `L[1/2]`-type for large genus |
| hyperelliptic genus 3 to non-hyperelliptic genus 3 (isogeny) | when the isogeny exists and is computable | `Õ(q)` for plane quartics (Diem) |
| cover or isogeny walk to a weaker curve, then descent | a computable isogeny to a curve the descent applies to | transfer cost plus the weak curve's cost |
| pairing (MOV / Frey–Rück) | small embedding degree `k` | finite-field DLP in `F_{q^k}`: `L_{q^k}[1/3]` |
| anomalous (Smart; Satoh–Araki; Semaev) | `#E(F_p) = p` | polynomial time |

Every transfer claim records and charges:

1. **The map**: its construction (search for the cover, isogeny or pairing
   data), per curve; its evaluation on `P` and `Q`, per target. Both are in
   the cold cost. A construction that is reusable across curves is still
   charged once to the first curve and reported as such.
2. **A correctness certificate**: the map is a group homomorphism on `⟨P⟩`
   that is injective there (the image of `P` has order `r`), checked by
   code independent of the producer; the final answer is verified on `E`
   (`[k]P = Q`), not only on the target group.
3. **The unit conversion**: target-group operations converted to GAE on `E`
   by a measured factor, recorded with its host (§2).
4. **Applicability**: the exact class of curves it applies to, checked on the
   exact instance. A result on a weak subfamily (curves with a suitable
   cover, a small embedding degree, a special order) is a result **for that
   subfamily**, labelled so, and never generalized to the family. For
   ECC2K-130, `m = 131` is prime, so descents that need an intermediate
   subfield do not apply directly; any route must be shown on that exact
   curve (§8b).
5. **The chain** as a §3a cost chain: transfer, solve, return, each line one
   replaced component, the interface (instance, target, unit) unchanged from
   first line to last.

The skills: `ecdlp-surfaces` (surfaces, Jacobian transfers, hyperelliptic
covers, Mumford arithmetic), `curve-algebra`, `isogeny-volcano`, and
`docs/endomorphism-rules.md` for the map's existence, construction, subgroup
action and evaluation costs as separate claims.

## 5. What does not count

AGENTS.md §6 applies in full. In addition:

- An exponent from fewer than six sizes, a span under 24 bits, or a path
  changed between sizes (T2).
- A subexponential claim from data alone, from a solver stage, or with an
  unnamed heuristic (T3).
- A weak-instance or weak-subfamily result stated for the family.
- A transfer whose map construction, evaluation or conversion factor is not
  charged, or whose answer is verified only on the target group.
- A multi-target amortized cost as the headline.
- A comparison with a textbook rho formula where a measured rho could run, or
  with the high end of a rho bound's interval where it could not.
- A held-out prediction written after the held-out measurement.

## 6. How a claim is written

Every row, note or PR that claims T1 to T3 states, in this order:

1. tier; instance(s) and family path; sizes;
2. the cost chain (§3a) with each phase's cost and the dominant phase;
3. the ratio to matched rho with its paired interval at the band and at
   `w_cal` (T1), the fitted `α` with interval next to rho's (T2), or the
   L-model, its heuristics and their per-size tests (T3);
4. the held-out prediction and its outcome (T2, T3);
5. for a transfer, the map, its certificate, the conversion factor and the
   applicability class;
6. the evidence: `VALIDATION.json`, replay certificate, validator report,
   the pcc claim id and standing, the Lean status;
7. the distance remaining to the next tier.

The machine-readable checklist is [`success_criteria.json`](success_criteria.json);
a validator checks a claim against it field by field.

Sources checked: research-kb was not available in the session that wrote this
file. The complexities in section 4 and the literature remarks in T3 are cited
from the standard literature and are marked for confirmation with
`research-kb` before they are quoted in a paper or a result.
