import Std
namespace PoweredImage
theorem signed_product {R : Type} [Lean.Grind.CommRing R]
    (a x y p q v : R) (image : x*y=v) :
    (a*x*p)*(a*y*q) = a*a*v*p*q := by grind
#print axioms signed_product
end PoweredImage
