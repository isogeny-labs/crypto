module
public import NodeMultiplicity
public import Mathlib.Algebra.Polynomial.Taylor
public import Mathlib.Algebra.Polynomial.RingDivision
public import Mathlib.RingTheory.Coprime.Lemmas
public section
namespace HermiteUniqueness
open Polynomial NodeMultiplicity

/-- Exact low Taylor coefficients, not finite-precision residue classes. -/
theorem taylorJetsDivisibility {R : Type*} [CommRing R] (t : R) (q : ℕ) (F G : Polynomial R)
    (hjet : ∀ j<q, (taylor t F).coeff j=(taylor t G).coeff j) :
    (X-C t)^q ∣ F-G := by
  have hd : (X:Polynomial R)^q ∣ taylor t (F-G) := by
    apply X_pow_dvd_iff.mpr
    intro j hj
    simp only [map_sub,coeff_sub,hjet j hj,sub_self]
  obtain ⟨H,hH⟩ := hd
  have he := congrArg (taylor (-t)) hH
  refine ⟨taylor (-t) H,?_⟩
  simpa only [taylor_taylor,neg_add_cancel,taylor_zero,taylor_mul,taylor_pow,
    taylor_X,map_neg,sub_eq_add_neg] using he

/-- Unit node differences suffice even when the coefficient ring has zero divisors. -/
theorem exactHermiteUnique {R ι : Type*} [CommRing R] [Fintype ι]
    (t : ι→R) (q : ι→ℕ) (hunit : ∀ i j, i≠j → IsUnit (t i-t j))
    (F G : Polynomial R) (hF : F.natDegree < ∑ i, q i) (hG : G.natDegree < ∑ i, q i)
    (hjet : ∀ i j, j<q i → (taylor (t i) F).coeff j=(taylor (t i) G).coeff j) : F=G := by
  classical
  nontriviality R
  let factors (i : ι) : Polynomial R := (X-C (t i))^(q i)
  have hmon (i : ι) : (factors i).Monic := (monic_X_sub_C _).pow _
  have hpair : Pairwise (fun i j => IsCoprime (factors i) (factors j)) := by
    intro i j hij
    exact (isCoprime_X_sub_C_of_isUnit_sub (hunit i j hij)).pow
  have hd : (∏ i, factors i) ∣ F-G :=
    Fintype.prod_dvd_of_coprime hpair (fun i => taylorJetsDivisibility (t i) (q i) F G (hjet i))
  have hpmon : (∏ i, factors i).Monic := monic_prod_of_monic _ _ (fun i hi => hmon i)
  have hpdeg : (∏ i, factors i).natDegree = ∑ i, q i := by
    rw [natDegree_prod_of_monic _ _ (fun i hi => hmon i)]
    apply Finset.sum_congr rfl
    intro i hi
    simp only [factors,(monic_X_sub_C (t i)).natDegree_pow,natDegree_X_sub_C,mul_one]
  by_contra hne
  have hn : F-G≠0 := sub_ne_zero.mpr hne
  have hlow := hpmon.natDegree_le_of_dvd hn hd
  rw [hpdeg] at hlow
  have hupper := (natDegree_sub_le F G).trans_lt (max_lt hF hG)
  omega

@[expose] def sourceOrder (a M u : ℕ) (m : Fin (2^(a+1))) : ℕ :=
  if gridNode a m=0 then 0 else sourceCeiling M u*(u-padicValInt 2 (gridNode a m))

@[expose] noncomputable def sourcePoint {R : Type*} [CommRing R] [Algebra ℚ R]
    (a : ℕ) (m : Fin (2^(a+1))) : R := algebraMap ℚ R (gridNode a m)

theorem sourceGridInjective (a : ℕ) : Function.Injective (gridNode a) := by
  intro i j h
  apply Fin.ext
  unfold gridNode at h
  omega

theorem sourcePointUnitDifferences {R : Type*} [CommRing R] [Algebra ℚ R]
    (a : ℕ) (i j : Fin (2^(a+1))) (hij : i≠j) :
    IsUnit (sourcePoint (R:=R) a i-sourcePoint (R:=R) a j) := by
  have hne : gridNode a i≠gridNode a j := (sourceGridInjective a).ne hij
  have hrat : ((gridNode a i:ℚ)-(gridNode a j:ℚ))≠0 := by exact_mod_cast sub_ne_zero.mpr hne
  have hu := (isUnit_iff_ne_zero.mpr hrat).map (algebraMap ℚ R)
  simpa only [sourcePoint,map_sub] using hu

