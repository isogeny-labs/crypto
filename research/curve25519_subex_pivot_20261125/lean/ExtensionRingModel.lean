module

public import GroupFactorization
public import Mathlib.Data.Fintype.Basic
public import Mathlib.Data.Nat.Bitwise
public import Mathlib.Algebra.Polynomial.Degree.IsMonicOfDegree

@[expose] public section

/-!
# Formal model of the native extension coefficient ring

The native implementation uses a power-basis presentation
`(Z/2^precision)[U]/f`, where `f` is monic with binary coefficients. This
module defines that quotient, decodes fixed-length serialized coordinate
vectors into it, proves the coefficientwise additive operations, and closes
the `2^precision = 0` premise needed by the exact Cauchy series theorem.

It also proves that a source-shaped model of Rust's convolution, retained-high
reduction rows, reverse schedule, and truncation implements quotient
multiplication. A mechanized binding from the concrete `Vec<BigUint>` code and
its deterministic irreducibility search to these definitions remains separate.
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

/-- Every in-range coefficient of a coordinate polynomial is the corresponding
serialized coordinate. -/
theorem coordinatePolynomial_coeff
    (precision degree : ℕ)
    (coordinates : Fin degree → ZMod (2 ^ precision)) (index : Fin degree) :
    (coordinatePolynomial precision degree coordinates).coeff index =
      coordinates index := by
  let coefficient :
      Polynomial (ZMod (2 ^ precision)) →+ ZMod (2 ^ precision) :=
    { toFun := fun polynomial => polynomial.coeff index
      map_zero' := coeff_zero index
      map_add' := fun left right => coeff_add left right index }
  rw [coordinatePolynomial]
  change coefficient
    (∑ coordinateIndex : Fin degree,
      monomial (coordinateIndex : ℕ) (coordinates coordinateIndex)) = _
  rw [map_sum]
  have hself :
      coefficient (monomial (index : ℕ) (coordinates index)) =
        coordinates index := by
    simp [coefficient]
  rw [← hself]
  apply Finset.sum_eq_single index
  · intro coordinateIndex _ hne
    have hcoe : (coordinateIndex : ℕ) ≠ (index : ℕ) := by
      intro heq
      exact hne (Fin.ext heq)
    simp [coefficient, coeff_monomial, hcoe]
  · simp

/-- Coordinate polynomials have no coefficient at or above their fixed vector
length. -/
theorem coordinatePolynomial_coeff_eq_zero_of_le
    (precision degree index : ℕ)
    (coordinates : Fin degree → ZMod (2 ^ precision)) (hindex : degree ≤ index) :
    (coordinatePolynomial precision degree coordinates).coeff index = 0 := by
  let coefficient :
      Polynomial (ZMod (2 ^ precision)) →+ ZMod (2 ^ precision) :=
    { toFun := fun polynomial => polynomial.coeff index
      map_zero' := coeff_zero index
      map_add' := fun left right => coeff_add left right index }
  rw [coordinatePolynomial]
  change coefficient
    (∑ coordinateIndex : Fin degree,
      monomial (coordinateIndex : ℕ) (coordinates coordinateIndex)) = 0
  rw [map_sum]
  apply Finset.sum_eq_zero
  intro coordinateIndex _
  have hne : (coordinateIndex : ℕ) ≠ index := by
    have hlt : (coordinateIndex : ℕ) < degree := coordinateIndex.isLt
    omega
  simp [coefficient, coeff_monomial, hne]

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

/-- One mutable native high-degree reduction update for a single lower
polynomial bit. The source retains the processed high slot and later truncates
it; this transition changes only the lower `offset+j` slot. -/
noncomputable def nativeReductionBufferStep
    (precision degree : ℕ) (lowerCoefficient : Fin degree → Bool)
    (buffer : ℕ → ZMod (2 ^ precision)) (offset : ℕ) (j : Fin degree) :
    ℕ → ZMod (2 ^ precision) :=
  let coefficient := buffer (degree + offset)
  if coefficient = 0 then buffer
  else if lowerCoefficient j then
    Function.update buffer (offset + (j : ℕ))
      (buffer (offset + (j : ℕ)) - coefficient)
  else buffer

/-- Reading any slot after one native lower-bit reduction update gives exactly
the old value plus the selected negative high coefficient. -/
theorem nativeReductionBufferStep_apply
    (precision degree index : ℕ) (lowerCoefficient : Fin degree → Bool)
    (buffer : ℕ → ZMod (2 ^ precision)) (offset : ℕ) (j : Fin degree) :
    nativeReductionBufferStep precision degree lowerCoefficient buffer offset j
        index =
      buffer index +
        if lowerCoefficient j then
          if offset + (j : ℕ) = index then -buffer (degree + offset) else 0
        else 0 := by
  by_cases hcoefficient : buffer (degree + offset) = 0
  · simp [nativeReductionBufferStep, hcoefficient]
  by_cases hbit : lowerCoefficient j
  · by_cases hindex : offset + (j : ℕ) = index
    · subst index
      simp [nativeReductionBufferStep, hcoefficient, hbit, sub_eq_add_neg]
    · have hindex' : index ≠ offset + (j : ℕ) := Ne.symm hindex
      simp [nativeReductionBufferStep, hcoefficient, hbit,
        hindex, hindex']
  · simp [nativeReductionBufferStep, hcoefficient, hbit]

private theorem nativeReductionBufferFold_apply
    (precision degree index : ℕ) (lowerCoefficient : Fin degree → Bool)
    (buffer : ℕ → ZMod (2 ^ precision)) (offset : ℕ)
    (indices : List (Fin degree)) :
    (indices.foldl
        (nativeReductionBufferStep precision degree lowerCoefficient · offset ·)
        buffer) index =
      buffer index +
        (indices.map fun (j : Fin degree) =>
          if lowerCoefficient j then
            if offset + (j : ℕ) = index then -buffer (degree + offset) else 0
          else 0).sum := by
  induction indices generalizing buffer with
  | nil => simp
  | cons j indices ih =>
      have htarget :
          nativeReductionBufferStep precision degree lowerCoefficient buffer
              offset j (degree + offset) = buffer (degree + offset) := by
        rw [nativeReductionBufferStep_apply]
        have hne : offset + (j : ℕ) ≠ degree + offset := by
          have hj : (j : ℕ) < degree := j.isLt
          omega
        simp [hne]
      rw [List.foldl_cons, ih, nativeReductionBufferStep_apply, htarget]
      simp only [List.map_cons, List.sum_cons]
      ring

/-- One complete native reduction inner loop over all lower polynomial bits. -/
noncomputable def nativeReductionBufferRow
    (precision degree : ℕ) (lowerCoefficient : Fin degree → Bool)
    (buffer : ℕ → ZMod (2 ^ precision)) (offset : ℕ) :
    ℕ → ZMod (2 ^ precision) :=
  (List.finRange degree).foldl
    (nativeReductionBufferStep precision degree lowerCoefficient · offset ·)
    buffer

/-- The complete mutable lower-bit loop has the exact conditional slot formula
induced by the cloned high coefficient and binary modulus bits. -/
theorem nativeReductionBufferRow_apply
    (precision degree index : ℕ) (lowerCoefficient : Fin degree → Bool)
    (buffer : ℕ → ZMod (2 ^ precision)) (offset : ℕ) :
    nativeReductionBufferRow precision degree lowerCoefficient buffer offset index =
      buffer index + ∑ j : Fin degree,
        if lowerCoefficient j then
          if offset + (j : ℕ) = index then -buffer (degree + offset) else 0
        else 0 := by
  rw [nativeReductionBufferRow, nativeReductionBufferFold_apply]
  congr 1

/-- The native inner reduction loop changes no slot at or above the cloned
high coefficient; in particular, the source retains that slot until truncation. -/
theorem nativeReductionBufferRow_high
    (precision degree index : ℕ) (lowerCoefficient : Fin degree → Bool)
    (buffer : ℕ → ZMod (2 ^ precision)) (offset : ℕ)
    (hindex : degree + offset ≤ index) :
    nativeReductionBufferRow precision degree lowerCoefficient buffer offset index =
      buffer index := by
  rw [nativeReductionBufferRow_apply]
  have hsum :
      (∑ j : Fin degree,
        if lowerCoefficient j then
          if offset + (j : ℕ) = index then -buffer (degree + offset) else 0
        else 0) = 0 := by
    apply Finset.sum_eq_zero
    intro j _
    have hne : offset + (j : ℕ) ≠ index := by
      have hj : (j : ℕ) < degree := j.isLt
      omega
    simp [hne]
  rw [hsum, add_zero]

/-- Polynomial carrying exactly the lower-slot mutations performed by one
native reduction row while retaining the processed high coefficient. -/
noncomputable def nativeReductionPolynomialStep
    (precision degree : ℕ) (lowerCoefficient : Fin degree → Bool)
    (polynomial : Polynomial (ZMod (2 ^ precision))) (offset : ℕ) :
    Polynomial (ZMod (2 ^ precision)) :=
  polynomial + ∑ j : Fin degree,
    if lowerCoefficient j then
      monomial (offset + (j : ℕ)) (-polynomial.coeff (degree + offset))
    else 0

/-- Coefficients of the retained-high polynomial step are exactly the contents
of the source-shaped mutable reduction row started from polynomial coefficients. -/
theorem nativeReductionPolynomialStep_coeff
    (precision degree index : ℕ) (lowerCoefficient : Fin degree → Bool)
    (polynomial : Polynomial (ZMod (2 ^ precision))) (offset : ℕ) :
    (nativeReductionPolynomialStep precision degree lowerCoefficient polynomial
        offset).coeff index =
      nativeReductionBufferRow precision degree lowerCoefficient
        (fun k => polynomial.coeff k) offset index := by
  rw [nativeReductionPolynomialStep, coeff_add,
    nativeReductionBufferRow_apply]
  congr 1
  let coefficient :
      Polynomial (ZMod (2 ^ precision)) →+ ZMod (2 ^ precision) :=
    { toFun := fun value => value.coeff index
      map_zero' := coeff_zero index
      map_add' := fun left right => coeff_add left right index }
  change coefficient
      (∑ j : Fin degree,
        if lowerCoefficient j then
          monomial (offset + (j : ℕ))
            (-polynomial.coeff (degree + offset))
        else 0) = _
  rw [map_sum]
  apply Finset.sum_congr rfl
  intro j _
  by_cases hbit : lowerCoefficient j
  · by_cases hindex : offset + (j : ℕ) = index
    · simp [coefficient, hbit, hindex]
    · simp [coefficient, hbit, coeff_monomial, hindex]
  · simp [coefficient, hbit]

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

/-- The source-shaped polynomial step differs from the logical clearing step
only by the high monomial that Rust retains until the buffer is truncated. -/
theorem nativeReductionPolynomialStep_eq
    (precision degree : ℕ) (lowerCoefficient : Fin degree → Bool)
    (polynomial : Polynomial (ZMod (2 ^ precision))) (offset : ℕ) :
    nativeReductionPolynomialStep precision degree lowerCoefficient polynomial
        offset =
      binaryReductionStep precision degree lowerCoefficient polynomial offset +
        monomial (degree + offset) (polynomial.coeff (degree + offset)) := by
  let coefficient := polynomial.coeff (degree + offset)
  have hnegative :
      (∑ j : Fin degree,
        if lowerCoefficient j then
          monomial (offset + (j : ℕ)) (-coefficient)
        else 0) =
      -(∑ j : Fin degree,
        if lowerCoefficient j then
          monomial (offset + (j : ℕ)) coefficient
        else 0) := by
    rw [← Finset.sum_neg_distrib]
    apply Finset.sum_congr rfl
    intro j _
    by_cases hbit : lowerCoefficient j
    · simp [hbit, monomial_neg]
    · simp [hbit]
  rw [nativeReductionPolynomialStep, binaryReductionStep, binaryPolynomial,
    mul_add, monomial_mul_X_pow, Finset.mul_sum]
  simp_rw [mul_ite, mul_zero, monomial_mul_monomial, mul_one]
  change polynomial +
      (∑ j : Fin degree,
        if lowerCoefficient j then
          monomial (offset + (j : ℕ)) (-coefficient)
        else 0) = _
  rw [hnegative]
  simp [coefficient, Nat.add_comm]
  abel

/-- Below the processed high slot, one complete mutable native reduction row
has exactly the coefficients of one logical clearing step. -/
theorem nativeReductionBufferRow_refines_coefficients
    (precision degree index : ℕ) (lowerCoefficient : Fin degree → Bool)
    (polynomial : Polynomial (ZMod (2 ^ precision))) (offset : ℕ)
    (hindex : index < degree + offset) :
    nativeReductionBufferRow precision degree lowerCoefficient
        (fun k => polynomial.coeff k) offset index =
      (binaryReductionStep precision degree lowerCoefficient polynomial offset).coeff
        index := by
  rw [← nativeReductionPolynomialStep_coeff,
    nativeReductionPolynomialStep_eq, coeff_add, coeff_monomial]
  have hne : degree + offset ≠ index := Nat.ne_of_gt hindex
  simp [hne]

/-- The same one-row refinement holds for any mutable buffer agreeing with the
logical polynomial through the current processed high slot. -/
theorem nativeReductionBufferRow_refines
    (precision degree index : ℕ) (lowerCoefficient : Fin degree → Bool)
    (buffer : ℕ → ZMod (2 ^ precision))
    (polynomial : Polynomial (ZMod (2 ^ precision))) (offset : ℕ)
    (hbuffer : ∀ k, k ≤ degree + offset → buffer k = polynomial.coeff k)
    (hindex : index < degree + offset) :
    nativeReductionBufferRow precision degree lowerCoefficient buffer offset
        index =
      (binaryReductionStep precision degree lowerCoefficient polynomial offset).coeff
        index := by
  calc
    nativeReductionBufferRow precision degree lowerCoefficient buffer offset index =
        polynomial.coeff index + ∑ j : Fin degree,
          if lowerCoefficient j then
            if offset + (j : ℕ) = index then
              -polynomial.coeff (degree + offset) else 0
          else 0 := by
      rw [nativeReductionBufferRow_apply, hbuffer index (by omega),
        hbuffer (degree + offset) (by omega)]
    _ = nativeReductionBufferRow precision degree lowerCoefficient
          (fun k => polynomial.coeff k) offset index := by
      rw [nativeReductionBufferRow_apply]
    _ = (binaryReductionStep precision degree lowerCoefficient polynomial offset).coeff
          index := nativeReductionBufferRow_refines_coefficients precision degree
            index lowerCoefficient polynomial offset hindex

/-- Apply the complete source-shaped mutable reduction rows in descending order
`count-1,...,0`, retaining processed high slots until final truncation. -/
noncomputable def nativeReduceBufferDescending
    (precision degree : ℕ) (lowerCoefficient : Fin degree → Bool) :
    ℕ → (ℕ → ZMod (2 ^ precision)) → (ℕ → ZMod (2 ^ precision))
  | 0, buffer => buffer
  | count + 1, buffer =>
      nativeReduceBufferDescending precision degree lowerCoefficient count
        (nativeReductionBufferRow precision degree lowerCoefficient buffer count)

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

/-- If a mutable buffer initially agrees with a polynomial through the active
range, the full descending native fold agrees with the logical schedule on all
returned low coordinates. -/
theorem nativeReduceBufferDescending_refines
    (precision degree count : ℕ) (lowerCoefficient : Fin degree → Bool)
    (buffer : ℕ → ZMod (2 ^ precision))
    (polynomial : Polynomial (ZMod (2 ^ precision)))
    (hbuffer : ∀ index, index < degree + count →
      buffer index = polynomial.coeff index)
    (index : ℕ) (hindex : index < degree) :
    nativeReduceBufferDescending precision degree lowerCoefficient count buffer
        index =
      (binaryReduceDescending precision degree lowerCoefficient count polynomial).coeff
        index := by
  induction count generalizing buffer polynomial with
  | zero =>
      rw [nativeReduceBufferDescending, binaryReduceDescending]
      exact hbuffer index (by omega)
  | succ count ih =>
      rw [nativeReduceBufferDescending, binaryReduceDescending]
      apply ih
      intro k hk
      apply nativeReductionBufferRow_refines precision degree k lowerCoefficient
        buffer polynomial count
      · intro sourceIndex hsourceIndex
        exact hbuffer sourceIndex (by omega)
      · exact hk

/-- Starting from the proved mutable convolution buffer, the complete native
descending reduction fold returns exactly the low coefficients of the proved
logical descending reduction polynomial. -/
theorem nativeConvolutionReduceDescending_refines
    (precision degree : ℕ) (lowerCoefficient : Fin degree → Bool)
    (left right : Fin degree → ZMod (2 ^ precision)) (index : Fin degree) :
    nativeReduceBufferDescending precision degree lowerCoefficient (degree - 1)
        (convolutionBuffer precision degree left right) index =
      (binaryReduceDescending precision degree lowerCoefficient (degree - 1)
        (convolutionPolynomial precision degree left right)).coeff index := by
  apply nativeReduceBufferDescending_refines precision degree (degree - 1)
    lowerCoefficient (convolutionBuffer precision degree left right)
      (convolutionPolynomial precision degree left right)
  · intro sourceIndex _
    rw [convolutionBuffer_apply, convolutionPolynomial_coeff]
  · exact index.isLt

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

/-- The fixed-length coordinate vector returned by the complete source-shaped
mutable multiplication buffer after descending reduction and truncation. -/
noncomputable def nativeProductCoordinates
    (precision degree : ℕ) (lowerCoefficient : Fin degree → Bool)
    (left right : Fin degree → ZMod (2 ^ precision)) :
    Fin degree → ZMod (2 ^ precision) :=
  fun index =>
    nativeReduceBufferDescending precision degree lowerCoefficient (degree - 1)
      (convolutionBuffer precision degree left right) index

/-- The returned native coordinate polynomial is exactly the fully reduced
logical convolution polynomial. -/
theorem nativeProductPolynomial_eq
    (precision degree : ℕ) (lowerCoefficient : Fin degree → Bool)
    (left right : Fin degree → ZMod (2 ^ precision)) (hdegree : 0 < degree) :
    coordinatePolynomial precision degree
        (nativeProductCoordinates precision degree lowerCoefficient left right) =
      binaryReduceDescending precision degree lowerCoefficient (degree - 1)
        (convolutionPolynomial precision degree left right) := by
  ext index
  by_cases hindex : index < degree
  · let coordinateIndex : Fin degree := ⟨index, hindex⟩
    change
      (coordinatePolynomial precision degree
          (nativeProductCoordinates precision degree lowerCoefficient left right)).coeff
          coordinateIndex = _
    rw [coordinatePolynomial_coeff, nativeProductCoordinates,
      nativeConvolutionReduceDescending_refines]
  · have hge : degree ≤ index := by omega
    rw [coordinatePolynomial_coeff_eq_zero_of_le precision degree index _ hge,
      convolutionReduction_coeff_eq_zero precision degree index lowerCoefficient
        left right hdegree hge]

/-- The complete zero-initialized convolution, descending native reduction, and
truncation loop decodes to multiplication in the formal native quotient ring. -/
theorem decodeNativeProductCoordinates
    (precision degree : ℕ) (lowerCoefficient : Fin degree → Bool)
    (left right : Fin degree → ZMod (2 ^ precision)) (hdegree : 0 < degree) :
    decodePowerBasis precision degree lowerCoefficient
        (nativeProductCoordinates precision degree lowerCoefficient left right) =
      decodePowerBasis precision degree lowerCoefficient left *
        decodePowerBasis precision degree lowerCoefficient right := by
  calc
    decodePowerBasis precision degree lowerCoefficient
        (nativeProductCoordinates precision degree lowerCoefficient left right) =
        AdjoinRoot.mk (binaryPolynomial precision degree lowerCoefficient)
          (coordinatePolynomial precision degree
            (nativeProductCoordinates precision degree lowerCoefficient left right)) :=
      (coordinatePolynomial_mk precision degree lowerCoefficient _).symm
    _ = AdjoinRoot.mk (binaryPolynomial precision degree lowerCoefficient)
          (binaryReduceDescending precision degree lowerCoefficient (degree - 1)
            (convolutionPolynomial precision degree left right)) := by
      rw [nativeProductPolynomial_eq precision degree lowerCoefficient left right
        hdegree]
    _ = AdjoinRoot.mk (binaryPolynomial precision degree lowerCoefficient)
          (convolutionPolynomial precision degree left right) :=
      binaryReduceDescending_mk precision degree lowerCoefficient (degree - 1) _
    _ = AdjoinRoot.mk (binaryPolynomial precision degree lowerCoefficient)
          (coordinatePolynomial precision degree left *
            coordinatePolynomial precision degree right) := by
      rw [coordinatePolynomial_mul]
    _ = decodePowerBasis precision degree lowerCoefficient left *
          decodePowerBasis precision degree lowerCoefficient right :=
      (decodePowerBasis_mul_eq_mk precision degree lowerCoefficient left right).symm

/-- Canonical comma-separated multiplication witness consumed by both the native
control and this formal model. The fields are degree, precision, packed
polynomial, lower bits, left/right/product coordinates, and transition counts. -/
def degreeFourWitnessArtifact : String :=
  include_str ".." / "ring_multiplication_witness.csv"

/-- Transparent decimal digit parser used by the kernel-checked artifact reader. -/
def degreeFourWitnessDigit : Char → Option ℕ
  | '0' => some 0
  | '1' => some 1
  | '2' => some 2
  | '3' => some 3
  | '4' => some 4
  | '5' => some 5
  | '6' => some 6
  | '7' => some 7
  | '8' => some 8
  | '9' => some 9
  | _ => none

/-- Transparent comma-separated decimal parser; the final newline is required. -/
def parseDegreeFourWitness : List Char → ℕ → List ℕ → Option (List ℕ)
  | [], _, _ => none
  | '\n' :: [], current, values => some ((current :: values).reverse)
  | ',' :: rest, current, values =>
      parseDegreeFourWitness rest 0 (current :: values)
  | character :: rest, current, values => do
      let digit ← degreeFourWitnessDigit character
      parseDegreeFourWitness rest (10 * current + digit) values

/-- Parse the shared decimal witness without trusting an opaque runtime parser. -/
def degreeFourWitnessNumbers : List ℕ :=
  (parseDegreeFourWitness degreeFourWitnessArtifact.toList 0 []).getD []

/-- The committed shared artifact has the exact accepted native fields. -/
theorem degreeFourWitnessArtifactValues :
    degreeFourWitnessNumbers =
      [4, 5, 19, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1,
        30, 29, 0, 3, 16, 6] := by
  decide

/-- Typed form of one native multiplication replay artifact. The parser below
checks all four degree-sized vector boundaries before constructing this value. -/
structure RingMultiplicationArtifact where
  degree : ℕ
  precision : ℕ
  polynomial : ℕ
  lowerCoefficients : List Bool
  left : List ℕ
  right : List ℕ
  product : List ℕ
  convolutionProducts : ℕ
  reductions : ℕ
  deriving DecidableEq, Inhabited, Repr

/-- Transparent shape predicate required before a parsed artifact can reach the
model. Keeping it Boolean preserves kernel computation for included fixtures. -/
def RingMultiplicationArtifact.wellFormed
    (artifact : RingMultiplicationArtifact) : Bool :=
  artifact.lowerCoefficients.length == artifact.degree &&
    artifact.left.length == artifact.degree &&
    artifact.right.length == artifact.degree &&
    artifact.product.length == artifact.degree

/-- Decode a native lower-polynomial bit and reject non-Boolean fields. -/
def ringMultiplicationArtifactBit : ℕ → Option Bool
  | 0 => some false
  | 1 => some true
  | _ => none

/-- Split the flat native record into named fields. The checked wrapper below
enforces the exact vector lengths before exposing a successful result. -/
def parseRingMultiplicationArtifactUnchecked :
    List ℕ → Option RingMultiplicationArtifact
  | degree :: precision :: polynomial :: fields => do
      let (lowerNumbers, afterLower) := fields.splitAt degree
      let (left, afterLeft) := afterLower.splitAt degree
      let (right, afterRight) := afterLeft.splitAt degree
      let (product, counts) := afterRight.splitAt degree
      let lowerCoefficients ← lowerNumbers.mapM ringMultiplicationArtifactBit
      match counts with
      | [convolutionProducts, reductions] =>
          some {
            degree, precision, polynomial, lowerCoefficients, left, right,
            product, convolutionProducts, reductions
          }
      | _ => none
  | _ => none

/-- Fail closed unless all four decoded vectors match the encoded degree. -/
def validateRingMultiplicationArtifact
    (artifact : RingMultiplicationArtifact) : Option RingMultiplicationArtifact :=
  if artifact.wellFormed then some artifact else none

/-- Parse the flat native record into named, length-checked fields. -/
def parseRingMultiplicationArtifact
    (values : List ℕ) : Option RingMultiplicationArtifact := do
  let artifact ← parseRingMultiplicationArtifactUnchecked values
  validateRingMultiplicationArtifact artifact

/-- Any successfully parsed multiplication artifact has all four vector lengths
equal to its encoded degree. -/
theorem parseRingMultiplicationArtifact_wellFormed
    {values : List ℕ} {artifact : RingMultiplicationArtifact}
    (hparse : parseRingMultiplicationArtifact values = some artifact) :
    artifact.wellFormed = true := by
  unfold parseRingMultiplicationArtifact at hparse
  rcases Option.bind_eq_some_iff.mp hparse with
    ⟨candidate, _, hvalidate⟩
  unfold validateRingMultiplicationArtifact at hvalidate
  split at hvalidate
  next hwellFormed =>
    have hartifact : candidate = artifact := Option.some.inj hvalidate
    simpa only [hartifact] using hwellFormed
  next =>
    simp at hvalidate

/-- Recompute the packed monic binary polynomial from the named lower bits. -/
def RingMultiplicationArtifact.expectedPolynomial
    (artifact : RingMultiplicationArtifact) : ℕ :=
  (List.range artifact.degree).foldl
    (fun polynomial index =>
      if artifact.lowerCoefficients.getD index false then
        polynomial + 2 ^ index
      else
        polynomial)
    (2 ^ artifact.degree)

/-- Every serialized coordinate is canonical modulo `2^precision`. -/
def RingMultiplicationArtifact.canonicalCoordinates
    (artifact : RingMultiplicationArtifact) : Bool :=
  let modulus := 2 ^ artifact.precision
  artifact.left.all (fun coordinate => decide (coordinate < modulus)) &&
    artifact.right.all (fun coordinate => decide (coordinate < modulus)) &&
    artifact.product.all (fun coordinate => decide (coordinate < modulus))

/-- Shape, packed-polynomial consistency, and canonical coordinate bounds. -/
def RingMultiplicationArtifact.canonical
    (artifact : RingMultiplicationArtifact) : Bool :=
  artifact.wellFormed &&
    artifact.polynomial == artifact.expectedPolynomial &&
    artifact.canonicalCoordinates

/-- Fail closed unless a shaped record is also algebraically canonical. -/
def validateCanonicalRingMultiplicationArtifact
    (artifact : RingMultiplicationArtifact) : Option RingMultiplicationArtifact :=
  if artifact.canonical then some artifact else none

/-- Checked import boundary for an algebraically canonical multiplication
record. -/
def parseCanonicalRingMultiplicationArtifact
    (values : List ℕ) : Option RingMultiplicationArtifact := do
  let artifact ← parseRingMultiplicationArtifact values
  validateCanonicalRingMultiplicationArtifact artifact

/-- Every successfully imported canonical record has consistent polynomial bits
and coordinate representatives as well as exact vector lengths. -/
theorem parseCanonicalRingMultiplicationArtifact_canonical
    {values : List ℕ} {artifact : RingMultiplicationArtifact}
    (hparse : parseCanonicalRingMultiplicationArtifact values = some artifact) :
    artifact.canonical = true := by
  unfold parseCanonicalRingMultiplicationArtifact at hparse
  rcases Option.bind_eq_some_iff.mp hparse with
    ⟨candidate, _, hvalidate⟩
  unfold validateCanonicalRingMultiplicationArtifact at hvalidate
  split at hvalidate
  next hcanonical =>
    have hartifact : candidate = artifact := Option.some.inj hvalidate
    simpa only [hartifact] using hcanonical
  next =>
    simp at hvalidate

/-- Decode the low `width` bits of a packed natural number as a polynomial over
`GF(2)`. This is the coefficient-level interface between native bit operations
and polynomial semantics. -/
noncomputable def packedGF2Polynomial (width value : ℕ) :
    Polynomial (ZMod 2) :=
  ∑ index : Fin width,
    if value.testBit index then monomial (index : ℕ) 1 else 0

/-- Every in-range coefficient of the packed polynomial is exactly the
corresponding native bit. -/
theorem packedGF2Polynomial_coeff (width value : ℕ) (index : Fin width) :
    (packedGF2Polynomial width value).coeff index =
      if value.testBit index then 1 else 0 := by
  let coefficient : Polynomial (ZMod 2) →+ ZMod 2 :=
    { toFun := fun polynomial => polynomial.coeff index
      map_zero' := coeff_zero index
      map_add' := fun left right => coeff_add left right index }
  rw [packedGF2Polynomial]
  change coefficient (∑ candidate : Fin width,
    if value.testBit candidate then monomial (candidate : ℕ) 1 else 0) = _
  rw [map_sum]
  have hself :
      coefficient (if value.testBit index then monomial (index : ℕ) 1 else 0) =
        (if value.testBit index then 1 else 0) := by
    by_cases hbit : value.testBit index <;> simp [hbit, coefficient]
  rw [← hself]
  apply Finset.sum_eq_single index
  · intro candidate _ hne
    have hcoe : (candidate : ℕ) ≠ (index : ℕ) := by
      intro heq
      exact hne (Fin.ext heq)
    by_cases hbit : value.testBit candidate <;>
      simp [hbit, coefficient, coeff_monomial, hcoe]
  · simp

/-- At every fixed width, native packed XOR is polynomial addition over
`GF(2)`. -/
theorem packedGF2Polynomial_xor (width left right : ℕ) :
    packedGF2Polynomial width (Nat.xor left right) =
      packedGF2Polynomial width left + packedGF2Polynomial width right := by
  rw [packedGF2Polynomial, packedGF2Polynomial, packedGF2Polynomial,
    ← Finset.sum_add_distrib]
  apply Finset.sum_congr rfl
  intro index _
  have hxor : (Nat.xor left right).testBit (index : ℕ) =
      ((left.testBit index) ^^ (right.testBit index)) := by
    change (left ^^^ right).testBit (index : ℕ) = _
    exact Nat.testBit_xor left right index
  rw [hxor]
  cases hleft : left.testBit index <;>
    cases hright : right.testBit index <;>
    simp_all
  ext coefficient
  by_cases heq : (index : ℕ) = coefficient
  · simp [coeff_add, heq]
    decide
  · simp [coeff_add, coeff_monomial, heq]

/-- Multiplication by `2^shift` moves every in-range native bit to the matching
higher polynomial coefficient. -/
theorem packedGF2Polynomial_shifted_coeff
    (width value shift : ℕ) (index : Fin width) :
    (packedGF2Polynomial (width + shift) (value * 2 ^ shift)).coeff
        (index + shift) =
      if value.testBit index then 1 else 0 := by
  let shiftedIndex : Fin (width + shift) :=
    ⟨index + shift, by omega⟩
  have hcoeff := packedGF2Polynomial_coeff
    (width + shift) (value * 2 ^ shift) shiftedIndex
  change (packedGF2Polynomial (width + shift) (value * 2 ^ shift)).coeff
      (index + shift) = _
  rw [hcoeff]
  rw [← Nat.shiftLeft_eq_mul_pow, Nat.testBit_shiftLeft]
  simp [shiftedIndex]

/-- Multiplication by `2^shift` leaves every polynomial coefficient below the
shift equal to zero. -/
theorem packedGF2Polynomial_shifted_low_coeff
    (width value shift : ℕ) (index : Fin shift) :
    (packedGF2Polynomial (width + shift) (value * 2 ^ shift)).coeff index = 0 := by
  let lowIndex : Fin (width + shift) :=
    ⟨index, by omega⟩
  have hcoeff := packedGF2Polynomial_coeff
    (width + shift) (value * 2 ^ shift) lowIndex
  change (packedGF2Polynomial (width + shift) (value * 2 ^ shift)).coeff index = _
  rw [hcoeff]
  rw [← Nat.shiftLeft_eq_mul_pow, Nat.testBit_shiftLeft]
  simp [lowIndex, index.isLt]

/-- Once a packed value fits in `width` bits, decoding more high bits does not
change its polynomial. -/
theorem packedGF2Polynomial_extend (width extra value : ℕ)
    (hvalue : value < 2 ^ width) :
    packedGF2Polynomial (width + extra) value =
      packedGF2Polynomial width value := by
  rw [packedGF2Polynomial, Fin.sum_univ_add, packedGF2Polynomial]
  have hhigh :
      (∑ index : Fin extra,
        if value.testBit (Fin.natAdd width index) then
          monomial (Fin.natAdd width index : ℕ) (1 : ZMod 2)
        else 0) = 0 := by
    apply Finset.sum_eq_zero
    intro index _
    have hpow : 2 ^ width ≤ 2 ^ (Fin.natAdd width index : ℕ) :=
      Nat.pow_le_pow_right (by decide) (by simp)
    have hbit := Nat.testBit_eq_false_of_lt (lt_of_lt_of_le hvalue hpow)
    change value.testBit (width + (index : ℕ)) = false at hbit
    simp [hbit]
  rw [hhigh, add_zero]
  rfl

/-- Truncated packed decoding commutes exactly with multiplication by a power
of two: shifting the bit window is multiplication by the matching monomial. -/
theorem packedGF2Polynomial_shift (width value shift : ℕ) :
    packedGF2Polynomial (shift + width) (value * 2 ^ shift) =
      X ^ shift * packedGF2Polynomial width value := by
  rw [packedGF2Polynomial, Fin.sum_univ_add]
  have hlow :
      (∑ index : Fin shift,
        if (value * 2 ^ shift).testBit (Fin.castAdd width index) then
          monomial (Fin.castAdd width index : ℕ) (1 : ZMod 2)
        else 0) = 0 := by
    apply Finset.sum_eq_zero
    intro index _
    rw [← Nat.shiftLeft_eq_mul_pow, Nat.testBit_shiftLeft]
    simp [index.isLt]
  simp only [hlow, zero_add]
  rw [packedGF2Polynomial, Finset.mul_sum]
  apply Finset.sum_congr rfl
  intro index _
  rw [← Nat.shiftLeft_eq_mul_pow, Nat.testBit_shiftLeft]
  by_cases hbit : value.testBit index
  · simp [hbit, monomial_one_right_eq_X_pow]
    rw [pow_add]
  · simp [hbit]

/-- A shifted value inside a larger product window is exactly the corresponding
monomial multiple when the source value fits its declared width. -/
theorem packedGF2Polynomial_shift_of_bound
    (leftWidth rightWidth left index : ℕ)
    (hindex : index < rightWidth) (hleft : left < 2 ^ leftWidth) :
    packedGF2Polynomial (leftWidth + rightWidth) (left * 2 ^ index) =
      X ^ index * packedGF2Polynomial leftWidth left := by
  calc
    packedGF2Polynomial (leftWidth + rightWidth) (left * 2 ^ index) =
        packedGF2Polynomial
          (index + (leftWidth + (rightWidth - index)))
          (left * 2 ^ index) := by
      apply congrArg (fun width =>
        packedGF2Polynomial width (left * 2 ^ index))
      omega
    _ = X ^ index *
        packedGF2Polynomial (leftWidth + (rightWidth - index)) left :=
      packedGF2Polynomial_shift (leftWidth + (rightWidth - index)) left index
    _ = X ^ index * packedGF2Polynomial leftWidth left := by
      rw [packedGF2Polynomial_extend leftWidth (rightWidth - index) left hleft]

/-- One native remainder clearing step is polynomial addition by a monomial
multiple of the modulus. For a monic decoded modulus, the step preserves
`modByMonic` exactly. -/
theorem packedGF2Polynomial_remainderStep
    (modulusWidth extra value modulus shift : ℕ)
    (hshift : shift < extra) (hmodulus : modulus < 2 ^ modulusWidth)
    (hmonic : (packedGF2Polynomial modulusWidth modulus).Monic) :
    packedGF2Polynomial (modulusWidth + extra)
        (Nat.xor value (modulus * 2 ^ shift)) =
          packedGF2Polynomial (modulusWidth + extra) value +
            X ^ shift * packedGF2Polynomial modulusWidth modulus ∧
      (packedGF2Polynomial (modulusWidth + extra)
          (Nat.xor value (modulus * 2 ^ shift))) %ₘ
          packedGF2Polynomial modulusWidth modulus =
        (packedGF2Polynomial (modulusWidth + extra) value) %ₘ
          packedGF2Polynomial modulusWidth modulus := by
  have hstep :
      packedGF2Polynomial (modulusWidth + extra)
          (Nat.xor value (modulus * 2 ^ shift)) =
        packedGF2Polynomial (modulusWidth + extra) value +
          X ^ shift * packedGF2Polynomial modulusWidth modulus := by
    rw [packedGF2Polynomial_xor]
    rw [packedGF2Polynomial_shift_of_bound
      modulusWidth extra modulus shift hshift hmodulus]
  refine ⟨hstep, ?_⟩
  rw [hstep]
  apply Polynomial.modByMonic_eq_of_dvd_sub hmonic
  use X ^ shift
  ring

/-- Any bounded list of native packed clearing shifts preserves the same
polynomial residue class. -/
theorem packedGF2Polynomial_remainderSteps
    (modulusWidth extra initial modulus : ℕ) (shifts : List ℕ)
    (hshifts : ∀ shift ∈ shifts, shift < extra)
    (hmodulus : modulus < 2 ^ modulusWidth)
    (hmonic : (packedGF2Polynomial modulusWidth modulus).Monic) :
    (packedGF2Polynomial (modulusWidth + extra)
        (shifts.foldl
          (fun value shift => Nat.xor value (modulus * 2 ^ shift))
          initial)) %ₘ packedGF2Polynomial modulusWidth modulus =
      (packedGF2Polynomial (modulusWidth + extra) initial) %ₘ
        packedGF2Polynomial modulusWidth modulus := by
  induction shifts generalizing initial with
  | nil => rfl
  | cons head tail ih =>
      simp only [List.foldl_cons]
      have hhead : head < extra := hshifts head (by simp)
      have htail : ∀ shift ∈ tail, shift < extra := by
        intro shift hmem
        exact hshifts shift (by simp [hmem])
      calc
        (packedGF2Polynomial (modulusWidth + extra)
            (tail.foldl
              (fun value shift => Nat.xor value (modulus * 2 ^ shift))
              (Nat.xor initial (modulus * 2 ^ head)))) %ₘ
              packedGF2Polynomial modulusWidth modulus =
          (packedGF2Polynomial (modulusWidth + extra)
            (Nat.xor initial (modulus * 2 ^ head))) %ₘ
              packedGF2Polynomial modulusWidth modulus :=
            ih (Nat.xor initial (modulus * 2 ^ head)) htail
        _ = (packedGF2Polynomial (modulusWidth + extra) initial) %ₘ
              packedGF2Polynomial modulusWidth modulus :=
          (packedGF2Polynomial_remainderStep modulusWidth extra initial
            modulus head hhead hmodulus hmonic).2

/-- Fuel-bounded polynomial remainder over `GF(2)`, using packed coefficient
bits exactly as the native ring search does. -/
def gf2PackedRemainderFuel : ℕ → ℕ → ℕ → ℕ
  | 0, value, _ => value
  | fuel + 1, value, modulus =>
      if value == 0 || decide (Nat.log2 value < Nat.log2 modulus) then
        value
      else
        let shift := Nat.log2 value - Nat.log2 modulus
        gf2PackedRemainderFuel fuel
          (Nat.xor value (modulus * 2 ^ shift)) modulus

/-- Packed polynomial remainder with enough fuel to clear every source degree. -/
def gf2PackedRemainder (value modulus : ℕ) : ℕ :=
  gf2PackedRemainderFuel (Nat.log2 value + 1) value modulus

/-- The exact sequence of shift offsets taken by the fuel-bounded packed
remainder loop. This trace is computational and carries no trusted result. -/
def gf2PackedRemainderShiftsFuel : ℕ → ℕ → ℕ → List ℕ
  | 0, _, _ => []
  | fuel + 1, value, modulus =>
      if value == 0 || decide (Nat.log2 value < Nat.log2 modulus) then
        []
      else
        let shift := Nat.log2 value - Nat.log2 modulus
        shift :: gf2PackedRemainderShiftsFuel fuel
          (Nat.xor value (modulus * 2 ^ shift)) modulus

/-- Replaying the emitted shift trace with the native XOR update reproduces
the fuel-bounded remainder result exactly. -/
theorem gf2PackedRemainderFuel_eq_fold_shifts
    (fuel value modulus : ℕ) :
    gf2PackedRemainderFuel fuel value modulus =
      (gf2PackedRemainderShiftsFuel fuel value modulus).foldl
        (fun current shift => Nat.xor current (modulus * 2 ^ shift))
        value := by
  induction fuel generalizing value with
  | zero => rfl
  | succ remaining ih =>
      rw [gf2PackedRemainderFuel, gf2PackedRemainderShiftsFuel]
      split
      next hstop => rfl
      next hcontinue =>
        simp only [List.foldl_cons]
        exact ih (Nat.xor value
          (modulus * 2 ^ (Nat.log2 value - Nat.log2 modulus)))

/-- Conditional semantic refinement for the fuel-bounded native remainder:
if every emitted shift fits the declared extra window, the decoded result has
the same polynomial residue as the input. -/
theorem packedGF2Polynomial_remainderFuel_residue
    (fuel modulusWidth extra value modulus : ℕ)
    (hshifts : ∀ shift ∈ gf2PackedRemainderShiftsFuel fuel value modulus,
      shift < extra)
    (hmodulus : modulus < 2 ^ modulusWidth)
    (hmonic : (packedGF2Polynomial modulusWidth modulus).Monic) :
    (packedGF2Polynomial (modulusWidth + extra)
        (gf2PackedRemainderFuel fuel value modulus)) %ₘ
          packedGF2Polynomial modulusWidth modulus =
      (packedGF2Polynomial (modulusWidth + extra) value) %ₘ
        packedGF2Polynomial modulusWidth modulus := by
  rw [gf2PackedRemainderFuel_eq_fold_shifts]
  exact packedGF2Polynomial_remainderSteps modulusWidth extra value modulus
    (gf2PackedRemainderShiftsFuel fuel value modulus) hshifts hmodulus hmonic

/-- Public packed remainder preserves polynomial residue under the explicit
monicity, modulus-width, and emitted-shift window hypotheses. -/
theorem packedGF2Polynomial_remainder_residue
    (modulusWidth extra value modulus : ℕ)
    (hshifts : ∀ shift ∈ gf2PackedRemainderShiftsFuel
        (Nat.log2 value + 1) value modulus, shift < extra)
    (hmodulus : modulus < 2 ^ modulusWidth)
    (hmonic : (packedGF2Polynomial modulusWidth modulus).Monic) :
    (packedGF2Polynomial (modulusWidth + extra)
        (gf2PackedRemainder value modulus)) %ₘ
          packedGF2Polynomial modulusWidth modulus =
      (packedGF2Polynomial (modulusWidth + extra) value) %ₘ
        packedGF2Polynomial modulusWidth modulus := by
  exact packedGF2Polynomial_remainderFuel_residue
    (Nat.log2 value + 1) modulusWidth extra value modulus
    hshifts hmodulus hmonic

/-- Unreduced carryless product of two packed `GF(2)` polynomials. -/
def gf2PackedRawProduct (left right : ℕ) : ℕ :=
  (List.range (Nat.log2 right + 1)).foldl
    (fun product index =>
      if right.testBit index then
        Nat.xor product (left * 2 ^ index)
      else
        product)
    0

/-- The native carryless-product fold decodes to the same fold with packed XOR
replaced by polynomial addition. Together with the shift coefficient theorems,
this is the reusable bit-to-polynomial boundary for raw multiplication. -/
theorem packedGF2Polynomial_rawProduct_fold (width left right : ℕ) :
    packedGF2Polynomial width (gf2PackedRawProduct left right) =
      (List.range (Nat.log2 right + 1)).foldl
        (fun product index =>
          if right.testBit index then
            product + packedGF2Polynomial width (left * 2 ^ index)
          else product)
        0 := by
  unfold gf2PackedRawProduct
  generalize List.range (Nat.log2 right + 1) = indices
  have hfold : ∀ initial,
      packedGF2Polynomial width
          (indices.foldl
            (fun product index =>
              if right.testBit index then
                Nat.xor product (left * 2 ^ index)
              else product)
            initial) =
        indices.foldl
          (fun product index =>
            if right.testBit index then
              product + packedGF2Polynomial width (left * 2 ^ index)
            else product)
          (packedGF2Polynomial width initial) := by
    intro initial
    induction indices generalizing initial with
    | nil => simp
    | cons head tail ih =>
        simp only [List.foldl_cons]
        by_cases hbit : right.testBit head
        · simp only [hbit, ite_true]
          rw [ih, packedGF2Polynomial_xor]
        · simp only [hbit]
          exact ih initial
  rw [hfold 0]
  simp [packedGF2Polynomial]

/-- With an explicit source-width bound, the complete native carryless product
decodes to ordinary polynomial multiplication over `GF(2)`. The right width is
the exact native loop bound `log2 right + 1`. -/
theorem packedGF2Polynomial_rawProduct_eq_mul
    (leftWidth left right : ℕ) (hleft : left < 2 ^ leftWidth) :
    packedGF2Polynomial (leftWidth + (Nat.log2 right + 1))
        (gf2PackedRawProduct left right) =
      packedGF2Polynomial leftWidth left *
        packedGF2Polynomial (Nat.log2 right + 1) right := by
  rw [packedGF2Polynomial_rawProduct_fold]
  let rightWidth := Nat.log2 right + 1
  have hfoldAux : ∀ count : ℕ,
      (List.range count).foldl
          (fun product index =>
            if right.testBit index then
              product + packedGF2Polynomial (leftWidth + rightWidth)
                (left * 2 ^ index)
            else product)
          0 =
        ∑ index ∈ Finset.range count,
          if right.testBit index then
            packedGF2Polynomial (leftWidth + rightWidth) (left * 2 ^ index)
          else 0 := by
    intro count
    induction count with
    | zero => simp
    | succ width ih =>
        rw [List.range_succ, List.foldl_append, ih, Finset.sum_range_succ]
        by_cases hbit : right.testBit width <;> simp [hbit]
  have hfold := hfoldAux rightWidth
  change (List.range rightWidth).foldl
      (fun product index =>
        if right.testBit index then
          product + packedGF2Polynomial (leftWidth + rightWidth)
            (left * 2 ^ index)
        else product)
      0 = _
  rw [hfold]
  change (∑ index ∈ Finset.range rightWidth,
      if right.testBit index then
        packedGF2Polynomial (leftWidth + rightWidth) (left * 2 ^ index)
      else 0) =
    packedGF2Polynomial leftWidth left *
      (∑ index : Fin rightWidth,
        if right.testBit index then monomial (index : ℕ) 1 else 0)
  rw [Fin.sum_univ_eq_sum_range
    (fun index : ℕ =>
      if right.testBit index then monomial index (1 : ZMod 2) else 0)
    rightWidth, Finset.mul_sum]
  apply Finset.sum_congr rfl
  intro index hindex
  have hindex' : index < rightWidth := Finset.mem_range.mp hindex
  rw [packedGF2Polynomial_shift_of_bound
    leftWidth rightWidth left index hindex' hleft]
  by_cases hbit : right.testBit index
  · simp [hbit, monomial_one_right_eq_X_pow, mul_comm]
  · simp [hbit]

/-- Carryless product reduced by a packed modulus. -/
def gf2PackedProduct (left right modulus : ℕ) : ℕ :=
  gf2PackedRemainder (gf2PackedRawProduct left right) modulus

/-- Fuel-bounded Euclidean gcd over packed `GF(2)` polynomials. -/
def gf2PackedGcdFuel : ℕ → ℕ → ℕ → ℕ
  | 0, left, _ => left
  | fuel + 1, left, right =>
      if right == 0 then
        left
      else
        gf2PackedGcdFuel fuel right (gf2PackedRemainder left right)

/-- Packed polynomial gcd with the sum-of-degrees Euclidean fuel bound. -/
def gf2PackedGcd (left right : ℕ) : ℕ :=
  gf2PackedGcdFuel (Nat.log2 left + Nat.log2 right + 2) left right

/-- Native-shaped Rabin loop: repeated Frobenius squaring, early gcd checks
through half the degree, and the final `x^(2^degree)=x` equality. -/
def gf2PackedIrreducibleAux (polynomial x degree : ℕ) : ℕ → ℕ → ℕ → Bool
  | 0, _, value => value == x
  | remaining + 1, index, value =>
      let next := gf2PackedProduct value value polynomial
      let nextIndex := index + 1
      if decide (nextIndex ≤ degree / 2) &&
          !(gf2PackedGcd (Nat.xor next x) polynomial == 1) then
        false
      else
        gf2PackedIrreducibleAux polynomial x degree remaining nextIndex next

/-- Rabin irreducibility predicate used by the deterministic packed search. -/
def gf2PackedIrreducible (polynomial degree : ℕ) : Bool :=
  let x := gf2PackedRemainder 2 polynomial
  gf2PackedIrreducibleAux polynomial x degree degree 0 x

/-- Search odd constant-one lower patterns in the exact native order. -/
def firstIrreducibleBinaryPolynomialAux (degree : ℕ) : ℕ → ℕ → Option ℕ
  | _, 0 => none
  | lower, remaining + 1 =>
      let polynomial := 2 ^ degree + lower
      if gf2PackedIrreducible polynomial degree then
        some polynomial
      else
        firstIrreducibleBinaryPolynomialAux degree (lower + 2) remaining

/-- Deterministic monic binary-polynomial selection modeled after `Ring::new`. -/
def firstIrreducibleBinaryPolynomial (degree : ℕ) : Option ℕ :=
  if degree == 0 then none
  else firstIrreducibleBinaryPolynomialAux degree 1 (2 ^ (degree - 1))

/-- At degree four the first odd candidate `U^4+1` is rejected while
`U^4+U+1` is accepted by the native-shaped Rabin predicate. -/
theorem degreeFourIrreducibilityBoundary :
    gf2PackedIrreducible 17 4 = false ∧
      gf2PackedIrreducible 19 4 = true := by
  decide

/-- The transparent deterministic search selects the artifact polynomial `19`. -/
theorem degreeFourDeterministicPolynomialSelection :
    firstIrreducibleBinaryPolynomial 4 = some 19 := by
  decide

/-- Independent algebraic certificate for the degree-four polynomial accepted
by the packed search. The proof exhausts the two monic linear and four monic
quadratic divisor shapes over `GF(2)`, rather than trusting the packed Rabin
predicate itself. -/
theorem degreeFourPolynomialGF2_irreducible :
    Irreducible (X ^ 4 + (X + 1) : Polynomial (ZMod 2)) := by
  let p : Polynomial (ZMod 2) := X ^ 4 + (X + 1)
  have hpShape : Polynomial.IsMonicOfDegree p 4 := by
    apply (Polynomial.isMonicOfDegree_X_pow (ZMod 2) 4).add_right
    norm_num [p]
  have hp : p.Monic := hpShape.monic
  have hpdeg : p.natDegree = 4 := hpShape.natDegree_eq
  have hp1 : p ≠ 1 := by
    intro h
    rw [h, natDegree_one] at hpdeg
    omega
  rw [hp.irreducible_iff_lt_natDegree_lt hp1]
  intro q hq hdegree hdiv
  rw [hpdeg] at hdegree
  simp only [Finset.mem_Ioc, Nat.reduceDiv] at hdegree
  have hcases : q.natDegree = 1 ∨ q.natDegree = 2 := by omega
  rcases hcases with hdegreeOne | hdegreeTwo
  · have hshape : Polynomial.IsMonicOfDegree q 1 := ⟨hdegreeOne, hq⟩
    obtain ⟨a, rfl⟩ := Polynomial.isMonicOfDegree_one_iff.mp hshape
    obtain ⟨r, hr⟩ := hdiv
    have heval := congrArg (Polynomial.eval (-a)) hr
    simp only [p, eval_add, eval_pow, eval_X, eval_one, eval_mul, eval_C,
      neg_add_cancel, zero_mul] at heval
    generalize a = a' at heval
    fin_cases a' <;> change (1 : ZMod 2) = 0 at heval <;>
      exact one_ne_zero heval
  · have hshape : Polynomial.IsMonicOfDegree q 2 := ⟨hdegreeTwo, hq⟩
    obtain ⟨a, b, rfl⟩ := Polynomial.isMonicOfDegree_two_iff.mp hshape
    obtain ⟨r, hr⟩ := hdiv
    have hproduct : Polynomial.IsMonicOfDegree
        ((X ^ 2 + C a * X + C b) * r) (2 + 2) := by
      rw [← hr]
      exact hpShape
    have hrshape : Polynomial.IsMonicOfDegree r 2 :=
      (Polynomial.isMonicOfDegree_add_add_two a b).of_mul_left hproduct
    obtain ⟨c, d, rfl⟩ := Polynomial.isMonicOfDegree_two_iff.mp hrshape
    have hexpand :
        (X ^ 2 + C a * X + C b) * (X ^ 2 + C c * X + C d) =
          X ^ 4 + C (a + c) * X ^ 3 + C (a * c + b + d) * X ^ 2 +
            C (a * d + b * c) * X + C (b * d) := by
      simp only [map_add, map_mul]
      ring
    rw [hexpand] at hr
    have hzero := congrArg (fun f : Polynomial (ZMod 2) => f.coeff 0) hr
    have hone := congrArg (fun f : Polynomial (ZMod 2) => f.coeff 1) hr
    have htwo := congrArg (fun f : Polynomial (ZMod 2) => f.coeff 2) hr
    have hthree := congrArg (fun f : Polynomial (ZMod 2) => f.coeff 3) hr
    simp [p, coeff_mul_X_pow', coeff_one, coeff_X] at hzero hone htwo hthree
    have hnoFactorCoefficients : ∀ a b c d : ZMod 2,
        ¬(1 = b * d ∧ 1 = a * d + b * c ∧
          0 = a * c + b + d ∧ 0 = a + c) := by
      decide
    exact hnoFactorCoefficients a b c d
      ⟨hzero, hone, htwo, hthree⟩

/-- Degree-indexed lower-polynomial view of an arbitrary typed artifact. -/
def RingMultiplicationArtifact.lowerCoefficient
    (artifact : RingMultiplicationArtifact) (index : Fin artifact.degree) : Bool :=
  artifact.lowerCoefficients.getD index.val false

/-- Degree-indexed canonical left coordinates of an arbitrary typed artifact. -/
def RingMultiplicationArtifact.leftCoordinates
    (artifact : RingMultiplicationArtifact)
    (index : Fin artifact.degree) : ZMod (2 ^ artifact.precision) :=
  artifact.left.getD index.val 0

/-- Degree-indexed canonical right coordinates of an arbitrary typed artifact. -/
def RingMultiplicationArtifact.rightCoordinates
    (artifact : RingMultiplicationArtifact)
    (index : Fin artifact.degree) : ZMod (2 ^ artifact.precision) :=
  artifact.right.getD index.val 0

/-- Degree-indexed canonical product coordinates of an arbitrary typed artifact. -/
def RingMultiplicationArtifact.productCoordinates
    (artifact : RingMultiplicationArtifact)
    (index : Fin artifact.degree) : ZMod (2 ^ artifact.precision) :=
  artifact.product.getD index.val 0

/-- Generic checked-import endpoint. Once an independent replay establishes that
the serialized product equals the universal native transition function, the
imported coordinates decode as quotient-ring multiplication. -/
theorem parsedCanonicalRingMultiplicationArtifact_decodes_mul
    {values : List ℕ} {artifact : RingMultiplicationArtifact}
    (hparse : parseCanonicalRingMultiplicationArtifact values = some artifact)
    (hdegree : 0 < artifact.degree)
    (hproduct : artifact.productCoordinates =
      nativeProductCoordinates artifact.precision artifact.degree
        artifact.lowerCoefficient artifact.leftCoordinates
        artifact.rightCoordinates) :
    artifact.canonical = true ∧
      decodePowerBasis artifact.precision artifact.degree
          artifact.lowerCoefficient artifact.productCoordinates =
        decodePowerBasis artifact.precision artifact.degree
            artifact.lowerCoefficient artifact.leftCoordinates *
          decodePowerBasis artifact.precision artifact.degree
            artifact.lowerCoefficient artifact.rightCoordinates := by
  constructor
  · exact parseCanonicalRingMultiplicationArtifact_canonical hparse
  · rw [hproduct]
    exact decodeNativeProductCoordinates artifact.precision artifact.degree
      artifact.lowerCoefficient artifact.leftCoordinates
      artifact.rightCoordinates hdegree

/-- Typed, length-checked view of the committed degree-four artifact. -/
def degreeFourWitnessRecord : RingMultiplicationArtifact :=
  (parseCanonicalRingMultiplicationArtifact degreeFourWitnessNumbers).getD default

/-- The shared bytes decode to every named witness field, including both input
vectors and the exact transition counts. -/
theorem degreeFourWitnessRecordValues :
    degreeFourWitnessRecord = {
      degree := 4
      precision := 5
      polynomial := 19
      lowerCoefficients := [true, true, false, false]
      left := [1, 1, 1, 1]
      right := [1, 1, 1, 1]
      product := [30, 29, 0, 3]
      convolutionProducts := 16
      reductions := 6
    } := by
  decide

/-- The committed record passes the generic canonical import boundary. -/
theorem degreeFourWitnessRecordCanonical :
    degreeFourWitnessRecord.canonical = true := by
  decide

/-- The artifact polynomial is exactly the result of the transparent
native-shaped deterministic search at its encoded degree. -/
theorem degreeFourWitnessRecord_polynomialSelected :
    firstIrreducibleBinaryPolynomial degreeFourWitnessRecord.degree =
      some degreeFourWitnessRecord.polynomial := by
  decide

/-- The committed bytes are accepted by the public canonical parser, rather
than reaching the record through its fallback value. -/
theorem degreeFourWitnessRecordAccepted :
    parseCanonicalRingMultiplicationArtifact degreeFourWitnessNumbers =
      some degreeFourWitnessRecord := by
  decide

/-- The exact lower-bit vector selected by native `Ring::new(4, 5)`: the
irreducible binary polynomial `U^4 + U + 1`, read from the shared artifact. -/
def degreeFourLowerCoefficient (index : Fin 4) : Bool :=
  degreeFourWitnessRecord.lowerCoefficients.getD index.val false

/-- The typed artifact's lower-bit vector decodes over `GF(2)` to the exact
quartic certified independently above. -/
theorem degreeFourWitnessRecord_polynomialGF2 :
    binaryPolynomial 1 4 degreeFourLowerCoefficient =
      (X ^ 4 + (X + 1) : Polynomial (ZMod 2)) := by
  rw [binaryPolynomial]
  norm_num [degreeFourLowerCoefficient, degreeFourWitnessRecordValues,
    Fin.sum_univ_succ]
  simp [monomial_one_right_eq_X_pow, add_comm]

/-- The committed artifact's selected binary polynomial is algebraically
irreducible over `GF(2)`, independently of the packed search predicate. -/
theorem degreeFourWitnessRecord_polynomialIrreducible :
    Irreducible (binaryPolynomial 1 4 degreeFourLowerCoefficient) := by
  rw [degreeFourWitnessRecord_polynomialGF2]
  exact degreeFourPolynomialGF2_irreducible

/-- Combined degree-four endpoint: the deterministic search selects the
artifact's packed polynomial and its decoded `GF(2)` polynomial is irreducible. -/
theorem degreeFourWitnessRecord_selectionAndIrreducibility :
    firstIrreducibleBinaryPolynomial degreeFourWitnessRecord.degree =
        some degreeFourWitnessRecord.polynomial ∧
      Irreducible (binaryPolynomial 1 4 degreeFourLowerCoefficient) := by
  exact ⟨degreeFourWitnessRecord_polynomialSelected,
    degreeFourWitnessRecord_polynomialIrreducible⟩

/-- The dense canonical left coordinate vector in the typed artifact. -/
def degreeFourDenseLeftCoordinates (index : Fin 4) : ZMod (2 ^ 5) :=
  degreeFourWitnessRecord.left.getD index.val 0

/-- The dense canonical right coordinate vector in the typed artifact. -/
def degreeFourDenseRightCoordinates (index : Fin 4) : ZMod (2 ^ 5) :=
  degreeFourWitnessRecord.right.getD index.val 0

/-- The exact canonical native product serialized by the same control. -/
def degreeFourDenseProductCoordinates (index : Fin 4) : ZMod (2 ^ 5) :=
  degreeFourWitnessRecord.product.getD index.val 0

/-- The serialized packed polynomial `19` is exactly the monic degree-four bit
plus the accepted lower-bit vector. -/
theorem degreeFourPolynomialBits :
    degreeFourWitnessRecord.polynomial = 2 ^ 4 + ∑ index : Fin 4,
      if degreeFourLowerCoefficient index then 2 ^ (index : ℕ) else 0 := by
  decide

/-- Kernel-checked concrete counterpart of the native dense multiplication
witness: `[1,1,1,1]^2 = [30,29,0,3]` modulo 32 and `U^4+U+1`. -/
theorem degreeFourDenseNativeProduct :
    nativeProductCoordinates 5 4 degreeFourLowerCoefficient
        degreeFourDenseLeftCoordinates degreeFourDenseRightCoordinates =
      degreeFourDenseProductCoordinates := by
  funext index
  fin_cases index <;>
    norm_num [nativeProductCoordinates, nativeReduceBufferDescending,
      nativeReductionBufferRow_apply, convolutionBuffer_apply,
      convolutionCoefficient, degreeFourLowerCoefficient,
      degreeFourDenseLeftCoordinates, degreeFourDenseRightCoordinates,
      degreeFourDenseProductCoordinates, degreeFourWitnessRecord,
      degreeFourWitnessNumbers, degreeFourWitnessArtifact,
      Fin.sum_univ_succ] <;>
    decide

/-- The committed record discharges the sole arithmetic replay premise of the
generic imported-record endpoint. -/
theorem degreeFourWitnessRecord_productReplay :
    degreeFourWitnessRecord.productCoordinates =
      nativeProductCoordinates degreeFourWitnessRecord.precision
        degreeFourWitnessRecord.degree degreeFourWitnessRecord.lowerCoefficient
        degreeFourWitnessRecord.leftCoordinates
        degreeFourWitnessRecord.rightCoordinates := by
  rw [degreeFourWitnessRecordValues]
  funext index
  fin_cases index <;>
    norm_num [nativeProductCoordinates, nativeReduceBufferDescending,
      nativeReductionBufferRow_apply, convolutionBuffer_apply,
      convolutionCoefficient, RingMultiplicationArtifact.productCoordinates,
      RingMultiplicationArtifact.lowerCoefficient,
      RingMultiplicationArtifact.leftCoordinates,
      RingMultiplicationArtifact.rightCoordinates, Fin.sum_univ_succ] <;>
    decide

/-- The committed fixture instantiates the arbitrary-degree checked-import
endpoint, including its canonicality receipt and decoded product. -/
theorem degreeFourWitnessRecord_generic_decodes_mul :
    degreeFourWitnessRecord.canonical = true ∧
      decodePowerBasis degreeFourWitnessRecord.precision
          degreeFourWitnessRecord.degree degreeFourWitnessRecord.lowerCoefficient
          degreeFourWitnessRecord.productCoordinates =
        decodePowerBasis degreeFourWitnessRecord.precision
            degreeFourWitnessRecord.degree
            degreeFourWitnessRecord.lowerCoefficient
            degreeFourWitnessRecord.leftCoordinates *
          decodePowerBasis degreeFourWitnessRecord.precision
            degreeFourWitnessRecord.degree
            degreeFourWitnessRecord.lowerCoefficient
            degreeFourWitnessRecord.rightCoordinates := by
  exact parsedCanonicalRingMultiplicationArtifact_decodes_mul
    degreeFourWitnessRecordAccepted (by decide)
    degreeFourWitnessRecord_productReplay

/-- The typed shared record reaches the universal quotient-product theorem:
its named product coordinates decode as its named left/right product. -/
theorem degreeFourWitnessRecord_decodes_mul :
    decodePowerBasis 5 4 degreeFourLowerCoefficient
        degreeFourDenseProductCoordinates =
      decodePowerBasis 5 4 degreeFourLowerCoefficient
          degreeFourDenseLeftCoordinates *
        decodePowerBasis 5 4 degreeFourLowerCoefficient
          degreeFourDenseRightCoordinates := by
  rw [← degreeFourDenseNativeProduct]
  exact decodeNativeProductCoordinates 5 4 degreeFourLowerCoefficient
    degreeFourDenseLeftCoordinates degreeFourDenseRightCoordinates (by decide)

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
