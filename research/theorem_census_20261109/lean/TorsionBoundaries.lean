import S3Completion
import StructuralAudit
import RationalCounterexample
import Mathlib.FieldTheory.IsAlgClosed.AlgebraicClosure
import Mathlib.Algebra.Polynomial.SpecificDegree

/-! Actual elliptic point witnesses at excluded torsion boundaries.
BK's qualified theorem is not refuted. FST2010 §2 defines finite-field
supersingularity by gcd(trace,q) != 1; that exact definition is used below.
No assertion about geometric p-rank is smuggled in as an axiom. -/
namespace TorsionBoundaryAudit
open Polynomial SemaevS3
private instance primeFive : Fact (Nat.Prime 5) := ⟨by decide⟩
private instance primeSeven : Fact (Nat.Prime 7) := ⟨by decide⟩

noncomputable def finiteFieldTrace (p : ℕ) (E : WeierstrassCurve.Affine (ZMod p)) : ℤ :=
  (p:ℤ)+1-Nat.card E.Point

/-- FST2010, §2, p230: the finite-field trace definition, not a new
geometric equivalence theorem. -/
def sourceSupersingular (p : ℕ) (E : WeierstrassCurve.Affine (ZMod p)) : Prop :=
  Int.gcd (finiteFieldTrace p E) (p:ℤ) ≠ 1

noncomputable def embeddingDegree (q r : ℕ) : ℕ := orderOf (q : ZMod r)

theorem small_curve_cardinality : Nat.card SkepticalAudit.rationalCurve.Point=6 := by
  rw [Nat.card_congr SkepticalAudit.rationalCurve.nonsingularPointEquiv]
  change Nat.card (Option {xy : ZMod 5 × ZMod 5 //
    SkepticalAudit.rationalCurve.Nonsingular xy.1 xy.2})=6
  letI : DecidablePred (fun xy : ZMod 5 × ZMod 5 =>
      SkepticalAudit.rationalCurve.Nonsingular xy.1 xy.2) := fun xy =>
    decidable_of_iff (xy.2^2=xy.1^3+1)
      (by change xy.2^2=xy.1^3+1 ↔ SkepticalAudit.rationalCurve.Nonsingular xy.1 xy.2
          erw [← WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
        SkepticalAudit.rationalCurve_nonsingular,
        WeierstrassCurve.Affine.equation_iff]
          simp [SkepticalAudit.rationalCurve])
  rw [Nat.card_eq_fintype_card]
  decide +kernel

theorem small_curve_trace : finiteFieldTrace 5 SkepticalAudit.rationalCurve=0 := by
  simp [finiteFieldTrace,small_curve_cardinality]

theorem small_curve_source_supersingular :
    sourceSupersingular 5 SkepticalAudit.rationalCurve := by
  norm_num [sourceSupersingular,small_curve_trace]

def twoPoint : SkepticalAudit.rationalCurve.Point := .some 4 0 (by
  rw [WeierstrassCurve.Affine.nonsingular_iff]
  constructor
  · rw [WeierstrassCurve.Affine.equation_iff]; decide
  · decide)

theorem twoPoint_nonzero : twoPoint ≠ 0 := by decide

theorem twoPoint_two_torsion : (2:ℕ) • twoPoint=0 := by
  rw [two_nsmul]
  change WeierstrassCurve.Affine.Point.some 4 0 _ +
    WeierstrassCurve.Affine.Point.some 4 0 _=0
  exact WeierstrassCurve.Affine.Point.add_self_of_Y_eq (by decide)

theorem twoPoint_order : addOrderOf twoPoint=2 := by
  letI : Fact (Nat.Prime 2) := ⟨by decide⟩
  exact addOrderOf_eq_prime twoPoint_two_torsion twoPoint_nonzero

theorem small_curve_embedding_degree : embeddingDegree 5 2=1 := by
  exact TheoremAudit.two_subgroup_embedding_degree 5 (by decide)

/-- Literal universal prime-subgroup reading of the corpus node fails at r=2.
All inputs, including the actual point group and source trace definition,
are explicit. No conclusion about large-prime screening is asserted. -/
theorem literal_supersingular_k_two_false :
    ¬ ∀ (p r : ℕ) (E : WeierstrassCurve.Affine (ZMod p)),
      Nat.Prime p → 5≤p → Nat.Prime r → Nat.Coprime r p → E.Δ ≠ 0 →
      sourceSupersingular p E → r ∣ Nat.card E.Point → embeddingDegree p r=2 := by
  intro h
  have hk := h 5 2 SkepticalAudit.rationalCurve (by decide) (by decide)
    (by decide) (by decide) SkepticalAudit.rationalCurve_nonsingular
    small_curve_source_supersingular (by rw [small_curve_cardinality]; decide)
  rw [small_curve_embedding_degree] at hk
  contradiction