/-- The actual tapered source jets uniquely determine a polynomial within the
actual repaired degree budget. This is exact uniqueness, not a precision theorem. -/
theorem actualSourceJetsUnique {R : Type*} [CommRing R] [Algebra ℚ R]
    (a u M : ℕ) (hu : 3≤u) (hua : u≤a) (hMu : u≤M)
    (F G : Polynomial R) (hF : F.natDegree≤2*M*(2^(a+1)-1))
    (hG : G.natDegree≤2*M*(2^(a+1)-1))
    (hjet : ∀ m j, j<sourceOrder a M u m →
      (taylor (sourcePoint a m) F).coeff j=(taylor (sourcePoint a m) G).coeff j) : F=G := by
  have hsize := sourceJetDegreeMargin a u M hu hua hMu
  have hsum : (∑ m, sourceOrder a M u m)=sourceJetTotal a (sourceCeiling M u) u := rfl
  apply exactHermiteUnique (sourcePoint a) (sourceOrder a M u) (sourcePointUnitDifferences a) F G
  · rw [hsum]; exact hF.trans_lt hsize
  · rw [hsum]; exact hG.trans_lt hsize
  · exact hjet

@[expose] noncomputable def sourceScale {R : Type*} [CommRing R] [Algebra ℚ R] (a : ℕ) : R :=
  algebraMap ℚ R ((2:ℚ)^(a+1))

@[expose] noncomputable def sourceChart {R : Type*} [CommRing R] [Algebra ℚ R]
    (a : ℕ) (m : Fin (2^(a+1))) (F : Polynomial R) : Polynomial R :=
  F.comp (C (sourcePoint a m)+C (sourceScale a)*X)

theorem sourceChartCoefficient {R : Type*} [CommRing R] [Algebra ℚ R]
    (a : ℕ) (m : Fin (2^(a+1))) (F : Polynomial R) (j : ℕ) :
    (sourceChart a m F).coeff j = (taylor (sourcePoint a m) F).coeff j*(sourceScale a)^j := by
  have he : sourceChart a m F = (taylor (sourcePoint a m) F).comp (C (sourceScale a)*X) := by
    simp only [sourceChart,taylor_apply,comp_assoc,add_comp,X_comp,C_comp]
    congr 1
    ring
  rw [he,comp_C_mul_X_coeff]

theorem actualScaledSourceJetsUnique {R : Type*} [CommRing R] [Algebra ℚ R]
    (a u M : ℕ) (hu : 3≤u) (hua : u≤a) (hMu : u≤M)
    (F G : Polynomial R) (hF : F.natDegree≤2*M*(2^(a+1)-1))
    (hG : G.natDegree≤2*M*(2^(a+1)-1))
    (hjet : ∀ m j, j<sourceOrder a M u m → (sourceChart a m F).coeff j=(sourceChart a m G).coeff j) :
    F=G := by
  have hs : IsUnit (sourceScale (R:=R) a) :=
    (isUnit_iff_ne_zero.mpr (by positivity : (2:ℚ)^(a+1)≠0)).map (algebraMap ℚ R)
  apply actualSourceJetsUnique a u M hu hua hMu F G hF hG
  intro m j hj
  have he := hjet m j hj
  rw [sourceChartCoefficient,sourceChartCoefficient] at he
  exact (hs.pow j).mul_left_inj.mp he

/-- Exact scaled jets determine the actual cleared packed contraction polynomial. -/
theorem actualPackedScaledJetsDetermine {R κ : Type*} [CommRing R] [Algebra ℚ R]
    [DecidableEq κ] {M : ℕ} (a u : ℕ) (hu : 3≤u) (hua : u≤a) (hMu : u≤M)
    (place : Fin M → κ × Fin (2^a)) (A B : (Fin M→Bool)→R)
    (G : Polynomial R) (hG : G.natDegree≤2*M*(2^(a+1)-1))
    (hjet : ∀ m j, j<sourceOrder a M u m →
      (sourceChart a m (PackedContraction.packedContractionPolynomial a place A B)).coeff j=
        (sourceChart a m G).coeff j) :
    PackedContraction.packedContractionPolynomial a place A B=G := by
  apply actualScaledSourceJetsUnique a u M hu hua hMu _ G _ hG hjet
  simpa only [Nat.mul_assoc,Nat.mul_comm,Nat.mul_left_comm] using
    PackedContraction.actualPackedContractionDegree a place A B

end HermiteUniqueness
#print axioms HermiteUniqueness.exactHermiteUnique
#print axioms HermiteUniqueness.actualSourceJetsUnique
#print axioms HermiteUniqueness.actualPackedScaledJetsDetermine
