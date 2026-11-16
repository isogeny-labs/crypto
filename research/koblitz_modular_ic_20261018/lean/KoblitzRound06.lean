import KoblitzRound06Matrices

namespace KoblitzRound06

def step {A : Type*} [AddCommGroup A] (f : A →+ A) (y : Fin 5 → A) : Fin 5 → A :=
  ![-y 3, y 0-y 3, y 1-y 3, y 2-y 3, y 4+f (y 3)]

/-- Effective order-five descent for every additive endomorphism pi^k. -/
theorem descent_order_five {A : Type*} [AddCommGroup A] (f : A →+ A) (y : Fin 5 → A) :
    step f (step f (step f (step f (step f y))))=y := by
  funext i
  fin_cases i <;> simp [step,map_add,map_sub,map_neg] <;> abel

/-- Universal bilinear pullback identity, including its explicit norm residual. -/
theorem polarization_lift {R : Type*} [CommRing R]
    (a b c d e f g h Q Rr p q alpha : R) :
  let sa := a+b+c+d
  let sb := e+f+g+h
  let sy := 5*sa
  let sw := 5*sb
  let z := -q*sa-Q
  let zz := -p*sb-Rr
  (a+sa)*(e+sb)+(b+sa)*(f+sb)+(c+sa)*(g+sb)+(d+sa)*(h+sb)+
    alpha*sy*sw+q*sy*zz+p*z*sw+5*z*zz =
    a*e+b*f+c*g+d*h+sa*sb+5*Q*Rr+5*(1+5*alpha-p*q)*sa*sb := by
  dsimp; ring

/-- The actual norm equation removes the residual, for k=0,4,8. -/
theorem polarization_pullback {R : Type*} [CommRing R]
    (a b c d e f g h Q Rr p q alpha : R) (hn : p*q=1+5*alpha) :
  let sa := a+b+c+d
  let sb := e+f+g+h
  let z := -q*sa-Q
  let zz := -p*sb-Rr
  (a+sa)*(e+sb)+(b+sa)*(f+sb)+(c+sa)*(g+sb)+(d+sa)*(h+sb)+
    alpha*(5*sa)*(5*sb)+q*(5*sa)*zz+p*z*(5*sb)+5*z*zz =
    a*e+b*f+c*g+d*h+sa*sb+5*Q*Rr := by
  dsimp
  have h := polarization_lift a b c d e f g h Q Rr p q alpha
  dsimp at h
  rw [hn] at h
  simpa using h

/-- Kernel of the split quotient: diagonal five-torsion and its pi^k image. -/
theorem split_kernel {A : Type*} [AddCommGroup A] (phi : A →+ A)
    (a b c d e Q : A) (hs : a+b+c+d+e=0)
    (ha : a-e=0) (hb : b-e=0) (hc : c-e=0) (hd : d-e=0)
    (hq : phi e-Q=0) :
    a=e ∧ b=e ∧ c=e ∧ d=e ∧ (5:ℕ) • e=0 ∧ Q=phi e := by
  have ea := sub_eq_zero.mp ha
  have eb := sub_eq_zero.mp hb
  have ec := sub_eq_zero.mp hc
  have ed := sub_eq_zero.mp hd
  refine ⟨ea,eb,ec,ed,?_,(sub_eq_zero.mp hq).symm⟩
  rw [ea,eb,ec,ed] at hs
  have z : (5:ℕ) • e=e+e+e+e+e := by abel
  rwa [z]

/-- Ordinary translated fibers can represent [5P]-[O] without a cusp fiber. -/
theorem reverse_ordinary_translation {A : Type*} [AddCommGroup A] (P : A) :
    (6:ℕ) • P-P=(5:ℕ) • P := by abel

/-- The seven-point fiber sum is the fixed cusp T; even pair sums cancel. -/
theorem seven_even_cancel {A : Type*} [AddCommGroup A]
    (p q : Fin 7 → A) (T : A) (hp : ∑ i,p i=T) (hq : ∑ i,q i=T)
    (h2 : T+T=0) : ∑ i,(p i+q i)=0 := by
  rw [Finset.sum_add_distrib,hp,hq,h2]

theorem seven_odd_cancel {A : Type*} [AddCommGroup A]
    (p q : Fin 7 → A) (T : A) (hp : ∑ i,p i=T) (hq : ∑ i,q i=T) :
    ∑ i,(p i-q i)=0 := by
  rw [Finset.sum_sub_distrib,hp,hq]; simp

/-- Cleared eta equation for the degree-seven map t=x^4/y. -/
theorem degree_seven_equation {R : Type*} [CommRing R] (x t : R) :
    (x^4)^2+x*x^4*t*(x^2+x+1)+x*(x^6+x^5+x^4+x^3+x^2+x+1)*t^2 =
    x*(x^7+t*(x^6+x^5+x^4)+t^2*(x^6+x^5+x^4+x^3+x^2+x+1)) := by ring

theorem reverse_fiber_degree : 12*5*11=660 := by norm_num

/-- Norm inversion in the etale quadratic coordinate algebra used by Miller. -/
theorem quadratic_coordinate_norm {K : Type*} [CommRing K] [CharP K 2]
    (u v x y f : K) (h : y^2+x*y=f) :
    (u+v*y)*(u+v*(y+x))=u^2+u*v*x+v^2*f := by
  have h2 : (2:K)=0 := by simpa using (CharP.natCast_eq_natCast_mod K 2 2)
  linear_combination v^2*h+u*v*y*h2

theorem fifth_line_wrap : (2 : ZMod 11)^5 = -1 := by decide +kernel

end KoblitzRound06

#print axioms KoblitzRound06.descent_order_five
#print axioms KoblitzRound06.polarization_lift
#print axioms KoblitzRound06.polarization_pullback
#print axioms KoblitzRound06.split_kernel
#print axioms KoblitzRound06.reverse_ordinary_translation
#print axioms KoblitzRound06.seven_even_cancel
#print axioms KoblitzRound06.seven_odd_cancel
#print axioms KoblitzRound06.degree_seven_equation
#print axioms KoblitzRound06.reverse_fiber_degree
#print axioms KoblitzRound06.quadratic_coordinate_norm
#print axioms KoblitzRound06.fifth_line_wrap
