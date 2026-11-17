import KoblitzRound14Relations
set_option maxRecDepth 100000
set_option maxHeartbeats 0
namespace KoblitzRound14Cusps
theorem tate_determinant_residual {R : Type*} [CommRing R] (a b c d i : R) : (2*a)*(i*d)-b*c-(a*d-b*c)=a*d*(2*i-1) := by ring
theorem tate_normalized_determinant {R : Type*} [CommRing R] (a b c d i : R) (h : 2*i=1) : (2*a)*(i*d)-b*c=a*d-b*c := by
 have h' := tate_determinant_residual a b c d i
 rw [h,sub_self,mul_zero] at h'
 exact sub_eq_zero.mp h'
abbrev Point := Option (Nat × Nat)
def step8 (a : Nat) := if a.testBit 2 then Nat.xor ((a<<<1)%8) 3 else (a<<<1)%8
def mul8Aux : Nat → Nat → Nat → Nat → Nat
 | 0,_,_,z => z
 | n+1,a,b,z => mul8Aux n (step8 a) (b>>>1) (if b.testBit 0 then Nat.xor z a else z)
def mul8 (a b : Nat) := mul8Aux 3 a b 0
def frob8 : Point → Point
 | none => none
 | some (x,y) => some (mul8 x x,mul8 y y)
def valid8 : Point → Bool
 | none => true
 | some (x,y) => (x<8 && y<8) && (Nat.xor (mul8 y y) (mul8 x y)==Nat.xor (Nat.xor (mul8 (mul8 x x) x) (mul8 x x)) 1)
def norm (b d : Nat) : Nat × Nat :=
 let nb := (11-b)%11
 let nd := (11-d)%11
 if b<nb || (b==nb && d≤nd) then (b,d) else (nb,nd)
