import Init

/- Finite inertia and CM trace arithmetic only. The signature formula,
   Bouw bound and cyclotomic endomorphism argument are cited externally. -/
namespace GHSCyclic11FourBranch

def units : List Nat := (List.range 10).map (· + 1)
def fourth (a b c : Nat) : Nat := (11 - (a + b + c) % 11) % 11
def residueMinus (i e : Nat) : Nat := (11 - (i * e) % 11) % 11
def signature (a b c i : Nat) : Nat :=
  (residueMinus i a + residueMinus i b + residueMinus i c +
   residueMinus i (fourth a b c)) / 11 - 1
def firstOrbit : List Nat := [1, 3, 4, 5, 9]
def secondOrbit : List Nat := [2, 6, 7, 8, 10]
def rowsForA (a : Nat) : List (Nat × Nat) :=
  units.flatMap (fun b =>
    units.filterMap (fun c => if fourth a b c != 0 then some (b, c) else none))
def orbitMin (orbit : List Nat) (a b c : Nat) : Nat :=
  (orbit.map (signature a b c)).foldl min 2
def bound (a b c : Nat) : Nat :=
  5 * orbitMin firstOrbit a b c + 5 * orbitMin secondOrbit a b c
def paired (a b c : Nat) : Bool :=
  let d := fourth a b c
  decide (((a+b)%11=0 ∧ (c+d)%11=0) ∨
    ((a+c)%11=0 ∧ (b+d)%11=0) ∨
    ((a+d)%11=0 ∧ (b+c)%11=0))

def summaryForA (a : Nat) : Nat × Nat × Nat × Nat × Bool :=
  let rs := rowsForA a
  (rs.length,
   (rs.filter (fun x => bound a x.1 x.2 = 0)).length,
   (rs.filter (fun x => bound a x.1 x.2 = 5)).length,
   (rs.filter (fun x => bound a x.1 x.2 = 10)).length,
   rs.all (fun x => decide (bound a x.1 x.2 = 10) == paired a x.1 x.2))

theorem slice_one : summaryForA 1 = (91,28,36,27,true) := by decide
theorem slice_two : summaryForA 2 = (91,28,36,27,true) := by decide
theorem slice_three : summaryForA 3 = (91,28,36,27,true) := by decide
theorem slice_four : summaryForA 4 = (91,28,36,27,true) := by decide
theorem slice_five : summaryForA 5 = (91,28,36,27,true) := by decide
theorem slice_six : summaryForA 6 = (91,28,36,27,true) := by decide
theorem slice_seven : summaryForA 7 = (91,28,36,27,true) := by decide
theorem slice_eight : summaryForA 8 = (91,28,36,27,true) := by decide
theorem slice_nine : summaryForA 9 = (91,28,36,27,true) := by decide
theorem slice_ten : summaryForA 10 = (91,28,36,27,true) := by decide

theorem all_slices : units.map summaryForA = List.replicate 10 (91,28,36,27,true) := by
  decide

theorem aggregate_arithmetic :
    10*91=910 ∧ 10*28=280 ∧ 10*36=360 ∧ 10*27=270 := by decide

def cmTraceRows (p cap mCap : Nat) : List (Int × Nat) :=
  ((List.range (2*cap+1)).map (fun n => Int.ofNat n - Int.ofNat cap)).flatMap
    (fun t => ((List.range mCap).map Nat.succ).filterMap (fun m =>
      if t*t + 11*Int.ofNat (m*m) = Int.ofNat (4*p^5) ∧
         t % Int.ofNat p ≠ 0 then some (t,m) else none))

theorem cm_trace_three : cmTraceRows 3 32 10 = [(-31,1),(31,1)] := by decide
theorem cm_trace_five : cmTraceRows 5 112 35 = [(-57,29),(57,29)] := by decide
theorem cm_enumeration_caps :
    4*3^5 < 32^2 ∧ 4*5^5 < 112^2 ∧
    4*3^5 < 11*10^2 ∧ 4*5^5 < 11*35^2 := by decide

def fifthTrace (s p : Int) : Int := s^5 - 5*p*s^3 + 5*p^2*s
theorem cm_three_descended :
    fifthTrace (-1) 3 = -31 ∧ fifthTrace 1 3 = 31 := by decide
theorem cm_five_descended :
    fifthTrace (-3) 5 = 57 ∧ fifthTrace 3 5 = -57 := by decide

/- These checks consume the native character-count outputs as fixed inputs.
   They do not certify the finite-field count or quotient geometry. -/
def f243Trace (n : Nat) : Int := (240 - 11 * Int.ofNat n) / 10
def f3125Trace (n : Nat) : Int := (3122 - 11 * Int.ofNat n) / 10
theorem rational_f243_trace_checks :
    f243Trace 0 = 24 ∧ f243Trace 20 = 2 ∧ f243Trace 30 = -9 ∧
    fifthTrace 2 3 = 2 := by decide
theorem rational_f3125_trace_checks :
    f3125Trace 282 = 2 ∧ f3125Trace 262 = 24 ∧
    2 % 5 ≠ 0 ∧ 24 % 5 ≠ 0 ∧
    2 * 2 < 4 * 3125 ∧ 24 * 24 < 4 * 3125 := by decide
theorem rational_f3125_no_prime_field_descent :
    ((List.range 9).map (fun n => Int.ofNat n - 4)).all
      (fun s => decide (fifthTrace s 5 ≠ 2 ∧ fifthTrace s 5 ≠ 24)) = true := by decide
theorem rational_f3125_source_orders :
    3125 + 1 - 2 = 3124 ∧ 3125 + 1 - 24 = 3102 := by decide
theorem discriminant_prime_checks :
    (2 : Int)^2 - 4*3125 = -12496 ∧
    (24 : Int)^2 - 4*3125 = -11924 ∧
    (-12496 : Int) % 71 = 0 ∧
    (-11924 : Int) % 47 = 14 := by decide

#print axioms slice_one
#print axioms slice_ten
#print axioms all_slices
#print axioms aggregate_arithmetic
#print axioms cm_trace_three
#print axioms cm_trace_five
#print axioms cm_enumeration_caps
#print axioms cm_three_descended
#print axioms cm_five_descended
#print axioms rational_f243_trace_checks
#print axioms rational_f3125_trace_checks
#print axioms rational_f3125_no_prime_field_descent
#print axioms rational_f3125_source_orders
#print axioms discriminant_prime_checks

end GHSCyclic11FourBranch
