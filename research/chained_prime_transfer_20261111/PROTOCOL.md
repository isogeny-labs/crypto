# Chained ECDLP transfers over prime fields — frozen protocol (2026-11-11)

Frozen before any measurement. Builds on PR #3877 (toy genus-2 transfer) and
the standing transfer survey. Native Rust only
(`src/cryptanalysis/chained_transfer/`, binary `chained_transfer_bench`);
no Python.

## Objective

Decide, by explicit construction and measurement, whether a *chain* of
same-characteristic algebraic maps carries an elliptic-curve DLP over `F_p`
into a representation where the **complete** discrete-log computation is
cheaper than solving it directly, or exhibit the obstruction.

## Constructions (all `F_p`-rational, `p > 3`)

- **Isogeny hop** `E → E'`: a separable Vélu isogeny from a rational
  `ℓ`-torsion point (`ℓ ∈ {2,3,5,7}`), evaluated by the kernel-sum formula.
- **Bielliptic cover** `C : y² = F(x²)` with its two degree-2 elliptic
  quotients `φ1 : C → E1`, `φ2 : C → E2`, and the induced Jacobian maps
  `φ^*` (pull-back, `E → J(C)`) and `φ_*` (push-forward, `J(C) → E`) on
  **arbitrary** reduced Mumford divisors, including non-split ones (roots
  taken over `F_{p²}`), via the quintic model of `genus2_fp`.
- **Chain**: `E1 → E1'` (isogeny) then `E1 → J(C)` (`φ^*`), each hop verified
  to preserve the order-`r` DLP before use.

## Admissibility (checked in code, per hop)

For each hop `h` and prime `r`: `h(P) ≠ 0`, `ord(h(P)) = r`, and
`h([x]P) = [x]h(P)` for sampled `x` (including `0,1,r−1`); the map is a
homomorphism on sampled pairs. A hop failing any check is rejected, not
trusted. `φ_*φ^* = [2]`, additivity, and the vanishing cross-composite
`φ2_* φ1^* = 0` are checked on random and exhaustive small divisors.

## Measurement (counted group operations, the common unit)

On the order-`r` subgroup of `E1`, with `Q = [x]P`:

1. `rho_src`: counted Pollard-rho on `E1`.
2. `rho_isog`: counted rho after the verified isogeny hop (destination `E1'`,
   same order `r`).
3. `rho_jac`: counted rho on the order-`r` subgroup of `J(C)` after `φ^*`.
4. `ic_jac`: a self-contained single-weight genus-2 index calculus on `J(C)`
   with the **full** degree-1 factor base, every Cantor operation counted;
   linear algebra over `Z/r` counted separately (field multiplications).

The index-calculus output is cross-checked against the known `x` (BSGS, cost
not charged). Group operations are the unit; a Cantor step costs several
elliptic additions, so the field-operation ratio only widens any gap.

## Falsification targets (declared in advance)

- **F1** — transfer gives a rho advantage: some destination's rho group-op
  count is below `rho_src` by more than measurement spread. *Prediction:
  false* (isogenous curves share `#E`; `J(C)` rho on a cyclic order-`r`
  subgroup needs the same `√r` steps). If F1 holds, it is a result.
- **F2** — index calculus beats rho: `ic_jac` group operations below
  `rho_src` at any measured size. *Prediction: false*; genus-2 IC collects
  `Ω(p)` relations, above rho's `√p`.
- **F3** — IC relation-phase exponent: fit `log(ic group ops)` vs `log p`.
  *Prediction:* slope `≈ 1`, and `rho` slope `≈ 0.5`; a measured IC slope
  `< 0.5 + band` would refute the boundary.

## Sizes

`p ∈ {211, 419, 821, 1619}` (dense `Z/r` linear algebra over `≈ p/2`
columns caps the top size). Deterministic coefficient search; fixed seeds.
`#J(C) = #E1 · #E2` from the `(2,2)`-isogeny (no genus-2 point counting, so
`p` is not capped by the `O(p²)` counter).

## Accounting and scope

- Operation **counts** are exact and deterministic: anyone re-running
  `chained_transfer_bench` reproduces them. This is a local development check
  (no wall-clock claim), so the isolation/EKS validation gate for timing
  results does not apply; a timing claim would require it.
- Same characteristic throughout. No cross-characteristic map is attempted or
  claimed. No security claim. Every reported number is `[pcc:unverified]`
  until CI's pcc gate signs the claims.

## Acceptance

A reproducible end-to-end transfer that is **verified** (all hops preserve
the DLP) and **measured** (the four cost columns), with the boundary stated
as the distance to `rho_src`; or, if F1/F2/F3 is met, a reported advantage
with the full accounting. Either way the obstruction is recorded in
`problems/`.