def bases : List Point := [none,some (0,1),some (2,7),some (4,3),some (3,3),some (6,5),some (7,7),some (5,5)]
def baseNext : List Nat := [0,1,3,5,7,2,4,6]
def labels : List (Nat × Nat) := [(0,1),(0,2),(0,3),(0,4),(0,5),(1,0),(1,1),(1,2),(1,3),(1,4),(1,5),(1,6),(1,7),(1,8),(1,9),(1,10),(2,0),(2,1),(2,2),(2,3),(2,4),(2,5),(2,6),(2,7),(2,8),(2,9),(2,10),(3,0),(3,1),(3,2),(3,3),(3,4),(3,5),(3,6),(3,7),(3,8),(3,9),(3,10),(4,0),(4,1),(4,2),(4,3),(4,4),(4,5),(4,6),(4,7),(4,8),(4,9),(4,10),(5,0),(5,1),(5,2),(5,3),(5,4),(5,5),(5,6),(5,7),(5,8),(5,9),(5,10)]
def nativePermutation : List Nat := [4,0,3,1,2,5,11,6,12,7,13,8,14,9,15,10,16,22,17,23,18,24,19,25,20,26,21,27,33,28,34,29,35,30,36,31,37,32,38,44,39,45,40,46,41,47,42,48,43,49,55,50,56,51,57,52,58,53,59,54,64,60,63,61,62,65,71,66,72,67,73,68,74,69,75,70,76,82,77,83,78,84,79,85,80,86,81,87,93,88,94,89,95,90,96,91,97,92,98,104,99,105,100,106,101,107,102,108,103,109,115,110,116,111,117,112,118,113,119,114,184,180,183,181,182,185,191,186,192,187,193,188,194,189,195,190,196,202,197,203,198,204,199,205,200,206,201,207,213,208,214,209,215,210,216,211,217,212,218,224,219,225,220,226,221,227,222,228,223,229,235,230,236,231,237,232,238,233,239,234,304,300,303,301,302,305,311,306,312,307,313,308,314,309,315,310,316,322,317,323,318,324,319,325,320,326,321,327,333,328,334,329,335,330,336,331,337,332,338,344,339,345,340,346,341,347,342,348,343,349,355,350,356,351,357,352,358,353,359,354,424,420,423,421,422,425,431,426,432,427,433,428,434,429,435,430,436,442,437,443,438,444,439,445,440,446,441,447,453,448,454,449,455,450,456,451,457,452,458,464,459,465,460,466,461,467,462,468,463,469,475,470,476,471,477,472,478,473,479,474,124,120,123,121,122,125,131,126,132,127,133,128,134,129,135,130,136,142,137,143,138,144,139,145,140,146,141,147,153,148,154,149,155,150,156,151,157,152,158,164,159,165,160,166,161,167,162,168,163,169,175,170,176,171,177,172,178,173,179,174,244,240,243,241,242,245,251,246,252,247,253,248,254,249,255,250,256,262,257,263,258,264,259,265,260,266,261,267,273,268,274,269,275,270,276,271,277,272,278,284,279,285,280,286,281,287,282,288,283,289,295,290,296,291,297,292,298,293,299,294,364,360,363,361,362,365,371,366,372,367,373,368,374,369,375,370,376,382,377,383,378,384,379,385,380,386,381,387,393,388,394,389,395,390,396,391,397,392,398,404,399,405,400,406,401,407,402,408,403,409,415,410,416,411,417,412,418,413,419,414]
def orbits : List (List Nat) := [[0,4,2,3,1],[5],[6,11,8,12,14,15,10,13,9,7],[16],[17,22,19,23,25,26,21,24,20,18],[27],[28,33,30,34,36,37,32,35,31,29],[38],[39,44,41,45,47,48,43,46,42,40],[49],[50,55,52,56,58,59,54,57,53,51],[60,64,62,63,61],[65],[66,71,68,72,74,75,70,73,69,67],[76],[77,82,79,83,85,86,81,84,80,78],[87],[88,93,90,94,96,97,92,95,91,89],[98],[99,104,101,105,107,108,103,106,102,100],[109],[110,115,112,116,118,119,114,117,113,111],[120,184,302,123,181,300,124,182,303,121,180,304,122,183,301],[125,185,305],[126,191,308,132,194,315,130,193,309,127,186,311,128,192,314,135,190,313,129,187,306,131,188,312,134,195,310,133,189,307],[136,196,316],[137,202,319,143,205,326,141,204,320,138,197,322,139,203,325,146,201,324,140,198,317,142,199,323,145,206,321,144,200,318],[147,207,327],[148,213,330,154,216,337,152,215,331,149,208,333,150,214,336,157,212,335,151,209,328,153,210,334,156,217,332,155,211,329],[158,218,338],[159,224,341,165,227,348,163,226,342,160,219,344,161,225,347,168,223,346,162,220,339,164,221,345,167,228,343,166,222,340],[169,229,349],[170,235,352,176,238,359,174,237,353,171,230,355,172,236,358,179,234,357,173,231,350,175,232,356,178,239,354,177,233,351],[240,424,362,243,421,360,244,422,363,241,420,364,242,423,361],[245,425,365],[246,431,368,252,434,375,250,433,369,247,426,371,248,432,374,255,430,373,249,427,366,251,428,372,254,435,370,253,429,367],[256,436,376],[257,442,379,263,445,386,261,444,380,258,437,382,259,443,385,266,441,384,260,438,377,262,439,383,265,446,381,264,440,378],[267,447,387],[268,453,390,274,456,397,272,455,391,269,448,393,270,454,396,277,452,395,271,449,388,273,450,394,276,457,392,275,451,389],[278,458,398],[279,464,401,285,467,408,283,466,402,280,459,404,281,465,407,288,463,406,282,460,399,284,461,405,287,468,403,286,462,400],[289,469,409],[290,475,412,296,478,419,294,477,413,291,470,415,292,476,418,299,474,417,293,471,410,295,472,416,298,479,414,297,473,411]]
def degrees : List Int := [5,1,10,1,10,1,10,1,10,1,10,5,1,10,1,10,1,10,1,10,1,10,15,3,30,3,30,3,30,3,30,3,30,15,3,30,3,30,3,30,3,30,3,30]
def cuspFrob (i : Nat) : Nat :=
 let v := labels[i%60]!
 baseNext[i/60]!*60+labels.idxOf (norm v.1 (6*v.2%11))
