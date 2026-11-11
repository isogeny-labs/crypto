# The jet exit against the equation budget: derivation, and the one measurement that decides it (2026-11-11)

**Class.** A derivation plus a pre-registered development check (this host, no isolation). No `S`,
no cost, no leaderboard row. Every measured value, when it exists, is `[pcc:unverified]`.

**Question (goal set 2026-11-11).** The note's §4 filter asks of any new prime-field idea: does it
supply `Θ(N)` bounded-degree cross-summand equations in `Θ(N)` bounded-fibre unknowns without new
`F_p`-valued unknowns? Of the three "bigger ring" exits (`F_{p^k}`, `Z/p^k`, jets), the
extension-field exit is closed by Gaudry's cost and the torsion-section exit is closed *as a
discrete-log channel* (atlas `LEAD-TRF-padic-canonical-section`: the formal logarithm kills the only
homomorphic section; `LEAD-EXO-padic-jet-canonical-lift`: prime-to-`p` jets lift uniquely with zero
formal defect). Neither entry asks the question the filter asks: not whether the lift *linearises the
logarithm*, but whether it supplies *extra low-degree equations for the decomposition system* in
the summands' tower variables. That is what is derived and tested here.

## 1. Dual numbers give nothing (derived)

Over `F_p[ε]/(ε²)` the points of `E` are pairs `(P, v)` with `P ∈ E(F_p)` and `v` in the Lie
algebra `Lie(E) ≅ F_p`, and `E(F_p[ε]) ≅ E(F_p) × (F_p, +)` as groups (the kernel of reduction is the
additive formal group to first order, and `p ∤ r` splits the extension). A relation among lifted
points therefore reads `Σ P_i = R` **and** `Σ v_i = v_R`. The second equation is linear in the `v_i`,
but the `v_i` are free: a lift `P ↦ (P, v(P))` that makes the second equation a *constraint* would
have to be a group homomorphism `E(F_p) → F_p`, and there is none when `p ∤ r` (the only
homomorphic section is `v ≡ 0`, giving `0 = 0`). With `v_i` free the exit adds one equation and `m`
new `F_p`-valued unknowns: filter item 3 fails, ratio `1/(m log₂ p)`.

## 2. `Z/p^k` with the torsion section gives `k − 1` equations in digit functions (derived)

For `p ∤ r` the prime-to-`p` torsion of `E(Z/p^k)` (any lift of `a, b`) maps isomorphically onto
`E(F_p)`, so there is a canonical homomorphic section `s_k: E(F_p) → E(Z/p^k)`, computable as
`s_k(P) = [u]·P̃` with `u ≡ 1 (mod r)`, `u ≡ 0 (mod p^{k−1})` and `P̃` any lift of `P`. Its
`x`-coordinate defines digit functions `x(s_k(P)) = x(P) + p·x₁(P) + … + p^{k−1}·x_{k−1}(P)`,
`x_j: E(F_p) → F_p`, each a *function of `P`* (even in `P`, so a function of `x(P)` on the base).
Because `s_k` is a homomorphism, the relation lifts exactly: `S_{m+1}(x(s_k P_1), …, x(s_k R)) ≡ 0
(mod p^k)` for the summation polynomial of the lifted curve. Expanding `p`-adically gives, per
relation, the mod-`p` equation (digit 0) plus `k − 1` further equations over `F_p`, the first of
which is

    σ₁(x) + Σ_i ∂_i S(x)·x₁(P_i) + ∂_R S(x)·x₁(R) ≡ 0  (mod p),

where `σ₁` is the second digit of the integer `S(x)` and the `∂_i S` have the tower degree of `S`.
So this exit **does** produce extra cross-summand equations without new unknowns, `k − 1` of them,
and `k` can be taken as large as one likes. The filter's remaining item is item 4: at which degree
do they enter the tower quotient ring? That is set by the carry-graded (popcount) degree of the digit
functions `x_j` restricted to the base `V = g·μ_{2^t}`: if `x_j` has multilinear degree `d_j` on `V`,
the `j`-th digit equation has degree about `d_j + deg S`.

**What is known.** `x₁(P)` is the second digit of a scalar multiple `[u]P̃` with `u ≈ p·r`, i.e. a
rational function of degree about `u²` in `x(P)` before reduction. On the `2^t`-point set `V` every
function is a multilinear polynomial in the tower variables of popcount degree at most `t`; a
*random* function on the `≈ 2^{t−1}` abscissae of `V` needs degree about the least `d` with
`Σ_{i ≤ d} C(t, i) ≥ 2^{t−1}`, about `t/2` (derived: 3 at `t = 6`, 4 at `t = 8`, 5 at `t = 10`).
Nothing in the atlas or in the files read says whether the torsion section's digits are generic in
this sense on a Kummer coset.

## 3. The measurement (pre-registered before any row; `examples/jet_digit_degree.rs`)

Instrument: for a prime-order curve over `p = 40961` (`5·2¹³ + 1`), Kummer bases at `t = 6, 8, 10`,
`k = 4` (digits `x₁, x₂, x₃`), compute `s_k(P)` by complete projective formulas over `Z/p^k`
(self-checks: `s_k(P) ≡ P (mod p)`, `[r] s_k(P) = O (mod p^k)`, homomorphism on random pairs), read
the digits on the abscissae `A ⊂ V`, and find the least popcount degree `d` at which the digit vector
lies in the span of `{x^e : popcount(e) ≤ d}` on `A` (Gaussian elimination mod `p`). Controls on the
same `A`: `x` (must read 1), `x³` (must read 2), `x^{2^t − 1}` (must read `t`), three random
vectors (the generic value), and `d_full`, the degree at which every function on `A` is
representable. Three curves per `t`.

**Predictions (stated now).**

- P-J1: the controls read 1, 2, `t`; the random vectors read the generic value (3, 4, 5 at
  `t = 6, 8, 10`), within ±1.
- P-J2: the digits `x₁, x₂, x₃` read the generic value on every curve and every `t`, i.e. they grow
  with `t` like `t/2`. Then each extra equation enters at degree about `t/2 + 2`, above the
  measured last fall degrees (4–7 at these `N`), and the exit fails filter item 4: closed at toy scale,
  with the mechanism named (the section is a scalar multiple by `u ≈ p·r`, whose digits are as
  complex on `V` as a random function).
- Discriminating outcome: a digit with degree bounded in `t` (at most 3 at `t = 10` on all three
  curves, where random reads 5). That would be the first bounded-degree extra equation source over
  `F_p` on record and would reopen the lane: the next step would be the closure (`d_F`) of the tower
  system augmented with the digit equations for `k = 2, 3, 4`.
- Caveat stated now: the digits depend on the lift of `(a, b)` (here the integers in `[0, p)`); a
  structured lift (canonical lift, or `a, b` chosen so that the curve has small-height CM) could
  behave differently and is not tested here.

## 4. Result

(filled from `rows/*.jsonl` after the run)
