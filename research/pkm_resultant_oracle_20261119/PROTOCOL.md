# EXP-PKM-RESULTANT-001 — a quasi-linear algebraic decomposition oracle at m = 2

Frozen before any row (2026-11-19). Owner's choice of next step after EXP-PKM-DENSE-DEC-001.

## Question

EXP-PKM-DENSE-DEC-001 found that the F4 decomposition oracle with dense membership `∏(x − v)` costs
`S^2.00` counted multiplications at `m = 2` and loses to enumeration over `V` by a factor that grows
like `S` [pcc:unverified]. Is that gap a property of algebraic decomposition, or of F4? This study replaces
F4 by a structured algebraic oracle with quasi-linear cost and measures it against enumeration and against the
F4 dense oracle on the same targets.

## The oracle (exact)

For a target `R = (x_R, y_R)` and factor base `V` (`|V| = S`):

1. `Z(X) = ∏_{v∈V} (X − v)` as a product tree (`isogeny_eval::poly::ProductTree`).
2. `g(X) = ∏_{v∈V} S₃(v, X, x_R)` (each factor a quadratic in `X`, 12 `M` to form), degree `≤ 2S`, by a
   balanced product (`poly::product`).
3. `g(x)` for every `x ∈ V` by the remainder tree (`ProductTree::evaluate`).
4. Every `x₁ ∈ V` with `g(x₁) = 0` is the first x-coordinate of a candidate. Solve `S₃(x₁, X, x_R) = 0`, keep
   roots in `V`, lift both points and keep the sign choices that sum to `R` (`signed_relations`), exactly as
   `enumerate_decompositions` does.

`g(x₁) = 0` iff some factor vanishes, i.e. iff `x₁` has a partner in `V`; step 4 then yields exactly the
enumerated relations. Correctness is checked on every target (V1).

Counted unit: `field_ops` = `M` + NTT word multiplications (`W`, one `M` each, the toolkit's stated conversion)
+ `2·⌈log₂ p⌉` per inversion, plus step 4 counted with the enumeration's own constants (quadratic root
`3 + 4 log p (+ 2 log p + 2)`, lift `6 log p`). Polynomial products use `PolyMul::Auto` (Karatsuba or the
three-prime NTT, whichever counts cheaper per shape). Additions are not counted, as in both baselines.
Only `m = 2`: at `m = 3` the same construction needs a degree-`S²` product and is not expected to help.

## Cost model (prior, stated before data)

Product tree, balanced product and remainder tree are each `O(M(S) log S)`; with NTT products about
`c · S log² S`. Enumeration is linear (≈ 200 `M` per element at 32-bit `p`). So the prior is: far below the F4
dense oracle (`S^2`), still above enumeration, by a factor growing only polylogarithmically.

## Cells

Instrument `examples/pkm_algebraic_oracle.rs --membership resultant` (same curve, `V` and target draws as
EXP-PKM-DENSE-DEC-001; seed 20261114; 4 planted + 2 random targets per cell; `m = 2`; degrees `2,…,2`):

- `P3 = 3221225473`: `t = 8 … 16` (`S = 2^8 … 2^16`), `vkind` coset and random.
- `P1 = 1399680001`: `t = 4 … 10`, coset (shared targets with the committed tower/dense rows).

## Predictions (falsifiable; analysis `analyze.py`)

- **V1 (validity)**: on every target the oracle's relations equal enumeration's (`agree`), and every planted
  target has a relation. Where a cell shares parameters with a committed EXP-PKM-DENSE-DEC-001 row, the
  `target` and `enum_relations` fields are identical. Any failure invalidates the study.
- **R1 (vs F4 dense)**: resultant ÷ dense `< 0.01` (planted medians) at every shared cell with `S ≥ 256`.
- **R2 (vs enumeration)**: resultant ÷ enumeration `> 1` at every `S`.
- **R3 (growth)**: least-squares exponent of resultant `field_ops` in `S` over `P3` coset `S = 2^8 … 2^16` in
  `[1.05, 1.45]`.
- **R4 (polylog gap)**: (resultant ÷ enumeration at `S = 2^16`) ÷ (same at `S = 2^8`) `< 16`, `P3` coset.
- **R5 (V-independence)**: random ÷ coset `V`, resultant, in `(0.8, 1.25)` at every `P3` `S`.

A cell whose wall time passes 2 h on the local host is reported as censored, not imputed.
Nothing here concerns cryptographic sizes or Pollard rho.

## Sources checked

`research-kb` was not reachable in this runtime (`scripts/research-kb`: no executable). Repository search:
`src/cryptanalysis/isogeny_eval/poly.rs` (counted product/remainder trees, reused unchanged);
`research/pkm_dense_decompose_20261115/` (baselines); no prior resultant or multipoint decomposition oracle
for `S₃` found under `src/`, `examples/` or `research/`.
