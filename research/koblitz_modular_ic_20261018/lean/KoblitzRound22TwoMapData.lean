import KoblitzRound22TwoMap
namespace KoblitzRound22TwoMapData
open scoped BigOperators
open KoblitzRound19OrbitCover KoblitzRound22Recovery
def smallB : Fin 4→Fin 4→CM := ![![(-1,-2),(-2,-5),(-1,-3),(1,-5)],![(-1,5),(-2,3),(-3,0),(-2,2)],![(2,-2),(1,3),(0,1),(-1,-2)],![(1,2),(3,0),(2,5),(1,3)]]
def smallC : Fin 4→Fin 4→CM := ![![(-2,4),(0,0),(-2,3),(-3,0)],![(3,0),(1,4),(3,0),(1,3)],![(-1,-3),(2,-3),(0,1),(2,-3)],![(-2,3),(-3,0),(0,0),(-2,4)]]
def powers : Fin 4→Fin 4→Fin 4→CM := ![![![(1,0),(0,0),(0,0),(0,0)],![(0,0),(1,0),(0,0),(0,0)],![(0,0),(0,0),(1,0),(0,0)],![(0,0),(0,0),(0,0),(1,0)]],![![(-1,0),(-1,0),(-1,0),(-1,0)],![(1,0),(0,0),(0,0),(0,0)],![(0,0),(1,0),(0,0),(0,0)],![(0,0),(0,0),(1,0),(0,0)]],![![(0,0),(0,0),(0,0),(1,0)],![(-1,0),(-1,0),(-1,0),(-1,0)],![(1,0),(0,0),(0,0),(0,0)],![(0,0),(1,0),(0,0),(0,0)]],![![(0,0),(0,0),(1,0),(0,0)],![(0,0),(0,0),(0,0),(1,0)],![(-1,0),(-1,0),(-1,0),(-1,0)],![(1,0),(0,0),(0,0),(0,0)]]]
def bcoeff : Fin 4→CM := ![(1,3),(2,5),(3,0),(1,2)]
def ccoeff : Fin 4→CM := ![(-2,4),(0,0),(-3,0),(-2,3)]
theorem actual_two_map_recovery : ∀i j:Fin 4,(∑k:Fin 4,(cmul (smallB i k) (map1 k j)+cmul (smallC i k) (map3 k j)))=(if i=j then((1,0):CM)else(0,0)) := by decide +kernel
theorem smallB_commutes : ∀i j:Fin 4,(∑k:Fin 4,cmul (smallB i k) (rotation k j))=(∑k:Fin 4,cmul (rotation i k) (smallB k j)) := by decide +kernel
theorem smallC_commutes : ∀i j:Fin 4,(∑k:Fin 4,cmul (smallC i k) (rotation k j))=(∑k:Fin 4,cmul (rotation i k) (smallC k j)) := by decide +kernel
theorem actual_b_coefficients : ∀i j:Fin 4,smallB i j=(∑k:Fin 4,cmul (bcoeff k) (powers k i j)) := by decide +kernel
theorem actual_c_coefficients : ∀i j:Fin 4,smallC i j=(∑k:Fin 4,cmul (ccoeff k) (powers k i j)) := by decide +kernel
end KoblitzRound22TwoMapData
#print axioms KoblitzRound22TwoMapData.actual_two_map_recovery
#print axioms KoblitzRound22TwoMapData.smallB_commutes
#print axioms KoblitzRound22TwoMapData.smallC_commutes
#print axioms KoblitzRound22TwoMapData.actual_b_coefficients
#print axioms KoblitzRound22TwoMapData.actual_c_coefficients
