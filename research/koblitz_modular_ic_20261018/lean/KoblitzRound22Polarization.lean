import KoblitzRound22Recovery
namespace KoblitzRound22Polarization
open scoped BigOperators
open KoblitzRound19OrbitCover KoblitzRound22Recovery
def metric : Matrix (Fin 4) (Fin 4) ℤ :=
  ![![2,1,1,1],![1,2,1,1],![1,1,2,1],![1,1,1,2]]
def J : Matrix (Fin 5) (Fin 4) ℤ :=
  ![![0,0,0,1],![0,0,1,0],![0,1,0,0],![1,0,0,0],![-1,-1,-1,-1]]
theorem actual_induced_metric : J.transpose*J=metric := by decide +kernel
theorem actual_metric_determinant : metric.det=5 := by decide +kernel
theorem actual_weighted_polarization : ∀i j:Fin 4,
    (∑ block:Fin 4,(∑ s:Fin 4,(∑ t:Fin 4,
      scale (weights block) (cmul (bar (actualMap block s i))
        (cmul (metric s t,0) (actualMap block t j))))))=
    scale 33075 (H i j) := by decide +kernel
end KoblitzRound22Polarization
#print axioms KoblitzRound22Polarization.actual_induced_metric
#print axioms KoblitzRound22Polarization.actual_metric_determinant
#print axioms KoblitzRound22Polarization.actual_weighted_polarization
