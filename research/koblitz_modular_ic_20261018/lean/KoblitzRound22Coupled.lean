import KoblitzRound22
import Lean.Elab.Tactic.Omega
namespace KoblitzRound22Coupled

theorem both_factors_have_poles (N a b : ℤ)
    (ha : -N<a) (hb : -N<b) (hab : a+b = -N) : a<0 ∧ b<0 := by
  omega

theorem third_pole_contradicts_regular_product (N a b c : ℤ)
    (hab : a+b = -N) (hc : c<0) (hregular : 0≤N+a+b+c) : False := by
  omega

/-- The residue-field group consequence of the actual proper descent.
No Frobenius action on the arc parameter is assumed. -/
theorem coupled_proper_reduction
    {I A : Type*} [AddCommGroup A] (τ : I→I) (p0 p1 p2 p3 : I→A)
    (h1 : ∀i,p1 i=p0 (τ i)-p3 (τ i))
    (h2 : ∀i,p2 i=p1 (τ i)-p3 (τ i))
    (h0 : ∀i,p0 i=0) (h01 : ∀i,p1 i=0) : ∀i,p2 i=0 := by
  intro i
  have h3 : p3 (τ i)=0 := by simpa [h0,h01] using (h1 i).symm
  simp [h2,h01,h3]

/-- Complete simultaneous valuation obstruction for every coefficient component.
The pole equivalences and proper-specialization identities are explicit
geometric hypotheses; the report supplies their actual native/written bridge.
This is not a theorem asserting membership in a polynomial ideal. -/
theorem no_node187_valuative_model
    {I A : Type*} [Nonempty I] [AddCommGroup A]
    (τ : I→I) (p0 p1 p2 p3 : I→A) (v0 v1 v2 : I→ℤ) (N : ℤ)
    (h1 : ∀i,p1 i=p0 (τ i)-p3 (τ i))
    (h2 : ∀i,p2 i=p1 (τ i)-p3 (τ i))
    (pole0 : ∀i,v0 i<0 ↔ p0 i=0)
    (pole1 : ∀i,v1 i<0 ↔ p1 i=0)
    (pole2 : ∀i,v2 i<0 ↔ p2 i=0)
    (bound0 : ∀i,-N<v0 i) (bound1 : ∀i,-N<v1 i)
    (unitProduct : ∀i,v0 i+v1 i = -N)
    (regularTriple : ∀i,0≤N+v0 i+v1 i+v2 i) : False := by
  have hn (i : I) : v0 i<0 ∧ v1 i<0 :=
    both_factors_have_poles N (v0 i) (v1 i) (bound0 i) (bound1 i) (unitProduct i)
  have hp0 (i : I) : p0 i=0 := (pole0 i).mp (hn i).1
  have hp1 (i : I) : p1 i=0 := (pole1 i).mp (hn i).2
  have hp2 := coupled_proper_reduction τ p0 p1 p2 p3 h1 h2 hp0 hp1
  obtain ⟨i⟩ := ‹Nonempty I›
  exact third_pole_contradicts_regular_product N (v0 i) (v1 i) (v2 i)
    (unitProduct i) ((pole2 i).mpr (hp2 i)) (regularTriple i)

theorem uncoupled_valuation_control :
    (-2:ℤ)< -1 ∧ -1+(-1)= -2 ∧ 0≤2+(-1)+(-1)+1 := by omega

end KoblitzRound22Coupled
#print axioms KoblitzRound22Coupled.both_factors_have_poles
#print axioms KoblitzRound22Coupled.third_pole_contradicts_regular_product
#print axioms KoblitzRound22Coupled.coupled_proper_reduction
#print axioms KoblitzRound22Coupled.no_node187_valuative_model
#print axioms KoblitzRound22Coupled.uncoupled_valuation_control
