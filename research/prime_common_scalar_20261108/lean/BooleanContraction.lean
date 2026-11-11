module
public import ResidueEncoding
public section
namespace BooleanContraction

@[expose] def bitValue {R : Type*} [Zero R] [One R] (b : Bool) : R := if b then 1 else 0
@[expose] def ellFactor {R : Type*} [CommRing R] (b : Bool) (z : R) : R := if b then z else 1-z

@[expose] def linearRow {R : Type*} [CommRing R] {M : ℕ}
    (W : Fin M → Fin M → R) (x : Fin M → Bool) (j : Fin M) : R :=
  ∑ k, W j k * bitValue (x k)

/-- Source D.8 Boolean kernel: one affine factor for every REAL row port. -/
@[expose] def booleanLift {R : Type*} [CommRing R] {M : ℕ}
    (W : Fin M → Fin M → R) (y x : Fin M → Bool) : R :=
  ∏ j, ellFactor (y j) (linearRow W x j)

/-- Homogenization retains d in every zero-output factor. -/
@[expose] def clearedLift {R : Type*} [CommRing R] {M : ℕ}
    (d : R) (N : Fin M → Fin M → R) (y x : Fin M → Bool) : R :=
  ∏ j, if y j then linearRow N x j else d-linearRow N x j

@[expose] def sourceContraction {R : Type*} [CommRing R] {M : ℕ}
    (A B : (Fin M → Bool) → R) (W : Fin M → Fin M → R) : R :=
  ∑ x, ∑ y, A y * B x * booleanLift W y x

@[expose] def clearedContraction {R : Type*} [CommRing R] {M : ℕ}
    (A B : (Fin M → Bool) → R) (d : R) (N : Fin M → Fin M → R) : R :=
  ∑ x, ∑ y, A y * B x * clearedLift d N y x

/-- No homogeneous-in-W premise: the affine constant is explicitly cleared. -/
theorem affineFactorClearing {R : Type*} [CommRing R] (d : Rˣ) (b : Bool) (z : R) :
    (d:R) * ellFactor b (z*(↑d⁻¹:R)) = if b then z else (d:R)-z := by
  have hu : (d:R)*(↑d⁻¹:R)=1 := by rw [← Units.val_mul]; simp
  cases b
  · change (d:R)*(1-z*(↑d⁻¹:R)) = (d:R)-z
    calc
      _ = (d:R)-z*((d:R)*(↑d⁻¹:R)) := by ring
      _ = _ := by rw [hu,mul_one]
  · change (d:R)*(z*(↑d⁻¹:R)) = z
    calc
      _ = z*((d:R)*(↑d⁻¹:R)) := by ring
      _ = _ := by rw [hu,mul_one]

theorem scaledLinearRow {R : Type*} [CommRing R] {M : ℕ}
    (N : Fin M → Fin M → R) (d : Rˣ) (x : Fin M → Bool) (j : Fin M) :
    linearRow (fun i k => N i k * (↑d⁻¹:R)) x j = linearRow N x j * (↑d⁻¹:R) := by
  unfold linearRow
  rw [Finset.sum_mul]
  apply Finset.sum_congr rfl
  intro k hk; ring

/-- Exponent M is derived from the actual product over real row ports. -/
theorem booleanLiftClearing {R : Type*} [CommRing R] {M : ℕ}
    (N : Fin M → Fin M → R) (d : Rˣ) (y x : Fin M → Bool) :
    (d:R)^M * booleanLift (fun i k => N i k*(↑d⁻¹:R)) y x = clearedLift (d:R) N y x := by
  unfold booleanLift clearedLift
  have hd : (d:R)^M = ∏ _j : Fin M, (d:R) := by simp
  rw [hd, ← Finset.prod_mul_distrib]
  apply Finset.prod_congr rfl
  intro j hj
  rw [scaledLinearRow, affineFactorClearing]

/-- Actual D.8 affine Boolean contraction, including both immutable tensors.
Valid over arbitrary commutative coefficient rings, including nilpotents. -/
theorem sourceContractionClearing {R : Type*} [CommRing R] {M : ℕ}
    (A B : (Fin M → Bool) → R) (N : Fin M → Fin M → R) (d : Rˣ) :
    (d:R)^M * sourceContraction A B (fun i k => N i k*(↑d⁻¹:R)) =
      clearedContraction A B (d:R) N := by
  unfold sourceContraction clearedContraction
  rw [Finset.mul_sum]
  apply Finset.sum_congr rfl
  intro x hx
  rw [Finset.mul_sum]
  apply Finset.sum_congr rfl
  intro y hy
  calc
    _ = A y*B x*((d:R)^M * booleanLift (fun i k => N i k*(↑d⁻¹:R)) y x) := by ring
    _ = _ := by rw [booleanLiftClearing]

