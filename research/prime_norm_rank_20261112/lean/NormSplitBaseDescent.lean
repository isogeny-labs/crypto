module
public import NormRowClosure
public import NormBranchRepair
public import Mathlib.FieldTheory.Separable
public import Mathlib.FieldTheory.Finite.Basic
import all NormBranchRepair
import all NormTargetFiber
public section

namespace NormSplitBaseDescent
open Polynomial
variable {L : Type*} [Field L]

/-- A fixed public square root fixes the successful sign without receiving it. -/
theorem branchCross (phi : L →+* L) (a b y : L) (hy : phi y=y)
    (h : a^2=y^2*b^2) : a*phi b-phi a*b=0 := by
  have hz : (a-y*b)*(a+y*b)=0 := by linear_combination h
  rcases mul_eq_zero.mp hz with hm | hp
  · have ha := sub_eq_zero.mp hm
    rw [ha,map_mul,hy]
    ring
  · have ha := eq_neg_iff_add_eq_zero.mpr hp
    rw [ha,map_neg,map_mul,hy]
    ring

theorem evalMapFixed (phi : L →+* L) (A : L[X]) (x : L) (hx : phi x=x) :
    (A.map phi).eval x=phi (A.eval x) := by
  simpa only [hx] using (eval_map_apply phi x (p:=A))

theorem nodalMapFixed (phi : L →+* L) (S : Finset L)
    (hx : ∀ x∈S, phi x=x) : (Lagrange.nodal S id).map phi=Lagrange.nodal S id := by
  simp only [Lagrange.nodal,Polynomial.map_prod,Polynomial.map_sub,map_X,map_C,id_eq]
  apply Finset.prod_congr rfl
  intro x hxs
  rw [hx x hxs]

/-- Derive the cross-product roots from actual norm equations and public
square roots. No successful branch assignment is supplied. -/
theorem crossFromNorm (phi : L →+* L) (S : Finset L) (f A B : L[X])
    (hNorm : A^2-f*B^2=Lagrange.nodal S id)
    (hx : ∀ x∈S, phi x=x)
    (hy : ∀ x∈S, ∃ y : L, phi y=y ∧ y^2=f.eval x)
    (a b : ℕ) (hA : A.natDegree≤a) (hB : B.natDegree≤b)
    (hcard : a+b<S.card) : A*(B.map phi)-(A.map phi)*B=0 := by
  have hdiv : Lagrange.nodal S id ∣ A*(B.map phi)-(A.map phi)*B := by
    apply NormSignedHankel.nodalDivides S
    intro x hxs
    obtain ⟨y,hyf,hysq⟩ := hy x hxs
    have hn := congrArg (fun P : L[X] => P.eval x) hNorm
    have hv0 : (Lagrange.nodal S id).eval x=0 := by
      simpa only [id_eq] using (Lagrange.eval_nodal_at_node (v:=id) hxs)
    simp only [eval_sub,eval_mul,eval_pow,hv0] at hn
    have heq : (A.eval x)^2=y^2*(B.eval x)^2 := by rw [hysq]; exact sub_eq_zero.mp hn
    rw [eval_sub,eval_mul,eval_mul,evalMapFixed phi B x (hx x hxs),
      evalMapFixed phi A x (hx x hxs)]
    exact branchCross phi _ _ y hyf heq
  by_contra hn
  have hhigh := natDegree_le_of_dvd hdiv hn
  rw [Lagrange.natDegree_nodal] at hhigh
  have hlow := natDegree_sub_le (A*(B.map phi)) ((A.map phi)*B)
  have hleft := natDegree_mul_le (p:=A) (q:=B.map phi)
  have hright := natDegree_mul_le (p:=A.map phi) (q:=B)
  rw [natDegree_map] at hleft hright
  omega

/-- A monic norm pair is unique once its cross product and actual norm agree.
The lower polynomial may be zero. -/
theorem monicPairUnique (f G A0 B0 A1 B1 : L[X]) (hG : G≠0)
    (hA0 : A0.Monic) (hA1 : A1.Monic)
    (hn0 : A0^2-f*B0^2=G) (hn1 : A1^2-f*B1^2=G)
    (hcross : A0*B1-A1*B0=0) : A1=A0 ∧ B1=B0 := by
  have hb : B1^2=B0^2 := by
    simpa only [map_one,one_mul] using
      NormBranchRepair.normSquareRelation f G A0 B0 A1 B1 1 1 hG
        (by simpa using hn0) (by simpa using hn1) hcross
  have ha : A1^2=A0^2 := by linear_combination hn1-hn0+f*hb
  obtain ⟨t,ht⟩ := NormBranchRepair.squaresProportional A1 A0 1 1
    (by exact one_ne_zero) hA0.ne_zero (by simpa using ha)
  have ht1 : t=1 := by
    have hl := congrArg Polynomial.leadingCoeff ht
    simpa only [leadingCoeff_mul,leadingCoeff_C,hA0.leadingCoeff,hA1.leadingCoeff,mul_one] using hl.symm
  have hAA : A1=A0 := by simpa only [ht1,map_one,one_mul] using ht
  refine ⟨hAA,?_⟩
  rw [hAA] at hcross
  have hz : A0*(B1-B0)=0 := by linear_combination hcross
  exact sub_eq_zero.mp ((mul_eq_zero.mp hz).resolve_left hA0.ne_zero)

