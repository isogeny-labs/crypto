import KoblitzRound22Recovery
namespace KoblitzRound22TwoMap
variable {K : Type*} [CommRing K]
def m (t w : K) : K := (2+w)+7*t+(6+2*w)*t^3
def b (t w : K) : K := (1+3*w)+(2+5*w)*t+3*t^2+(1+2*w)*t^3
def c (t w : K) : K := (-2+4*w)-3*t^2+(-2+3*w)*t^3
def cyclotomic (t : K) : K := 1+t+t^2+t^3+t^4
theorem two_map_integer_lift (t w : K) :
    b t w*m t w+7*c t w-1=
    (3+5*t+8*t^3+10*t^4+4*t^6)*(w^2+w+2)+
    ((-19+32*w)+(20-4*w)*t+(-2+10*w)*t^2)*cyclotomic t := by
  unfold b m c cyclotomic
  ring
theorem two_map_equivariant_recovery (t w : K)
    (hw : w^2+w+2=0) (ht : cyclotomic t=0) : b t w*m t w+7*c t w=1 := by
  have h:=two_map_integer_lift t w
  rw [hw,ht] at h
  linear_combination h
end KoblitzRound22TwoMap
#print axioms KoblitzRound22TwoMap.two_map_integer_lift
#print axioms KoblitzRound22TwoMap.two_map_equivariant_recovery