/-- Negative control against simply substituting the numerator in ell_0. -/
theorem affineZeroOutputControl :
    (3:ℚ)*ellFactor false ((2:ℚ)/3) = 1 ∧ ellFactor false (2:ℚ) = -1 := by
  norm_num [ellFactor]

theorem booleanBitDelta {R : Type*} [CommRing R] (b c : Bool) :
    ellFactor b (bitValue (R:=R) c) = if b=c then 1 else 0 := by
  cases b <;> cases c <;> simp [ellFactor,bitValue]

theorem booleanLiftIdentity {R : Type*} [CommRing R] {M : ℕ} (y x : Fin M → Bool) :
    booleanLift (fun i k => if i=k then (1:R) else 0) y x = if y=x then 1 else 0 := by
  classical
  unfold booleanLift
  have hr (j : Fin M) : linearRow (fun i k => if i=k then (1:R) else 0) x j = bitValue (x j) := by
    simp [linearRow]
  simp_rw [hr,booleanBitDelta]
  by_cases h : y=x
  · subst y; simp
  · rw [ite_eq_right h]
    obtain ⟨j,hj⟩ : ∃ j, y j ≠ x j := by
      by_contra! hn
      exact h (funext hn)
    apply Finset.prod_eq_zero (Finset.mem_univ j)
    simp [hj]

theorem sourceIdentityContraction {R : Type*} [CommRing R] {M : ℕ}
    (A B : (Fin M → Bool) → R) :
    sourceContraction A B (fun i k => if i=k then (1:R) else 0) = ∑ x, A x*B x := by
  classical
  simp [sourceContraction,booleanLiftIdentity,mul_ite]

theorem mapBitValue {R S : Type*} [CommRing R] [CommRing S] (f : R →+* S) (b : Bool) :
    f (bitValue (R:=R) b) = bitValue (R:=S) b := by cases b <;> simp [bitValue]

theorem mapBooleanLift {R S : Type*} [CommRing R] [CommRing S] {M : ℕ}
    (f : R →+* S) (W : Fin M → Fin M → R) (y x : Fin M → Bool) :
    f (booleanLift W y x) = booleanLift (fun i k => f (W i k)) y x := by
  unfold booleanLift
  rw [map_prod]
  apply Finset.prod_congr rfl
  intro j hj
  unfold ellFactor
  split_ifs <;> simp [linearRow,mapBitValue]

theorem mapSourceContraction {R S : Type*} [CommRing R] [CommRing S] {M : ℕ}
    (f : R →+* S) (A B : (Fin M → Bool) → R) (W : Fin M → Fin M → R) :
    f (sourceContraction A B W) =
      sourceContraction (fun y => f (A y)) (fun x => f (B x)) (fun i k => f (W i k)) := by
  simp only [sourceContraction,map_sum,map_mul,mapBooleanLift]

theorem mapClearedLift {R S : Type*} [CommRing R] [CommRing S] {M : ℕ}
    (f : R →+* S) (d : R) (N : Fin M → Fin M → R) (y x : Fin M → Bool) :
    f (clearedLift d N y x) = clearedLift (f d) (fun i k => f (N i k)) y x := by
  unfold clearedLift
  rw [map_prod]
  apply Finset.prod_congr rfl
  intro j hj
  split_ifs <;> simp [linearRow,mapBitValue]

theorem mapClearedContraction {R S : Type*} [CommRing R] [CommRing S] {M : ℕ}
    (f : R →+* S) (A B : (Fin M → Bool) → R) (d : R) (N : Fin M → Fin M → R) :
    f (clearedContraction A B d N) =
      clearedContraction (fun y => f (A y)) (fun x => f (B x)) (f d) (fun i k => f (N i k)) := by
  simp only [clearedContraction,map_sum,map_mul,mapClearedLift]

