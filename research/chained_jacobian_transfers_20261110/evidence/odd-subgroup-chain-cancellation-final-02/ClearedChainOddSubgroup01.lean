import Init.Grind.Module.Basic

/-! The middle norm/pullback relation clears the apparent repeated quarter.
These are module consequences with explicit map identities and linearity.
No torsion-free assumption on geometric point groups is used. The second
statement explicitly concerns a domain where multiplication by4 is injective;
global cancellation in Hom remains a separate written geometric bridge. -/
set_option autoImplicit false
namespace ChainCompression
open Lean.Grind

theorem two_hop_double_cleared {J0 J1 J2 E0 E1 E2 : Type}
    [IntModule J1] [IntModule J2] [IntModule E1] [IntModule E2]
    (f : J0 → J1) (g : J1 → J2)
    (h0 : J0 → E0) (k1 : E1 → J1) (h1 : J1 → E1) (k2 : E2 → J2)
    (a : E0 → E1) (b : E1 → E2)
    (hf : ∀ x, (4 : Int) • f x = k1 (a (h0 x)))
    (hg : ∀ x, (4 : Int) • g x = k2 (b (h1 x)))
    (hm : ∀ e, h1 (k1 e) = (4 : Int) • e)
    (gl : ∀ n : Int, ∀ x, g (n • x) = n • g x)
    (bl : ∀ n : Int, ∀ e, b (n • e) = n • b e)
    (kl : ∀ n : Int, ∀ e, k2 (n • e) = n • k2 e) :
    ∀ x, (16 : Int) • g (f x) = (4 : Int) • k2 (b (a (h0 x))) := by
  intro x
  calc
    (16 : Int) • g (f x) = (4 : Int) • g ((4 : Int) • f x) := by
      rw [gl]
      grind
    _ = (4 : Int) • g (k1 (a (h0 x))) := by rw [hf]
    _ = k2 (b (h1 (k1 (a (h0 x))))) := hg _
    _ = k2 (b ((4 : Int) • a (h0 x))) := by rw [hm]
    _ = (4 : Int) • k2 (b (a (h0 x))) := by rw [bl, kl]

theorem rational_two_hop_factorization {J0 J1 J2 E0 E1 E2 : Type}
    [IntModule J1] [IntModule J2] [IntModule E1] [IntModule E2]
    (f : J0 → J1) (g : J1 → J2)
    (h0 : J0 → E0) (k1 : E1 → J1) (h1 : J1 → E1) (k2 : E2 → J2)
    (a : E0 → E1) (b : E1 → E2)
    (hf : ∀ x, (4 : Int) • f x = k1 (a (h0 x)))
    (hg : ∀ x, (4 : Int) • g x = k2 (b (h1 x)))
    (hm : ∀ e, h1 (k1 e) = (4 : Int) • e)
    (gl : ∀ n : Int, ∀ x, g (n • x) = n • g x)
    (bl : ∀ n : Int, ∀ e, b (n • e) = n • b e)
    (kl : ∀ n : Int, ∀ e, k2 (n • e) = n • k2 e)
    (cancel4 : ∀ u v : J2, (4 : Int) • u = (4 : Int) • v → u = v) :
    ∀ x, (4 : Int) • g (f x) = k2 (b (a (h0 x))) := by
  intro x
  apply cancel4
  have h := two_hop_double_cleared f g h0 k1 h1 k2 a b hf hg hm gl bl kl x
  grind

end ChainCompression
#print axioms ChainCompression.two_hop_double_cleared
#print axioms ChainCompression.rational_two_hop_factorization


/-! Additive point/subgroup cancellation. The original entire module is the
unchanged prefix. These are explicit module consequences, with no assertion
that ambient Jacobian point groups have injective multiplication by four. -/
namespace ClearedChainOddSubgroup01
open Lean.Grind

/-- A supplied Bezout certificate recovers a point annihilated by r. -/
theorem recover_annihilated_point {A : Type} [IntModule A]
    (d r u v : Int) (hbez : u*d + v*r = 1)
    (x : A) (hr : r • x = 0) : u • (d • x) = x := by
  calc
    u • (d • x) = (u*d) • x := (IntModule.mul_zsmul u d x).symm
    _ = (u*d) • x + (v*r) • x := by
      rw [IntModule.mul_zsmul v r,hr,IntModule.zsmul_zero,AddCommMonoid.add_zero]
    _ = (u*d+v*r) • x := (IntModule.add_zsmul _ _ _).symm
    _ = x := by rw [hbez,IntModule.one_zsmul]

/-- Cancellation is local to the two annihilated points; A may have d-torsion. -/
theorem cancel_on_annihilated_points {A : Type} [IntModule A]
    (d r u v : Int) (hbez : u*d+v*r=1)
    (x y : A) (hx : r • x=0) (hy : r • y=0)
    (hclear : d • x=d • y) : x=y := by
  calc
    x = u • (d • x) := (recover_annihilated_point d r u v hbez x hx).symm
    _ = u • (d • y) := congrArg (fun z : A => u • z) hclear
    _ = y := recover_annihilated_point d r u v hbez y hy

