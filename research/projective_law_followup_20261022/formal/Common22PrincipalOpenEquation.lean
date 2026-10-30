import Mathlib.Tactic.Ring
import Mathlib.RingTheory.Ideal.Quotient.Operations
import Mathlib.RingTheory.Localization.Away.Basic
import Common22ExplicitChart

/-! A universal algebra identity for the principal-open overlap. It holds in
every commutative ring with a chosen inverse pair `x*s=1`. This transports the
two equations up to a unit factor but does not yet build the corresponding
localized quotient-ring isomorphism. -/

set_option autoImplicit false

namespace Common22PrincipalOpenEquation

variable {R : Type*} [CommRing R]

theorem inverse_power_cancel (x s : R) (hxs : x * s = 1) (n k : ℕ) :
    s ^ (n + k) * x ^ n = s ^ k := by
  calc
    s ^ (n + k) * x ^ n = (x * s) ^ n * s ^ k := by
      rw [pow_add, mul_pow]
      ring
    _ = s ^ k := by rw [hxs]; simp

theorem equation_unit_multiple
    (a3 a2 a1 a0 x y s : R) (hxs : x * s = 1) :
    s ^ 6 * (y ^ 2 - (x ^ 5 + a3 * x ^ 3 + a2 * x ^ 2 + a1 * x + a0)) =
      (y * s ^ 3) ^ 2 -
        s * (1 + a3 * s ^ 2 + a2 * s ^ 3 + a1 * s ^ 4 + a0 * s ^ 5) := by
  have h5 : s ^ 6 * x ^ 5 = s := by
    simpa using inverse_power_cancel x s hxs 5 1
  have h3 : s ^ 6 * x ^ 3 = s ^ 3 := by
    simpa using inverse_power_cancel x s hxs 3 3
  have h2 : s ^ 6 * x ^ 2 = s ^ 4 := by
    simpa using inverse_power_cancel x s hxs 2 4
  have h1 : s ^ 6 * x = s ^ 5 := by
    simpa only [pow_one] using
      inverse_power_cancel x s hxs 1 5
  calc
    s ^ 6 * (y ^ 2 - (x ^ 5 + a3 * x ^ 3 + a2 * x ^ 2 + a1 * x + a0)) =
      s ^ 6 * y ^ 2 - (s ^ 6 * x ^ 5 + a3 * (s ^ 6 * x ^ 3) +
        a2 * (s ^ 6 * x ^ 2) + a1 * (s ^ 6 * x) + a0 * s ^ 6) := by ring
    _ = _ := by rw [h5, h3, h2, h1]; ring

theorem equation_iff_on_principal_open
    (a3 a2 a1 a0 x y s : R) (hxs : x * s = 1) :
    y ^ 2 = x ^ 5 + a3 * x ^ 3 + a2 * x ^ 2 + a1 * x + a0 ↔
      (y * s ^ 3) ^ 2 =
        s * (1 + a3 * s ^ 2 + a2 * s ^ 3 + a1 * s ^ 4 + a0 * s ^ 5) := by
  have hmul := equation_unit_multiple a3 a2 a1 a0 x y s hxs
  have hpow : x ^ 6 * s ^ 6 = 1 := by
    rw [← mul_pow, hxs, one_pow]
  constructor
  · intro h
    have hF : y ^ 2 -
        (x ^ 5 + a3 * x ^ 3 + a2 * x ^ 2 + a1 * x + a0) = 0 :=
      sub_eq_zero.mpr h
    rw [hF, mul_zero] at hmul
    exact sub_eq_zero.mp hmul.symm
  · intro h
    have hG : (y * s ^ 3) ^ 2 -
        s * (1 + a3 * s ^ 2 + a2 * s ^ 3 + a1 * s ^ 4 + a0 * s ^ 5) = 0 :=
      sub_eq_zero.mpr h
    rw [hG] at hmul
    have hcancel := congrArg (fun v : R => x ^ 6 * v) hmul
    have hF : y ^ 2 -
        (x ^ 5 + a3 * x ^ 3 + a2 * x ^ 2 + a1 * x + a0) = 0 := by
      simpa [← mul_assoc, hpow] using hcancel
    exact sub_eq_zero.mp hF

