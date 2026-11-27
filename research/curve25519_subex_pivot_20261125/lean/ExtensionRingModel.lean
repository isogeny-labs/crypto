module

public import GroupFactorization
public import Mathlib.Data.Fintype.Basic

@[expose] public section

/-!
# Formal model of the native extension coefficient ring

The native implementation uses a power-basis presentation
`(Z/2^precision)[U]/f`, where `f` is monic with binary coefficients. This
module defines that quotient, decodes fixed-length serialized coordinate
vectors into it, proves the coefficientwise additive operations, and closes
the `2^precision = 0` premise needed by the exact Cauchy series theorem.

It does not yet prove that the Rust convolution/reduction loop implements the
quotient multiplication or that its deterministic irreducibility search
selects a particular formal bit vector.
-/

open scoped BigOperators
open Polynomial
open Curve25519GroupFactorization

namespace Curve25519ExtensionRingModel

/-- The monic binary-coefficient polynomial represented natively by a leading
degree bit and one Boolean for every lower coefficient. -/
noncomputable def binaryPolynomial
    (precision degree : ℕ) (lowerCoefficient : Fin degree → Bool) :
    Polynomial (ZMod (2 ^ precision)) :=
  X ^ degree +
    ∑ index, if lowerCoefficient index then monomial (index : ℕ) 1 else 0

/-- The formal quotient ring matching the native coefficient-ring shape. -/
abbrev NativeExtension
    (precision degree : ℕ) (lowerCoefficient : Fin degree → Bool) :=
  AdjoinRoot (binaryPolynomial precision degree lowerCoefficient)

/-- Decode one fixed-length power-basis coordinate vector into the quotient. -/
noncomputable def decodePowerBasis
    (precision degree : ℕ) (lowerCoefficient : Fin degree → Bool)
    (coordinates : Fin degree → ZMod (2 ^ precision)) :
    NativeExtension precision degree lowerCoefficient :=
  ∑ index,
    AdjoinRoot.of (binaryPolynomial precision degree lowerCoefficient)
      (coordinates index) *
      AdjoinRoot.root (binaryPolynomial precision degree lowerCoefficient) ^
        (index : ℕ)

/-- Decode canonical natural-number representatives from a serialized native
element. The native verifier separately checks vector length and the strict
`coordinate < 2^precision` bounds. -/
noncomputable def decodeSerialized
    (precision degree : ℕ) (lowerCoefficient : Fin degree → Bool)
    (coordinates : Fin degree → ℕ) :
    NativeExtension precision degree lowerCoefficient :=
  decodePowerBasis precision degree lowerCoefficient
    (fun index => coordinates index)

/-- Polynomial whose low power-basis coefficients are one serialized
coordinate vector. -/
noncomputable def coordinatePolynomial
    (precision degree : ℕ)
    (coordinates : Fin degree → ZMod (2 ^ precision)) :
    Polynomial (ZMod (2 ^ precision)) :=
  ∑ index : Fin degree, monomial (index : ℕ) (coordinates index)

/-- Power-basis decoding is exactly projection of the coordinate polynomial
into the quotient. -/
theorem coordinatePolynomial_mk
    (precision degree : ℕ) (lowerCoefficient : Fin degree → Bool)
    (coordinates : Fin degree → ZMod (2 ^ precision)) :
    AdjoinRoot.mk (binaryPolynomial precision degree lowerCoefficient)
        (coordinatePolynomial precision degree coordinates) =
      decodePowerBasis precision degree lowerCoefficient coordinates := by
  rw [coordinatePolynomial, map_sum]
  apply Finset.sum_congr rfl
  intro index _
  rw [← C_mul_X_pow_eq_monomial, map_mul, map_pow]
  rfl

/-- Source-shaped nested `i,j` convolution sum accumulated by the native
multiplication loops before high-degree reduction. -/
noncomputable def convolutionPolynomial
    (precision degree : ℕ)
    (left right : Fin degree → ZMod (2 ^ precision)) :
    Polynomial (ZMod (2 ^ precision)) :=
  ∑ i : Fin degree, ∑ j : Fin degree,
    monomial ((i : ℕ) + (j : ℕ)) (left i * right j)

