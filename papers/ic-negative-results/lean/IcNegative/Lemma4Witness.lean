/-!
# Nagao ePrint 2015/984, Lemma 4: witness A, kernel-checked

Companion to `papers/ic-negative-results/`, statement S5, and to open target O3 of
`lean/OPEN_TARGETS.md`.  Nagao's Lemma 4 asserts `d_F ≤ d'_F`: the first fall degree
of a system over `F_2` is at most its *fake* first fall degree, the one computed after
reduction modulo the field equations `X_j^2 + X_j`.  An estimate of the form
"the fake degree is small, so by Lemma 4 the true first fall degree is small" rests on
that inequality.  Witness A, `f_1 = X^2 Y`, `f_2 = X Y + X` in `F_2[X, Y]`, has
`d'_F = 2` and `d_F = 3`, with the field equations outside or inside the generating
set, so the inequality fails.

Definitions (Nagao Definitions 5 and 6, *equality reading*, as in Definition 1 of the
paper `ic_negative_results.tex`): a fall at degree `d` is a tuple `(g_i)` with
`max_i deg(g_i f_i) = d`, `deg(Σ g_i f_i) < d` and `Σ g_i f_i ≠ 0`; `d_F` is the least
such `d`.  `d'_F` is defined the same way with every degree taken after reduction
modulo the field equations.

## Encodings (the modelling steps, stated so they can be audited)

* `F_2[X, Y]` up to degree 7.  A polynomial is a mask `p : Nat`; bit `8 a + b` is the
  coefficient of `X^a Y^b` (`a, b < 8`).  Addition is `xor`.  Because
  `(8 a₁ + b₁) + (8 a₂ + b₂) = 8 (a₁ + a₂) + (b₁ + b₂)`, the product of two
  polynomials whose product has degree at most 7 is the carry-less product of their
  masks (`clmul`).  Every product formed below has degree at most 3.
* Multipliers.  At degree `d` the admissible `g_i` are `0` and the polynomials with
  `deg g_i + deg f_i ≤ d`.  For `k ≤ 7` every polynomial of degree `≤ k` has a mask
  below `2 ^ (8 k + 1)` (its largest monomial index is at most `8 k`), and
  `polysUpTo k` is exactly the masks below that bound whose degree is at most `k`.
  So `tuples fs d` lists every admissible tuple.  In `F_2[X, Y]`, a domain,
  `deg (g f) = deg g + deg f`, which is why the bound is on `deg g_i`.
* The quotient `F_2[X, Y] / (X^2 + X, Y^2 + Y)`.  Its basis is the four multilinear
  monomials, indexed by subsets `e ⊆ {X, Y}` (bit 0 of `e` is `X`, bit 1 is `Y`); a
  polynomial is a 4-bit mask over them and the product of two monomials is their
  union.  Reduction is a ring homomorphism, so `red (g f) = red g · red f`; every
  multiplier therefore acts through one of the 16 elements of the quotient, and
  `qtuples` ranges over all of them.  The field equations themselves reduce to `0`
  (`red_feX`, `red_feY`), so their multiples are `0` (`qmul_zero_right`) and add
  nothing to the maximum or the sum; the fake system is checked on `[red f_1, red f_2]`.

What is *not* formalised: the bridge lemmas named in the two encoding bullets (mask
completeness and the ring-homomorphism property of `red`) are argued here, not proved
in Lean; witness B (three variables) and the census are checked natively only
(`research/blindspots_20261018/instruments/blindspots_19_checks.rs`).  Nothing here
bears on Nagao's Proposition 5 or on any complexity estimate built on it.
-/

namespace IcNegative.Lemma4

/-! ## `F_2[X, Y]` as masks -/

/-- Carry-less product of two masks (bits 0..63 of the left factor). -/
def clmul (p q : Nat) : Nat := go 64 0
where
  go : Nat → Nat → Nat
  | 0, acc => acc
  | n + 1, acc => go n (if p.testBit n then acc ^^^ (q <<< n) else acc)

/-- Total degree of the monomial with bit index `i = 8 a + b`. -/
def monoDeg (i : Nat) : Nat := i / 8 + i % 8

