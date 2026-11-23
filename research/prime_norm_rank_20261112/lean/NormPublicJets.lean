module
public import NormWronskian
public section
namespace NormPublicJets
open Polynomial
variable {K : Type*} [Field K]

/-- The exact polynomial correction, including the quadratic remainder. -/
theorem expansion (f G A B J H : K[X]) :
    (A+G*J)^2-f*(B+G*H)^2 =
      (A^2-f*B^2)+G*(2*A*J-2*f*B*H)+G^2*(J^2-f*H^2) := by ring

/-- Polynomial corrections divisible by G preserve values at every support node. -/
theorem preserveNode (G A J : K[X]) (q : K) (hq : G.eval q=0) :
    (A+G*J).eval q=A.eval q := by simp [hq]

/-- Exact public first-jet map from an explicit Bezout inverse, no norm witness. -/
theorem bezoutLinear (f G A B U V W Delta : K[X]) (u : K)
    (hu : (2 : K)*u=1) (hbez : A*U+f*B*V+G*W=1) :
    2*A*(C u*U*Delta)-2*f*B*(-C u*V*Delta) = Delta-G*W*Delta := by
  have htwo : (2 : K[X])*C u=1 := by
    simpa only [map_mul,map_ofNat,map_one] using congrArg (C : K →+* K[X]) hu
  calc
    _ = (2*C u)*(A*U+f*B*V)*Delta := by ring
    _ = Delta-G*W*Delta := by rw [htwo]; linear_combination hbez*Delta

/-- Every first quotient jet is reachable when the explicit inverse exists. -/
theorem firstJet (f G A B U V W Delta : K[X]) (u : K)
    (hu : (2 : K)*u=1) (hbez : A*U+f*B*V+G*W=1) :
    (A+G*(C u*U*Delta))^2-f*(B+G*(-C u*V*Delta))^2
      -(A^2-f*B^2)-G*Delta =
      G^2*((C u*U*Delta)^2-f*(-C u*V*Delta)^2-W*Delta) := by
  rw [expansion,bezoutLinear f G A B U V W Delta u hu hbez]
  ring

/-- A local root residue and public inverse lift toward cG modulo G squared.
The higher-order residual is retained explicitly, rather than assumed zero. -/
theorem publicRootLift (f G s R V W : K[X]) (u c : K)
    (hu : (2 : K)*u=1) (hroot : s^2-f=G*R) (hbez : f*V+G*W=1) :
    s^2-f*(1+G*(-C u*V*(C c-R)))^2-C c*G =
      G^2*(-f*(-C u*V*(C c-R))^2-W*(C c-R)) := by
  have hb : s*0+f*1*V+G*W=1 := by simpa using hbez
  have h := firstJet f G s 1 0 V W (C c-R) u hu hb
  simp only [mul_zero,zero_mul,add_zero,zero_pow (by decide : 2≠0),one_pow,mul_one] at h
  rw [hroot] at h
  linear_combination h

