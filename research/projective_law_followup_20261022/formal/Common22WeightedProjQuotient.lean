import Common22WeightedProjHead
import Mathlib.RingTheory.Ideal.Quotient.Operations

/-!
The `Z = 1` specialization of the weighted homogeneous coordinate-ring
quotient.  This is a map of ordinary quotient rings; identifying the degree
zero part of the localization at `Z` is a further obligation.
-/

set_option autoImplicit false
noncomputable section

namespace Common22WeightedProjQuotient

open MvPolynomial Common22WeightedProjHead

variable {K : Type*} [Field K]

abbrev WeightedRing (a3 a2 a1 a0 : K) :=
  Poly K ⧸ relationIdeal a3 a2 a1 a0

abbrev AffinePoly (K : Type*) [CommRing K] := MvPolynomial (Fin 2) K

def affineEquation (a3 a2 a1 a0 : K) : AffinePoly K :=
  X 1 ^ 2 - (X 0 ^ 5 + C a3 * X 0 ^ 3 + C a2 * X 0 ^ 2 + C a1 * X 0 + C a0)

def affineIdeal (a3 a2 a1 a0 : K) : Ideal (AffinePoly K) :=
  Ideal.span {affineEquation a3 a2 a1 a0}

abbrev AffineRing (a3 a2 a1 a0 : K) :=
  AffinePoly K ⧸ affineIdeal a3 a2 a1 a0

def xbar (a3 a2 a1 a0 : K) : AffineRing a3 a2 a1 a0 :=
  Ideal.Quotient.mk (affineIdeal a3 a2 a1 a0) (X 0)

def ybar (a3 a2 a1 a0 : K) : AffineRing a3 a2 a1 a0 :=
  Ideal.Quotient.mk (affineIdeal a3 a2 a1 a0) (X 1)

def specialize (a3 a2 a1 a0 : K) :
    Poly K →ₐ[K] AffineRing a3 a2 a1 a0 :=
  MvPolynomial.aeval (fun i : Fin 3 =>
    if i = 0 then xbar a3 a2 a1 a0
    else if i = 1 then ybar a3 a2 a1 a0 else 1)

theorem specialize_equation (a3 a2 a1 a0 : K) :
    specialize a3 a2 a1 a0 (equation a3 a2 a1 a0) = 0 := by
  have h : (Ideal.Quotient.mk (affineIdeal a3 a2 a1 a0))
      (affineEquation a3 a2 a1 a0) = 0 := by
    exact Ideal.Quotient.eq_zero_iff_mem.mpr (Ideal.subset_span (by simp))
  have hC (a : K) :
      (Ideal.Quotient.mk (affineIdeal a3 a2 a1 a0)) (C a) =
        algebraMap K (AffineRing a3 a2 a1 a0) a := rfl
  simpa [specialize, equation, affineEquation, xbar, ybar, map_sub,
    map_add, map_mul, map_pow, hC] using h

def toAffine (a3 a2 a1 a0 : K) :
    WeightedRing a3 a2 a1 a0 →ₐ[K] AffineRing a3 a2 a1 a0 :=
  Ideal.Quotient.liftₐ (relationIdeal a3 a2 a1 a0)
    (specialize a3 a2 a1 a0) (by
      intro p hp
      rw [relationIdeal, Ideal.mem_span_singleton] at hp
      obtain ⟨q, rfl⟩ := hp
      simp [specialize_equation])

theorem toAffine_mk (a3 a2 a1 a0 : K) (p : Poly K) :
    toAffine a3 a2 a1 a0
      (Ideal.Quotient.mk (relationIdeal a3 a2 a1 a0) p) =
        specialize a3 a2 a1 a0 p := rfl

theorem toAffine_X (a3 a2 a1 a0 : K) :
    toAffine a3 a2 a1 a0
      (Ideal.Quotient.mk (relationIdeal a3 a2 a1 a0) (X 0)) =
        xbar a3 a2 a1 a0 := by simp [toAffine_mk, specialize]

theorem toAffine_Y (a3 a2 a1 a0 : K) :
    toAffine a3 a2 a1 a0
      (Ideal.Quotient.mk (relationIdeal a3 a2 a1 a0) (X 1)) =
        ybar a3 a2 a1 a0 := by simp [toAffine_mk, specialize]

theorem toAffine_Z (a3 a2 a1 a0 : K) :
    toAffine a3 a2 a1 a0
      (Ideal.Quotient.mk (relationIdeal a3 a2 a1 a0) (X 2)) = 1 := by
  simp [toAffine_mk, specialize]

def embedAffine : AffinePoly K →ₐ[K] Poly K :=
  MvPolynomial.aeval (fun i : Fin 2 => X i.castSucc)

theorem specialize_embedAffine (a3 a2 a1 a0 : K) (p : AffinePoly K) :
    specialize a3 a2 a1 a0 (embedAffine p) =
      Ideal.Quotient.mk (affineIdeal a3 a2 a1 a0) p := by
  let f : AffinePoly K →ₐ[K] AffineRing a3 a2 a1 a0 :=
    (specialize a3 a2 a1 a0).comp embedAffine
  let g : AffinePoly K →ₐ[K] AffineRing a3 a2 a1 a0 :=
    Ideal.Quotient.mkₐ K (affineIdeal a3 a2 a1 a0)
  have hfg : f = g := by
    apply MvPolynomial.algHom_ext
    intro i
    fin_cases i <;> simp [f, g, embedAffine, specialize, xbar, ybar]
  exact AlgHom.congr_fun hfg p

theorem toAffine_surjective (a3 a2 a1 a0 : K) :
    Function.Surjective (toAffine a3 a2 a1 a0) := by
  intro b
  obtain ⟨p, rfl⟩ := Ideal.Quotient.mk_surjective b
  exact ⟨Ideal.Quotient.mk (relationIdeal a3 a2 a1 a0) (embedAffine p),
    by simpa only [toAffine_mk] using specialize_embedAffine a3 a2 a1 a0 p⟩

theorem z_sub_one_mem_kernel (a3 a2 a1 a0 : K) :
    Ideal.Quotient.mk (relationIdeal a3 a2 a1 a0) (X 2) - 1 ∈
      RingHom.ker (toAffine a3 a2 a1 a0).toRingHom := by
  simp [RingHom.mem_ker, toAffine_Z]

#print axioms toAffine_mk
#print axioms toAffine_X
#print axioms toAffine_Y
#print axioms toAffine_Z
#print axioms specialize_embedAffine
#print axioms toAffine_surjective
#print axioms z_sub_one_mem_kernel

#print axioms specialize_equation

end Common22WeightedProjQuotient
