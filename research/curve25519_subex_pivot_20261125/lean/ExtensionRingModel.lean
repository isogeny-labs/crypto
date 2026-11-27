module

public import GroupFactorization
public import Mathlib.Data.Fintype.Basic
public import Mathlib.Data.Nat.Bitwise
public import Mathlib.Algebra.Polynomial.Degree.IsMonicOfDegree
public import Mathlib.FieldTheory.Finite.Extension
public import Mathlib.RingTheory.Polynomial.UniqueFactorization

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

/-- Packed decoding has no coefficient at or above its fixed bit window. -/
theorem packedGF2Polynomial_coeff_eq_zero_of_le
    (width value index : ℕ) (hindex : width ≤ index) :
    (packedGF2Polynomial width value).coeff index = 0 := by
  let coefficient : Polynomial (ZMod 2) →+ ZMod 2 :=
    { toFun := fun polynomial => polynomial.coeff index
      map_zero' := coeff_zero index
      map_add' := fun left right => coeff_add left right index }
  rw [packedGF2Polynomial]
  change coefficient
    (∑ candidate : Fin width,
      if value.testBit candidate then monomial (candidate : ℕ) 1 else 0) = 0
  rw [map_sum]
  apply Finset.sum_eq_zero
  intro candidate _
  have hne : (candidate : ℕ) ≠ index := by
    omega
  by_cases hbit : value.testBit candidate <;>
    simp [hbit, coefficient, coeff_monomial, hne]

/-- If a nonzero packed value's leading bit is inside the decoder window, the
decoded polynomial degree is exactly the native `log2`. -/
theorem packedGF2Polynomial_natDegree
    (width value : ℕ) (hvalue : value ≠ 0)
    (hlead : Nat.log2 value < width) :
    (packedGF2Polynomial width value).natDegree = Nat.log2 value := by
  let top : Fin width := ⟨Nat.log2 value, hlead⟩
  apply natDegree_eq_of_le_of_coeff_ne_zero
  · apply natDegree_le_iff_coeff_eq_zero.mpr
    intro index hindex
    by_cases hin : index < width
    · let candidate : Fin width := ⟨index, hin⟩
      have hcoeff := packedGF2Polynomial_coeff width value candidate
      change (packedGF2Polynomial width value).coeff index = _ at hcoeff
      rw [hcoeff]
      have hpow : value < 2 ^ index :=
        (Nat.log2_lt hvalue).mp hindex
      have hbit := Nat.testBit_eq_false_of_lt hpow
      simp [candidate, hbit]
    · exact packedGF2Polynomial_coeff_eq_zero_of_le width value index
        (Nat.le_of_not_gt hin)
  · have hcoeff := packedGF2Polynomial_coeff width value top
    change (packedGF2Polynomial width value).coeff (Nat.log2 value) = _ at hcoeff
    rw [Nat.testBit_log2 hvalue] at hcoeff
    simp only [ite_true] at hcoeff
    rw [hcoeff]
    exact one_ne_zero

/-- A nonzero packed value whose leading bit lies inside the window decodes to
a nonzero polynomial. -/
theorem packedGF2Polynomial_ne_zero
    (width value : ℕ) (hvalue : value ≠ 0)
    (hlead : Nat.log2 value < width) :
    packedGF2Polynomial width value ≠ 0 := by
  let top : Fin width := ⟨Nat.log2 value, hlead⟩
  intro hzero
  have hcoeff := packedGF2Polynomial_coeff width value top
  change (packedGF2Polynomial width value).coeff (Nat.log2 value) = _ at hcoeff
  rw [Nat.testBit_log2 hvalue] at hcoeff
  simp only [ite_true] at hcoeff
  rw [hzero, coeff_zero] at hcoeff
  exact zero_ne_one hcoeff

/-- Every nonzero binary packed value decodes to a monic polynomial when its
leading bit lies inside the fixed window. -/
theorem packedGF2Polynomial_monic
    (width value : ℕ) (hvalue : value ≠ 0)
    (hlead : Nat.log2 value < width) :
    (packedGF2Polynomial width value).Monic := by
  rw [Polynomial.Monic, leadingCoeff,
    packedGF2Polynomial_natDegree width value hvalue hlead]
  let top : Fin width := ⟨Nat.log2 value, hlead⟩
  have hcoeff := packedGF2Polynomial_coeff width value top
  change (packedGF2Polynomial width value).coeff (Nat.log2 value) = _ at hcoeff
  rw [Nat.testBit_log2 hvalue] at hcoeff
  simpa using hcoeff

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

/-- An aligned nonzero-modulus XOR step clears the current leading bit and has
no coefficient at or above that bit afterward. -/
theorem gf2PackedRemainderStep_lt_pow_log2
    (value modulus : ℕ) (hvalue : value ≠ 0) (hmodulus : modulus ≠ 0)
    (hdegree : Nat.log2 modulus ≤ Nat.log2 value) :
    Nat.xor value
        (modulus * 2 ^ (Nat.log2 value - Nat.log2 modulus)) <
      2 ^ Nat.log2 value := by
  let valueDegree := Nat.log2 value
  let modulusDegree := Nat.log2 modulus
  let shift := valueDegree - modulusDegree
  have hshiftDegree : shift ≤ valueDegree := Nat.sub_le _ _
  have hdegreeShift : modulusDegree + shift = valueDegree := by
    dsimp [shift, valueDegree, modulusDegree]
    omega
  have hvalueTop : value.testBit valueDegree = true := by
    exact Nat.testBit_log2 hvalue
  have hmodulusTop : modulus.testBit modulusDegree = true := by
    exact Nat.testBit_log2 hmodulus
  have hshiftedTop : (modulus * 2 ^ shift).testBit valueDegree = true := by
    rw [← Nat.shiftLeft_eq_mul_pow, Nat.testBit_shiftLeft]
    have hsub : valueDegree - shift = modulusDegree := by omega
    simp [hshiftDegree, hsub, hmodulusTop]
  apply Nat.lt_pow_two_of_testBit
  intro index hindex
  have hxor :
      (Nat.xor value (modulus * 2 ^ shift)).testBit index =
        (value.testBit index ^^ (modulus * 2 ^ shift).testBit index) := by
    change (value ^^^ (modulus * 2 ^ shift)).testBit index = _
    exact Nat.testBit_xor value (modulus * 2 ^ shift) index
  rw [hxor]
  by_cases heq : index = valueDegree
  · subst index
    simp [hvalueTop, hshiftedTop]
  · have hgt : valueDegree < index := by omega
    have hvalueUpper : value < 2 ^ (valueDegree + 1) := by
      exact ((Nat.log2_eq_iff hvalue).mp rfl).2
    have hvaluePow : value < 2 ^ index :=
      lt_of_lt_of_le hvalueUpper
        (Nat.pow_le_pow_right (by decide) (by omega))
    have hvalueBit := Nat.testBit_eq_false_of_lt hvaluePow
    have hmodulusUpper : modulus < 2 ^ (modulusDegree + 1) := by
      exact ((Nat.log2_eq_iff hmodulus).mp rfl).2
    have hshiftedUpper : modulus * 2 ^ shift < 2 ^ (valueDegree + 1) := by
      calc
        modulus * 2 ^ shift < 2 ^ (modulusDegree + 1) * 2 ^ shift :=
          Nat.mul_lt_mul_of_pos_right hmodulusUpper (Nat.two_pow_pos shift)
        _ = 2 ^ ((modulusDegree + 1) + shift) := by rw [← pow_add]
        _ = 2 ^ (valueDegree + 1) := by
          apply congrArg (fun exponent => 2 ^ exponent)
          omega
    have hshiftedPow : modulus * 2 ^ shift < 2 ^ index :=
      lt_of_lt_of_le hshiftedUpper
        (Nat.pow_le_pow_right (by decide) (by omega))
    have hshiftedBit := Nat.testBit_eq_false_of_lt hshiftedPow
    simp [hvalueBit, hshiftedBit]

/-- With a nonzero modulus the aligned clearing step strictly decreases the
packed natural value. -/
theorem gf2PackedRemainderStep_lt
    (value modulus : ℕ) (hvalue : value ≠ 0) (hmodulus : modulus ≠ 0)
    (hdegree : Nat.log2 modulus ≤ Nat.log2 value) :
    Nat.xor value
        (modulus * 2 ^ (Nat.log2 value - Nat.log2 modulus)) < value := by
  exact lt_of_lt_of_le
    (gf2PackedRemainderStep_lt_pow_log2 value modulus
      hvalue hmodulus hdegree)
    ((Nat.log2_eq_iff hvalue).mp rfl).1

/-- The aligned clearing step never increases the packed value; this includes
the degenerate zero-modulus case. -/
theorem gf2PackedRemainderStep_le
    (value modulus : ℕ) (hvalue : value ≠ 0)
    (hdegree : Nat.log2 modulus ≤ Nat.log2 value) :
    Nat.xor value
        (modulus * 2 ^ (Nat.log2 value - Nat.log2 modulus)) ≤ value := by
  by_cases hmodulus : modulus = 0
  · simp [hmodulus]
  · exact (gf2PackedRemainderStep_lt value modulus
      hvalue hmodulus hdegree).le

/-- Every shift emitted by the exact remainder trace fits the uniform input
window `log2 value + 1`. -/
theorem gf2PackedRemainderShiftsFuel_lt_log2_succ
    (fuel value modulus : ℕ) :
    ∀ shift ∈ gf2PackedRemainderShiftsFuel fuel value modulus,
      shift < Nat.log2 value + 1 := by
  have haux : ∀ remaining current : ℕ, current ≤ value →
      ∀ shift ∈ gf2PackedRemainderShiftsFuel remaining current modulus,
        shift < Nat.log2 value + 1 := by
    intro remaining
    induction remaining with
    | zero => simp [gf2PackedRemainderShiftsFuel]
    | succ prior ih =>
        intro current hcurrent shift hmem
        rw [gf2PackedRemainderShiftsFuel] at hmem
        split at hmem
        next hstop => simp at hmem
        next hcontinue =>
          simp only [List.mem_cons] at hmem
          have hcurrent0 : current ≠ 0 := by
            intro hzero
            subst current
            simp at hcontinue
          have hdegree : Nat.log2 modulus ≤ Nat.log2 current := by
            by_contra hnot
            have hlt : Nat.log2 current < Nat.log2 modulus :=
              Nat.lt_of_not_ge hnot
            simp [hlt] at hcontinue
          have hvalue0 : value ≠ 0 := by
            intro hzero
            subst value
            omega
          have hlog : Nat.log2 current ≤ Nat.log2 value := by
            apply (Nat.le_log2 hvalue0).mpr
            exact le_trans ((Nat.log2_eq_iff hcurrent0).mp rfl).1 hcurrent
          rcases hmem with rfl | htail
          · omega
          · apply ih
              (Nat.xor current
                (modulus * 2 ^ (Nat.log2 current - Nat.log2 modulus)))
              (le_trans
                (gf2PackedRemainderStep_le current modulus hcurrent0 hdegree)
                hcurrent)
              shift htail
  exact haux fuel value (le_refl value)

/-- The trace-window hypothesis of the public residue theorem is discharged by
the uniform input-logarithm bound. -/
theorem packedGF2Polynomial_remainder_residue_log2
    (modulusWidth value modulus : ℕ)
    (hmodulus : modulus < 2 ^ modulusWidth)
    (hmonic : (packedGF2Polynomial modulusWidth modulus).Monic) :
    (packedGF2Polynomial (modulusWidth + (Nat.log2 value + 1))
        (gf2PackedRemainder value modulus)) %ₘ
          packedGF2Polynomial modulusWidth modulus =
      (packedGF2Polynomial (modulusWidth + (Nat.log2 value + 1)) value) %ₘ
        packedGF2Polynomial modulusWidth modulus := by
  exact packedGF2Polynomial_remainder_residue
    modulusWidth (Nat.log2 value + 1) value modulus
    (gf2PackedRemainderShiftsFuel_lt_log2_succ
      (Nat.log2 value + 1) value modulus)
    hmodulus hmonic

/-- If the initial degree fits in the supplied fuel, the remainder loop returns
zero or a packed value of degree below the nonzero modulus. -/
theorem gf2PackedRemainderFuel_stops_below_modulus
    (fuel value modulus : ℕ) (hmodulus : modulus ≠ 0)
    (hfuel : Nat.log2 value < fuel) :
    gf2PackedRemainderFuel fuel value modulus = 0 ∨
      Nat.log2 (gf2PackedRemainderFuel fuel value modulus) <
        Nat.log2 modulus := by
  induction fuel generalizing value with
  | zero => omega
  | succ remaining ih =>
      rw [gf2PackedRemainderFuel]
      split
      next hstop =>
        simpa using hstop
      next hcontinue =>
        have hvalue : value ≠ 0 := by
          intro hzero
          subst value
          simp at hcontinue
        have hdegree : Nat.log2 modulus ≤ Nat.log2 value := by
          by_contra hnot
          have hlt : Nat.log2 value < Nat.log2 modulus :=
            Nat.lt_of_not_ge hnot
          simp [hlt] at hcontinue
        let nextValue := Nat.xor value
          (modulus * 2 ^ (Nat.log2 value - Nat.log2 modulus))
        by_cases hnext : nextValue = 0
        · left
          change gf2PackedRemainderFuel remaining nextValue modulus = 0
          rw [hnext]
          cases remaining <;> rfl
        · apply ih nextValue
          have hnextDegree : Nat.log2 nextValue < Nat.log2 value := by
            apply (Nat.log2_lt hnext).mpr
            exact gf2PackedRemainderStep_lt_pow_log2 value modulus
              hvalue hmodulus hdegree
          omega

/-- The public fuel choice `log2 value + 1` is sufficient: the packed
remainder is zero or has degree below every nonzero modulus. -/
theorem gf2PackedRemainder_stops_below_modulus
    (value modulus : ℕ) (hmodulus : modulus ≠ 0) :
    gf2PackedRemainder value modulus = 0 ∨
      Nat.log2 (gf2PackedRemainder value modulus) < Nat.log2 modulus := by
  exact gf2PackedRemainderFuel_stops_below_modulus
    (Nat.log2 value + 1) value modulus hmodulus (by omega)

/-- Complete packed-remainder correctness: when the nonzero modulus leading bit
lies inside its declared window, decoding the native result is exactly
`modByMonic` of the decoded input. -/
theorem packedGF2Polynomial_remainder_eq_modByMonic
    (modulusWidth value modulus : ℕ) (hmodulus : modulus ≠ 0)
    (hlead : Nat.log2 modulus < modulusWidth) :
    packedGF2Polynomial (modulusWidth + (Nat.log2 value + 1))
        (gf2PackedRemainder value modulus) =
      (packedGF2Polynomial (modulusWidth + (Nat.log2 value + 1)) value) %ₘ
        packedGF2Polynomial modulusWidth modulus := by
  let result := gf2PackedRemainder value modulus
  let modulusPolynomial := packedGF2Polynomial modulusWidth modulus
  have hmodulusBound : modulus < 2 ^ modulusWidth := by
    have hupper := ((Nat.log2_eq_iff hmodulus).mp rfl).2
    exact lt_of_lt_of_le hupper
      (Nat.pow_le_pow_right (by decide) (by omega))
  have hmonic : modulusPolynomial.Monic :=
    packedGF2Polynomial_monic modulusWidth modulus hmodulus hlead
  have hresidue := packedGF2Polynomial_remainder_residue_log2
    modulusWidth value modulus hmodulusBound hmonic
  have hself :
      (packedGF2Polynomial (modulusWidth + (Nat.log2 value + 1)) result) %ₘ
          modulusPolynomial =
        packedGF2Polynomial (modulusWidth + (Nat.log2 value + 1)) result := by
    rcases gf2PackedRemainder_stops_below_modulus value modulus hmodulus with
      hzero | hdegree
    · simp [result, hzero, packedGF2Polynomial]
    · by_cases hresult : result = 0
      · simp [hresult, packedGF2Polynomial]
      · apply (modByMonic_eq_self_iff hmonic).2
        change Nat.log2 result < Nat.log2 modulus at hdegree
        have hresultLead :
            Nat.log2 result < modulusWidth + (Nat.log2 value + 1) := by
          omega
        have hresultPolynomial :
            packedGF2Polynomial
              (modulusWidth + (Nat.log2 value + 1)) result ≠ 0 :=
          packedGF2Polynomial_ne_zero
            (modulusWidth + (Nat.log2 value + 1)) result hresult hresultLead
        apply (natDegree_lt_natDegree_iff hresultPolynomial).mp
        rw [packedGF2Polynomial_natDegree
              (modulusWidth + (Nat.log2 value + 1)) result
              hresult hresultLead,
          packedGF2Polynomial_natDegree modulusWidth modulus hmodulus hlead]
        exact hdegree
  exact hself.symm.trans hresidue

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

/-- The native carryless fold fits in the sum of the declared left width and
the exact native right loop width. -/
theorem gf2PackedRawProduct_lt_two_pow
    (leftWidth left right : ℕ) (hleft : left < 2 ^ leftWidth) :
    gf2PackedRawProduct left right <
      2 ^ (leftWidth + (Nat.log2 right + 1)) := by
  let rightWidth := Nat.log2 right + 1
  let bound := 2 ^ (leftWidth + rightWidth)
  have hterm : ∀ index < rightWidth, left * 2 ^ index < bound := by
    intro index hindex
    have hmul : left * 2 ^ index < 2 ^ leftWidth * 2 ^ index :=
      Nat.mul_lt_mul_of_pos_right hleft (Nat.two_pow_pos index)
    have hpow : 2 ^ leftWidth * 2 ^ index ≤
        2 ^ leftWidth * 2 ^ rightWidth :=
      Nat.mul_le_mul_left _ (Nat.pow_le_pow_right (by decide) hindex.le)
    exact lt_of_lt_of_le hmul (by simpa [bound, pow_add] using hpow)
  have hfold : ∀ indices : List ℕ, ∀ product < bound,
      (∀ index ∈ indices, index < rightWidth) →
      indices.foldl
          (fun current index =>
            if right.testBit index then
              Nat.xor current (left * 2 ^ index)
            else current)
          product < bound := by
    intro indices
    induction indices with
    | nil => simp
    | cons head tail ih =>
        intro product hproduct hindices
        simp only [List.foldl_cons]
        apply ih
        · by_cases hbit : right.testBit head
          · simp only [hbit, ite_true]
            exact Nat.xor_lt_two_pow hproduct
              (hterm head (hindices head (by simp)))
          · simpa [hbit] using hproduct
        · intro index hmem
          exact hindices index (by simp [hmem])
  unfold gf2PackedRawProduct
  change (List.range rightWidth).foldl
      (fun current index =>
        if right.testBit index then Nat.xor current (left * 2 ^ index)
        else current)
      0 < bound
  apply hfold (List.range rightWidth) 0 (Nat.two_pow_pos _)
  intro index hmem
  simpa using hmem

/-- The raw-product multiplication identity remains exact in every wider output
window. -/
theorem packedGF2Polynomial_rawProduct_eq_mul_extended
    (leftWidth extra left right : ℕ) (hleft : left < 2 ^ leftWidth) :
    packedGF2Polynomial
        ((leftWidth + (Nat.log2 right + 1)) + extra)
        (gf2PackedRawProduct left right) =
      packedGF2Polynomial leftWidth left *
        packedGF2Polynomial (Nat.log2 right + 1) right := by
  rw [packedGF2Polynomial_extend
    (leftWidth + (Nat.log2 right + 1)) extra
    (gf2PackedRawProduct left right)
    (gf2PackedRawProduct_lt_two_pow leftWidth left right hleft)]
  exact packedGF2Polynomial_rawProduct_eq_mul leftWidth left right hleft

/-- Carryless product reduced by a packed modulus. -/
def gf2PackedProduct (left right modulus : ℕ) : ℕ :=
  gf2PackedRemainder (gf2PackedRawProduct left right) modulus

/-- End-to-end packed-product correctness with an explicit common-window
condition: decoding native carryless multiplication plus reduction equals
polynomial multiplication followed by `modByMonic`. -/
theorem packedGF2Polynomial_product_eq_mul_modByMonic
    (leftWidth modulusWidth left right modulus : ℕ)
    (hleft : left < 2 ^ leftWidth) (hmodulus : modulus ≠ 0)
    (hmodulusLead : Nat.log2 modulus < modulusWidth)
    (hwindow : leftWidth + (Nat.log2 right + 1) ≤
      modulusWidth + (Nat.log2 (gf2PackedRawProduct left right) + 1)) :
    packedGF2Polynomial
        (modulusWidth + (Nat.log2 (gf2PackedRawProduct left right) + 1))
        (gf2PackedProduct left right modulus) =
      (packedGF2Polynomial leftWidth left *
        packedGF2Polynomial (Nat.log2 right + 1) right) %ₘ
          packedGF2Polynomial modulusWidth modulus := by
  let raw := gf2PackedRawProduct left right
  let baseWidth := leftWidth + (Nat.log2 right + 1)
  let remainderWidth := modulusWidth + (Nat.log2 raw + 1)
  change baseWidth ≤ remainderWidth at hwindow
  have hwidth : baseWidth + (remainderWidth - baseWidth) = remainderWidth := by
    omega
  have hraw : packedGF2Polynomial remainderWidth raw =
      packedGF2Polynomial leftWidth left *
        packedGF2Polynomial (Nat.log2 right + 1) right := by
    rw [← hwidth]
    exact packedGF2Polynomial_rawProduct_eq_mul_extended
      leftWidth (remainderWidth - baseWidth) left right hleft
  unfold gf2PackedProduct
  change packedGF2Polynomial remainderWidth (gf2PackedRemainder raw modulus) = _
  rw [packedGF2Polynomial_remainder_eq_modByMonic
    modulusWidth raw modulus hmodulus hmodulusLead]
  rw [hraw]

/-- For nonzero inputs, minimal `log2+1` windows satisfy the common-window
condition automatically because carryless-product degree is additive. -/
theorem packedGF2Polynomial_product_eq_mul_modByMonic_log2
    (left right modulus : ℕ)
    (hleft : left ≠ 0) (hright : right ≠ 0) (hmodulus : modulus ≠ 0) :
    packedGF2Polynomial
        ((Nat.log2 modulus + 1) +
          (Nat.log2 (gf2PackedRawProduct left right) + 1))
        (gf2PackedProduct left right modulus) =
      (packedGF2Polynomial (Nat.log2 left + 1) left *
        packedGF2Polynomial (Nat.log2 right + 1) right) %ₘ
          packedGF2Polynomial (Nat.log2 modulus + 1) modulus := by
  have hleftBound : left < 2 ^ (Nat.log2 left + 1) :=
    ((Nat.log2_eq_iff hleft).mp rfl).2
  have hleftPolynomial :
      packedGF2Polynomial (Nat.log2 left + 1) left ≠ 0 :=
    packedGF2Polynomial_ne_zero (Nat.log2 left + 1) left hleft (by omega)
  have hrightPolynomial :
      packedGF2Polynomial (Nat.log2 right + 1) right ≠ 0 :=
    packedGF2Polynomial_ne_zero (Nat.log2 right + 1) right hright (by omega)
  have hrawEquation := packedGF2Polynomial_rawProduct_eq_mul
    (Nat.log2 left + 1) left right hleftBound
  have hrawPolynomial :
      packedGF2Polynomial
        ((Nat.log2 left + 1) + (Nat.log2 right + 1))
        (gf2PackedRawProduct left right) ≠ 0 := by
    rw [hrawEquation]
    exact mul_ne_zero hleftPolynomial hrightPolynomial
  have hraw : gf2PackedRawProduct left right ≠ 0 := by
    intro hzero
    rw [hzero] at hrawPolynomial
    exact hrawPolynomial (by simp [packedGF2Polynomial])
  have hrawBound := gf2PackedRawProduct_lt_two_pow
    (Nat.log2 left + 1) left right hleftBound
  have hrawLead : Nat.log2 (gf2PackedRawProduct left right) <
      (Nat.log2 left + 1) + (Nat.log2 right + 1) :=
    (Nat.log2_lt hraw).mpr hrawBound
  have hrawDegree : Nat.log2 (gf2PackedRawProduct left right) =
      Nat.log2 left + Nat.log2 right := by
    have hdegree := congrArg Polynomial.natDegree hrawEquation
    rw [packedGF2Polynomial_natDegree
          ((Nat.log2 left + 1) + (Nat.log2 right + 1))
          (gf2PackedRawProduct left right) hraw hrawLead,
      Polynomial.natDegree_mul hleftPolynomial hrightPolynomial,
      packedGF2Polynomial_natDegree (Nat.log2 left + 1) left hleft (by omega),
      packedGF2Polynomial_natDegree (Nat.log2 right + 1) right hright (by omega)]
      at hdegree
    exact hdegree
  apply packedGF2Polynomial_product_eq_mul_modByMonic
    (Nat.log2 left + 1) (Nat.log2 modulus + 1)
    left right modulus hleftBound hmodulus (by omega)
  rw [hrawDegree]
  omega

/-- Minimal `log2+1` packed decoding implements polynomial multiplication
modulo every nonzero packed modulus, including zero left or right inputs. -/
theorem packedGF2Polynomial_product_eq_mul_modByMonic_all
    (left right modulus : ℕ) (hmodulus : modulus ≠ 0) :
    packedGF2Polynomial
        ((Nat.log2 modulus + 1) +
          (Nat.log2 (gf2PackedRawProduct left right) + 1))
        (gf2PackedProduct left right modulus) =
      (packedGF2Polynomial (Nat.log2 left + 1) left *
        packedGF2Polynomial (Nat.log2 right + 1) right) %ₘ
          packedGF2Polynomial (Nat.log2 modulus + 1) modulus := by
  by_cases hleft : left = 0
  · subst left
    simp [gf2PackedProduct, gf2PackedRawProduct, gf2PackedRemainder,
      gf2PackedRemainderFuel, packedGF2Polynomial]
  by_cases hright : right = 0
  · subst right
    simp [gf2PackedProduct, gf2PackedRawProduct, gf2PackedRemainder,
      gf2PackedRemainderFuel, packedGF2Polynomial]
  exact packedGF2Polynomial_product_eq_mul_modByMonic_log2
    left right modulus hleft hright hmodulus

/-- The native product theorem can be stated in the result's minimal decoding
window: termination below the nonzero modulus makes every wider high
coefficient vanish. -/
theorem packedGF2Polynomial_product_minimal_eq_mul_modByMonic
    (left right modulus : ℕ) (hmodulus : modulus ≠ 0) :
    packedGF2Polynomial
        (Nat.log2 (gf2PackedProduct left right modulus) + 1)
        (gf2PackedProduct left right modulus) =
      (packedGF2Polynomial (Nat.log2 left + 1) left *
        packedGF2Polynomial (Nat.log2 right + 1) right) %ₘ
          packedGF2Polynomial (Nat.log2 modulus + 1) modulus := by
  let product := gf2PackedProduct left right modulus
  let raw := gf2PackedRawProduct left right
  let minimumWidth := Nat.log2 product + 1
  let wideWidth := (Nat.log2 modulus + 1) + (Nat.log2 raw + 1)
  have hminimum : minimumWidth ≤ Nat.log2 modulus + 1 := by
    rcases gf2PackedRemainder_stops_below_modulus raw modulus hmodulus with
      hzero | hdegree
    · have hproductZero : product = 0 := by
        simpa [product, gf2PackedProduct, raw] using hzero
      dsimp [minimumWidth]
      rw [hproductZero]
      simp
    · change Nat.log2 product < Nat.log2 modulus at hdegree
      omega
  have hwidth : minimumWidth + (wideWidth - minimumWidth) = wideWidth := by
    dsimp [wideWidth]
    omega
  have hfit : product < 2 ^ minimumWidth := by
    by_cases hproduct : product = 0
    · simp [hproduct, minimumWidth]
    · exact ((Nat.log2_eq_iff hproduct).mp rfl).2
  have hextend : packedGF2Polynomial wideWidth product =
      packedGF2Polynomial minimumWidth product := by
    rw [← hwidth]
    exact packedGF2Polynomial_extend minimumWidth
      (wideWidth - minimumWidth) product hfit
  rw [← hextend]
  exact packedGF2Polynomial_product_eq_mul_modByMonic_all
    left right modulus hmodulus

/-- Repeated native Frobenius squaring, separated from the Rabin gcd guards so
its exact value trace can be refined independently. -/
def gf2PackedFrobeniusIterate (modulus : ℕ) : ℕ → ℕ → ℕ
  | 0, value => value
  | steps + 1, value =>
      gf2PackedFrobeniusIterate modulus steps
        (gf2PackedProduct value value modulus)

/-- Polynomial repeated squaring and reduction in the same update order as the
native packed iterator. -/
noncomputable def gf2PolynomialFrobeniusIterate
    (modulus : Polynomial (ZMod 2)) :
    ℕ → Polynomial (ZMod 2) → Polynomial (ZMod 2)
  | 0, value => value
  | steps + 1, value =>
      gf2PolynomialFrobeniusIterate modulus steps
        ((value * value) %ₘ modulus)

/-- Every repeated native square-and-reduce update decodes as the corresponding
polynomial square followed by `modByMonic`. -/
theorem packedGF2Polynomial_frobeniusIterate
    (steps value modulus : ℕ) (hmodulus : modulus ≠ 0) :
    packedGF2Polynomial
        (Nat.log2 (gf2PackedFrobeniusIterate modulus steps value) + 1)
        (gf2PackedFrobeniusIterate modulus steps value) =
      gf2PolynomialFrobeniusIterate
        (packedGF2Polynomial (Nat.log2 modulus + 1) modulus) steps
        (packedGF2Polynomial (Nat.log2 value + 1) value) := by
  induction steps generalizing value with
  | zero => rfl
  | succ remaining ih =>
      rw [gf2PackedFrobeniusIterate, gf2PolynomialFrobeniusIterate]
      rw [ih (gf2PackedProduct value value modulus)]
      rw [packedGF2Polynomial_product_minimal_eq_mul_modByMonic
        value value modulus hmodulus]

/-- Reduction by a monic GF(2) polynomial is idempotent. -/
theorem gf2Polynomial_modByMonic_idem
    (value modulus : Polynomial (ZMod 2)) (hmonic : modulus.Monic) :
    (value %ₘ modulus) %ₘ modulus = value %ₘ modulus := by
  apply (modByMonic_eq_self_iff hmonic).2
  exact degree_modByMonic_lt value hmonic

/-- Reducing a GF(2) polynomial before exponentiation does not change the
reduced power. -/
theorem gf2Polynomial_pow_modByMonic
    (value modulus : Polynomial (ZMod 2)) (exponent : ℕ)
    (hmonic : modulus.Monic) :
    (value ^ exponent) %ₘ modulus =
      ((value %ₘ modulus) ^ exponent) %ₘ modulus := by
  induction exponent with
  | zero => rfl
  | succ prior ih =>
      simp only [pow_succ]
      calc
        (value ^ prior * value) %ₘ modulus =
            (value ^ prior %ₘ modulus) * (value %ₘ modulus) %ₘ modulus :=
          Polynomial.mul_modByMonic _ _ _
        _ = ((value %ₘ modulus) ^ prior %ₘ modulus) *
            (value %ₘ modulus) %ₘ modulus := by rw [ih]
        _ = ((value %ₘ modulus) ^ prior %ₘ modulus) *
            ((value %ₘ modulus) %ₘ modulus) %ₘ modulus := by
          exact congrArg
            (fun right : Polynomial (ZMod 2) =>
              ((value %ₘ modulus) ^ prior %ₘ modulus) * right %ₘ modulus)
            (gf2Polynomial_modByMonic_idem value modulus hmonic).symm
        _ = ((value %ₘ modulus) ^ prior * (value %ₘ modulus)) %ₘ
            modulus := (Polynomial.mul_modByMonic _ _ _).symm

/-- Starting from a reduced polynomial, the abstract Frobenius iterator is the
exact `2^steps` power modulo the monic modulus. -/
theorem gf2PolynomialFrobeniusIterate_eq_pow_modByMonic
    (steps : ℕ) (value modulus : Polynomial (ZMod 2))
    (hmonic : modulus.Monic) (hreduced : value %ₘ modulus = value) :
    gf2PolynomialFrobeniusIterate modulus steps value =
      (value ^ (2 ^ steps)) %ₘ modulus := by
  induction steps generalizing value with
  | zero => simpa [gf2PolynomialFrobeniusIterate] using hreduced.symm
  | succ remaining ih =>
      rw [gf2PolynomialFrobeniusIterate]
      rw [ih ((value * value) %ₘ modulus)
        (gf2Polynomial_modByMonic_idem (value * value) modulus hmonic)]
      rw [← gf2Polynomial_pow_modByMonic
        (value * value) modulus (2 ^ remaining) hmonic]
      apply congrArg (fun polynomial : Polynomial (ZMod 2) =>
        polynomial %ₘ modulus)
      calc
        (value * value) ^ (2 ^ remaining) =
            (value ^ 2) ^ (2 ^ remaining) := by rw [pow_two]
        _ = value ^ (2 * 2 ^ remaining) := by rw [pow_mul]
        _ = value ^ (2 ^ (remaining + 1)) := by
          congr 1
          simp [pow_succ, mul_comm]

/-- Every native packed remainder is a reduced polynomial in its minimal
decoding window. -/
theorem packedGF2Polynomial_remainder_minimal_reduced
    (value modulus : ℕ) (hmodulus : modulus ≠ 0) :
    (packedGF2Polynomial
        (Nat.log2 (gf2PackedRemainder value modulus) + 1)
        (gf2PackedRemainder value modulus)) %ₘ
          packedGF2Polynomial (Nat.log2 modulus + 1) modulus =
      packedGF2Polynomial
        (Nat.log2 (gf2PackedRemainder value modulus) + 1)
        (gf2PackedRemainder value modulus) := by
  let result := gf2PackedRemainder value modulus
  let resultPolynomial := packedGF2Polynomial (Nat.log2 result + 1) result
  let modulusPolynomial :=
    packedGF2Polynomial (Nat.log2 modulus + 1) modulus
  have hmonic : modulusPolynomial.Monic :=
    packedGF2Polynomial_monic (Nat.log2 modulus + 1) modulus
      hmodulus (by omega)
  change resultPolynomial %ₘ modulusPolynomial = resultPolynomial
  rcases gf2PackedRemainder_stops_below_modulus value modulus hmodulus with
    hzero | hdegree
  · have hresult : result = 0 := by simpa [result] using hzero
    simp [resultPolynomial, hresult, packedGF2Polynomial]
  · by_cases hresult : result = 0
    · simp [resultPolynomial, hresult, packedGF2Polynomial]
    · apply (modByMonic_eq_self_iff hmonic).2
      have hresultPolynomial : resultPolynomial ≠ 0 :=
        packedGF2Polynomial_ne_zero (Nat.log2 result + 1) result
          hresult (by omega)
      apply (natDegree_lt_natDegree_iff hresultPolynomial).mp
      rw [packedGF2Polynomial_natDegree
            (Nat.log2 result + 1) result hresult (by omega),
        packedGF2Polynomial_natDegree
          (Nat.log2 modulus + 1) modulus hmodulus (by omega)]
      exact hdegree

/-- For any reduced packed start value, the native Frobenius iterator decodes
as the exact `2^steps` power modulo the packed modulus. -/
theorem packedGF2Polynomial_frobeniusIterate_eq_pow_modByMonic
    (steps value modulus : ℕ) (hmodulus : modulus ≠ 0)
    (hreduced :
      (packedGF2Polynomial (Nat.log2 value + 1) value) %ₘ
          packedGF2Polynomial (Nat.log2 modulus + 1) modulus =
        packedGF2Polynomial (Nat.log2 value + 1) value) :
    packedGF2Polynomial
        (Nat.log2 (gf2PackedFrobeniusIterate modulus steps value) + 1)
        (gf2PackedFrobeniusIterate modulus steps value) =
      ((packedGF2Polynomial (Nat.log2 value + 1) value) ^ (2 ^ steps)) %ₘ
        packedGF2Polynomial (Nat.log2 modulus + 1) modulus := by
  rw [packedGF2Polynomial_frobeniusIterate steps value modulus hmodulus]
  exact gf2PolynomialFrobeniusIterate_eq_pow_modByMonic steps
    (packedGF2Polynomial (Nat.log2 value + 1) value)
    (packedGF2Polynomial (Nat.log2 modulus + 1) modulus)
    (packedGF2Polynomial_monic (Nat.log2 modulus + 1) modulus
      hmodulus (by omega)) hreduced

/-- The value trace used by the Rabin loop starts from a native remainder, so
its `2^steps`-power semantics needs no extra normalization premise. -/
theorem packedGF2Polynomial_frobeniusIterate_from_remainder
    (steps value modulus : ℕ) (hmodulus : modulus ≠ 0) :
    packedGF2Polynomial
        (Nat.log2
          (gf2PackedFrobeniusIterate modulus steps
            (gf2PackedRemainder value modulus)) + 1)
        (gf2PackedFrobeniusIterate modulus steps
          (gf2PackedRemainder value modulus)) =
      ((packedGF2Polynomial
          (Nat.log2 (gf2PackedRemainder value modulus) + 1)
          (gf2PackedRemainder value modulus)) ^ (2 ^ steps)) %ₘ
        packedGF2Polynomial (Nat.log2 modulus + 1) modulus := by
  exact packedGF2Polynomial_frobeniusIterate_eq_pow_modByMonic steps
    (gf2PackedRemainder value modulus) modulus hmodulus
    (packedGF2Polynomial_remainder_minimal_reduced value modulus hmodulus)

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

/-- Exact packed remainder correctness in the returned value's minimal decoder
and the input value's minimal decoder. -/
theorem packedGF2Polynomial_remainder_minimal_eq_modByMonic
    (value modulus : ℕ) (hmodulus : modulus ≠ 0) :
    packedGF2Polynomial
        (Nat.log2 (gf2PackedRemainder value modulus) + 1)
        (gf2PackedRemainder value modulus) =
      (packedGF2Polynomial (Nat.log2 value + 1) value) %ₘ
        packedGF2Polynomial (Nat.log2 modulus + 1) modulus := by
  let result := gf2PackedRemainder value modulus
  let resultWidth := Nat.log2 result + 1
  let valueWidth := Nat.log2 value + 1
  let modulusWidth := Nat.log2 modulus + 1
  let wideWidth := modulusWidth + valueWidth
  have hresultWidth : resultWidth ≤ modulusWidth := by
    rcases gf2PackedRemainder_stops_below_modulus value modulus hmodulus with
      hzero | hdegree
    · have hresult : result = 0 := by simpa [result] using hzero
      dsimp [resultWidth, modulusWidth]
      rw [hresult]
      simp
    · change Nat.log2 result < Nat.log2 modulus at hdegree
      dsimp [resultWidth, modulusWidth]
      omega
  have hresultFit : result < 2 ^ resultWidth := by
    by_cases hresult : result = 0
    · simp [hresult, resultWidth]
    · exact ((Nat.log2_eq_iff hresult).mp rfl).2
  have hresultExtend : packedGF2Polynomial wideWidth result =
      packedGF2Polynomial resultWidth result := by
    have hwidth : resultWidth + (wideWidth - resultWidth) = wideWidth := by
      dsimp [wideWidth]
      omega
    rw [← hwidth]
    exact packedGF2Polynomial_extend resultWidth
      (wideWidth - resultWidth) result hresultFit
  have hvalueFit : value < 2 ^ valueWidth := by
    by_cases hvalue : value = 0
    · simp [hvalue, valueWidth]
    · exact ((Nat.log2_eq_iff hvalue).mp rfl).2
  have hvalueExtend : packedGF2Polynomial wideWidth value =
      packedGF2Polynomial valueWidth value := by
    have hwidth : valueWidth + modulusWidth = wideWidth := by
      dsimp [wideWidth]
      omega
    rw [← hwidth]
    exact packedGF2Polynomial_extend valueWidth modulusWidth value hvalueFit
  calc
    packedGF2Polynomial resultWidth result =
        packedGF2Polynomial wideWidth result := hresultExtend.symm
    _ = (packedGF2Polynomial wideWidth value) %ₘ
        packedGF2Polynomial modulusWidth modulus := by
      exact packedGF2Polynomial_remainder_eq_modByMonic modulusWidth
        value modulus hmodulus (by dsimp [modulusWidth]; omega)
    _ = (packedGF2Polynomial valueWidth value) %ₘ
        packedGF2Polynomial modulusWidth modulus := by rw [hvalueExtend]

/-- With enough fuel to cover the current nonzero right degree, the native
packed Euclidean loop decodes exactly as polynomial GCD. The zero-right case is
valid even with zero remaining fuel. -/
theorem packedGF2Polynomial_gcdFuel_eq_gcd
    (fuel left right : ℕ)
    (hfuel : right = 0 ∨ Nat.log2 right < fuel) :
    packedGF2Polynomial
        (Nat.log2 (gf2PackedGcdFuel fuel left right) + 1)
        (gf2PackedGcdFuel fuel left right) =
      EuclideanDomain.gcd
        (packedGF2Polynomial (Nat.log2 right + 1) right)
        (packedGF2Polynomial (Nat.log2 left + 1) left) := by
  induction fuel generalizing left right with
  | zero =>
      rcases hfuel with hright | hfalse
      · subst right
        simp [gf2PackedGcdFuel, packedGF2Polynomial]
        rfl
      · omega
  | succ remaining ih =>
      rw [gf2PackedGcdFuel]
      by_cases hright : right = 0
      · have hbeq : (right == 0) = true := by simp [hright]
        rw [hbeq]
        simp [hright, packedGF2Polynomial]
      · have hbeq : (right == 0) = false := by
          simpa [beq_iff_eq] using hright
        rw [hbeq]
        let remainder := gf2PackedRemainder left right
        have hremainderFuel : remainder = 0 ∨ Nat.log2 remainder < remaining := by
          rcases gf2PackedRemainder_stops_below_modulus left right hright with
            hzero | hdegree
          · exact Or.inl (by simpa [remainder] using hzero)
          · right
            rcases hfuel with hzero | hbound
            · contradiction
            · change Nat.log2 remainder < Nat.log2 right at hdegree
              omega
        have hrightMonic :
            (packedGF2Polynomial (Nat.log2 right + 1) right).Monic :=
          packedGF2Polynomial_monic (Nat.log2 right + 1) right
            hright (by omega)
        calc
          packedGF2Polynomial
              (Nat.log2 (gf2PackedGcdFuel remaining right remainder) + 1)
              (gf2PackedGcdFuel remaining right remainder) =
            EuclideanDomain.gcd
              (packedGF2Polynomial (Nat.log2 remainder + 1) remainder)
              (packedGF2Polynomial (Nat.log2 right + 1) right) :=
                ih right remainder hremainderFuel
          _ = EuclideanDomain.gcd
              ((packedGF2Polynomial (Nat.log2 left + 1) left) %ₘ
                packedGF2Polynomial (Nat.log2 right + 1) right)
              (packedGF2Polynomial (Nat.log2 right + 1) right) := by
                rw [packedGF2Polynomial_remainder_minimal_eq_modByMonic
                  left right hright]
          _ = EuclideanDomain.gcd
              ((packedGF2Polynomial (Nat.log2 left + 1) left) %
                packedGF2Polynomial (Nat.log2 right + 1) right)
              (packedGF2Polynomial (Nat.log2 right + 1) right) := by
                rw [Polynomial.modByMonic_eq_mod _ hrightMonic]
          _ = EuclideanDomain.gcd
              (packedGF2Polynomial (Nat.log2 right + 1) right)
              (packedGF2Polynomial (Nat.log2 left + 1) left) :=
                (EuclideanDomain.gcd_val _ _).symm

/-- The public sum-of-degrees fuel bound always suffices, so packed GCD decodes
exactly as Euclidean polynomial GCD without a caller premise. -/
theorem packedGF2Polynomial_gcd_eq_gcd
    (left right : ℕ) :
    packedGF2Polynomial
        (Nat.log2 (gf2PackedGcd left right) + 1)
        (gf2PackedGcd left right) =
      EuclideanDomain.gcd
        (packedGF2Polynomial (Nat.log2 right + 1) right)
        (packedGF2Polynomial (Nat.log2 left + 1) left) := by
  unfold gf2PackedGcd
  apply packedGF2Polynomial_gcdFuel_eq_gcd
  by_cases hright : right = 0
  · exact Or.inl hright
  · exact Or.inr (by omega)

/-- Minimal packed decoding equals polynomial one exactly when the source
natural number equals one. -/
theorem packedGF2Polynomial_minimal_eq_one_iff (value : ℕ) :
    packedGF2Polynomial (Nat.log2 value + 1) value = 1 ↔ value = 1 := by
  constructor
  · intro heq
    have hvalue : value ≠ 0 := by
      intro hzero
      subst value
      simp [packedGF2Polynomial] at heq
    have hdegree := congrArg Polynomial.natDegree heq
    rw [packedGF2Polynomial_natDegree (Nat.log2 value + 1) value
        hvalue (by omega), natDegree_one] at hdegree
    have hbounds := (Nat.log2_eq_iff hvalue).mp hdegree
    norm_num at hbounds
    omega
  · rintro rfl
    change packedGF2Polynomial 1 1 = 1
    ext index
    by_cases hindex : index = 0
    · subst index
      simpa using packedGF2Polynomial_coeff 1 1 ⟨0, by omega⟩
    · rw [packedGF2Polynomial_coeff_eq_zero_of_le 1 1 index (by omega)]
      simp [coeff_one, hindex]

/-- The packed GCD result is one exactly when the corresponding polynomial GCD
is one. -/
theorem gf2PackedGcd_eq_one_iff (left right : ℕ) :
    gf2PackedGcd left right = 1 ↔
      EuclideanDomain.gcd
        (packedGF2Polynomial (Nat.log2 right + 1) right)
        (packedGF2Polynomial (Nat.log2 left + 1) left) = 1 := by
  have hsemantic := packedGF2Polynomial_gcd_eq_gcd left right
  constructor
  · intro hgcd
    exact hsemantic.symm.trans
      ((packedGF2Polynomial_minimal_eq_one_iff _).2 hgcd)
  · intro hgcd
    exact (packedGF2Polynomial_minimal_eq_one_iff _).1
      (hsemantic.trans hgcd)

/-- The exact Boolean guard used by the Rabin loop is equivalent to polynomial
GCD being one. -/
theorem gf2PackedGcd_guard_iff (left right : ℕ) :
    (gf2PackedGcd left right == 1) = true ↔
      EuclideanDomain.gcd
        (packedGF2Polynomial (Nat.log2 right + 1) right)
        (packedGF2Polynomial (Nat.log2 left + 1) left) = 1 := by
  simpa [beq_iff_eq] using gf2PackedGcd_eq_one_iff left right

/-- Fixed-window packed decoding is injective when both source naturals fit in
the declared bit window. -/
theorem packedGF2Polynomial_injective_of_lt
    (width left right : ℕ) (hleft : left < 2 ^ width)
    (hright : right < 2 ^ width)
    (heq : packedGF2Polynomial width left = packedGF2Polynomial width right) :
    left = right := by
  apply Nat.eq_of_testBit_eq
  intro index
  by_cases hindex : index < width
  · let packedIndex : Fin width := ⟨index, hindex⟩
    have hcoeff := congrArg (fun polynomial : Polynomial (ZMod 2) =>
      polynomial.coeff index) heq
    have hleftCoeff := packedGF2Polynomial_coeff width left packedIndex
    have hrightCoeff := packedGF2Polynomial_coeff width right packedIndex
    change (packedGF2Polynomial width left).coeff index = _ at hleftCoeff
    change (packedGF2Polynomial width right).coeff index = _ at hrightCoeff
    rw [hleftCoeff, hrightCoeff] at hcoeff
    change (if left.testBit index then (1 : ZMod 2) else 0) =
      if right.testBit index then 1 else 0 at hcoeff
    cases hleftBit : left.testBit index <;>
      cases hrightBit : right.testBit index <;> simp_all
  · have hpow : 2 ^ width ≤ 2 ^ index :=
      Nat.pow_le_pow_right (by decide) (by omega)
    have hleftBit := Nat.testBit_eq_false_of_lt (lt_of_lt_of_le hleft hpow)
    have hrightBit := Nat.testBit_eq_false_of_lt (lt_of_lt_of_le hright hpow)
    simp [hleftBit, hrightBit]

/-- Minimal `log2+1` packed decoding is injective even when the two source
values use different minimal windows. -/
theorem packedGF2Polynomial_minimal_injective (left right : ℕ)
    (heq : packedGF2Polynomial (Nat.log2 left + 1) left =
      packedGF2Polynomial (Nat.log2 right + 1) right) :
    left = right := by
  let leftWidth := Nat.log2 left + 1
  let rightWidth := Nat.log2 right + 1
  have hleftFit : left < 2 ^ leftWidth := by
    by_cases hleft : left = 0
    · simp [hleft, leftWidth]
    · exact ((Nat.log2_eq_iff hleft).mp rfl).2
  have hrightFit : right < 2 ^ rightWidth := by
    by_cases hright : right = 0
    · simp [hright, rightWidth]
    · exact ((Nat.log2_eq_iff hright).mp rfl).2
  apply packedGF2Polynomial_injective_of_lt
    (leftWidth + rightWidth) left right
  · exact lt_of_lt_of_le hleftFit
      (Nat.pow_le_pow_right (by decide) (by omega))
  · exact lt_of_lt_of_le hrightFit
      (Nat.pow_le_pow_right (by decide) (by omega))
  · calc
      packedGF2Polynomial (leftWidth + rightWidth) left =
          packedGF2Polynomial leftWidth left :=
        packedGF2Polynomial_extend leftWidth rightWidth left hleftFit
      _ = packedGF2Polynomial rightWidth right := heq
      _ = packedGF2Polynomial (leftWidth + rightWidth) right := by
        rw [add_comm]
        exact (packedGF2Polynomial_extend rightWidth leftWidth right
          hrightFit).symm

/-- Minimal packed XOR decoding is polynomial addition even when the three
source values use different `log2+1` windows. -/
theorem packedGF2Polynomial_minimal_xor (left right : ℕ) :
    packedGF2Polynomial (Nat.log2 (Nat.xor left right) + 1)
        (Nat.xor left right) =
      packedGF2Polynomial (Nat.log2 left + 1) left +
        packedGF2Polynomial (Nat.log2 right + 1) right := by
  let leftWidth := Nat.log2 left + 1
  let rightWidth := Nat.log2 right + 1
  let xorValue := Nat.xor left right
  let xorWidth := Nat.log2 xorValue + 1
  let wideWidth := leftWidth + rightWidth
  have hleftFit : left < 2 ^ leftWidth := by
    by_cases hleft : left = 0
    · simp [hleft, leftWidth]
    · exact ((Nat.log2_eq_iff hleft).mp rfl).2
  have hrightFit : right < 2 ^ rightWidth := by
    by_cases hright : right = 0
    · simp [hright, rightWidth]
    · exact ((Nat.log2_eq_iff hright).mp rfl).2
  have hleftWide : left < 2 ^ wideWidth :=
    lt_of_lt_of_le hleftFit
      (Nat.pow_le_pow_right (by decide) (by dsimp [wideWidth]; omega))
  have hrightWide : right < 2 ^ wideWidth :=
    lt_of_lt_of_le hrightFit
      (Nat.pow_le_pow_right (by decide) (by dsimp [wideWidth]; omega))
  have hxorWide : xorValue < 2 ^ wideWidth := by
    exact Nat.xor_lt_two_pow hleftWide hrightWide
  have hxorWidth : xorWidth ≤ wideWidth := by
    by_cases hxor : xorValue = 0
    · simp [hxor, xorWidth]
      dsimp [wideWidth, leftWidth, rightWidth]
      omega
    · have hlog : Nat.log2 xorValue < wideWidth :=
        (Nat.log2_lt hxor).mpr hxorWide
      dsimp [xorWidth]
      omega
  have hxorExtend : packedGF2Polynomial wideWidth xorValue =
      packedGF2Polynomial xorWidth xorValue := by
    have hwidth : xorWidth + (wideWidth - xorWidth) = wideWidth := by omega
    rw [← hwidth]
    exact packedGF2Polynomial_extend xorWidth (wideWidth - xorWidth)
      xorValue (by
        by_cases hxor : xorValue = 0
        · simp [hxor, xorWidth]
        · exact ((Nat.log2_eq_iff hxor).mp rfl).2)
  have hleftExtend : packedGF2Polynomial wideWidth left =
      packedGF2Polynomial leftWidth left :=
    packedGF2Polynomial_extend leftWidth rightWidth left hleftFit
  have hrightExtend : packedGF2Polynomial wideWidth right =
      packedGF2Polynomial rightWidth right := by
    dsimp [wideWidth]
    rw [add_comm]
    exact packedGF2Polynomial_extend rightWidth leftWidth right hrightFit
  calc
    packedGF2Polynomial xorWidth xorValue =
        packedGF2Polynomial wideWidth xorValue := hxorExtend.symm
    _ = packedGF2Polynomial wideWidth left +
        packedGF2Polynomial wideWidth right :=
      packedGF2Polynomial_xor wideWidth left right
    _ = packedGF2Polynomial leftWidth left +
        packedGF2Polynomial rightWidth right := by
      rw [hleftExtend, hrightExtend]

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

/-- Polynomial-level meaning of every Boolean branch in the packed Rabin
auxiliary loop. Native square-and-reduce values are decoded at each guard;
their arithmetic semantics are supplied by the Frobenius trace theorem. -/
def gf2PackedRabinSemanticAux
    (polynomial x degree : ℕ) : ℕ → ℕ → ℕ → Prop
  | 0, _, value =>
      packedGF2Polynomial (Nat.log2 value + 1) value =
        packedGF2Polynomial (Nat.log2 x + 1) x
  | remaining + 1, index, value =>
      let next := gf2PackedProduct value value polynomial
      let nextIndex := index + 1
      (nextIndex ≤ degree / 2 →
        EuclideanDomain.gcd
          (packedGF2Polynomial (Nat.log2 polynomial + 1) polynomial)
          (packedGF2Polynomial (Nat.log2 (Nat.xor next x) + 1)
            (Nat.xor next x)) = 1) ∧
        gf2PackedRabinSemanticAux
          polynomial x degree remaining nextIndex next

/-- The native Rabin auxiliary loop returns true exactly when every active
early guard has polynomial GCD one and the final minimally decoded Frobenius
value equals the start. -/
theorem gf2PackedIrreducibleAux_iff_semantic
    (polynomial x degree remaining index value : ℕ) :
    gf2PackedIrreducibleAux polynomial x degree remaining index value = true ↔
      gf2PackedRabinSemanticAux
        polynomial x degree remaining index value := by
  induction remaining generalizing index value with
  | zero =>
      simp only [gf2PackedIrreducibleAux, gf2PackedRabinSemanticAux,
        beq_iff_eq]
      constructor
      · rintro rfl
        rfl
      · exact packedGF2Polynomial_minimal_injective value x
  | succ prior ih =>
      rw [gf2PackedIrreducibleAux, gf2PackedRabinSemanticAux]
      let next := gf2PackedProduct value value polynomial
      let nextIndex := index + 1
      change
        (if decide (nextIndex ≤ degree / 2) &&
            !(gf2PackedGcd (Nat.xor next x) polynomial == 1) then
          false
        else
          gf2PackedIrreducibleAux
            polynomial x degree prior nextIndex next) = true ↔ _
      by_cases hhalf : nextIndex ≤ degree / 2
      · cases hguard :
          (gf2PackedGcd (Nat.xor next x) polynomial == 1) with
        | false =>
            have hhalf' : index < degree / 2 := by
              dsimp [nextIndex] at hhalf
              omega
            have hnotGuard : ¬EuclideanDomain.gcd
                (packedGF2Polynomial (Nat.log2 polynomial + 1) polynomial)
                (packedGF2Polynomial (Nat.log2 (Nat.xor next x) + 1)
                  (Nat.xor next x)) = 1 := by
              intro hpoly
              have hnative := (gf2PackedGcd_guard_iff
                (Nat.xor next x) polynomial).2 hpoly
              rw [hguard] at hnative
              contradiction
            have hnotGuard' : ¬EuclideanDomain.gcd
                (packedGF2Polynomial (Nat.log2 polynomial + 1) polynomial)
                (packedGF2Polynomial
                  (Nat.log2
                    (gf2PackedProduct value value polynomial ^^^ x) + 1)
                  (gf2PackedProduct value value polynomial ^^^ x)) = 1 := by
              simpa [next] using hnotGuard
            simp [hhalf, hnotGuard', nextIndex]
        | true =>
            have hhalf' : index < degree / 2 := by
              dsimp [nextIndex] at hhalf
              omega
            have hpoly := (gf2PackedGcd_guard_iff
              (Nat.xor next x) polynomial).1 hguard
            have hpoly' : EuclideanDomain.gcd
                (packedGF2Polynomial (Nat.log2 polynomial + 1) polynomial)
                (packedGF2Polynomial
                  (Nat.log2
                    (gf2PackedProduct value value polynomial ^^^ x) + 1)
                  (gf2PackedProduct value value polynomial ^^^ x)) = 1 := by
              simpa [next] using hpoly
            simp [hhalf, hpoly', ih, next, nextIndex]
      · have hhalf' : ¬index < degree / 2 := by
          dsimp [nextIndex] at hhalf
          omega
        simp [hhalf, ih, next, nextIndex]

/-- Rabin irreducibility predicate used by the deterministic packed search. -/
def gf2PackedIrreducible (polynomial degree : ℕ) : Bool :=
  let x := gf2PackedRemainder 2 polynomial
  gf2PackedIrreducibleAux polynomial x degree degree 0 x

/-- Polynomial GCD/final-equality specification of the complete packed Rabin
control flow. -/
def gf2PackedRabinSemantic (polynomial degree : ℕ) : Prop :=
  let x := gf2PackedRemainder 2 polynomial
  gf2PackedRabinSemanticAux polynomial x degree degree 0 x

/-- The public packed Rabin Boolean is equivalent to its polynomial semantic
guard trace. This is a control-flow refinement, not yet the external theorem
equating that criterion with irreducibility under all required hypotheses. -/
theorem gf2PackedIrreducible_iff_semantic (polynomial degree : ℕ) :
    gf2PackedIrreducible polynomial degree = true ↔
      gf2PackedRabinSemantic polynomial degree := by
  unfold gf2PackedIrreducible gf2PackedRabinSemantic
  exact gf2PackedIrreducibleAux_iff_semantic
    polynomial (gf2PackedRemainder 2 polynomial)
    degree degree 0 (gf2PackedRemainder 2 polynomial)

/-- Nonrecursive indexed form of the packed Rabin semantic trace: every active
offset checks the corresponding Frobenius iterate, and the final iterate equals
the start. -/
def gf2PackedRabinClosedAux
    (polynomial x degree remaining index value : ℕ) : Prop :=
  (∀ offset, offset < remaining → index + offset + 1 ≤ degree / 2 →
    EuclideanDomain.gcd
      (packedGF2Polynomial (Nat.log2 polynomial + 1) polynomial)
      (packedGF2Polynomial
        (Nat.log2
          (Nat.xor
            (gf2PackedFrobeniusIterate polynomial (offset + 1) value) x) + 1)
        (Nat.xor
          (gf2PackedFrobeniusIterate polynomial (offset + 1) value) x)) = 1) ∧
    packedGF2Polynomial
        (Nat.log2 (gf2PackedFrobeniusIterate polynomial remaining value) + 1)
        (gf2PackedFrobeniusIterate polynomial remaining value) =
      packedGF2Polynomial (Nat.log2 x + 1) x

/-- The implementation-shaped semantic recursion is equivalent to a single
indexed family of Frobenius GCD guards plus its final equality. -/
theorem gf2PackedRabinSemanticAux_iff_closed
    (polynomial x degree remaining index value : ℕ) :
    gf2PackedRabinSemanticAux polynomial x degree remaining index value ↔
      gf2PackedRabinClosedAux
        polynomial x degree remaining index value := by
  induction remaining generalizing index value with
  | zero =>
      simp [gf2PackedRabinSemanticAux, gf2PackedRabinClosedAux,
        gf2PackedFrobeniusIterate]
  | succ prior ih =>
      let next := gf2PackedProduct value value polynomial
      let nextIndex := index + 1
      rw [gf2PackedRabinSemanticAux]
      constructor
      · rintro ⟨hfirst, htail⟩
        have htailClosed := (ih nextIndex next).1 htail
        refine ⟨?_, ?_⟩
        · intro offset hoffset hactive
          cases offset with
          | zero =>
              change EuclideanDomain.gcd
                (packedGF2Polynomial (Nat.log2 polynomial + 1) polynomial)
                (packedGF2Polynomial
                  (Nat.log2 (Nat.xor next x) + 1) (Nat.xor next x)) = 1
              apply hfirst
              dsimp [nextIndex] at hactive
              omega
          | succ priorOffset =>
              have hprior : priorOffset < prior := by omega
              have htailActive :
                  nextIndex + priorOffset + 1 ≤ degree / 2 := by
                dsimp [nextIndex]
                omega
              have hguard := htailClosed.1 priorOffset hprior htailActive
              exact hguard
        · exact htailClosed.2
      · rintro ⟨hguards, hfinal⟩
        refine ⟨?_, (ih nextIndex next).2 ?_⟩
        · intro hactive
          have hguard := hguards 0 (by omega) (by omega)
          exact hguard
        · refine ⟨?_, ?_⟩
          · intro offset hoffset hactive
            have hguard := hguards (offset + 1) (by omega) (by
              dsimp [nextIndex] at hactive
              omega)
            exact hguard
          · exact hfinal

/-- Closed indexed polynomial specification of the public packed Rabin
predicate. -/
def gf2PackedRabinClosed (polynomial degree : ℕ) : Prop :=
  let x := gf2PackedRemainder 2 polynomial
  gf2PackedRabinClosedAux polynomial x degree degree 0 x

/-- The public packed Rabin predicate is equivalent to the closed family of
all required Frobenius GCD guards and its terminal equality. -/
theorem gf2PackedIrreducible_iff_closed (polynomial degree : ℕ) :
    gf2PackedIrreducible polynomial degree = true ↔
      gf2PackedRabinClosed polynomial degree := by
  rw [gf2PackedIrreducible_iff_semantic]
  unfold gf2PackedRabinSemantic gf2PackedRabinClosed
  exact gf2PackedRabinSemanticAux_iff_closed
    polynomial (gf2PackedRemainder 2 polynomial)
    degree degree 0 (gf2PackedRemainder 2 polynomial)

/-- The terminal equality in the closed Rabin trace is the exact
`x^(2^degree)=x` condition modulo the nonzero packed polynomial. -/
theorem gf2PackedRabinClosed_final_power
    (polynomial degree : ℕ) (hpolynomial : polynomial ≠ 0)
    (hsemantic : gf2PackedRabinClosed polynomial degree) :
    ((packedGF2Polynomial
        (Nat.log2 (gf2PackedRemainder 2 polynomial) + 1)
        (gf2PackedRemainder 2 polynomial)) ^ (2 ^ degree)) %ₘ
          packedGF2Polynomial (Nat.log2 polynomial + 1) polynomial =
      packedGF2Polynomial
        (Nat.log2 (gf2PackedRemainder 2 polynomial) + 1)
        (gf2PackedRemainder 2 polynomial) := by
  unfold gf2PackedRabinClosed gf2PackedRabinClosedAux at hsemantic
  have htrace := packedGF2Polynomial_frobeniusIterate_from_remainder
    degree 2 polynomial hpolynomial
  exact htrace.symm.trans hsemantic.2

/-- Every active closed Rabin guard is the GCD of the packed modulus with the
corresponding reduced `2^(offset+1)` power plus the packed start. -/
theorem gf2PackedRabinClosed_guard_power_modByMonic
    (polynomial degree offset : ℕ) (hpolynomial : polynomial ≠ 0)
    (hsemantic : gf2PackedRabinClosed polynomial degree)
    (hhalf : offset < degree / 2) :
    EuclideanDomain.gcd
        (packedGF2Polynomial (Nat.log2 polynomial + 1) polynomial)
        ((((packedGF2Polynomial
            (Nat.log2 (gf2PackedRemainder 2 polynomial) + 1)
            (gf2PackedRemainder 2 polynomial)) ^ (2 ^ (offset + 1))) %ₘ
              packedGF2Polynomial (Nat.log2 polynomial + 1) polynomial) +
          packedGF2Polynomial
            (Nat.log2 (gf2PackedRemainder 2 polynomial) + 1)
            (gf2PackedRemainder 2 polynomial)) = 1 := by
  let x := gf2PackedRemainder 2 polynomial
  let value := gf2PackedFrobeniusIterate polynomial (offset + 1) x
  unfold gf2PackedRabinClosed gf2PackedRabinClosedAux at hsemantic
  have hguard := hsemantic.1 offset (by omega) (by omega)
  have hxor := packedGF2Polynomial_minimal_xor value x
  have htrace := packedGF2Polynomial_frobeniusIterate_from_remainder
    (offset + 1) 2 polynomial hpolynomial
  change packedGF2Polynomial (Nat.log2 (Nat.xor value x) + 1)
    (Nat.xor value x) = _ at hxor
  change packedGF2Polynomial (Nat.log2 value + 1) value = _ at htrace
  rw [hxor, htrace] at hguard
  exact hguard

/-- Reducing one GCD input modulo a monic polynomial before adding a second
input does not change the GCD. -/
theorem gf2Polynomial_gcd_modByMonic_add
    (value addend modulus : Polynomial (ZMod 2)) (hmonic : modulus.Monic) :
    EuclideanDomain.gcd modulus ((value %ₘ modulus) + addend) =
      EuclideanDomain.gcd modulus (value + addend) := by
  have hresidue : ((value %ₘ modulus) + addend) %ₘ modulus =
      (value + addend) %ₘ modulus := by
    apply Polynomial.modByMonic_eq_of_dvd_sub hmonic
    simpa only [add_sub_add_right_eq_sub] using
      Polynomial.dvd_modByMonic_sub value modulus
  calc
    EuclideanDomain.gcd modulus ((value %ₘ modulus) + addend) =
        EuclideanDomain.gcd (((value %ₘ modulus) + addend) % modulus)
          modulus := EuclideanDomain.gcd_val _ _
    _ = EuclideanDomain.gcd ((value + addend) % modulus) modulus := by
      rw [← Polynomial.modByMonic_eq_mod _ hmonic,
        ← Polynomial.modByMonic_eq_mod _ hmonic, hresidue]
    _ = EuclideanDomain.gcd modulus (value + addend) :=
      (EuclideanDomain.gcd_val _ _).symm

/-- The inner implementation remainder can be removed from every active
closed Rabin GCD guard. -/
theorem gf2PackedRabinClosed_guard_add
    (polynomial degree offset : ℕ) (hpolynomial : polynomial ≠ 0)
    (hsemantic : gf2PackedRabinClosed polynomial degree)
    (hhalf : offset < degree / 2) :
    EuclideanDomain.gcd
        (packedGF2Polynomial (Nat.log2 polynomial + 1) polynomial)
        ((packedGF2Polynomial
            (Nat.log2 (gf2PackedRemainder 2 polynomial) + 1)
            (gf2PackedRemainder 2 polynomial)) ^ (2 ^ (offset + 1)) +
          packedGF2Polynomial
            (Nat.log2 (gf2PackedRemainder 2 polynomial) + 1)
            (gf2PackedRemainder 2 polynomial)) = 1 := by
  let modulusPolynomial :=
    packedGF2Polynomial (Nat.log2 polynomial + 1) polynomial
  let xPolynomial :=
    packedGF2Polynomial
      (Nat.log2 (gf2PackedRemainder 2 polynomial) + 1)
      (gf2PackedRemainder 2 polynomial)
  have hguard := gf2PackedRabinClosed_guard_power_modByMonic
    polynomial degree offset hpolynomial hsemantic hhalf
  have hmonic : modulusPolynomial.Monic :=
    packedGF2Polynomial_monic (Nat.log2 polynomial + 1) polynomial
      hpolynomial (by omega)
  change EuclideanDomain.gcd modulusPolynomial
    (((xPolynomial ^ (2 ^ (offset + 1))) %ₘ modulusPolynomial) +
      xPolynomial) = 1 at hguard
  rw [gf2Polynomial_gcd_modByMonic_add
    (xPolynomial ^ (2 ^ (offset + 1))) xPolynomial modulusPolynomial hmonic]
    at hguard
  exact hguard

/-- Addition and subtraction coincide for polynomials over `GF(2)`. -/
theorem gf2Polynomial_add_eq_sub (left right : Polynomial (ZMod 2)) :
    left + right = left - right := by
  rw [sub_eq_add_neg]
  congr 1
  ext index
  rw [Polynomial.coeff_neg, ZMod.neg_eq_self_mod_two]

/-- Every active closed Rabin guard has the standard textbook form
`gcd(f, x^(2^i) - x) = 1`. -/
theorem gf2PackedRabinClosed_guard_sub
    (polynomial degree offset : ℕ) (hpolynomial : polynomial ≠ 0)
    (hsemantic : gf2PackedRabinClosed polynomial degree)
    (hhalf : offset < degree / 2) :
    EuclideanDomain.gcd
        (packedGF2Polynomial (Nat.log2 polynomial + 1) polynomial)
        ((packedGF2Polynomial
            (Nat.log2 (gf2PackedRemainder 2 polynomial) + 1)
            (gf2PackedRemainder 2 polynomial)) ^ (2 ^ (offset + 1)) -
          packedGF2Polynomial
            (Nat.log2 (gf2PackedRemainder 2 polynomial) + 1)
            (gf2PackedRemainder 2 polynomial)) = 1 := by
  rw [← gf2Polynomial_add_eq_sub]
  exact gf2PackedRabinClosed_guard_add polynomial degree offset
    hpolynomial hsemantic hhalf

/-- A degree-`n` polynomial over `GF(2)` is irreducible if it is coprime to
`X^(2^i)-X` at every positive index through `n/2`. This all-half-degrees
criterion is stronger than the usual prime-divisor subset of Rabin guards. -/
theorem gf2Polynomial_irreducible_of_half_degree_gcd
    (polynomial : Polynomial (ZMod 2)) (degree : ℕ)
    (hdegree : polynomial.natDegree = degree) (hdegreeTwo : 2 ≤ degree)
    (hguards : ∀ index, 0 < index → index ≤ degree / 2 →
      EuclideanDomain.gcd polynomial (X ^ (2 ^ index) - X) = 1) :
    Irreducible polynomial := by
  have hpolynomial : polynomial ≠ 0 := by
    intro hzero
    rw [hzero, natDegree_zero] at hdegree
    omega
  rw [irreducible_iff]
  refine ⟨?_, ?_⟩
  · intro hunit
    have hzero := natDegree_eq_zero_of_isUnit hunit
    omega
  · intro left right hfactor
    by_cases hleftUnit : IsUnit left
    · exact Or.inl hleftUnit
    by_cases hrightUnit : IsUnit right
    · exact Or.inr hrightUnit
    exfalso
    have hproduct : left * right ≠ 0 := by
      rw [← hfactor]
      exact hpolynomial
    have hleft : left ≠ 0 := left_ne_zero_of_mul hproduct
    have hright : right ≠ 0 := right_ne_zero_of_mul hproduct
    have hleftPos : 0 < left.natDegree := by
      apply Nat.pos_of_ne_zero
      intro hzero
      apply hleftUnit
      rw [Polynomial.isUnit_iff_degree_eq_zero,
        Polynomial.degree_eq_natDegree hleft, hzero]
      rfl
    have hrightPos : 0 < right.natDegree := by
      apply Nat.pos_of_ne_zero
      intro hzero
      apply hrightUnit
      rw [Polynomial.isUnit_iff_degree_eq_zero,
        Polynomial.degree_eq_natDegree hright, hzero]
      rfl
    have hsum : left.natDegree + right.natDegree = degree := by
      rw [← Polynomial.natDegree_mul hleft hright, ← hfactor, hdegree]
    have factor_contradiction
        (small : Polynomial (ZMod 2)) (hsmall : small ≠ 0)
        (hsmallPos : 0 < small.natDegree) (hsmallDvd : small ∣ polynomial)
        (hsmallDegree : small.natDegree ≤ degree / 2) : False := by
      obtain ⟨factor, hfactorIrreducible, hfactorSmall⟩ :=
        Polynomial.exists_irreducible_of_natDegree_pos hsmallPos
      have hfactorPolynomial : factor ∣ polynomial :=
        dvd_trans hfactorSmall hsmallDvd
      have hfactorDegree : factor.natDegree ≤ degree / 2 :=
        le_trans (Polynomial.natDegree_le_of_dvd hfactorSmall hsmall)
          hsmallDegree
      have hfactorPos := hfactorIrreducible.natDegree_pos
      have hfactorPower : factor ∣ X ^ (2 ^ factor.natDegree) - X := by
        have hpower :=
          (Irreducible.natDegree_dvd_iff_dvd_X_pow_card_pow_sub_X
            hfactorIrreducible).1 dvd_rfl
        simpa using hpower
      have hfactorGcd : factor ∣ EuclideanDomain.gcd polynomial
          (X ^ (2 ^ factor.natDegree) - X) :=
        EuclideanDomain.dvd_gcd hfactorPolynomial hfactorPower
      rw [hguards factor.natDegree hfactorPos hfactorDegree] at hfactorGcd
      exact hfactorIrreducible.not_isUnit (isUnit_iff_dvd_one.2 hfactorGcd)
    by_cases hleftDegree : left.natDegree ≤ degree / 2
    · exact factor_contradiction left hleft hleftPos
        (hfactor ▸ dvd_mul_right left right) hleftDegree
    · have hrightDegree : right.natDegree ≤ degree / 2 := by omega
      exact factor_contradiction right hright hrightPos
        (hfactor ▸ dvd_mul_left right left) hrightDegree

/-- When the packed modulus has degree at least two, the native Rabin start
`2` is already reduced. -/
theorem gf2PackedRemainder_two_of_log2 (polynomial degree : ℕ)
    (hlog : Nat.log2 polynomial = degree) (hdegreeTwo : 2 ≤ degree) :
    gf2PackedRemainder 2 polynomial = 2 := by
  unfold gf2PackedRemainder
  rw [gf2PackedRemainderFuel]
  simp [hlog]
  omega

/-- Minimal packed decoding sends the native bit pattern `2` to `X`. -/
theorem packedGF2Polynomial_two :
    packedGF2Polynomial (Nat.log2 2 + 1) 2 = X := by
  change packedGF2Polynomial 2 2 = X
  ext index
  by_cases hzero : index = 0
  · subst index
    have hcoeff := packedGF2Polynomial_coeff 2 2 (⟨0, by omega⟩ : Fin 2)
    norm_num at hcoeff ⊢
    exact hcoeff
  by_cases hone : index = 1
  · subst index
    have hcoeff := packedGF2Polynomial_coeff 2 2 (⟨1, by omega⟩ : Fin 2)
    norm_num at hcoeff ⊢
    exact hcoeff
  have htwo : 2 ≤ index := by omega
  rw [packedGF2Polynomial_coeff_eq_zero_of_le 2 2 index htwo, coeff_X]
  simp [show (1 : ℕ) ≠ index by omega]

/-- Soundness of the public packed Rabin predicate: when the supplied degree
matches the packed modulus and is at least two, acceptance implies genuine
polynomial irreducibility over `GF(2)`. -/
theorem gf2PackedIrreducible_sound
    (polynomial degree : ℕ) (hlog : Nat.log2 polynomial = degree)
    (hdegreeTwo : 2 ≤ degree)
    (haccepted : gf2PackedIrreducible polynomial degree = true) :
    Irreducible
      (packedGF2Polynomial (Nat.log2 polynomial + 1) polynomial) := by
  have hpolynomial : polynomial ≠ 0 := by
    intro hzero
    subst polynomial
    simp at hlog
    omega
  have hclosed :=
    (gf2PackedIrreducible_iff_closed polynomial degree).1 haccepted
  apply gf2Polynomial_irreducible_of_half_degree_gcd
      (packedGF2Polynomial (Nat.log2 polynomial + 1) polynomial) degree
  · exact (packedGF2Polynomial_natDegree
      (Nat.log2 polynomial + 1) polynomial hpolynomial (by omega)).trans hlog
  · exact hdegreeTwo
  · intro index hindex hhalf
    have hguard := gf2PackedRabinClosed_guard_sub
      polynomial degree (index - 1) hpolynomial hclosed (by omega)
    have hremainder :=
      gf2PackedRemainder_two_of_log2 polynomial degree hlog hdegreeTwo
    rw [hremainder] at hguard
    rw [packedGF2Polynomial_two] at hguard
    have hoffset : index - 1 + 1 = index := by omega
    simpa only [hoffset] using hguard

/-- Every unit polynomial over `GF(2)` is one: the coefficient field has only
one nonzero scalar. -/
theorem gf2Polynomial_eq_one_of_isUnit
    (polynomial : Polynomial (ZMod 2)) (hunit : IsUnit polynomial) :
    polynomial = 1 := by
  obtain ⟨coefficient, hcoefficient, heq⟩ := Polynomial.isUnit_iff.mp hunit
  subst polynomial
  have hcoefficient0 : coefficient ≠ 0 := hcoefficient.ne_zero
  have hcoefficient1 : coefficient = 1 := by
    fin_cases coefficient
    · exfalso
      exact hcoefficient0 rfl
    · rfl
  rw [hcoefficient1, map_one]

/-- An irreducible exact-degree polynomial is coprime to every
`X^(2^i)-X` with `0 < i < degree`. -/
theorem gf2Polynomial_irreducible_guard
    (polynomial : Polynomial (ZMod 2)) (degree index : ℕ)
    (hirreducible : Irreducible polynomial)
    (hdegree : polynomial.natDegree = degree)
    (hindex : 0 < index) (hlt : index < degree) :
    EuclideanDomain.gcd polynomial (X ^ (2 ^ index) - X) = 1 := by
  have hnotDvd : ¬polynomial ∣ X ^ (2 ^ index) - X := by
    intro hdvd
    have hdegreeDvd : polynomial.natDegree ∣ index :=
      (Irreducible.natDegree_dvd_iff_dvd_X_pow_card_pow_sub_X
        hirreducible).2 (by simpa using hdvd)
    rw [hdegree] at hdegreeDvd
    exact (Nat.not_le_of_gt hlt) (Nat.le_of_dvd hindex hdegreeDvd)
  apply gf2Polynomial_eq_one_of_isUnit
  exact EuclideanDomain.gcd_isUnit_iff.mpr
    (hirreducible.coprime_iff_not_dvd.mpr hnotDvd)

/-- Completeness of the public packed Rabin predicate: an irreducible packed
polynomial of matching degree at least two makes every native GCD guard and the
terminal Frobenius comparison succeed. -/
theorem gf2PackedIrreducible_complete
    (polynomial degree : ℕ) (hlog : Nat.log2 polynomial = degree)
    (hdegreeTwo : 2 ≤ degree)
    (hirreducible : Irreducible
      (packedGF2Polynomial (Nat.log2 polynomial + 1) polynomial)) :
    gf2PackedIrreducible polynomial degree = true := by
  let modulusPolynomial :=
    packedGF2Polynomial (Nat.log2 polynomial + 1) polynomial
  let x := gf2PackedRemainder 2 polynomial
  let xPolynomial := packedGF2Polynomial (Nat.log2 x + 1) x
  have hpolynomial : polynomial ≠ 0 := by
    intro hzero
    subst polynomial
    simp at hlog
    omega
  have hx : x = 2 :=
    gf2PackedRemainder_two_of_log2 polynomial degree hlog hdegreeTwo
  have hxPolynomial : xPolynomial = X := by
    dsimp [xPolynomial]
    rw [hx, packedGF2Polynomial_two]
  have hstartPolynomial :
      packedGF2Polynomial
        (Nat.log2 (gf2PackedRemainder 2 polynomial) + 1)
        (gf2PackedRemainder 2 polynomial) = X := by
    change xPolynomial = X
    exact hxPolynomial
  have hdegree : modulusPolynomial.natDegree = degree :=
    (packedGF2Polynomial_natDegree
      (Nat.log2 polynomial + 1) polynomial hpolynomial (by omega)).trans hlog
  have hmonic : modulusPolynomial.Monic :=
    packedGF2Polynomial_monic (Nat.log2 polynomial + 1) polynomial
      hpolynomial (by omega)
  apply (gf2PackedIrreducible_iff_closed polynomial degree).2
  unfold gf2PackedRabinClosed gf2PackedRabinClosedAux
  change (∀ offset, offset < degree → 0 + offset + 1 ≤ degree / 2 →
      EuclideanDomain.gcd modulusPolynomial
        (packedGF2Polynomial
          (Nat.log2
            (Nat.xor
              (gf2PackedFrobeniusIterate polynomial (offset + 1) x) x) + 1)
          (Nat.xor
            (gf2PackedFrobeniusIterate polynomial (offset + 1) x) x)) = 1) ∧
    packedGF2Polynomial
        (Nat.log2 (gf2PackedFrobeniusIterate polynomial degree x) + 1)
        (gf2PackedFrobeniusIterate polynomial degree x) = xPolynomial
  refine ⟨?_, ?_⟩
  · intro offset hoffset hactive
    let value := gf2PackedFrobeniusIterate polynomial (offset + 1) x
    have hstandard := gf2Polynomial_irreducible_guard modulusPolynomial degree
      (offset + 1) hirreducible hdegree (by omega) (by omega)
    have hplus : EuclideanDomain.gcd modulusPolynomial
        (X ^ (2 ^ (offset + 1)) + X) = 1 := by
      rw [gf2Polynomial_add_eq_sub]
      exact hstandard
    have hreduced : EuclideanDomain.gcd modulusPolynomial
        (((X ^ (2 ^ (offset + 1))) %ₘ modulusPolynomial) + X) = 1 := by
      rw [gf2Polynomial_gcd_modByMonic_add
        (X ^ (2 ^ (offset + 1))) X modulusPolynomial hmonic]
      exact hplus
    have hxor := packedGF2Polynomial_minimal_xor value x
    have htrace := packedGF2Polynomial_frobeniusIterate_from_remainder
      (offset + 1) 2 polynomial hpolynomial
    change packedGF2Polynomial (Nat.log2 (Nat.xor value x) + 1)
      (Nat.xor value x) = _ at hxor
    change packedGF2Polynomial (Nat.log2 value + 1) value = _ at htrace
    rw [hxor, htrace]
    rw [hstartPolynomial]
    exact hreduced
  · have htrace := packedGF2Polynomial_frobeniusIterate_from_remainder
      degree 2 polynomial hpolynomial
    have hfinalDvd : modulusPolynomial ∣ X ^ (2 ^ degree) - X := by
      have hpower :=
        (Irreducible.natDegree_dvd_iff_dvd_X_pow_card_pow_sub_X
          hirreducible).1 dvd_rfl
      rw [hdegree] at hpower
      simpa using hpower
    have hXReduced : X %ₘ modulusPolynomial = X := by
      apply (Polynomial.modByMonic_eq_self_iff hmonic).2
      rw [degree_X, degree_eq_natDegree hmonic.ne_zero, hdegree]
      exact_mod_cast (show 1 < degree by omega)
    have hfinal : (X ^ (2 ^ degree)) %ₘ modulusPolynomial = X := by
      calc
        (X ^ (2 ^ degree)) %ₘ modulusPolynomial = X %ₘ modulusPolynomial :=
          Polynomial.modByMonic_eq_of_dvd_sub hmonic hfinalDvd
        _ = X := hXReduced
    change packedGF2Polynomial
      (Nat.log2 (gf2PackedFrobeniusIterate polynomial degree x) + 1)
      (gf2PackedFrobeniusIterate polynomial degree x) = xPolynomial
    rw [htrace]
    rw [hstartPolynomial]
    rw [hxPolynomial]
    exact hfinal

/-- Full correctness of the public packed Rabin predicate for its native
domain: at matching packed degree at least two, the Boolean returns true
exactly for irreducible `GF(2)` polynomials. -/
theorem gf2PackedIrreducible_iff_irreducible
    (polynomial degree : ℕ) (hlog : Nat.log2 polynomial = degree)
    (hdegreeTwo : 2 ≤ degree) :
    gf2PackedIrreducible polynomial degree = true ↔
      Irreducible
        (packedGF2Polynomial (Nat.log2 polynomial + 1) polynomial) := by
  constructor
  · exact gf2PackedIrreducible_sound polynomial degree hlog hdegreeTwo
  · exact gf2PackedIrreducible_complete polynomial degree hlog hdegreeTwo

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
