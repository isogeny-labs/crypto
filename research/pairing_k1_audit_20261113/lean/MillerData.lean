import ActualQuotient

namespace PairingKOneAudit

def millerLine : curve.Point → curve.Point → F
  | .zero,_ => 1
  | .some _ _ _,.zero => 0
  | .some xp yp _,.some xr yr _ => yr-yp-((3*xp^2+3)/(2*yp))*(xr-xp)

def tangentDenominator : curve.Point → F
  | .zero => 1
  | .some _ y _ => 2*y

def goodAux (U V S : curve.Point) : Prop := millerLine U S≠0 ∧ millerLine U (V+S)≠0
instance goodAuxDecidable (U V S : curve.Point) : Decidable (goodAux U V S) := by
  unfold goodAux
  infer_instance

/-- Deterministic finite support search; correctness on all actual inputs is proved below. -/
def auxiliary (U V : curve.Point) : curve.Point :=
  (((List.range 9).map (fun n => indexedPoint (n:ZMod 9))).find?
    (fun S => decide (goodAux U V S))).getD G

def reducedTateMiller (U V : curve.Point) : F :=
  (millerLine U (V+auxiliary U V)/millerLine U (auxiliary U V))^2

/-- The orientation of Galbraith26.1, with the fixed actual auxiliary G. -/
def weilMiller (U V : curve.Point) : F :=
  (millerLine V (U+G)/millerLine V G)*(millerLine U (-G)/millerLine U (V-G))

def indexValue (u v : ZMod 3) : F := (2:F)^(u*v).val

theorem torsion_tangent_denominator : ∀ u : ZMod 3,tangentDenominator (torsionPoint u)≠0 := by
  decide +kernel

theorem chosen_auxiliary_support : ∀ (u : ZMod 3) (j : ZMod 9),
    goodAux (torsionPoint u) (indexedPoint j)
      (auxiliary (torsionPoint u) (indexedPoint j)) := by
  decide +kernel

theorem all_valid_auxiliary_values : ∀ (u : ZMod 3) (j s : ZMod 9),
    goodAux (torsionPoint u) (indexedPoint j) (indexedPoint s) →
      (millerLine (torsionPoint u) (indexedPoint j+indexedPoint s)/
        millerLine (torsionPoint u) (indexedPoint s))^2=indexValue u (reduce9 j) := by
  decide +kernel

theorem tate_table : ∀ (u : ZMod 3) (j : ZMod 9),
    reducedTateMiller (torsionPoint u) (indexedPoint j)=indexValue u (reduce9 j) := by
  decide +kernel

theorem weil_support_table : ∀ u v : ZMod 3,
    millerLine (torsionPoint v) (torsionPoint u+G)≠0 ∧
    millerLine (torsionPoint v) G≠0 ∧ millerLine (torsionPoint u) (-G)≠0 ∧
    millerLine (torsionPoint u) (torsionPoint v-G)≠0 := by
  decide +kernel

theorem weil_table : ∀ u v : ZMod 3,weilMiller (torsionPoint u) (torsionPoint v)=1 := by
  decide +kernel

theorem tate_generator_value : reducedTateMiller ((3:ℕ) • G) G=2 := by
  decide +kernel

theorem two_primitive_cube_root : IsPrimitiveRoot (2:F) 3 := by
  apply (IsPrimitiveRoot.iff (by decide : 0<3)).mpr
  constructor
  · decide +kernel
  · intro m hm hn
    interval_cases m <;> decide +kernel

theorem indexValue_left_add : ∀ u v w : ZMod 3,
    indexValue (u+v) w=indexValue u w*indexValue v w := by decide +kernel

theorem indexValue_right_add : ∀ u v w : ZMod 3,
    indexValue u (v+w)=indexValue u v*indexValue u w := by decide +kernel

theorem indexValue_one_iff : ∀ u : ZMod 3,indexValue u 1=1 ↔ u=0 := by decide +kernel
theorem indexValue_one_left_iff : ∀ u : ZMod 3,indexValue 1 u=1 ↔ u=0 := by decide +kernel
theorem indexValue_cube : ∀ u v : ZMod 3,(indexValue u v)^3=1 := by decide +kernel
theorem indexValue_nonzero : ∀ u v : ZMod 3,indexValue u v≠0 := by decide +kernel

#print axioms all_valid_auxiliary_values
#print axioms tate_table
#print axioms weil_table
#print axioms tate_generator_value
end PairingKOneAudit