/-- Endomorphism rigidity of the actual norm pair on fully fixed public points. -/
theorem pairFixedOnNodes (phi : L →+* L) (S : Finset L) (f A B : L[X])
    (hNorm : A^2-f*B^2=Lagrange.nodal S id) (hm : A.Monic)
    (hf : f.map phi=f) (hx : ∀ x∈S, phi x=x)
    (hy : ∀ x∈S, ∃ y : L, phi y=y ∧ y^2=f.eval x)
    (a b : ℕ) (hA : A.natDegree≤a) (hB : B.natDegree≤b)
    (hcard : a+b<S.card) : A.map phi=A ∧ B.map phi=B := by
  have hn := congrArg (Polynomial.map phi) hNorm
  simp only [Polynomial.map_sub,Polynomial.map_pow,Polynomial.map_mul,hf,nodalMapFixed phi S hx] at hn
  exact monicPairUnique f _ A B (A.map phi) (B.map phi)
    (Lagrange.nodal_ne_zero (s:=S) (v:=id)) hm (hm.map phi) hNorm hn
      (crossFromNorm phi S f A B hNorm hx hy a b hA hB hcard)

/-- A successful support is derived from an actual monic divisor of the
public nodal polynomial. Neither the support nor its signs is an oracle. -/
theorem divisorSupport (T : Finset L) (N : L[X]) (hm : N.Monic)
    (hd : N ∣ Lagrange.nodal T id) :
    ∃ S : Finset L, S⊆T ∧ N=Lagrange.nodal S id ∧ S.card=N.natDegree := by
  classical
  have hsplitT : (Lagrange.nodal T id).Splits := by
    unfold Lagrange.nodal
    apply Splits.prod
    intro x _
    simpa only [id_eq,sub_eq_add_neg,← map_neg] using Splits.X_add_C (-x)
  have hsepT : (Lagrange.nodal T id).Separable := by
    apply (separable_prod_X_sub_C_iff').mpr
    intro x _ y _ h
    exact h
  have hs := hsplitT.of_dvd (Lagrange.nodal_ne_zero (s:=T) (v:=id)) hd
  have hn := nodup_roots (hsepT.of_dvd hd)
  let S : Finset L := ⟨N.roots,hn⟩
  refine ⟨S,?_,?_,?_⟩
  · intro x hx
    have hxr : x∈N.roots := hx
    have hz : N.eval x=0 := (mem_roots hm.ne_zero).mp hxr
    have hzT : (Lagrange.nodal T id).eval x=0 := by
      obtain ⟨Q,hQ⟩ := hd
      rw [hQ,eval_mul,hz,zero_mul]
    by_contra hnot
    apply (Lagrange.eval_nodal_not_at_node (s:=T) (v:=id) ?_) hzT
    intro y hy hxy
    exact hnot (hxy ▸ hy)
  · change N=∏ x∈(⟨N.roots,hn⟩ : Finset L), (X-C x)
    rw [Finset.prod_mk]
    exact hs.eq_prod_roots_of_monic hm
  · change N.roots.card=N.natDegree
    exact hs.natDegree_eq_card_roots.symm

/-- The pole window itself proves the actual full norm monic of degree 2a. -/
theorem normWindow (f A B : L[X]) (a : ℕ) (hm : A.Monic)
    (hA : A.natDegree=a) (hf : f.natDegree≤3) (hB : B.natDegree<a-1) :
    (A^2-f*B^2).Monic ∧ (A^2-f*B^2).natDegree=2*a := by
  have hb : (f*B^2).natDegree<(A^2).natDegree := by
    have hb' := natDegree_mul_le (p:=f) (q:=B^2)
    rw [natDegree_pow] at hb' ⊢
    rw [hA]
    omega
  constructor
  · apply (hm.pow 2).sub_of_left
    rw [degree_eq_natDegree (pow_ne_zero 2 hm.ne_zero)]
    exact degree_le_natDegree.trans_lt (by exact_mod_cast hb)
  · rw [natDegree_sub_eq_left_of_natDegree_lt hb,natDegree_pow,hA]

/-- Fully fixed public points force coefficient rigidity of every successful
norm pair in the actual pole window. No successful support/signs are supplied. -/
theorem pairFixedFromDivisor (phi : L →+* L) (T : Finset L) (f A B : L[X])
    (a : ℕ) (hm : A.Monic) (hA : A.natDegree=a)
    (hfDegree : f.natDegree≤3) (hB : B.natDegree<a-1)
    (hdiv : A^2-f*B^2 ∣ Lagrange.nodal T id)
    (hf : f.map phi=f) (hx : ∀ x∈T, phi x=x)
    (hy : ∀ x∈T, ∃ y : L, phi y=y ∧ y^2=f.eval x) :
    A.map phi=A ∧ B.map phi=B := by
  have hw := normWindow f A B a hm hA hfDegree hB
  obtain ⟨S,hST,hN,hcard⟩ := divisorSupport T _ hw.1 hdiv
  apply pairFixedOnNodes phi S f A B hN hm hf
    (fun x hxS => hx x (hST hxS)) (fun x hxS => hy x (hST hxS)) a (a-2) hA.le
    (by omega)
  rw [hcard,hw.2]
  omega

section Prime
variable (p : ℕ) [Fact p.Prime] [CharP L p]

/-- Exact coefficient-field characterization; no finite sampling. -/
theorem frobeniusFixedIff (P : L[X]) :
    P.map (frobenius L p)=P ↔ ∀ j, P.coeff j∈(⊥ : Subfield L) := by
  constructor
  · intro h j
    apply (Subfield.mem_bot_iff_pow_eq_self L p).mpr
    have hc := congrArg (fun Q : L[X] => Q.coeff j) h
    rw [coeff_map] at hc
    exact hc
  · intro h
    ext j
    rw [coeff_map]
    exact (Subfield.mem_bot_iff_pow_eq_self L p).mp (h j)

/-- Every successful norm pair descends when all allowed points are genuinely
prime-field points. The successful support and signs are derived. -/
theorem pairPrimeCoefficients (p : ℕ) [Fact p.Prime] [CharP L p] (T : Finset L) (f A B : L[X])
    (a : ℕ) (hm : A.Monic) (hA : A.natDegree=a)
    (hfDegree : f.natDegree≤3) (hB : B.natDegree<a-1)
    (hdiv : A^2-f*B^2 ∣ Lagrange.nodal T id)
    (hf : ∀ j, f.coeff j∈(⊥ : Subfield L))
    (hx : ∀ x∈T, x∈(⊥ : Subfield L))
    (hy : ∀ x∈T, ∃ y : L, y∈(⊥ : Subfield L) ∧ y^2=f.eval x) :
    (∀ j, A.coeff j∈(⊥ : Subfield L)) ∧ ∀ j, B.coeff j∈(⊥ : Subfield L) := by
  have hp := pairFixedFromDivisor (frobenius L p) T f A B a hm hA hfDegree hB hdiv
    ((frobeniusFixedIff p f).mpr hf)
    (fun x hxt => (Subfield.mem_bot_iff_pow_eq_self L p).mp (hx x hxt))
    (by
      intro x hxt
      obtain ⟨y,hyf,hysq⟩ := hy x hxt
      exact ⟨y,(Subfield.mem_bot_iff_pow_eq_self L p).mp hyf,hysq⟩)
  exact ⟨(frobeniusFixedIff p A).mp hp.1,(frobeniusFixedIff p B).mp hp.2⟩

/-- Descent is literal polynomial lifting to the prime subfield. -/
theorem primePolynomialLift (P : L[X]) (h : ∀ j, P.coeff j∈(⊥ : Subfield L)) :
    ∃ P0 : (⊥ : Subfield L)[X], P0.map (Subfield.subtype _)=P := by
  apply (Polynomial.mem_lifts (f:=Subfield.subtype (⊥ : Subfield L)) P).mp
  apply (Polynomial.lifts_iff_coeff_lifts P).mpr
  intro j
  exact ⟨⟨P.coeff j,h j⟩,rfl⟩

end Prime

/-- Bind the full norm divisor to the actual computed monic support and
explicit public base polynomial, with the target outside that base. -/
theorem actualNormDivides [DecidableEq L] (T : Finset L) (a b a1 a2 c0 c1 r z : L)
    (hp : z^2=r^3+a*r+b) (hr : r∉T)
    (hR : (Lagrange.nodal T id) %ₘ NormTargetFiber.support a b a1 a2 c0 c1 r z=0) :
    NormPoleTransport.actualNorm a b a1 a2 c0 c1 r z ∣ Lagrange.nodal (insert r T) id := by
  classical
  have hw := NormTargetFiber.supportWindowRing a b a1 a2 c0 c1 r z hp
  have hu := (modByMonic_eq_zero_iff_dvd hw.1).mp hR
  have hn : (X-C r)*NormTargetFiber.support a b a1 a2 c0 c1 r z=
      NormPoleTransport.actualNorm a b a1 a2 c0 c1 r z :=
    NormPoleTransport.actualSupportExact a b a1 a2 c0 c1 r z hp
  have hd := mul_dvd_mul_left (X-C r) hu
  rw [hn] at hd
  simpa only [Lagrange.nodal_insert_eq_nodal (v:=id) hr,id_eq] using hd

/-- The actual selected chart has no extension-field core solutions when
its allowed points are prime-field points. This is certificate descent;
no finder algorithm or cost is supplied. -/
theorem actualCorePrimeCoefficients (p : ℕ) [Fact p.Prime] [CharP L p]
    (T : Finset L) (a b a1 a2 c0 c1 r z : L)
    (hp : z^2=r^3+a*r+b) (hr : r∉T)
    (ha : a∈(⊥ : Subfield L)) (hb : b∈(⊥ : Subfield L))
    (hpr : r∈(⊥ : Subfield L)) (hpz : z∈(⊥ : Subfield L))
    (hx : ∀ x∈T, x∈(⊥ : Subfield L))
    (hy : ∀ x∈T, ∃ y : L, y∈(⊥ : Subfield L) ∧ y^2=x^3+a*x+b)
    (hR : (Lagrange.nodal T id) %ₘ NormTargetFiber.support a b a1 a2 c0 c1 r z=0) :
    a1∈(⊥ : Subfield L) ∧ a2∈(⊥ : Subfield L) ∧
      c0∈(⊥ : Subfield L) ∧ c1∈(⊥ : Subfield L) := by
  classical
  let f : L[X] := X^3+C a*X+C b
  let A : L[X] := NormPoleTransport.actualA a1 a2 c0 c1 r z
  let B : L[X] := C c0+C c1*X
  have hmA : A.Monic := by
    dsimp [A,NormPoleTransport.actualA,NormMembershipElimination.completeTarget]
    monicity <;> norm_num
  have hdA : A.natDegree=3 := by
    dsimp [A,NormPoleTransport.actualA,NormMembershipElimination.completeTarget]
    compute_degree!
  have hf : f.map (frobenius L p)=f := by
    have ha' := (Subfield.mem_bot_iff_pow_eq_self L p).mp ha
    have hb' := (Subfield.mem_bot_iff_pow_eq_self L p).mp hb
    change f.map (frobenius L p)=f
    dsimp [f]
    simp only [Polynomial.map_add,Polynomial.map_pow,Polynomial.map_mul,map_X,map_C]
    change X^3+C (a^p)*X+C (b^p)=X^3+C a*X+C b
    rw [ha',hb']
  have hpair := pairPrimeCoefficients p (insert r T) f A B 3 hmA hdA
    (by dsimp [f]; compute_degree!) (by dsimp [B]; compute_degree!)
    (actualNormDivides T a b a1 a2 c0 c1 r z hp hr hR)
    ((frobeniusFixedIff p f).mp hf)
    (by
      intro x hxt
      rcases Finset.mem_insert.mp hxt with h | h
      · subst x
        exact hpr
      · exact hx x h)
    (by
      intro x hxt
      rcases Finset.mem_insert.mp hxt with h | h
      · subst x
        exact ⟨z,hpz,by simpa [f] using hp⟩
      · obtain ⟨y,hy0,hy1⟩ := hy x h
        exact ⟨y,hy0,by simpa [f] using hy1⟩)
  have ha1 := hpair.1 1
  have ha2 := hpair.1 2
  have hc0 := hpair.2 0
  have hc1 := hpair.2 1
  simp [A,B,NormPoleTransport.actualA,NormMembershipElimination.completeTarget,-map_pow] at ha1 ha2 hc0 hc1
  exact ⟨ha1,ha2,hc0,hc1⟩

end NormSplitBaseDescent

#print axioms NormSplitBaseDescent.branchCross
#print axioms NormSplitBaseDescent.evalMapFixed
#print axioms NormSplitBaseDescent.nodalMapFixed
#print axioms NormSplitBaseDescent.crossFromNorm
#print axioms NormSplitBaseDescent.monicPairUnique
#print axioms NormSplitBaseDescent.pairFixedOnNodes
#print axioms NormSplitBaseDescent.divisorSupport
#print axioms NormSplitBaseDescent.normWindow
#print axioms NormSplitBaseDescent.pairFixedFromDivisor
#print axioms NormSplitBaseDescent.frobeniusFixedIff
#print axioms NormSplitBaseDescent.pairPrimeCoefficients
#print axioms NormSplitBaseDescent.primePolynomialLift
#print axioms NormSplitBaseDescent.actualNormDivides
#print axioms NormSplitBaseDescent.actualCorePrimeCoefficients
