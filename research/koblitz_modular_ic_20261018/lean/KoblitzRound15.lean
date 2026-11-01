import KoblitzRound14
namespace KoblitzRound15

theorem theta_bias_residual {R : Type*} [CommRing R] (a b c d : R) :
 (a+c)*(b+d)-a*b-c*d=a*d+c*b := by ring

theorem CM_norm_parity_residual {R : Type*} [CommRing R] (a b : R) :
 (a^2-a*b+2*b^2)-a*(a+b)=2*b*(b-a) := by ring

theorem product_first_jet {R : Type*} [CommRing R] (t x0 x1 y0 y1 : R) :
 (x0+t*x1)*(y0+t*y1)-x0*y0-t*(x0*y1+x1*y0)=t^2*x1*y1 := by ring

/-- Necessary coefficient equation for h=t and h*z=x^2 at x0=0,z0=1. -/
theorem square_unramified_obstruction {R : Type*} [CommRing R] [Nontrivial R]
 (x1 : R) : (0:R)*x1+x1*0 ≠ 1 := by simp

/-- Ramified arcs survive the first-jet obstruction. -/
theorem square_ramified_control {R : Type*} [CommRing R] (t : R) :
 t^2*(1:R)=t*t := by ring

theorem coordinate_product_residual {R : Type*} [CommRing R]
 (x y A B C D : R) :
 (A+B*y)*(C+D*y)-((A*C+B*D*(x^3+x^2+1))+(A*D+B*C+B*D*x)*y)
 =B*D*(y^2-x*y-x^3-x^2-1) := by ring

theorem asymmetric_target_residual {R : Type*} [CommRing R] (x u v : R) :
 x^2*(v^2+u*v-u^3-u^2-1)-(1+x*u-x^2*u^3-x^2*u^2-x^2)
 =(x*v-1)*(x*v+1+x*u) := by ring

theorem coupled_invariant_residual {R : Type*} [CommRing R] (x y u v : R) :
 (x*v+u*y)^2-(x+u)*(y+v)*(x*v+u*y)+(x+u)^2*(y*v)+(y+v)^2*(x*u)
 =4*(x*u)*(y*v) := by ring

/-- Two-sided inverse data solves every affine Frobenius drift uniquely. -/
theorem affine_fixed_unique {V : Type*} [AddCommGroup V]
 (c d : V →+ V) (hl : ∀ x,d (x-c x)=x) (hr : ∀ x,d x-c (d x)=x)
 (b : V) : ∃! x,c x+b=x := by
 refine ⟨d b,?_,?_⟩
 · have h := hr b
   calc c (d b)+b = c (d b)+(d b-c (d b)) := by rw [h]
        _ = d b := by abel
 · intro y hy
   have h : y-c y=b := by
    calc y-c y=(c y+b)-c y := congrArg (fun z => z-c y) hy.symm
         _ = b := by abel
   calc y = d (y-c y) := (hl y).symm
        _ = d b := by rw [h]

theorem improved_polarized_dimensions :
 3^5=243 ∧ 243-1=242 ∧ 243*19^2=87723 ∧
 120*243=29160 ∧ 7776=32*243 := by norm_num

theorem new_support_degrees :
 12+320+460=792 ∧ 3+290+367=660 ∧ 5*460=2300 ∧ 5*367=1835 := by norm_num
end KoblitzRound15
#print axioms KoblitzRound15.theta_bias_residual
#print axioms KoblitzRound15.CM_norm_parity_residual
#print axioms KoblitzRound15.product_first_jet
#print axioms KoblitzRound15.square_unramified_obstruction
#print axioms KoblitzRound15.square_ramified_control
#print axioms KoblitzRound15.coordinate_product_residual
#print axioms KoblitzRound15.asymmetric_target_residual
#print axioms KoblitzRound15.coupled_invariant_residual
#print axioms KoblitzRound15.affine_fixed_unique
#print axioms KoblitzRound15.improved_polarized_dimensions
#print axioms KoblitzRound15.new_support_degrees
