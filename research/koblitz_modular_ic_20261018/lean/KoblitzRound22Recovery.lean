import KoblitzRound21DescentData
namespace KoblitzRound22Recovery
open scoped BigOperators
open KoblitzRound19OrbitCover
def rotation : Fin 4→Fin 4→CM := ![![(-1,0),(-1,0),(-1,0),(-1,0)],![(1,0),(0,0),(0,0),(0,0)],![(0,0),(1,0),(0,0),(0,0)],![(0,0),(0,0),(1,0),(0,0)]]
def map0 : Fin 4→Fin 4→CM := ![![(14,0),(0,7),(7,0),(21,7)],![(-21,-7),(-7,-7),(-21,0),(-14,-7)],![(14,7),(-7,0),(7,0),(-7,7)],![(7,-7),(21,0),(0,-7),(14,-7)]]
def recovery0 : Fin 4→Fin 4→CM := ![![(-2437,5677),(2408,11991),(-3892,-658),(1806,5103)],![(-1806,-5103),(-4243,574),(602,6888),(-5698,-5761)],![(5698,5761),(3892,658),(1455,6335),(6300,12649)],![(-6300,-12649),(-602,-6888),(-2408,-11991),(-4845,-6314)]]
def map1 : Fin 4→Fin 4→CM := ![![(-5,1),(-7,0),(-1,2),(-7,0)],![(7,0),(2,1),(0,0),(6,2)],![(-6,-2),(1,-2),(-4,-1),(-6,-2)],![(6,2),(0,0),(7,0),(2,1)]]
def recovery1 : Fin 4→Fin 4→CM := ![![(19529,5443),(-2,-5),(-1,-3),(1,-5)],![(-1,5),(19528,5448),(-3,0),(-2,2)],![(2,-2),(1,3),(19530,5446),(-1,-2)],![(1,2),(3,0),(2,5),(19531,5448)]]
def map2 : Fin 4→Fin 4→CM := ![![(10,4),(0,0),(15,0),(0,0)],![(0,0),(10,4),(0,0),(15,0)],![(-15,0),(-15,0),(-5,4),(-15,0)],![(15,0),(0,0),(0,0),(10,4)]]
def recovery2 : Fin 4→Fin 4→CM := ![![(4315,11393),(0,0),(0,0),(0,0)],![(0,0),(4315,11393),(0,0),(0,0)],![(0,0),(0,0),(4315,11393),(0,0)],![(0,0),(0,0),(0,0),(4315,11393)]]
def map3 : Fin 4→Fin 4→CM := ![![(7,0),(0,0),(0,0),(0,0)],![(0,0),(7,0),(0,0),(0,0)],![(0,0),(0,0),(7,0),(0,0)],![(0,0),(0,0),(0,0),(7,0)]]
def recovery3 : Fin 4→Fin 4→CM := ![![(4932,836),(0,0),(0,0),(0,0)],![(0,0),(4932,836),(0,0),(0,0)],![(0,0),(0,0),(4932,836),(0,0)],![(0,0),(0,0),(0,0),(4932,836)]]
theorem actual_integral_recovery : ∀i j:Fin 4,(∑k:Fin 4,(cmul (recovery0 i k) (map0 k j)+cmul (recovery1 i k) (map1 k j)+cmul (recovery2 i k) (map2 k j)+cmul (recovery3 i k) (map3 k j)))=(if i=j then((1,0):CM)else(0,0)) := by decide +kernel
theorem actual_map_commutes0 : ∀i j:Fin 4,(∑k:Fin 4,cmul (map0 i k) (rotation k j))=(∑k:Fin 4,cmul (rotation i k) (map0 k j)) := by decide +kernel
theorem actual_recovery_commutes0 : ∀i j:Fin 4,(∑k:Fin 4,cmul (recovery0 i k) (rotation k j))=(∑k:Fin 4,cmul (rotation i k) (recovery0 k j)) := by decide +kernel
theorem actual_map_commutes1 : ∀i j:Fin 4,(∑k:Fin 4,cmul (map1 i k) (rotation k j))=(∑k:Fin 4,cmul (rotation i k) (map1 k j)) := by decide +kernel
theorem actual_recovery_commutes1 : ∀i j:Fin 4,(∑k:Fin 4,cmul (recovery1 i k) (rotation k j))=(∑k:Fin 4,cmul (rotation i k) (recovery1 k j)) := by decide +kernel
theorem actual_map_commutes2 : ∀i j:Fin 4,(∑k:Fin 4,cmul (map2 i k) (rotation k j))=(∑k:Fin 4,cmul (rotation i k) (map2 k j)) := by decide +kernel
theorem actual_recovery_commutes2 : ∀i j:Fin 4,(∑k:Fin 4,cmul (recovery2 i k) (rotation k j))=(∑k:Fin 4,cmul (rotation i k) (recovery2 k j)) := by decide +kernel
theorem actual_map_commutes3 : ∀i j:Fin 4,(∑k:Fin 4,cmul (map3 i k) (rotation k j))=(∑k:Fin 4,cmul (rotation i k) (map3 k j)) := by decide +kernel
theorem actual_recovery_commutes3 : ∀i j:Fin 4,(∑k:Fin 4,cmul (recovery3 i k) (rotation k j))=(∑k:Fin 4,cmul (rotation i k) (recovery3 k j)) := by decide +kernel
def integerSystem : Matrix (Fin 8) (Fin 32) ℤ := ![![14,14,-7,-14,-14,14,21,-14,2,-2,-6,4,6,-4,-7,0,10,-8,-15,0,15,0,0,0,7,0,0,0,0,0,0,0],![-7,21,7,-14,-7,-7,7,14,1,1,-2,-4,2,4,0,-7,4,6,0,-15,0,15,0,0,0,7,0,0,0,0,0,0],![0,14,7,0,-21,0,7,0,7,0,-4,2,0,0,-1,-4,0,0,-5,-8,0,0,15,0,0,0,7,0,0,0,0,0],![-7,7,0,7,0,-21,0,7,0,7,-1,-3,0,0,2,-3,0,0,4,-9,0,0,0,15,0,0,0,7,0,0,0,0],![21,0,-7,0,-7,14,0,-14,0,0,1,4,2,-2,-7,0,0,0,-15,0,10,-8,0,0,0,0,0,0,7,0,0,0],![0,21,0,-7,-7,0,7,-7,0,0,-2,3,1,1,0,-7,0,0,0,-15,4,6,0,0,0,0,0,0,0,7,0,0],![7,14,14,-14,-21,14,14,0,6,-4,-6,4,7,0,-5,-2,15,0,-15,0,0,0,10,-8,0,0,0,0,0,0,7,0],![-7,14,7,7,-7,-14,0,14,2,4,-2,-4,0,7,1,-6,0,15,0,-15,0,0,4,6,0,0,0,0,0,0,0,7]]
def integerRightInverse : Matrix (Fin 32) (Fin 8) ℤ := ![![-4845,12628,6300,-25298,-5698,11522,1806,-10206],![-6314,1469,12649,-6349,-5761,63,5103,-3297],![-2408,23982,1455,-12670,602,-13776,-3892,1316],![-11991,9583,6335,-4880,6888,-6286,-658,-3234],![-602,13776,3892,-1316,-4243,-1148,2408,-23982],![-6888,6286,658,3234,574,-4817,11991,-9583],![-6300,25298,5698,-11522,-1806,10206,-2437,-11354],![-12649,6349,5761,-63,-5103,3297,5677,-8114],![19531,-10896,-1,4,-2,-4,1,10],![5448,14083,-2,1,2,-4,-5,6],![2,-10,19530,-10892,-3,0,-1,6],![5,-3,5446,14084,0,-3,-3,2],![3,0,1,-6,19528,-10896,-2,10],![0,3,3,-2,5448,14080,-5,3],![1,-4,2,4,-1,-10,19529,-10886],![2,-1,-2,4,5,-6,5443,14086],![4315,-22786,0,0,0,0,0,0],![11393,-7078,0,0,0,0,0,0],![0,0,4315,-22786,0,0,0,0],![0,0,11393,-7078,0,0,0,0],![0,0,0,0,4315,-22786,0,0],![0,0,0,0,11393,-7078,0,0],![0,0,0,0,0,0,4315,-22786],![0,0,0,0,0,0,11393,-7078],![4932,-1672,0,0,0,0,0,0],![836,4096,0,0,0,0,0,0],![0,0,4932,-1672,0,0,0,0],![0,0,836,4096,0,0,0,0],![0,0,0,0,4932,-1672,0,0],![0,0,0,0,836,4096,0,0],![0,0,0,0,0,0,4932,-1672],![0,0,0,0,0,0,836,4096]]
theorem actual_full_integer_lattice : integerSystem*integerRightInverse=1 := by decide +kernel
def actualMap (j : Fin 4) : Fin 4→Fin 4→CM := ![map0,map1,map2,map3] j
theorem actual_augmentation_binding : ∀j i k:Fin 4,actualMap j i k=KoblitzRound20Retraction.row ⟨5*j.val+3-i.val,by omega⟩ k := by decide +kernel
theorem actual_regular_norm_zero : ∀j k:Fin 4,(∑s:Fin 5,KoblitzRound20Retraction.row ⟨5*j.val+s.val,by omega⟩ k)=(0,0) := by decide +kernel
theorem actual_rotation_binding : rotation=KoblitzRound21DescentData.rotation := by decide +kernel
end KoblitzRound22Recovery
#print axioms KoblitzRound22Recovery.actual_integral_recovery
#print axioms KoblitzRound22Recovery.actual_map_commutes0
#print axioms KoblitzRound22Recovery.actual_recovery_commutes0
#print axioms KoblitzRound22Recovery.actual_map_commutes1
#print axioms KoblitzRound22Recovery.actual_recovery_commutes1
#print axioms KoblitzRound22Recovery.actual_map_commutes2
#print axioms KoblitzRound22Recovery.actual_recovery_commutes2
#print axioms KoblitzRound22Recovery.actual_map_commutes3
#print axioms KoblitzRound22Recovery.actual_recovery_commutes3
#print axioms KoblitzRound22Recovery.actual_full_integer_lattice
#print axioms KoblitzRound22Recovery.actual_augmentation_binding
#print axioms KoblitzRound22Recovery.actual_regular_norm_zero
#print axioms KoblitzRound22Recovery.actual_rotation_binding