/-- Total degree of a mask (`0` for the zero mask; every use below that needs a
genuine degree also checks that the polynomial is nonzero). -/
def deg (p : Nat) : Nat := go 128 0
where
  go : Nat → Nat → Nat
  | 0, m => m
  | n + 1, m => go n (if p.testBit n then max m (monoDeg n) else m)

/-- The products `g_i f_i`. -/
def products (fs gs : List Nat) : List Nat := List.zipWith clmul gs fs

/-- `Σ_i p_i` over `F_2`. -/
def xorSum (ps : List Nat) : Nat := ps.foldl (· ^^^ ·) 0

/-- `max_i deg p_i` over the nonzero `p_i` (`0` if all are zero). -/
def maxDeg (ps : List Nat) : Nat :=
  ps.foldl (fun m p => if p = 0 then m else max m (deg p)) 0

/-- Nagao Definition 5, equality reading: `(g_i)` is a fall of `(f_i)` at degree `d`. -/
abbrev IsFall (fs gs : List Nat) (d : Nat) : Prop :=
  gs.length = fs.length ∧ maxDeg (products fs gs) = d ∧
    xorSum (products fs gs) ≠ 0 ∧ deg (xorSum (products fs gs)) < d

/-- Every polynomial of degree at most `k` (`k ≤ 7`), as a mask. -/
def polysUpTo (k : Nat) : List Nat :=
  (List.range (2 ^ (8 * k + 1))).filter fun g => g == 0 || decide (deg g ≤ k)

/-- Admissible multipliers of `f` at degree `d`. -/
def multipliers (f d : Nat) : List Nat :=
  if deg f ≤ d then polysUpTo (d - deg f) else [0]

/-- Every admissible multiplier tuple at degree `d`. -/
def tuples : List Nat → Nat → List (List Nat)
  | [], _ => [[]]
  | f :: fs, d => (multipliers f d).flatMap fun g => (tuples fs d).map (g :: ·)

/-! ## The quotient by the field equations -/

/-- Degree of the multilinear monomial `e ⊆ {X, Y}`. -/
def qMonoDeg (e : Nat) : Nat := e % 2 + e / 2 % 2

/-- Product in `F_2[X, Y] / (X^2 + X, Y^2 + Y)` on 4-bit masks. -/
def qmul (p q : Nat) : Nat :=
  (List.range 4).foldl (fun acc i =>
    if p.testBit i then
      (List.range 4).foldl (fun acc' j =>
        if q.testBit j then acc' ^^^ (1 <<< (i ||| j)) else acc') acc
    else acc) 0

/-- Degree of a 4-bit mask in the quotient (`0` for the zero mask). -/
def qdeg (p : Nat) : Nat :=
  (List.range 4).foldl (fun m i => if p.testBit i then max m (qMonoDeg i) else m) 0

/-- Reduction of a mask of `F_2[X, Y]` modulo the field equations:
`X^a Y^b ↦ X^min(a,1) Y^min(b,1)`. -/
def red (p : Nat) : Nat :=
  (List.range 64).foldl (fun acc i =>
    if p.testBit i then acc ^^^ (1 <<< (min (i / 8) 1 + 2 * min (i % 8) 1)) else acc) 0

def qproducts (fs gs : List Nat) : List Nat := List.zipWith qmul gs fs

def qmaxDeg (ps : List Nat) : Nat :=
  ps.foldl (fun m p => if p = 0 then m else max m (qdeg p)) 0

/-- Nagao Definition 6, equality reading: a fall at degree `d` after reduction. -/
abbrev IsFakeFall (fs gs : List Nat) (d : Nat) : Prop :=
  gs.length = fs.length ∧ qmaxDeg (qproducts fs gs) = d ∧
    xorSum (qproducts fs gs) ≠ 0 ∧ qdeg (xorSum (qproducts fs gs)) < d

/-- Every multiplier tuple over the 16 elements of the quotient. -/
def qtuples : List Nat → List (List Nat)
  | [] => [[]]
  | _ :: fs => (List.range 16).flatMap fun g => (qtuples fs).map (g :: ·)

/-! ## Witness A -/