theorem principal_equation_ideals_equal
    (a3 a2 a1 a0 x y s : R) (hxs : x * s = 1) :
    Ideal.span ({(y * s ^ 3) ^ 2 -
      s * (1 + a3 * s ^ 2 + a2 * s ^ 3 + a1 * s ^ 4 + a0 * s ^ 5)} : Set R) =
    Ideal.span ({y ^ 2 -
      (x ^ 5 + a3 * x ^ 3 + a2 * x ^ 2 + a1 * x + a0)} : Set R) := by
  have hpow : x ^ 6 * s ^ 6 = 1 := by
    rw [← mul_pow, hxs, one_pow]
  have hs : IsUnit (s ^ 6) :=
    isUnit_iff_dvd_one.mpr ⟨x ^ 6, by simpa only [mul_comm] using hpow.symm⟩
  rw [← equation_unit_multiple a3 a2 a1 a0 x y s hxs]
  exact Ideal.span_singleton_mul_left_unit hs _

noncomputable def common_ambient_quotient_equiv
    (a3 a2 a1 a0 x y s : R) (hxs : x * s = 1) :
    (R ⧸ Ideal.span ({(y * s ^ 3) ^ 2 -
      s * (1 + a3 * s ^ 2 + a2 * s ^ 3 + a1 * s ^ 4 + a0 * s ^ 5)} : Set R)) ≃+*
    (R ⧸ Ideal.span ({y ^ 2 -
      (x ^ 5 + a3 * x ^ 3 + a2 * x ^ 2 + a1 * x + a0)} : Set R)) := by
  rw [principal_equation_ideals_equal a3 a2 a1 a0 x y s hxs]

#print axioms inverse_power_cancel
#print axioms equation_unit_multiple
#print axioms equation_iff_on_principal_open
#print axioms principal_equation_ideals_equal
#print axioms common_ambient_quotient_equiv

end Common22PrincipalOpenEquation

namespace Common22PrincipalOpenEquation

variable {K : Type*} [Field K]

abbrev OpenPolynomial (K : Type*) [CommRing K] := MvPolynomial (Fin 2) K

abbrev XInvertedAmbient (K : Type*) [CommRing K] :=
  Localization.Away (MvPolynomial.X 0 : OpenPolynomial K)

noncomputable def openX : XInvertedAmbient K :=
  algebraMap (OpenPolynomial K) (XInvertedAmbient K) (MvPolynomial.X 0)

noncomputable def openY : XInvertedAmbient K :=
  algebraMap (OpenPolynomial K) (XInvertedAmbient K) (MvPolynomial.X 1)

noncomputable def openInvX : XInvertedAmbient K :=
  IsLocalization.Away.invSelf (S := XInvertedAmbient K)
    (MvPolynomial.X 0 : OpenPolynomial K)

theorem openX_mul_openInvX : openX (K := K) * openInvX (K := K) = 1 :=
  IsLocalization.Away.mul_invSelf (MvPolynomial.X 0 : OpenPolynomial K)

noncomputable def explicit_common_ambient_quotient_equiv
    (a3 a2 a1 a0 : K) :
    let R := XInvertedAmbient K
    let c := algebraMap (OpenPolynomial K) R ∘ MvPolynomial.C
    R ⧸ Ideal.span ({(openY (K := K) * openInvX (K := K) ^ 3) ^ 2 -
      openInvX (K := K) *
        (1 + c a3 * openInvX (K := K) ^ 2 +
          c a2 * openInvX (K := K) ^ 3 +
          c a1 * openInvX (K := K) ^ 4 +
          c a0 * openInvX (K := K) ^ 5)} : Set R) ≃+*
    R ⧸ Ideal.span ({openY (K := K) ^ 2 -
      (openX (K := K) ^ 5 + c a3 * openX (K := K) ^ 3 +
        c a2 * openX (K := K) ^ 2 + c a1 * openX (K := K) + c a0)} : Set R) := by
  exact common_ambient_quotient_equiv
    ((algebraMap (OpenPolynomial K) (XInvertedAmbient K)) (MvPolynomial.C a3))
    ((algebraMap (OpenPolynomial K) (XInvertedAmbient K)) (MvPolynomial.C a2))
    ((algebraMap (OpenPolynomial K) (XInvertedAmbient K)) (MvPolynomial.C a1))
    ((algebraMap (OpenPolynomial K) (XInvertedAmbient K)) (MvPolynomial.C a0))
    (openX (K := K)) (openY (K := K)) (openInvX (K := K))
    (openX_mul_openInvX (K := K))

#print axioms openX_mul_openInvX
#print axioms explicit_common_ambient_quotient_equiv

end Common22PrincipalOpenEquation