/-- Arithmetic Hasse/trace step. Hasse's theorem itself is an explicit
hypothesis; it is not proved by this arithmetic lemma. -/
theorem hasse_trace_zero (p : ℕ) (t : ℤ) (hp : 5≤p)
    (hdiv : (p:ℤ) ∣ t) (hHasse : t^2 ≤ 4*(p:ℤ)) : t=0 := by
  obtain ⟨a,rfl⟩ := hdiv
  by_cases ha : a=0
  · simp [ha]
  have ha' : 1 ≤ a^2 := by
    have hcases : a≤ -1 ∨ 1≤a := by omega
    rcases hcases with h | h <;> nlinarith
  have hm := mul_le_mul_of_nonneg_left ha' (sq_nonneg (p:ℤ))
  rw [mul_pow] at hHasse
  have hp' : (5:ℤ)≤p := by exact_mod_cast hp
  nlinarith

theorem prime_non_coprime_trace_divisible (p : ℕ) (t : ℤ) (hp : Nat.Prime p)
    (h : Int.gcd t (p:ℤ)≠1) : (p:ℤ) ∣ t := by
  apply Int.natCast_dvd.mpr
  by_contra hd
  have hc : Nat.gcd p t.natAbs=1 := hp.coprime_iff_not_dvd.mpr hd
  apply h
  simpa [Int.gcd_def,Nat.gcd_comm] using hc

/-- The corrected odd-prime conclusion, explicitly conditional on Hasse.
The trace definition and actual rational point count are already connected;
this lemma does not silently assume the conclusion q=-1 modulo r. -/
theorem corrected_prime_embedding_degree (p r : ℕ) [Fact (Nat.Prime r)]
    (hp : Nat.Prime p) (hp5 : 5≤p) (hr : 2<r)
    (E : WeierstrassCurve.Affine (ZMod p)) (hss : sourceSupersingular p E)
    (hHasse : (finiteFieldTrace p E)^2 ≤ 4*(p:ℤ))
    (hrN : r ∣ Nat.card E.Point) : embeddingDegree p r=2 := by
  have ht := hasse_trace_zero p (finiteFieldTrace p E) hp5
    (prime_non_coprime_trace_divisible p _ hp hss) hHasse
  have hN : Nat.card E.Point=p+1 := by
    unfold finiteFieldTrace at ht
    omega
  exact TheoremAudit.trace_zero_prime_embedding_degree hr p (hN ▸ hrN)

abbrev F := ZMod 7
abbrev K := AlgebraicClosure F
noncomputable section
local instance : DecidableEq K := Classical.decEq K

def bkCurve : WeierstrassCurve.Affine F := ⟨0,0,0,3,2⟩

theorem bkCurve_nonsingular : bkCurve.Δ ≠ 0 := by decide

