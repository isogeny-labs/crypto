# Results: the eliminated target-free S₄ solves one degree above the S₄ target system at every measured cell, and the progression base gives it no dependencies

Scored natively by `dreg_score s4_free runs/cell-*.jsonl`
(`score-output.txt`) against `PREREGISTRATION.md` (commit `7ec7ac760`).
Every value below comes from a committed record in `runs/`. Run report with
graph and PDF: `reports/run-20261018T1853Z/`. *Class:* stage diagnostic; no
`S`, no rho ratio, no speed claim.

**Registered verdict: killed.** None of the 15 measured (cell, basis,
twists) readings is *lower*; all 15 are *higher*. Ten of the readings are on
the progression base. Lead L1 is now closed in both presentations: the
chained one (`research/target_free_relations_20261018/`) and the eliminated
one the lemma is about (this study).

## What was measured

Solving degrees are F4's (field equations included). Every target draw was
unsatisfiable (Gröbner basis `{1}`). Every target-free system has
solutions, the diagonal `x₁ = x₃, x₂ = x₄` among them, and its basis is the
listed size.

| (n, ℓ) | arm | base, twists | solving degree | top-form dependencies | largest step (rows × cols) | wall s | basis size |
|---|---|---|---|---|---|--:|--:|
| (13, 3) | target | gp, plain | 6, 6, 6, 6 | 4 | 430 × 445 | < 0.1 | 1 |
| (13, 3) | target-free | gp, plain | **7** | 1 | 4,994 × 3,778 | 0.4 | 13 |
| (13, 3) | target-free | gp, `[0,1,2,3]` | 8 (target 7) | 0 | 8,130 × 3,854 | 0.8 | 27 |
| (13, 3) | target-free | random, plain | 8, 8, 8, 8 (target 6) | 0 | about 5,400 × 3,765 | 0.4 | 13 |
| (16–19, 4) | target | every base and twist | 7 on all 48 draws | 0 or 1 | 2,896–3,935 × 2,426–3,020 | 0.06–0.10 | 1 |
| (16–19, 4) | target-free | gp, plain | **8** at all four cells | 0 | 80,204–82,665 × 50,312–50,637 | 212–242 | 26–37 |
| (16–19, 4) | target-free | gp, `[0,1,2,3]` | 8 at all four cells | 0 | 94,082–106,402 × 50,528–50,570 | 314–406 | 24–68 |
| (16–19, 4) | target-free | random, plain | 8 on all 16 draws | 0 | 84,062–86,233 × 50,357–50,409 | 188–238 | 26–64 |
| (20, 5) | all three arms | — | **unmeasured**: aborted on allocation under the 12 GB address-space limit within 70 s (exit 134, `.log`), empty records | — | — | — | — |
| (24, 6) | — | — | not run (registered stop condition) | — | — | — | — |

At ℓ = 4 the target-free system costs about twenty times the columns of the
target system, and two to four thousand times the wall time, for a solving
degree one higher.

## Predictions

| prediction | outcome | confidence | evidence |
|---|---|---|---|
| P1: the target-free arm reads higher than the target arm everywhere | **holds**, 15 of 15 | about 85% | `score-output.txt` |
| P2: the gp target-free top deficiency is above the random median at every cell | **fails**: above at (13, 3) only (1 against 0), equal (0 against 0) at the four ℓ = 4 cells | about 80% | same |
| P3: a shape effect inside the target-free arm at two or more cells | **fails**: gp 7 against random 8 at (13, 3), equal (8 and 8) at every ℓ = 4 cell | about 50% | same |
| P4: the twist brings the gp deficiency to the random level or below | **holds** at all five cells (0 everywhere) | about 60% | same |

## What the data say about the lemma (descriptive)

The corollary to Lemma 1 (`research/ic_leads_20261018/LEMMAS.md`) puts the
field-level leading part of the target-free `S₄`, the products
`x₁^{(a)}x₂^{(b)}x₃^{(c)}x₄^{(d)}`, inside a space of dimension
`≤ 4ℓ − 3 < n` for a progression base. The measured top *Boolean* degree of
every target-free system is 7. The F₂-rank of those degree-7 parts is full
(`n` of `n`) at every ℓ = 4 cell on the progression base. Only (13, 3)
shows one dependency. So the dependency the corollary counts does not reach
the Boolean leading forms F4 works with. Boolean degree is not field degree
(a field monomial of degree `d` has Boolean degree up to the binary weight
of its exponents), and the degree-7 part is made of terms the corollary's
count does not constrain. The corollary is still true as stated. The
measurements show that it is the wrong quantity for the solving degree. The
note's statement of L1 should say so. This reading is descriptive: the
preregistration did not register it.

## Deviations

None from the protocol. Two tooling defects were caught before the report
was written, and they changed no record. First, `make_report.sh`, copied
from the first study, called the old `target_free` scorer subcommand; it
now calls `s4_free`. Second, the template's ℓ glyph did not render in the
body font and now goes through Typst math.

## Scope

Cells `(13, 3)` and `(16..19, 4)`, `b = 1`, seed 20261018, a 4-core 15 GB
container that is not isolated, so wall times are practicality notes only.
`(20, 5)` is beyond this host for this engine: 20 Boolean unknowns at
degree 8 or more. Nothing here bears on yield, on `S`, or on curves of
cryptographic size.

## Success criterion

This is a negative result. It is recorded as a valid outcome and is not an
improvement. The index-calculus baseline and its distance to the matched
rho are unchanged.

## Next

The remaining leads in `research/ic_leads_20261018/` are L2, a pure
dense-bit-row XL pass at degree 5 on the `m·ℓ = 21` cells, which would
resolve them exactly if it refutes, and L8, the K-233 derived row. L2 is
the one that could move the measured degree wall.
