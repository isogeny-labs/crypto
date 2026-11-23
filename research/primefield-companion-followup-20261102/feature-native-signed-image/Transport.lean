import Std
namespace SignedTransport
-- Scaling both members of the old reciprocal pair would not give an inverse pair.
theorem inverse_pair {R : Type} [Lean.Grind.CommRing R]
    (s t v vi : R) (hs : s*t=1) (hv : v*vi=1) :
    (s*v)*(t*vi)=1 := by grind
-- The trace in the new chart generally depends on the old orientation v.
theorem transported_trace {R : Type} [Lean.Grind.CommRing R]
    (s t v vi w : R) (hw : v+vi=w) :
    s*v+t*vi = t*w+(s-t)*v := by grind
#print axioms inverse_pair
#print axioms transported_trace
end SignedTransport