theorem actual_base_curve_and_frobenius : ((bases.all valid8) && ((List.range 8).all fun i => frob8 bases[i]! == bases[baseNext[i]!]!))=true := by decide +kernel
theorem actual_normalized_permutation : ((List.range 480).all fun i => cuspFrob i==nativePermutation[i]!)=true := by decide +kernel
theorem actual_orbits_cover : orbits.flatten.length=480 ∧ ((List.range 480).all fun i => (orbits.flatten.filter (fun j => j==i)).length==1)=true := by decide +kernel
theorem actual_orbits_frobenius : (orbits.all fun o => !o.isEmpty && ((o.zip (o.drop 1 ++ [o[0]!])).all fun ij => cuspFrob ij.1==ij.2))=true := by decide +kernel
theorem actual_degree_histogram : (([(1,10),(3,10),(5,2),(10,10),(15,2),(30,10)] : List (Nat × Nat)).all fun dn => (orbits.filter (fun o => o.length==dn.1)).length==dn.2)=true := by decide +kernel
def multiplicities0 : List Int := [-242,-242,-2,-242,-2,-242,-2,-242,-2,-242,-2,2,2,2,2,2,2,2,2,2,2,2,0,0,1,0,1,0,1,0,1,0,1,0,0,1,0,1,0,1,0,1,0,1]
theorem divisor0_signed_degree : (degrees.zipWith (fun d m => d*m) multiplicities0).sum+2100=0 := by decide +kernel
def multiplicities1 : List Int := [-242,-242,-2,-242,-2,-242,-2,-242,-2,-242,-2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
theorem divisor1_signed_degree : (degrees.zipWith (fun d m => d*m) multiplicities1).sum+2520=0 := by decide +kernel
def multiplicities2 : List Int := [-363,-363,-3,-363,-3,-363,-3,-363,-3,-363,-3,1,1,1,1,1,1,1,1,1,1,1,0,0,1,0,1,0,1,0,1,0,1,0,0,1,0,1,0,1,0,1,0,1]
theorem divisor2_signed_degree : (degrees.zipWith (fun d m => d*m) multiplicities2).sum+3420=0 := by decide +kernel
def multiplicities3 : List Int := [-363,-363,-3,-363,-3,-363,-3,-363,-3,-363,-3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
theorem divisor3_signed_degree : (degrees.zipWith (fun d m => d*m) multiplicities3).sum+3780=0 := by decide +kernel
def multiplicities4 : List Int := [-244,-244,-4,-244,-4,-244,-4,-244,-4,-244,-4,244,244,4,244,4,244,4,244,4,244,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
theorem divisor4_signed_degree : (degrees.zipWith (fun d m => d*m) multiplicities4).sum+0=0 := by decide +kernel
def multiplicities5 : List Int := [-244,-244,-4,-244,-4,-244,-4,-244,-4,-244,-4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
theorem divisor5_signed_degree : (degrees.zipWith (fun d m => d*m) multiplicities5).sum+2640=0 := by decide +kernel
def multiplicities6 : List Int := [-365,-365,-5,-365,-5,-365,-5,-365,-5,-365,-5,2,2,2,2,2,2,2,2,2,2,2,0,0,1,0,1,0,1,0,1,0,1,1,1,1,1,1,1,1,1,1,1,1]
theorem divisor6_signed_degree : (degrees.zipWith (fun d m => d*m) multiplicities6).sum+3450=0 := by decide +kernel
def multiplicities7 : List Int := [-365,-365,-5,-365,-5,-365,-5,-365,-5,-365,-5,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
theorem divisor7_signed_degree : (degrees.zipWith (fun d m => d*m) multiplicities7).sum+3900=0 := by decide +kernel
theorem old_product_closed_control : (multiplicities4.filter (fun m => m != 0)).length=22 ∧ ((degrees.zip multiplicities4).foldl (fun s dm => if 0<dm.2 then s+dm.1*dm.2 else s) 0)=2640 ∧ ((degrees.zip multiplicities4).foldl (fun s dm => if dm.2<0 then s+dm.1*dm.2 else s) 0)= -2640 := by decide +kernel
end KoblitzRound14Cusps
#print axioms KoblitzRound14Cusps.tate_determinant_residual
#print axioms KoblitzRound14Cusps.tate_normalized_determinant
#print axioms KoblitzRound14Cusps.actual_base_curve_and_frobenius
#print axioms KoblitzRound14Cusps.actual_normalized_permutation
#print axioms KoblitzRound14Cusps.actual_orbits_cover
#print axioms KoblitzRound14Cusps.actual_orbits_frobenius
#print axioms KoblitzRound14Cusps.actual_degree_histogram
#print axioms KoblitzRound14Cusps.divisor0_signed_degree
#print axioms KoblitzRound14Cusps.divisor1_signed_degree
#print axioms KoblitzRound14Cusps.divisor2_signed_degree
#print axioms KoblitzRound14Cusps.divisor3_signed_degree
#print axioms KoblitzRound14Cusps.divisor4_signed_degree
#print axioms KoblitzRound14Cusps.divisor5_signed_degree
#print axioms KoblitzRound14Cusps.divisor6_signed_degree
#print axioms KoblitzRound14Cusps.divisor7_signed_degree
#print axioms KoblitzRound14Cusps.old_product_closed_control