/-- Multiplying coordinate polynomials produces exactly the source-shaped
nested finite convolution sum. -/
theorem coordinatePolynomial_mul
    (precision degree : ℕ)
    (left right : Fin degree → ZMod (2 ^ precision)) :
    coordinatePolynomial precision degree left *
        coordinatePolynomial precision degree right =
      convolutionPolynomial precision degree left right := by
  rw [coordinatePolynomial, coordinatePolynomial, convolutionPolynomial,
    Finset.sum_mul]
  simp_rw [Finset.mul_sum, monomial_mul_monomial]

/-- Exact value accumulated in one native convolution-buffer slot. -/
noncomputable def convolutionCoefficient
    (precision degree index : ℕ)
    (left right : Fin degree → ZMod (2 ^ precision)) :
    ZMod (2 ^ precision) :=
  ∑ i : Fin degree, ∑ j : Fin degree,
    if (i : ℕ) + (j : ℕ) = index then left i * right j else 0

/-- Each coefficient of the source-shaped convolution polynomial is exactly
the corresponding native buffer-slot sum. -/
theorem convolutionPolynomial_coeff
    (precision degree index : ℕ)
    (left right : Fin degree → ZMod (2 ^ precision)) :
    (convolutionPolynomial precision degree left right).coeff index =
      convolutionCoefficient precision degree index left right := by
  let coefficient : Polynomial (ZMod (2 ^ precision)) →+
      ZMod (2 ^ precision) :=
    { toFun := fun polynomial => polynomial.coeff index
      map_zero' := coeff_zero index
      map_add' := fun a b => coeff_add a b index }
  change coefficient
      (∑ i : Fin degree, ∑ j : Fin degree,
        monomial ((i : ℕ) + (j : ℕ)) (left i * right j)) = _
  rw [map_sum]
  apply Finset.sum_congr rfl
  intro i _
  rw [map_sum]
  apply Finset.sum_congr rfl
  intro j _
  simp [coefficient, coeff_monomial]

/-- The convolution has no nonzero coefficient beyond its native `2d-1`
buffer support. -/
theorem convolutionCoefficient_eq_zero_of_bound
    (precision degree index : ℕ)
    (left right : Fin degree → ZMod (2 ^ precision))
    (hbound : degree + degree ≤ index + 1) :
    convolutionCoefficient precision degree index left right = 0 := by
  rw [convolutionCoefficient]
  apply Finset.sum_eq_zero
  intro i _
  apply Finset.sum_eq_zero
  intro j _
  have hne : (i : ℕ) + (j : ℕ) ≠ index := by omega
  simp [hne]

/-- One source-shaped mutable convolution update, including the native
zero-coordinate fast paths and the masked-ring addition semantics. -/
noncomputable def convolutionBufferStep
    (precision degree : ℕ)
    (left right : Fin degree → ZMod (2 ^ precision))
    (buffer : ℕ → ZMod (2 ^ precision)) (i j : Fin degree) :
    ℕ → ZMod (2 ^ precision) :=
  if left i = 0 then buffer
  else if right j = 0 then buffer
  else Function.update buffer ((i : ℕ) + (j : ℕ))
    (buffer ((i : ℕ) + (j : ℕ)) + left i * right j)

/-- Reading any slot after one mutable convolution update gives the old value
plus exactly the source pair's contribution when its indices select that slot. -/
theorem convolutionBufferStep_apply
    (precision degree index : ℕ)
    (left right : Fin degree → ZMod (2 ^ precision))
    (buffer : ℕ → ZMod (2 ^ precision)) (i j : Fin degree) :
    convolutionBufferStep precision degree left right buffer i j index =
      buffer index +
        if (i : ℕ) + (j : ℕ) = index then left i * right j else 0 := by
  by_cases hleft : left i = 0
  · simp [convolutionBufferStep, hleft]
  by_cases hright : right j = 0
  · simp [convolutionBufferStep, hleft, hright]
  by_cases hindex : (i : ℕ) + (j : ℕ) = index
  · subst index
    simp [convolutionBufferStep, hleft, hright]
  · have hindex' : index ≠ (i : ℕ) + (j : ℕ) := Ne.symm hindex
    simp [convolutionBufferStep, Function.update, hleft, hright, hindex, hindex']

private theorem convolutionBufferFoldJ_apply
    (precision degree index : ℕ)
    (left right : Fin degree → ZMod (2 ^ precision))
    (buffer : ℕ → ZMod (2 ^ precision)) (i : Fin degree)
    (indices : List (Fin degree)) :
    (indices.foldl
        (convolutionBufferStep precision degree left right · i ·) buffer) index =
      buffer index +
        (indices.map fun (j : Fin degree) =>
          if (i : ℕ) + (j : ℕ) = index then left i * right j else 0).sum := by
  induction indices generalizing buffer with
  | nil => simp
  | cons j indices ih =>
      rw [List.foldl_cons, ih, convolutionBufferStep_apply]
      simp only [List.map_cons, List.sum_cons]
      ring

/-- One complete native inner-loop row adds exactly the conditional sum over
all right-hand coordinates to every buffer slot. -/
theorem convolutionBufferRow_apply
    (precision degree index : ℕ)
    (left right : Fin degree → ZMod (2 ^ precision))
    (buffer : ℕ → ZMod (2 ^ precision)) (i : Fin degree) :
    ((List.finRange degree).foldl
        (convolutionBufferStep precision degree left right · i ·) buffer) index =
      buffer index + ∑ j : Fin degree,
        if (i : ℕ) + (j : ℕ) = index then left i * right j else 0 := by
  rw [convolutionBufferFoldJ_apply]
  congr 1

/-- The full zero-initialized mutable nested-loop convolution buffer. -/
noncomputable def convolutionBuffer
    (precision degree : ℕ)
    (left right : Fin degree → ZMod (2 ^ precision)) :
    ℕ → ZMod (2 ^ precision) :=
  (List.finRange degree).foldl
    (fun buffer i =>
      (List.finRange degree).foldl
        (convolutionBufferStep precision degree left right · i ·) buffer)
    0

private theorem convolutionBufferFoldI_apply
    (precision degree index : ℕ)
    (left right : Fin degree → ZMod (2 ^ precision))
    (buffer : ℕ → ZMod (2 ^ precision)) (indices : List (Fin degree)) :
    (indices.foldl
        (fun buffer i =>
          (List.finRange degree).foldl
            (convolutionBufferStep precision degree left right · i ·) buffer)
        buffer) index =
      buffer index +
        (indices.map fun (i : Fin degree) => ∑ j : Fin degree,
          if (i : ℕ) + (j : ℕ) = index then left i * right j else 0).sum := by
  induction indices generalizing buffer with
  | nil => simp
  | cons i indices ih =>
      rw [List.foldl_cons, ih, convolutionBufferRow_apply]
      simp only [List.map_cons, List.sum_cons]
      ring

/-- The final contents of every slot in the mutable source-shaped nested-loop
buffer equal the kernel-checked conditional convolution coefficient. -/
theorem convolutionBuffer_apply
    (precision degree index : ℕ)
    (left right : Fin degree → ZMod (2 ^ precision)) :
    convolutionBuffer precision degree left right index =
      convolutionCoefficient precision degree index left right := by
  rw [convolutionBuffer, convolutionBufferFoldI_apply,
    Pi.zero_apply, zero_add, convolutionCoefficient]
  rw [← List.sum_toFinset _ (List.nodup_finRange degree),
    List.toFinset_finRange]

/-- Multiplication of decoded coordinate vectors is projection of the ordinary
coordinate-polynomial convolution product. -/
theorem decodePowerBasis_mul_eq_mk
    (precision degree : ℕ) (lowerCoefficient : Fin degree → Bool)
    (left right : Fin degree → ZMod (2 ^ precision)) :
    decodePowerBasis precision degree lowerCoefficient left *
        decodePowerBasis precision degree lowerCoefficient right =
      AdjoinRoot.mk (binaryPolynomial precision degree lowerCoefficient)
        (coordinatePolynomial precision degree left *
          coordinatePolynomial precision degree right) := by
  rw [← coordinatePolynomial_mk, ← coordinatePolynomial_mk, map_mul]

/-- The adjoined power-basis generator satisfies the exact binary-polynomial
relation used by the native high-degree reduction loop. -/
theorem nativeExtension_root_relation
    (precision degree : ℕ) (lowerCoefficient : Fin degree → Bool) :
    AdjoinRoot.root (binaryPolynomial precision degree lowerCoefficient) ^ degree =
      -∑ index,
        AdjoinRoot.of (binaryPolynomial precision degree lowerCoefficient)
            (if lowerCoefficient index then 1 else 0) *
          AdjoinRoot.root (binaryPolynomial precision degree lowerCoefficient) ^
            (index : ℕ) := by
  let f := binaryPolynomial precision degree lowerCoefficient
  let root := AdjoinRoot.root f
  let of := AdjoinRoot.of f
  have hlow :
      Polynomial.eval₂ of root
          (∑ index, if lowerCoefficient index then monomial (index : ℕ) 1 else 0) =
        ∑ index, of (if lowerCoefficient index then 1 else 0) *
          root ^ (index : ℕ) := by
    change (Polynomial.eval₂RingHom of root)
        (∑ index, if lowerCoefficient index then monomial (index : ℕ) 1 else 0) = _
    rw [map_sum]
    apply Finset.sum_congr rfl
    intro index _
    by_cases h : lowerCoefficient index
    · simp [h, Polynomial.eval₂RingHom]
    · simp [h, Polynomial.eval₂RingHom]
  have hroot := AdjoinRoot.eval₂_root f
  change Polynomial.eval₂ of root
      (X ^ degree +
        ∑ index, if lowerCoefficient index then monomial (index : ℕ) 1 else 0) = 0 at hroot
  rw [eval₂_add, eval₂_pow, eval₂_X, hlow] at hroot
  exact eq_neg_of_add_eq_zero_left hroot

/-- One native high-degree reduction step is sound in the formal quotient:
subtracting the current coefficient at every set lower-polynomial bit replaces
`coefficient * U^(degree+offset)` exactly. -/
theorem nativeExtension_reduce_high_power
    (precision degree : ℕ) (lowerCoefficient : Fin degree → Bool)
    (coefficient : ZMod (2 ^ precision)) (offset : ℕ) :
    AdjoinRoot.of (binaryPolynomial precision degree lowerCoefficient) coefficient *
        AdjoinRoot.root (binaryPolynomial precision degree lowerCoefficient) ^
          (degree + offset) =
      ∑ index,
        AdjoinRoot.of (binaryPolynomial precision degree lowerCoefficient)
            (if lowerCoefficient index then -coefficient else 0) *
          AdjoinRoot.root (binaryPolynomial precision degree lowerCoefficient) ^
            (offset + (index : ℕ)) := by
  let f := binaryPolynomial precision degree lowerCoefficient
  let root := AdjoinRoot.root f
  let of := AdjoinRoot.of f
  calc
    of coefficient * root ^ (degree + offset) =
        of coefficient * root ^ degree * root ^ offset := by
          simp [pow_add, mul_assoc]
    _ = of coefficient *
          (-∑ index, of (if lowerCoefficient index then 1 else 0) *
            root ^ (index : ℕ)) * root ^ offset := by
          rw [nativeExtension_root_relation]
    _ = ∑ index,
          of (if lowerCoefficient index then -coefficient else 0) *
            root ^ (offset + (index : ℕ)) := by
      rw [mul_neg, Finset.mul_sum, neg_mul, Finset.sum_mul,
        ← Finset.sum_neg_distrib]
      apply Finset.sum_congr rfl
      intro index _
      by_cases h : lowerCoefficient index
      · simp [h, pow_add]
        ring
      · simp [h]

/-- The leading degree coefficient of the binary polynomial is one, including
in degenerate coefficient rings. -/
theorem binaryPolynomial_coeff_degree
    (precision degree : ℕ) (lowerCoefficient : Fin degree → Bool) :
    (binaryPolynomial precision degree lowerCoefficient).coeff degree = 1 := by
  let coefficient :
      Polynomial (ZMod (2 ^ precision)) →+ ZMod (2 ^ precision) :=
    { toFun := fun polynomial : Polynomial (ZMod (2 ^ precision)) =>
        polynomial.coeff degree
      map_zero' := coeff_zero degree
      map_add' := fun left right => coeff_add left right degree }
  rw [binaryPolynomial, coeff_add, coeff_X_pow_self]
  change 1 + coefficient
    (∑ index, if lowerCoefficient index then monomial (index : ℕ) 1 else 0) = 1
  rw [map_sum]
  have hsum :
      ∑ index, coefficient
        (if lowerCoefficient index then monomial (index : ℕ) 1 else 0) = 0 := by
    apply Finset.sum_eq_zero
    intro index _
    by_cases h : lowerCoefficient index
    · simp [coefficient, h, coeff_monomial, ne_of_lt index.isLt]
    · simp [coefficient, h]
  rw [hsum, add_zero]

/-- The binary modulus has no coefficient above its declared monic degree. -/
theorem binaryPolynomial_coeff_eq_zero_of_lt
    (precision degree index : ℕ) (lowerCoefficient : Fin degree → Bool)
    (hindex : degree < index) :
    (binaryPolynomial precision degree lowerCoefficient).coeff index = 0 := by
  let coefficient :
      Polynomial (ZMod (2 ^ precision)) →+ ZMod (2 ^ precision) :=
    { toFun := fun polynomial => polynomial.coeff index
      map_zero' := coeff_zero index
      map_add' := fun left right => coeff_add left right index }
  have hne : index ≠ degree := Nat.ne_of_gt hindex
  have hx : (X ^ degree : Polynomial (ZMod (2 ^ precision))).coeff index = 0 := by
    rw [coeff_X_pow]
    simp [hne]
  rw [binaryPolynomial, coeff_add, hx, zero_add]
  change coefficient
    (∑ coefficientIndex,
      if lowerCoefficient coefficientIndex then
        monomial (coefficientIndex : ℕ) 1 else 0) = 0
  rw [map_sum]
  apply Finset.sum_eq_zero
  intro coefficientIndex _
  by_cases h : lowerCoefficient coefficientIndex
  · simp [coefficient, h, coeff_monomial,
      ne_of_lt (lt_trans coefficientIndex.isLt hindex)]
  · simp [coefficient, h]

/-- Logical polynomial form of one descending native reduction update. It
subtracts the targeted coefficient times a shifted copy of the modulus. -/
noncomputable def binaryReductionStep
    (precision degree : ℕ) (lowerCoefficient : Fin degree → Bool)
    (polynomial : Polynomial (ZMod (2 ^ precision))) (offset : ℕ) :
    Polynomial (ZMod (2 ^ precision)) :=
  polynomial -
    monomial offset (polynomial.coeff (degree + offset)) *
      binaryPolynomial precision degree lowerCoefficient

/-- One logical reduction step clears its targeted high coefficient. -/
theorem binaryReductionStep_coeff
    (precision degree : ℕ) (lowerCoefficient : Fin degree → Bool)
    (polynomial : Polynomial (ZMod (2 ^ precision))) (offset : ℕ) :
    (binaryReductionStep precision degree lowerCoefficient polynomial offset).coeff
      (degree + offset) = 0 := by
  rw [binaryReductionStep, coeff_sub, coeff_monomial_mul,
    binaryPolynomial_coeff_degree]
  simp

/-- A logical reduction update cannot change any coefficient strictly above
the step's targeted high index. -/
theorem binaryReductionStep_coeff_of_lt
    (precision degree : ℕ) (lowerCoefficient : Fin degree → Bool)
    (polynomial : Polynomial (ZMod (2 ^ precision))) (offset index : ℕ)
    (hindex : degree + offset < index) :
    (binaryReductionStep precision degree lowerCoefficient polynomial offset).coeff
        index = polynomial.coeff index := by
  have hoffset : offset ≤ index := by omega
  have hsplit : index - offset + offset = index := Nat.sub_add_cancel hoffset
  have hdegree : degree < index - offset := by omega
  have hproduct :
      (monomial offset (polynomial.coeff (degree + offset)) *
          binaryPolynomial precision degree lowerCoefficient).coeff index = 0 := by
    rw [← hsplit, coeff_monomial_mul,
      binaryPolynomial_coeff_eq_zero_of_lt precision degree (index - offset)
        lowerCoefficient hdegree,
      mul_zero]
  rw [binaryReductionStep, coeff_sub, hproduct, sub_zero]

/-- One logical reduction step preserves the represented quotient value. -/
theorem binaryReductionStep_mk
    (precision degree : ℕ) (lowerCoefficient : Fin degree → Bool)
    (polynomial : Polynomial (ZMod (2 ^ precision))) (offset : ℕ) :
    AdjoinRoot.mk (binaryPolynomial precision degree lowerCoefficient)
        (binaryReductionStep precision degree lowerCoefficient polynomial offset) =
      AdjoinRoot.mk (binaryPolynomial precision degree lowerCoefficient) polynomial := by
  simp [binaryReductionStep]

/-- Apply a caller-supplied finite schedule of logical reduction offsets. The
native loop uses the descending offsets of its high convolution buffer. -/
noncomputable def binaryReduceOffsets
    (precision degree : ℕ) (lowerCoefficient : Fin degree → Bool) :
    List ℕ → Polynomial (ZMod (2 ^ precision)) →
      Polynomial (ZMod (2 ^ precision))
  | [], polynomial => polynomial
  | offset :: offsets, polynomial =>
      binaryReduceOffsets precision degree lowerCoefficient offsets
        (binaryReductionStep precision degree lowerCoefficient polynomial offset)

/-- Every finite schedule of logical modulus-subtraction steps preserves the
quotient value. Schedule completeness and its in-place Rust array refinement
are separate obligations. -/
theorem binaryReduceOffsets_mk
    (precision degree : ℕ) (lowerCoefficient : Fin degree → Bool)
    (offsets : List ℕ) (polynomial : Polynomial (ZMod (2 ^ precision))) :
    AdjoinRoot.mk (binaryPolynomial precision degree lowerCoefficient)
        (binaryReduceOffsets precision degree lowerCoefficient offsets polynomial) =
      AdjoinRoot.mk (binaryPolynomial precision degree lowerCoefficient) polynomial := by
  induction offsets generalizing polynomial with
  | nil => rfl
  | cons offset offsets ih =>
      rw [binaryReduceOffsets, ih, binaryReductionStep_mk]

/-- Apply exactly the descending source offsets `count-1,...,0`. For the native
`2d-1` product buffer, `count=d-1`. -/
noncomputable def binaryReduceDescending
    (precision degree : ℕ) (lowerCoefficient : Fin degree → Bool) :
    ℕ → Polynomial (ZMod (2 ^ precision)) →
      Polynomial (ZMod (2 ^ precision))
  | 0, polynomial => polynomial
  | count + 1, polynomial =>
      binaryReduceDescending precision degree lowerCoefficient count
        (binaryReductionStep precision degree lowerCoefficient polynomial count)

/-- A descending schedule of `count` steps leaves every coefficient at or
above `degree+count` unchanged. -/
theorem binaryReduceDescending_coeff_of_bound
    (precision degree : ℕ) (lowerCoefficient : Fin degree → Bool)
    (count index : ℕ) (polynomial : Polynomial (ZMod (2 ^ precision)))
    (hbound : degree + count ≤ index) :
    (binaryReduceDescending precision degree lowerCoefficient count polynomial).coeff
        index = polynomial.coeff index := by
  induction count generalizing polynomial with
  | zero => rfl
  | succ count ih =>
      rw [binaryReduceDescending]
      calc
        (binaryReduceDescending precision degree lowerCoefficient count
            (binaryReductionStep precision degree lowerCoefficient polynomial
              count)).coeff index =
            (binaryReductionStep precision degree lowerCoefficient polynomial
              count).coeff index := ih _ (by omega)
        _ = polynomial.coeff index :=
          binaryReductionStep_coeff_of_lt precision degree lowerCoefficient
            polynomial count index (by omega)

/-- The exact descending schedule clears every targeted coefficient
`degree+offset` for `offset<count`; later lower steps cannot reintroduce it. -/
theorem binaryReduceDescending_coeff
    (precision degree : ℕ) (lowerCoefficient : Fin degree → Bool)
    (count offset : ℕ) (polynomial : Polynomial (ZMod (2 ^ precision)))
    (hoffset : offset < count) :
    (binaryReduceDescending precision degree lowerCoefficient count polynomial).coeff
        (degree + offset) = 0 := by
  induction count generalizing polynomial with
  | zero => omega
  | succ count ih =>
      rw [binaryReduceDescending]
      by_cases hlast : offset = count
      · subst offset
        calc
          (binaryReduceDescending precision degree lowerCoefficient count
              (binaryReductionStep precision degree lowerCoefficient polynomial
                count)).coeff (degree + count) =
              (binaryReductionStep precision degree lowerCoefficient polynomial
                count).coeff (degree + count) :=
            binaryReduceDescending_coeff_of_bound precision degree
              lowerCoefficient count (degree + count) _ (by omega)
          _ = 0 := binaryReductionStep_coeff precision degree lowerCoefficient
            polynomial count
      · exact ih _ (by omega)

/-- The exact descending source schedule preserves the represented quotient. -/
theorem binaryReduceDescending_mk
    (precision degree : ℕ) (lowerCoefficient : Fin degree → Bool)
    (count : ℕ) (polynomial : Polynomial (ZMod (2 ^ precision))) :
    AdjoinRoot.mk (binaryPolynomial precision degree lowerCoefficient)
        (binaryReduceDescending precision degree lowerCoefficient count polynomial) =
      AdjoinRoot.mk (binaryPolynomial precision degree lowerCoefficient)
        polynomial := by
  induction count generalizing polynomial with
  | zero => rfl
  | succ count ih =>
      rw [binaryReduceDescending, ih, binaryReductionStep_mk]

/-- For positive degree, the native `d-1` descending offsets clear every high
coefficient of the `2d-1` coordinate convolution, so only indices below `d`
remain for truncation. -/
theorem convolutionReduction_coeff_eq_zero
    (precision degree index : ℕ) (lowerCoefficient : Fin degree → Bool)
    (left right : Fin degree → ZMod (2 ^ precision))
    (hdegree : 0 < degree) (hindex : degree ≤ index) :
    (binaryReduceDescending precision degree lowerCoefficient (degree - 1)
        (convolutionPolynomial precision degree left right)).coeff index = 0 := by
  by_cases hwithin : index < degree + (degree - 1)
  · let offset := index - degree
    have hoffset : offset < degree - 1 := by omega
    have hsplit : degree + offset = index := by omega
    rw [← hsplit]
    exact binaryReduceDescending_coeff precision degree lowerCoefficient
      (degree - 1) offset _ hoffset
  · rw [binaryReduceDescending_coeff_of_bound precision degree lowerCoefficient
      (degree - 1) index _ (by omega), convolutionPolynomial_coeff]
    exact convolutionCoefficient_eq_zero_of_bound precision degree index left right
      (by omega)

/-- Coordinate-polynomial convolution followed by any finite logical reduction
schedule represents the product of the decoded inputs. -/
theorem decodePowerBasis_mul_eq_reduced_mk
    (precision degree : ℕ) (lowerCoefficient : Fin degree → Bool)
    (left right : Fin degree → ZMod (2 ^ precision)) (offsets : List ℕ) :
    decodePowerBasis precision degree lowerCoefficient left *
        decodePowerBasis precision degree lowerCoefficient right =
      AdjoinRoot.mk (binaryPolynomial precision degree lowerCoefficient)
        (binaryReduceOffsets precision degree lowerCoefficient offsets
          (coordinatePolynomial precision degree left *
            coordinatePolynomial precision degree right)) := by
  rw [binaryReduceOffsets_mk]
  exact decodePowerBasis_mul_eq_mk precision degree lowerCoefficient left right

/-- The base characteristic relation survives in every polynomial quotient:
the formal native extension satisfies the modulus premise used by the Cauchy
series theorem. -/
theorem nativeExtension_pow_two_zero
    (precision degree : ℕ) (lowerCoefficient : Fin degree → Bool) :
    (2 : NativeExtension precision degree lowerCoefficient) ^ precision = 0 := by
  have hbase : (2 : ZMod (2 ^ precision)) ^ precision = 0 := by
    change ((Nat.castRingHom (ZMod (2 ^ precision))) 2) ^ precision = 0
    rw [← map_pow]
    exact CharP.cast_eq_zero (ZMod (2 ^ precision)) (2 ^ precision)
  change
    (AdjoinRoot.of (binaryPolynomial precision degree lowerCoefficient) 2) ^
      precision = 0
  rw [← map_pow, hbase]
  simp

/-- Native coordinatewise addition decodes to quotient-ring addition. -/
theorem decodePowerBasis_add
    (precision degree : ℕ) (lowerCoefficient : Fin degree → Bool)
    (a b : Fin degree → ZMod (2 ^ precision)) :
    decodePowerBasis precision degree lowerCoefficient
        (fun index => a index + b index) =
      decodePowerBasis precision degree lowerCoefficient a +
        decodePowerBasis precision degree lowerCoefficient b := by
  simp [decodePowerBasis, add_mul, Finset.sum_add_distrib]

/-- Native coordinatewise negation decodes to quotient-ring negation. -/
theorem decodePowerBasis_neg
    (precision degree : ℕ) (lowerCoefficient : Fin degree → Bool)
    (a : Fin degree → ZMod (2 ^ precision)) :
    decodePowerBasis precision degree lowerCoefficient (fun index => -a index) =
      -decodePowerBasis precision degree lowerCoefficient a := by
  simp [decodePowerBasis]

/-- Exact Rust-shaped Cauchy cutoff in the formal native extension. The
extension's `2^precision = 0` premise is discharged by the quotient model. -/
theorem nativeExtension_cauchy_weight_mode_series_at_precision
    (precision degree : ℕ) (lowerCoefficient : Fin degree → Bool)
    (numerator denominator denominatorInv shiftedInv sample :
      NativeExtension precision degree lowerCoefficient)
    (a i v : ℕ)
    (hvi : v ≤ i) (hia : i ≤ a) (hprecision : 0 < precision)
    (hinverse : denominator * denominatorInv = 1)
    (hshiftedInverse :
      (denominator - (2 : NativeExtension precision degree lowerCoefficient) ^
        (i + 1 - v) * sample) * shiftedInv = 1) :
    numerator * shiftedInv *
        (2 : NativeExtension precision degree lowerCoefficient) ^ (a + 1 - v) =
      ∑ index ∈ Finset.range (precision - 1),
        (numerator * denominatorInv ^ (index + 1) *
          (2 : NativeExtension precision degree lowerCoefficient) ^
            ((a + 1 - v) + (i + 1 - v) * index)) * sample ^ index := by
  exact cauchy_weight_mode_series_at_precision
    numerator denominator denominatorInv shiftedInv sample a i v precision
    hvi hia hprecision
    (nativeExtension_pow_two_zero precision degree lowerCoefficient)
    hinverse hshiftedInverse

end Curve25519ExtensionRingModel
