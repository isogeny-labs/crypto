import CMRecovery42

set_option maxRecDepth 100000

set_option maxHeartbeats 0

namespace CMIntegerWitness42

theorem n5_primitive_witness :
    (1 : ℤ)*(22 : ℤ) + (-7 : ℤ)*(3 : ℤ) = 1 := by decide

theorem n11_primitive_witness :
    (-3 : ℤ)*(506 : ℤ) + (-49 : ℤ)*(-31 : ℤ) = 1 := by decide

theorem n83_primitive_witness :
    (-235927425229 : ℤ)*(12302938186694 : ℤ) + (3293831049789 : ℤ)*(881223258043 : ℤ) = 1 := by decide

theorem n131_primitive_witness :
    (167436349564251576440 : ℤ)*(10095126166020618049226 : ℤ) + (3587427483850483898103 : ℤ)*(-471170798918788296713 : ℤ) = 1 := by decide

theorem target_unit_1 : (506 : ℤ)*(472 : ℤ)+(991 : ℤ)*(-241 : ℤ)=1 := by decide

theorem target_unit_2 : (12302938186694 : ℤ)*(428818359078177785614 : ℤ)+(2417851639230796216685689 : ℤ)*(-2181989035 : ℤ)=1 := by decide

theorem target_unit_3 : (10095126166020618049226 : ℤ)*(57113890985849303318556078924674977460 : ℤ)+(680564733841876926932320129493409985129 : ℤ)*(-847196316035467244271 : ℤ)=1 := by decide

end CMIntegerWitness42
