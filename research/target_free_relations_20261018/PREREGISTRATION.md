# Pre-registration: does a target-free 4-sum system solve at a lower degree than the chained target system on the same base? (lead L1)

Registered 2026-10-18, before any cell below was measured. The lead is
L1 of `research/ic_leads_20261018/RESEARCH_IC_LEADS_20261018.md`; the
argument it escapes is Lemma 1's corollary in
`research/ic_leads_20261018/LEMMAS.md`.

## Why

Every closure study in this repository decomposes a random target `x_R`,
and with a generic multiplier the leading forms of the descended system
are independent for every subspace `V` (Lemma 1). A target-free relation
`P₁ + P₂ = ±(P₃ + P₄)` with all four `x_i ∈ V` has no generic multiplier:
for a progression base of dimension `ℓ`, every leading form lives in a
space of dimension `≤ 4ℓ − 3`, so for `4ℓ − 3 < n` there are at least
`n − 4ℓ + 3` dependencies among them at the lowest degree. If that lowers
the solving degree, relation collection can run target-free, one solve
yielding every relation among `2^ℓ` base points at once.

## Instrument

- **Systems.** `build_target_free_system(basis, b = 1, twists, st)`:
  unknowns `x₁..x₄ ∈ V` (ℓ each) and one intermediate `u ∈ F_{2^n}`
  (n), equations the Weil restrictions of `S₃(x₁^{2^{a₁}}, x₂^{2^{a₂}}, u)`
  and `S₃(x₃^{2^{a₃}}, x₄^{2^{a₄}}, u)` (2n equations, `4ℓ + n` unknowns).
  Control arm: `build_decomposition_system(basis, x_R, b, m = 3, st)` on the
  **same basis** with a random `x_R` (`3ℓ + n` unknowns, `2n` equations):
  the same two links with the fourth `V`-block replaced by a constant.
- **Measure.** The F4 solving degree (`pq_f4_f2::groebner_basis_f4`,
  field equations included, highest step degree that added a basis
  element) and the degree reached, under a wall budget of 1,200 s and the
  default 1 GiB step-matrix cap; `timed_out` or `oversize` is unmeasured.
  The target-free system is always satisfiable (the diagonal
  `x₁ = x₃, x₂ = x₄` is a solution family), so the refutation-degree
  instrument of the closure studies does not apply; the solving degree of
  the full Gröbner basis is the comparable quantity, and it is measured on
  both arms with the same engine.
- **Bases.** Progression `span(1, θ, …, θ^{ℓ−1})` (the arm the lead is
  about) and random subspaces (the arm where no closure is expected).
- **Twists.** `[0,0,0,0]` (plain 4-sum) and `[0,1,2,3]` (Frobenius-twisted,
  no diagonal family on a random base).
- **Draws.** Four per cell and arm, seed `20261018`, bases and targets
  committed under `draws/` as the example prints them, before any F4 runs.

## Cells (frozen)

| (n, ℓ) | target-free unknowns | target m = 3 unknowns | no-wrap? (`4ℓ − 3 < n`) |
|---|--:|--:|---|
| (16, 4) | 32 | 28 | yes (13 < 16) |
| (18, 4) | 34 | 30 | yes |
| (20, 5) | 40 | 35 | yes (17 < 20) |
| (24, 6) | 48 | 42 | yes (21 < 24); likely over the matrix cap at degree ≥ 5 |

Arms per cell: `{gp, random} × {[0,0,0,0], [0,1,2,3]}` target-free, and
`{gp, random}` target `m = 3`. 24 F4 runs per cell at most.

## Verdict rules

Per cell and basis type, with both arms measured on ≥ 3 draws:

- **lower:** the median target-free solving degree is at least one below
  the median target-arm solving degree;
- **equal:** medians equal;
- **higher:** otherwise;
- **unmeasured:** fewer than 3 measured draws on either arm.

Study: **the lead survives** if `gp` reads *lower* at two cells and
`random` does not read lower at the same cells (the effect is the shape,
not the arm); **killed** if no cell reads lower for `gp`; **undecided**
otherwise.

## Predictions

- **P1.** Plain `[0,0,0,0]` on `gp` reads *higher* or *equal* at every
  measured cell: the diagonal solution family adds basis elements and the
  dependencies among leading forms do not reach the solving degree.
  Confidence about 60%.
- **P2.** Twisted `[0,1,2,3]` on `gp` reads *equal* at every measured cell.
  Confidence about 55%.
- **P3.** `random` never reads *lower*. Confidence about 85%.
- **P4 (instrument).** The target arm's F4 solving degree on unsatisfiable
  draws equals the committed dense closure degree (4) at `(16, 4)` and
  `(18, 4)`, as it did on the `(18, 6)` anchor. Confidence about 80%.

## Stop conditions

Budget 1,200 s and the default matrix cap per run; a cell whose target-free
arm is unmeasured on every draw stops the larger cells.

## Scope and class

Stage diagnostic, exploratory lead; no `S`, no rho ratio. Yield is not
measured here: the lead's price (`ℓ ≳ n/3 − 7` relations against
`ℓ < n/4` for no wrap) is noted and left for a later study. No Conductor
task (CLI not installed; hooks fail open).
