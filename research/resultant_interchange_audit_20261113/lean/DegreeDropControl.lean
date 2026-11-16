import Definitions

namespace ResultantInterchangeAudit
open Polynomial
noncomputable section

def controlP : Polynomial ℚ := X^2-X
def controlQ : Polynomial (Polynomial ℚ) := C X*X+1
def controlR : Polynomial ℚ := 2*X+1

theorem controlP_degree : controlP.natDegree=2 := by
  unfold controlP
  compute_degree
  all_goals norm_num

theorem controlR_degree : controlR.natDegree=1 := by
  unfold controlR
  compute_degree
  all_goals norm_num

theorem control_innerY : innerY controlP controlQ 1=X+1 := by
  unfold innerY
  rw [controlP_degree]
  unfold Polynomial.resultant
  rw [Matrix.det_fin_three]
  norm_num [lift,controlP,controlQ,Polynomial.sylvester,Fin.addCases,coeff_X,coeff_one]
  all_goals ring

theorem control_innerX : innerX controlQ controlR 1=X-2 := by
  unfold innerX
  rw [controlR_degree]
  unfold Polynomial.resultant
  rw [Matrix.det_fin_two]
  norm_num [lift,controlR,controlQ,Polynomial.Bivariate.swap,Polynomial.sylvester,Fin.addCases,coeff_X,coeff_one]

theorem controlQ_swap : Polynomial.Bivariate.swap controlQ = controlQ := by
  simp only [controlQ, map_add, map_mul, Polynomial.Bivariate.swap_C,
    Polynomial.Bivariate.swap_Y, map_one, Polynomial.map_X]
  ring

theorem controlQ_degree : controlQ.natDegree = 1 := by
  unfold controlQ
  compute_degree
  all_goals simp

theorem control_bidegrees : controlQ.natDegree = 1 ∧
    (Polynomial.Bivariate.swap controlQ).natDegree = 1 := by
  rw [controlQ_swap, controlQ_degree]
  exact ⟨rfl, rfl⟩

theorem control_actual_degree_drop :
    (innerY controlP controlQ 1).natDegree < controlP.natDegree * 1 := by
  rw [control_innerY, controlP_degree]
  have hd : (X+1 : Polynomial ℚ).natDegree=1 := by compute_degree; all_goals norm_num
  rw [hd]
  norm_num

theorem control_natural_left : leftNatural controlP controlQ controlR 1= -1 := by
  rw [leftNatural,control_innerY]
  have hd : (X+1 : Polynomial ℚ).natDegree=1 := by compute_degree; all_goals norm_num
  rw [hd,controlR_degree]
  unfold Polynomial.resultant
  rw [Matrix.det_fin_two]
  norm_num [controlR,Polynomial.sylvester,Fin.addCases,coeff_X,coeff_one]

theorem control_natural_right : rightNatural controlP controlQ controlR 1=2 := by
  rw [rightNatural,control_innerX,controlP_degree]
  have hd : (X-2 : Polynomial ℚ).natDegree=1 := by compute_degree; all_goals norm_num
  rw [hd]
  unfold Polynomial.resultant
  rw [Matrix.det_fin_three]
  norm_num [controlP,Polynomial.sylvester,Fin.addCases,coeff_X,coeff_one]

theorem control_fixed_left : leftFixed controlP controlQ controlR 1 1=2 := by
  rw [leftFixed,control_innerY,controlP_degree,controlR_degree]
  unfold Polynomial.resultant
  rw [Matrix.det_fin_three]
  norm_num [controlR,Polynomial.sylvester,Fin.addCases,coeff_X,coeff_one]

theorem control_fixed_right : rightFixed controlP controlQ controlR 1 1=2 := by
  rw [rightFixed,control_innerX,controlP_degree,controlR_degree]
  unfold Polynomial.resultant
  rw [Matrix.det_fin_three]
  norm_num [controlP,Polynomial.sylvester,Fin.addCases,coeff_X,coeff_one]

theorem natural_interchange_counterexample :
    leftNatural controlP controlQ controlR 1≠rightNatural controlP controlQ controlR 1 ∧
    leftFixed controlP controlQ controlR 1 1=rightFixed controlP controlQ controlR 1 1 := by
  rw [control_natural_left,control_natural_right,control_fixed_left,control_fixed_right]
  norm_num

#print axioms natural_interchange_counterexample

end
end ResultantInterchangeAudit
