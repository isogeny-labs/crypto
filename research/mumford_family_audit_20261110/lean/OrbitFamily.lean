import CurveFamily

namespace MumfordOrbitFamilyAudit
noncomputable section
open Polynomial MumfordFamilyPairsAudit MumfordCurveFamilyAudit

abbrev OrbitIndex (t : ℕ) := Option (Bool×Fin (genus t))

def orbitPair (t : ℕ) : OrbitIndex t→ReducedPair (genus t) (curvePolynomial t)
  | none => identity _ _
  | some (false,d) => powerPair t (d.val+1) (by omega) (by have h:=d.isLt;omega) 1 (by ring)
  | some (true,d) => powerPair t (d.val+1) (by omega) (by have h:=d.isLt;omega) (-1) (by ring)

def signature (t : ℕ) : OrbitIndex t→ℕ×K
  | none => (0,0)
  | some (false,d) => (d.val+1,1)
  | some (true,d) => (d.val+1,-1)

theorem signature_injective (t : ℕ) : Function.Injective (signature t) := by
  intro i j h
  cases i with
  | none =>
    cases j with
    | none => rfl
    | some p =>
      rcases p with ⟨b,d⟩
      have hh:=congrArg Prod.fst h
      cases b <;> simp [signature] at hh
  | some p =>
    cases j with
    | none =>
      rcases p with ⟨b,d⟩
      have hh:=congrArg Prod.fst h
      cases b <;> simp [signature] at hh
    | some q =>
      rcases p with ⟨b,d⟩
      rcases q with ⟨c,e⟩
      have he : d.val=e.val := by
        have hh:=congrArg Prod.fst h
        cases b <;> cases c <;> simp only [signature] at hh <;> omega
      have hde : d=e := Fin.ext he
      subst e
      have hbc : b=c := by
        cases b <;> cases c
        · rfl
        · have hh:=congrArg Prod.snd h
          change (1:K)= -1 at hh
          exact False.elim ((by decide +kernel : (1:K)≠ -1) hh)
        · have hh:=congrArg Prod.snd h
          change (-1:K)=1 at hh
          exact False.elim ((by decide +kernel : (-1:K)≠1) hh)
        · rfl
      subst c
      rfl

theorem orbit_signature (t : ℕ) (i : OrbitIndex t) :
    ((orbitPair t i).u.natDegree,((orbitPair t i).v).coeff 0)=signature t i := by
  cases i with
  | none => simp [orbitPair,identity,signature]
  | some p => rcases p with ⟨b,d⟩;cases b <;> simp [orbitPair,powerPair,signature]

theorem orbitPair_injective (t : ℕ) : Function.Injective (orbitPair t) := by
  intro i j h
  apply signature_injective t
  have hh:=congrArg (fun P : ReducedPair (genus t) (curvePolynomial t)=>
    (P.u.natDegree,P.v.coeff 0)) h
  simpa only [orbit_signature] using hh

theorem orbitPair_smooth (t : ℕ) (i : OrbitIndex t) : Smooth (orbitPair t i) := by
  cases i with
  | none => exact Polynomial.Splits.one
  | some p => rcases p with ⟨b,d⟩;cases b <;> exact Polynomial.Splits.X.pow (d.val+1)

def OrbitPair (t : ℕ) := Set.range (orbitPair t)

theorem index_cardinality (t : ℕ) : Nat.card (OrbitIndex t)=exponent t := by
  rw [Nat.card_eq_fintype_card]
  simp only [OrbitIndex,Fintype.card_option,Fintype.card_prod,Fintype.card_bool,Fintype.card_fin]
  unfold exponent genus
  omega

theorem orbit_cardinality (t : ℕ) : Nat.card (OrbitPair t)=exponent t := by
  have he:=Equiv.ofInjective (orbitPair t) (orbitPair_injective t)
  exact (Nat.card_congr he).symm.trans (index_cardinality t)

theorem all_orbit_smooth (t : ℕ) (D : OrbitPair t) : Smooth D.val := by
  obtain ⟨i,hi⟩:=D.property
  rw [←hi]
  exact orbitPair_smooth t i

theorem bad_outside_orbit (t : ℕ) : badPair t∉Set.range (orbitPair t) := by
  rintro ⟨i,hi⟩
  exact badPair_nonsmooth t (hi ▸ orbitPair_smooth t i)

theorem smooth_orbit_cardinality (t : ℕ) :
    Nat.card {D : OrbitPair t // Smooth D.val}=exponent t := by
  let e : {D : OrbitPair t // Smooth D.val}≃OrbitPair t := {
    toFun := Subtype.val
    invFun := fun D=>⟨D,all_orbit_smooth t D⟩
    left_inv := by intro D;apply Subtype.ext;rfl
    right_inv := by intro D;rfl }
  exact (Nat.card_congr e).trans (orbit_cardinality t)

theorem smooth_orbit_fraction_one (t : ℕ) :
    (Nat.card {D : OrbitPair t // Smooth D.val}:ℚ)/Nat.card (OrbitPair t)=1 := by
  rw [smooth_orbit_cardinality,orbit_cardinality]
  apply div_self
  exact_mod_cast (Nat.ne_of_gt (exponent_positive t))

theorem smooth_ambient_fraction_lt_one (t : ℕ) :
    (Nat.card {D : ReducedPair (genus t) (curvePolynomial t) // Smooth D}:ℚ)/
      Nat.card (ReducedPair (genus t) (curvePolynomial t))<1 := by
  have hh:=nonsmooth_strict_count (badPair t) (badPair_nonsmooth t)
  have hp : 0<Nat.card (ReducedPair (genus t) (curvePolynomial t)) := by omega
  have hp' : (0:ℚ)<Nat.card (ReducedPair (genus t) (curvePolynomial t)) := by exact_mod_cast hp
  apply (div_lt_one hp').mpr
  exact_mod_cast hh

theorem orbit_ambient_fractions_differ (t : ℕ) :
    (Nat.card {D : OrbitPair t // Smooth D.val}:ℚ)/Nat.card (OrbitPair t)≠
    (Nat.card {D : ReducedPair (genus t) (curvePolynomial t) // Smooth D}:ℚ)/
      Nat.card (ReducedPair (genus t) (curvePolynomial t)) := by
  rw [smooth_orbit_fraction_one]
  exact ne_of_gt (smooth_ambient_fraction_lt_one t)

#print axioms orbit_cardinality
#print axioms orbit_ambient_fractions_differ
end
end MumfordOrbitFamilyAudit