/-- Actual homomorphism inputs transport the source annihilator, so no
injectivity hypothesis is needed anywhere on the ambient target module. -/
theorem cancel_cleared_maps_on_source_exponent
    {A B : Type} [IntModule A] [IntModule B]
    (f g : A → B) (fz : f 0=0) (gz : g 0=0)
    (fl : ∀ (n : Int) x, f (n • x)=n • f x)
    (gl : ∀ (n : Int) x, g (n • x)=n • g x)
    (d r u v : Int) (hbez : u*d+v*r=1)
    (hr : ∀ x : A, r • x=0)
    (hclear : ∀ x, d • f x=d • g x) : ∀ x, f x=g x := by
  intro x
  have hf : r • f x=0 := by rw [← fl r x,hr x,fz]
  have hg : r • g x=0 := by rw [← gl r x,hr x,gz]
  exact cancel_on_annihilated_points d r u v hbez (f x) (g x) hf hg (hclear x)

/-- The old two-hop cleared identity cancels on the two stated r-annihilated
outputs. It does not require multiplication by four injective on all J2. -/
theorem two_hop_factorization_on_annihilated_outputs
    {J0 J1 J2 E0 E1 E2 : Type}
    [IntModule J1] [IntModule J2] [IntModule E1] [IntModule E2]
    (f : J0 → J1) (g : J1 → J2)
    (h0 : J0 → E0) (k1 : E1 → J1) (h1 : J1 → E1) (k2 : E2 → J2)
    (a : E0 → E1) (b : E1 → E2)
    (hf : ∀ x, (4 : Int) • f x=k1 (a (h0 x)))
    (hg : ∀ x, (4 : Int) • g x=k2 (b (h1 x)))
    (hm : ∀ e, h1 (k1 e)=(4 : Int) • e)
    (gl : ∀ (n : Int) x, g (n • x)=n • g x)
    (bl : ∀ (n : Int) e, b (n • e)=n • b e)
    (kl : ∀ (n : Int) e, k2 (n • e)=n • k2 e)
    (r u v : Int) (hbez : u*4+v*r=1) (x : J0)
    (hleft : r • g (f x)=0)
    (hright : r • k2 (b (a (h0 x)))=0) :
    (4 : Int) • g (f x)=k2 (b (a (h0 x))) := by
  have hscaled : r • ((4 : Int) • g (f x))=0 := by
    calc
      r • ((4 : Int) • g (f x)) = (r*4) • g (f x) :=
        (IntModule.mul_zsmul r 4 _).symm
      _ = (4*r) • g (f x) := by rw [Int.mul_comm r 4]
      _ = (4 : Int) • (r • g (f x)) := IntModule.mul_zsmul 4 r _
      _ = 0 := by rw [hleft,IntModule.zsmul_zero]
  have hclear : (4 : Int) • ((4 : Int) • g (f x))=
      (4 : Int) • k2 (b (a (h0 x))) := by
    rw [← IntModule.mul_zsmul]
    exact ChainCompression.two_hop_double_cleared f g h0 k1 h1 k2 a b hf hg hm gl bl kl x
  exact cancel_on_annihilated_points 4 r u v hbez
    ((4 : Int) • g (f x)) (k2 (b (a (h0 x)))) hscaled hright hclear

/-- Standard public quarters for the existing fixed subgroup hypotheses. -/
theorem quarter_order109 {A : Type} [IntModule A]
    (x : A) (hr : (109 : Int) • x=0) :
    (82 : Int) • ((4 : Int) • x)=x :=
  recover_annihilated_point 4 109 82 (-3) (by decide) x hr

theorem quarter_order113 {A : Type} [IntModule A]
    (x : A) (hr : (113 : Int) • x=0) :
    (85 : Int) • ((4 : Int) • x)=x :=
  recover_annihilated_point 4 113 85 (-3) (by decide) x hr

theorem quarter_order12317 {A : Type} [IntModule A]
    (x : A) (hr : (12317 : Int) • x=0) :
    (9238 : Int) • ((4 : Int) • x)=x :=
  recover_annihilated_point 4 12317 9238 (-3) (by decide) x hr

end ClearedChainOddSubgroup01
#print axioms ClearedChainOddSubgroup01.recover_annihilated_point
#print axioms ClearedChainOddSubgroup01.cancel_on_annihilated_points
#print axioms ClearedChainOddSubgroup01.cancel_cleared_maps_on_source_exponent
#print axioms ClearedChainOddSubgroup01.two_hop_factorization_on_annihilated_outputs
#print axioms ClearedChainOddSubgroup01.quarter_order109
#print axioms ClearedChainOddSubgroup01.quarter_order113
#print axioms ClearedChainOddSubgroup01.quarter_order12317