/-- `X^2 Y`. -/
def fA1 : Nat := 1 <<< 17
/-- `X Y + X`. -/
def fA2 : Nat := (1 <<< 9) ^^^ (1 <<< 8)
/-- `X^2 + X`. -/
def feX : Nat := (1 <<< 16) ^^^ (1 <<< 8)
/-- `Y^2 + Y`. -/
def feY : Nat := (1 <<< 2) ^^^ (1 <<< 1)

/-- Generating set with the field equations outside. -/
def outsideA : List Nat := [fA1, fA2]
/-- Generating set with the field equations inside. -/
def insideA : List Nat := [fA1, fA2, feX, feY]
/-- The reduced system `[red f_1, red f_2] = [X Y, X Y + X]`. -/
def fakeA : List Nat := [red fA1, red fA2]

theorem deg_fA1 : deg fA1 = 3 := by decide +kernel
theorem deg_fA2 : deg fA2 = 2 := by decide +kernel

/-- `red (X^2 Y) = X Y` and `red (X Y + X) = X Y + X` in the quotient basis. -/
theorem red_fA1 : red fA1 = 1 <<< 3 := by decide +kernel
theorem red_fA2 : red fA2 = (1 <<< 3) ^^^ (1 <<< 1) := by decide +kernel
/-- The field equations reduce to zero. -/
theorem red_feX : red feX = 0 := by decide +kernel
theorem red_feY : red feY = 0 := by decide +kernel
/-- Multiples of a zero generator are zero, for every element of the quotient. -/
theorem qmul_zero_right : ∀ g ∈ List.range 16, qmul g 0 = 0 := by decide +kernel

/-- True first fall degree, outside placement: a fall at 3 (`g = (1, X)`, sum `X^2`). -/
theorem outside_fall3 : IsFall outsideA [1, 1 <<< 8] 3 := by decide +kernel
/-- True first fall degree, outside placement: no fall at 0, 1 or 2. -/
theorem outside_noFall_below3 :
    ∀ d ∈ [0, 1, 2], ∀ gs ∈ tuples outsideA d, ¬ IsFall outsideA gs d := by
  decide +kernel

/-- True first fall degree, inside placement: the same fall at 3. -/
theorem inside_fall3 : IsFall insideA [1, 1 <<< 8, 0, 0] 3 := by decide +kernel
/-- True first fall degree, inside placement: no fall at 0, 1 or 2. -/
theorem inside_noFall_below3 :
    ∀ d ∈ [0, 1, 2], ∀ gs ∈ tuples insideA d, ¬ IsFall insideA gs d := by
  decide +kernel

/-- Fake first fall degree: a fall at 2 (`g = (1, 1)`, reduced sum `X`). -/
theorem fake_fall2 : IsFakeFall fakeA [1, 1] 2 := by decide +kernel
/-- Fake first fall degree: no fall at 0 or 1 over the whole quotient. -/
theorem fake_noFall_below2 :
    ∀ d ∈ [0, 1], ∀ gs ∈ qtuples fakeA, ¬ IsFakeFall fakeA gs d := by
  decide +kernel

/-- Witness A refutes Lemma 4 (`d_F ≤ d'_F`) in both placements: `d'_F = 2`, while
`d_F = 3` with the field equations outside and inside the generating set. -/
theorem lemma4_fails_witnessA :
    ((∃ gs, IsFakeFall fakeA gs 2) ∧
      ∀ d ∈ [0, 1], ∀ gs ∈ qtuples fakeA, ¬ IsFakeFall fakeA gs d) ∧
    ((∃ gs, IsFall outsideA gs 3) ∧
      ∀ d ∈ [0, 1, 2], ∀ gs ∈ tuples outsideA d, ¬ IsFall outsideA gs d) ∧
    ((∃ gs, IsFall insideA gs 3) ∧
      ∀ d ∈ [0, 1, 2], ∀ gs ∈ tuples insideA d, ¬ IsFall insideA gs d) :=
  ⟨⟨⟨_, fake_fall2⟩, fake_noFall_below2⟩,
   ⟨⟨_, outside_fall3⟩, outside_noFall_below3⟩,
   ⟨⟨_, inside_fall3⟩, inside_noFall_below3⟩⟩

end IcNegative.Lemma4
