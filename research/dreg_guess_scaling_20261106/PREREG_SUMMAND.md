# Pre-registration (Phase 3): fix a whole summand. Does the bilinear residual refute at a constant degree?

Frozen by the commit that adds this file, before any whole-summand run. Class:
**stage diagnostic** (exact algebra). No `S`, no rho ratio, no speed claim.

## Structure (derived from `sym_semaev_s3`, `src/cryptanalysis/koblitz_groebner.rs`)

- `S₃(x₁,x₂,x₃) = (x₁+x₂)²x₃² + x₁x₂x₃ + (x₁x₂)² + b`. Squaring is `F₂`-linear, so the
  descended equations are `F₂`-affine in each block once the other blocks are fixed.
- Fixing the whole of x₁ (k = ℓ guesses) leaves two links:
  - link 1, `S₃(x₁, x₂, e₁)`, bilinear in (x₂, e₁);
  - link 2, `S₃(e₁, x₃, x_R)`, bilinear in (e₁, x₃).
- That is a **bilinear chain** x₂ — e₁ — x₃ with 2n equations in 2ℓ + n unknowns. On the
  registered cells it is overdetermined by n − 2ℓ = ℓ − 2.

## Question

On unsatisfiable draws, at which degree does the MutantXL closure of the whole-summand
residual refute (degree 3 or 4), and does that degree stay constant as ℓ goes from 7 to 9?

- **If it stays constant**, k = ℓ guesses at a constant degree bound k\*. That beats
  `K_marginal` (`3ℓ − 20`) from ℓ ≈ 10 on. Per target it is 2^ℓ branches of a
  polynomial-size closure: the pair table's exponent without its 2^{2ℓ} table.
- **If it grows**, the residual degree rises too, and fixing whole summands does not cap
  k\*.

## Runs

- **Tool:** `dreg_linear_guess --engine pe --to 4 --check`, the same build as
  `PREREGISTRATION.md`.
- **Cells and draws** (committed or registered files):
  - ℓ = 7: `research/dreg_closure_scale_20261016/draws/m3-19-7.jsonl`, unsatisfiable draws
    0, 3, 5 and 7;
  - ℓ = 8: `research/dreg_invariant_guess_20261029/draws/m3-22-8.jsonl`, draws 0, 3, 5
    and 6;
  - ℓ = 9: `research/dreg_guess_scaling_20261106/draws/m3-25-9.jsonl`, the 4 registered
    unsatisfiable draws.
- **Branches:** per draw, 4 values of x₁ fixed with `--fix 0=b0,…,(ℓ−1)=b(ℓ−1)`. The bits
  are `b_i = (h >> i) & 1` for `h ∈ {0, 0x55, 0xAA, 0xFF}` truncated to ℓ bits. Every
  branch of an unsatisfiable draw is unsatisfiable.

## Reading

- Per cell, the multiset of refutation degrees (3, 4, or "not by 4") over 16 branches.
- **"constant"** if every branch at ℓ = 7, 8 and 9 refutes at the same degree D ≤ 4.
- **"grows"** if the modal degree at ℓ = 9 exceeds the one at ℓ = 7.
- **"mixed"** otherwise.

## Prediction (before data)

"Constant at 3", confidence 0.6. The residual is overdetermined by ℓ − 2, and generic
overdetermined bilinear systems have a small degree of regularity
(Faugère–Safey El Din–Spaenlehauer). This is a prior only; it was not computed for this
system.

## Caveat fixed now

A constant degree caps k\* at ℓ only. It does not by itself beat rho, the pair table or
the hybrid optimum. Any cost comparison would be a separate, priced protocol.