/-- General uniqueness of a low-degree representative in a polynomial residue class. -/
theorem residueRigidity (G A A' : K[X])
    (ha : A.natDegree<G.natDegree) (ha' : A'.natDegree<G.natDegree)
    (hdiv : G ∣ A'-A) : A'=A := by
  by_contra hn
  have hz : A'-A≠0 := sub_ne_zero.mpr hn
  have hlow : (A'-A).natDegree<G.natDegree :=
    (natDegree_sub_le A' A).trans_lt (max_lt ha' ha)
  have hhigh := natDegree_le_of_dvd hdiv hz
  omega

/-- Within the relation window, a multiple-of-G polynomial correction cannot move A. -/
theorem correctionZero (G A J : K[X]) (hG : G≠0)
    (ha : A.natDegree<G.natDegree) (ha' : (A+G*J).natDegree<G.natDegree) : J=0 := by
  have heq := residueRigidity G A (A+G*J) ha ha'
    (by use J; ring)
  have hz : G*J=0 := by linear_combination heq
  exact (mul_eq_zero.mp hz).resolve_left hG

/-- Both bounded norm coordinates are fixed, irrespective of first-jet surjectivity. -/
theorem boundedPairRigid (G A B J H : K[X]) (hG : G≠0)
    (ha : A.natDegree<G.natDegree) (hb : B.natDegree<G.natDegree)
    (ha' : (A+G*J).natDegree<G.natDegree)
    (hb' : (B+G*H).natDegree<G.natDegree) : J=0 ∧ H=0 :=
  ⟨correctionZero G A J hG ha ha',correctionZero G B H hG hb hb'⟩

/-- The actual pole-degree budget implies the strict bounds used by rigidity. -/
theorem poleWindowStrict (f G A B : K[X]) (hD : 0<G.natDegree)
    (ha : 2*A.natDegree≤G.natDegree)
    (hb : f.natDegree+2*B.natDegree≤G.natDegree) :
    A.natDegree<G.natDegree ∧ B.natDegree<G.natDegree := by omega

/-- A degree-capped polynomial congruent to cG modulo G squared equals cG. -/
theorem squaredCongruenceExact (G N : K[X]) (c : K)
    (hD : 0<G.natDegree) (hN : N.natDegree≤G.natDegree)
    (hdiv : G^2 ∣ N-C c*G) : N=C c*G := by
  by_contra hn
  have hz : N-C c*G≠0 := sub_ne_zero.mpr hn
  have hlow : (N-C c*G).natDegree≤G.natDegree :=
    (natDegree_sub_le N (C c*G)).trans (max_le hN
      (natDegree_mul_le.trans (by simp)))
  have hhigh := natDegree_le_of_dvd hdiv hz
  rw [natDegree_pow] at hhigh
  omega

/-- Actual old and new pole budgets preclude any new norm by componentwise G-jets. -/
theorem poleWindowNoNewNorm (f G A B J H : K[X]) (c : K) (hG : G≠0)
    (hD : 0<G.natDegree)
    (ha : 2*A.natDegree≤G.natDegree)
    (hb : f.natDegree+2*B.natDegree≤G.natDegree)
    (ha' : 2*(A+G*J).natDegree≤G.natDegree)
    (hb' : f.natDegree+2*(B+G*H).natDegree≤G.natDegree) :
    ((A+G*J)^2-f*(B+G*H)^2=C c*G) ↔ (A^2-f*B^2=C c*G) := by
  obtain ⟨hA,hB⟩ := poleWindowStrict f G A B hD ha hb
  obtain ⟨hA',hB'⟩ := poleWindowStrict f G (A+G*J) (B+G*H) hD ha' hb'
  obtain ⟨rfl,rfl⟩ := boundedPairRigid G A B J H hG hA hB hA' hB'
  simp

/-- Coupled corrections preserve branch residues while allowing coordinates to move. -/
theorem coupledResidue (G s A B H J : K[X]) :
    (A+s*H+G*J)-s*(B+H) = (A-s*B)+G*J := by ring

/-- An explicit square-root residue turns the branch condition into norm divisibility. -/
theorem residueNormDivides (f G s A B : K[X])
    (hs : G ∣ s^2-f) (hbranch : G ∣ A-s*B) : G ∣ A^2-f*B^2 := by
  have heq : A^2-f*B^2=(A-s*B)*(A+s*B)+(s^2-f)*B^2 := by ring
  rw [heq]
  exact dvd_add (dvd_mul_of_dvd_left hbranch _) (dvd_mul_of_dvd_left hs _)

/-- Target-aware certificate in the public square-root residue chart, without moments.
Choosing the root residue and finding a nonzero bounded pair are still inputs. -/
theorem publicResidueCertificate (f G s A B : K[X]) (hG : G.Monic)
    (hs : G ∣ s^2-f) (hbranch : G ∣ A-s*B)
    (ha : 2*A.natDegree≤G.natDegree)
    (hb : f.natDegree+2*B.natDegree≤G.natDegree)
    (hn : A^2-f*B^2≠0) :
    ∃ c : K, c≠0 ∧ A^2-f*B^2=C c*G := by
  have heq := eq_leadingCoeff_mul_of_monic_of_dvd_of_natDegree_le hG
    (residueNormDivides f G s A B hs hbranch)
    (NormWronskian.normDegreeBound f A B G.natDegree ha hb)
  refine ⟨(A^2-f*B^2).leadingCoeff,?_,heq⟩
  exact leadingCoeff_ne_zero.mpr hn

#print axioms expansion
#print axioms preserveNode
#print axioms bezoutLinear
#print axioms firstJet
#print axioms publicRootLift
#print axioms residueRigidity
#print axioms correctionZero
#print axioms boundedPairRigid
#print axioms poleWindowStrict
#print axioms squaredCongruenceExact
#print axioms poleWindowNoNewNorm
#print axioms coupledResidue
#print axioms residueNormDivides
#print axioms publicResidueCertificate
end NormPublicJets