@[expose] noncomputable def contractionPolynomial {R : Type*} [CommRing R] {M : ℕ}
    (A B : (Fin M → Bool) → R) (D : Polynomial R) (N : Fin M → Fin M → Polynomial R) : Polynomial R :=
  clearedContraction (fun y => Polynomial.C (A y)) (fun x => Polynomial.C (B x)) D N

theorem contractionPolynomialEval {R : Type*} [CommRing R] {M : ℕ}
    (A B : (Fin M → Bool) → R) (D : Polynomial R) (N : Fin M → Fin M → Polynomial R) (t : R) :
    (contractionPolynomial A B D N).eval t =
      clearedContraction A B (D.eval t) (fun i k => (N i k).eval t) := by
  simpa only [contractionPolynomial,Polynomial.coe_evalRingHom,Polynomial.eval_C] using
    mapClearedContraction (Polynomial.evalRingHom t) (fun y => Polynomial.C (A y))
      (fun x => Polynomial.C (B x)) D N

theorem linearRowDegree {R : Type*} [CommRing R] {M : ℕ}
    (N : Fin M → Fin M → Polynomial R) (d : ℕ)
    (hN : ∀ i k, (N i k).natDegree ≤ d) (x : Fin M → Bool) (j : Fin M) :
    (linearRow N x j).natDegree ≤ d := by
  unfold linearRow
  apply Polynomial.natDegree_sum_le_of_forall_le
  intro k hk
  have hb : (bitValue (R:=Polynomial R) (x k)).natDegree = 0 := by cases x k <;> simp [bitValue]
  calc
    _ ≤ (N j k).natDegree + (bitValue (R:=Polynomial R) (x k)).natDegree := Polynomial.natDegree_mul_le
    _ ≤ d := by simpa only [hb,Nat.add_zero] using hN j k

theorem clearedLiftDegree {R : Type*} [CommRing R] {M : ℕ}
    (D : Polynomial R) (N : Fin M → Fin M → Polynomial R) (d : ℕ)
    (hD : D.natDegree ≤ d) (hN : ∀ i k, (N i k).natDegree ≤ d) (y x : Fin M → Bool) :
    (clearedLift D N y x).natDegree ≤ M*d := by
  unfold clearedLift
  have hp := Polynomial.natDegree_prod_le Finset.univ
    (fun j : Fin M => if y j then linearRow N x j else D-linearRow N x j)
  have hs : (∑ j : Fin M, (if y j then linearRow N x j else D-linearRow N x j).natDegree) ≤
      ∑ _j : Fin M, d := by
    apply Finset.sum_le_sum
    intro j hj
    split_ifs
    · exact linearRowDegree N d hN x j
    · exact (Polynomial.natDegree_sub_le _ _).trans (max_le hD (linearRowDegree N d hN x j))
  simpa only [Finset.sum_const,Finset.card_univ,Fintype.card_fin,nsmul_eq_mul,Nat.cast_id] using hp.trans hs

theorem contractionPolynomialDegree {R : Type*} [CommRing R] {M : ℕ}
    (A B : (Fin M → Bool) → R) (D : Polynomial R) (N : Fin M → Fin M → Polynomial R) (d : ℕ)
    (hD : D.natDegree ≤ d) (hN : ∀ i k, (N i k).natDegree ≤ d) :
    (contractionPolynomial A B D N).natDegree ≤ M*d := by
  unfold contractionPolynomial clearedContraction
  apply Polynomial.natDegree_sum_le_of_forall_le
  intro x hx
  apply Polynomial.natDegree_sum_le_of_forall_le
  intro y hy
  have hc : (Polynomial.C (A y)*Polynomial.C (B x)).natDegree = 0 := by
    rw [← Polynomial.C_mul,Polynomial.natDegree_C]
  calc
    _ ≤ (Polynomial.C (A y)*Polynomial.C (B x)).natDegree + (clearedLift D N y x).natDegree :=
      Polynomial.natDegree_mul_le
    _ ≤ M*d := by simpa only [hc,Nat.zero_add] using clearedLiftDegree D N d hD hN y x

end BooleanContraction
#print axioms BooleanContraction.affineFactorClearing
#print axioms BooleanContraction.booleanLiftClearing
#print axioms BooleanContraction.sourceContractionClearing
#print axioms BooleanContraction.sourceIdentityContraction
#print axioms BooleanContraction.contractionPolynomialDegree