theorem bkCurve_cardinality : Nat.card bkCurve.Point=9 := by
  rw [Nat.card_congr bkCurve.nonsingularPointEquiv]
  change Nat.card (Option {xy : F × F // bkCurve.Nonsingular xy.1 xy.2})=9
  letI : DecidablePred (fun xy : F × F => bkCurve.Nonsingular xy.1 xy.2) := fun xy =>
    decidable_of_iff (xy.2^2=xy.1^3+3*xy.1+2)
      (by change xy.2^2=xy.1^3+3*xy.1+2 ↔ bkCurve.Nonsingular xy.1 xy.2
          erw [← WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
        bkCurve_nonsingular,WeierstrassCurve.Affine.equation_iff]
          simp [bkCurve])
  rw [Nat.card_eq_fintype_card]
  decide +kernel

def cubic : Polynomial F := X^3+C 5*X^2+C 3*X+C 2

theorem cubic_no_base_root : ∀ x:F, x^3+5*x^2+3*x+2 ≠ 0 := by decide

theorem geometric_root_exists : ∃ α : K,
    α^3+5*α^2+3*α+2=0 ∧ ¬ ∃ a:F, algebraMap F K a=α := by
  let g := cubic.map (algebraMap F K)
  have hd : g.natDegree=3 := by
    have hg : g=(X^3+(C (5:K)*X^2+C 3*X+C 2)) := by
      simp [g,cubic,map_ofNat]
      ring
    rw [hg,Polynomial.natDegree_add_eq_left_of_natDegree_lt
      (by compute_degree! : (C (5:K)*X^2+C 3*X+C 2).natDegree < (X^3:Polynomial K).natDegree)]
    simp
  have hd' : g.degree≠0 := ne_of_gt (Polynomial.natDegree_pos_iff_degree_pos.mp
    (by omega : 0<g.natDegree))
  obtain ⟨α,hα⟩ := IsAlgClosed.exists_root g hd'
  have hα' : α^3+5*α^2+3*α+2=0 := by
    simpa [Polynomial.IsRoot,g,cubic,map_ofNat] using hα
  refine ⟨α,hα',?_⟩
  rintro ⟨a,rfl⟩
  have hm : algebraMap F K (a^3+5*a^2+3*a+2)=0 := by
    simpa only [map_add,map_mul,map_pow,map_ofNat] using hα'
  exact cubic_no_base_root a ((map_eq_zero (algebraMap F K)).mp hm)

theorem repeated_s3_three_torsion {L : Type*} [Field L] [DecidableEq L]
    (E : WeierstrassCurve.Affine L) (ha1:E.a₁=0) (ha2:E.a₂=0) (ha3:E.a₃=0)
    (h2:(2:L)≠0) {x y:L} (h:E.Nonsingular x y)
    (hs:S3 E.a₄ E.a₆ x x x=0) :
    (3:ℕ) • WeierstrassCurve.Affine.Point.some x y h=0 := by
  let P : E.Point := .some x y h
  have hx : pointX (P+P)=some x :=
    (TheoremAudit.S3_repeated_mathlib_iff E ha1 ha2 ha3 h2 h x).mp hs
  have hc : P+P=P ∨ P+P= -P := by
    cases he : P+P with
    | zero => rw [he] at hx; cases hx
    | some u v huv =>
      rw [he] at hx
      have hu : u=x := Option.some.inj hx
      subst u
      change WeierstrassCurve.Affine.Point.some x v huv =
        WeierstrassCurve.Affine.Point.some x y h ∨
        WeierstrassCurve.Affine.Point.some x v huv = -WeierstrassCurve.Affine.Point.some x y h
      exact WeierstrassCurve.Affine.Point.X_eq_iff.mp rfl
  rcases hc with hc | hc
  · have hz : P=0 := add_right_cancel (show P+P=0+P by simpa using hc)
    have hP : P≠0 := by simp [P]
    exact (hP hz).elim
  · change (3:ℕ) • P=0
    rw [show (3:ℕ)=1+1+1 by decide,add_nsmul,add_nsmul,one_nsmul,hc,neg_add_cancel]

theorem bk_geometric_nonrational_three_torsion :
    ∃ P : (bkCurve.baseChange K).Point,
      (3:ℕ) • P=0 ∧
      P ∉ Set.range (WeierstrassCurve.Affine.Point.map (W':=bkCurve) (Algebra.ofId F K)) := by
  classical
  obtain ⟨α,hα,hnot⟩ := geometric_root_exists
  obtain ⟨y,hy⟩ := IsAlgClosed.exists_pow_nat_eq (α^3+3*α+2) (n:=2) (by decide)
  let E := bkCurve.baseChange K
  have hΔ : E.Δ≠0 := by
    change (bkCurve.map (algebraMap F K)).Δ≠0
    rw [WeierstrassCurve.map_Δ]
    exact (map_ne_zero (algebraMap F K)).mpr bkCurve_nonsingular
  have heq : E.Equation α y := by
    rw [WeierstrassCurve.Affine.equation_iff]
    simpa [E,bkCurve,WeierstrassCurve.baseChange,WeierstrassCurve.map,map_ofNat] using hy
  have hn : E.Nonsingular α y :=
    (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero hΔ).mp heq
  let P : E.Point := .some α y hn
  have h2 : (2:K)≠0 := by
    have h : (2:F)≠0 := by decide
    simpa only [map_ofNat] using (map_ne_zero (algebraMap F K)).mpr h
  have hs : S3 E.a₄ E.a₆ α α α=0 := by
    have hc : (7:K)=0 := by
      have h : (7:F)=0 := by decide
      simpa only [map_ofNat,map_zero] using congrArg (algebraMap F K) h
    simp only [E,bkCurve,WeierstrassCurve.baseChange,WeierstrassCurve.map,
      map_zero,map_ofNat,S3]
    linear_combination -3*(α-5)*hα-(12*α^2+9*α+3)*hc
  refine ⟨P,repeated_s3_three_torsion E (by simp [E,bkCurve,WeierstrassCurve.baseChange,WeierstrassCurve.map])
    (by simp [E,bkCurve,WeierstrassCurve.baseChange,WeierstrassCurve.map])
    (by simp [E,bkCurve,WeierstrassCurve.baseChange,WeierstrassCurve.map]) h2 hn hs,?_⟩
  rintro ⟨Q,hQ⟩
  cases Q with
  | zero =>
    have hx := congrArg pointX hQ
    change none=some α at hx
    cases hx
  | some x z hz =>
    have hx := congrArg pointX hQ
    simp [WeierstrassCurve.Affine.Point.map,P,pointX] at hx
    exact hnot ⟨x,hx⟩

/-- Actual failure when BK's exclusion is dropped, even with r² dividing
the rational group order. The original BK theorem retains the exclusion. -/
theorem bk_excluded_boundary_counterexample :
    Nat.Prime 3 ∧ Nat.Coprime 3 7 ∧ 3 ∣ 7-1 ∧ 3^2 ∣ Nat.card bkCurve.Point ∧
      bkCurve.Δ ≠ 0 ∧ ∃ P : (bkCurve.baseChange K).Point,
        (3:ℕ) • P=0 ∧
        P ∉ Set.range (WeierstrassCurve.Affine.Point.map (W':=bkCurve) (Algebra.ofId F K)) := by
  exact ⟨by decide,by decide,by decide,by rw [bkCurve_cardinality]; decide,
    bkCurve_nonsingular,bk_geometric_nonrational_three_torsion⟩
end
end TorsionBoundaryAudit
