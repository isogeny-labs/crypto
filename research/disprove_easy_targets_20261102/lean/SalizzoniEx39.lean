/-!
# Salizzoni Example 3.9 (k = 2) over `F_2`: last fall degree 3 > `d_reg` = 2

Companion to `research/disprove_easy_targets_20261102/` (target B) and the corpus nodes
`CEX-GB-SALIZZONI-EX39` / `CLAIM-GB-LFD-LE-DREG`. Core Lean only; standalone file.

`F = {x^2 + y, y^2 + x, xy}` in `F_2[x, y]`.

* `d_reg = 2` (Bardet-Faugère-Salvy): the top-degree parts are `x^2, y^2, xy`, which are
  all of `R_2`; no generator has degree ≤ 1, so `(F^top)_1 = 0 ≠ R_1`.
* `x, y ∉ V_{F,2}`: every generator has degree 2, so `V_{F,2}` is spanned by the
  generators with constant coefficients (`v2`), and no such combination is `x` or `y`.
* `x, y ∈ V_{F,3}`: `x = (y^2 + x) + y (x^2 + y) + x (xy)` and
  `y = (x^2 + y) + x (y^2 + x) + y (xy)`, every product of degree ≤ 3.

Hence `d_x = d_y = 3`. With the reduced Gröbner basis `{x, y}`, Caminata–Gorla 2021
Theorem 2.9 gives last fall degree `max(d_x, d_y) = 3 > 2 = d_reg`: the inequality
"last fall degree ≤ degree of regularity" fails when the degree of regularity is read in
the BFS sense. (It holds when, as in HKYY, it means a solving degree.) The Theorem 2.9
step and the identification of `V_{F,2}` with `v2` are argued here, not formalised; the
exact linear algebra over `F_2`, `F_3`, `F_5` and `Q` is in `salizzoni_ex39.py`.

Encoding: a polynomial is a mask `p : Nat`, bit `8 a + b` the coefficient of `x^a y^b`;
addition is `xor`, products of degree ≤ 7 are carry-less products (as in
`papers/ic-negative-results/lean/IcNegative/Lemma4Witness.lean`).
-/

namespace SalizzoniEx39

/-- Carry-less product of two masks (bits 0..63 of the left factor). -/
def clmul (p q : Nat) : Nat := go 64 0
where
  go : Nat → Nat → Nat
  | 0, acc => acc
  | n + 1, acc => go n (if p.testBit n then acc ^^^ (q <<< n) else acc)

/-- Total degree of the monomial with bit index `8 a + b`. -/
def monoDeg (i : Nat) : Nat := i / 8 + i % 8

/-- Total degree of a mask (`0` for the zero mask). -/
def deg (p : Nat) : Nat := go 128 0
where
  go : Nat → Nat → Nat
  | 0, m => m
  | n + 1, m => go n (if p.testBit n then max m (monoDeg n) else m)

/-- Top-degree part of a mask. -/
def top (p : Nat) : Nat := go 128 0
where
  go : Nat → Nat → Nat
  | 0, acc => acc
  | n + 1, acc => go n (if p.testBit n && monoDeg n == deg p then acc ^^^ (1 <<< n) else acc)

/-- `x`. -/
def x : Nat := 1 <<< 8
/-- `y`. -/
def y : Nat := 1 <<< 1
/-- `x^2 + y`. -/
def f1 : Nat := (1 <<< 16) ^^^ y
/-- `y^2 + x`. -/
def f2 : Nat := (1 <<< 2) ^^^ x
/-- `xy`. -/
def f3 : Nat := 1 <<< 9

/-- The elements of `V_{F,2}`: the generators (all of degree 2) with constant
coefficients `a, b, c ∈ F_2`. -/
def v2 (a b c : Bool) : Nat :=
  (if a then f1 else 0) ^^^ (if b then f2 else 0) ^^^ (if c then f3 else 0)

/-- Every generator has degree 2, and the top-degree parts are `x^2, y^2, xy`: all three
degree-2 monomials, so `(F^top)_2 = R_2`. -/
theorem tops :
    deg f1 = 2 ∧ deg f2 = 2 ∧ deg f3 = 2 ∧
      top f1 = 1 <<< 16 ∧ top f2 = 1 <<< 2 ∧ top f3 = 1 <<< 9 := by
  decide +kernel

/-- `x, y ∉ V_{F,2}`. -/
theorem not_mem_V2 :
    ∀ a ∈ [false, true], ∀ b ∈ [false, true], ∀ c ∈ [false, true],
      v2 a b c ≠ x ∧ v2 a b c ≠ y := by
  decide +kernel

/-- `x ∈ V_{F,3}`: `x = f2 + y f1 + x f3` with every product of degree ≤ 3. -/
theorem x_mem_V3 :
    f2 ^^^ clmul y f1 ^^^ clmul x f3 = x ∧ deg (clmul y f1) ≤ 3 ∧ deg (clmul x f3) ≤ 3 := by
  decide +kernel

/-- `y ∈ V_{F,3}`: `y = f1 + x f2 + y f3` with every product of degree ≤ 3. -/
theorem y_mem_V3 :
    f1 ^^^ clmul x f2 ^^^ clmul y f3 = y ∧ deg (clmul x f2) ≤ 3 ∧ deg (clmul y f3) ≤ 3 := by
  decide +kernel

end SalizzoniEx39
