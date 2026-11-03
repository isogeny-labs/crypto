/- Kernel-checked finite witnesses for the scalar E[3] route over F_4096.
   The generic Frobenius-module and polarization theorems are argued in REPORT.md;
   this file checks the displayed mixed kernel, not quotient equations. -/

namespace ScalarKernel

private def b0 : List Nat := [1, 0, 0, 0, 2, 0]
private def b1 : List Nat := [0, 1, 0, 2, 0, 1]
private def b2 : List Nat := [0, 0, 1, 0, 1, 0]
private def basis : List (List Nat) := [b0, b1, b2]

private def pairing (v w : List Nat) : Nat :=
  (v[0]! * w[1]! + 2 * v[1]! * w[0]! +
   v[2]! * w[3]! + 2 * v[3]! * w[2]! +
   v[4]! * w[5]! + 2 * v[5]! * w[4]!) % 3

-- Frobenius on E[3]^3 when the cubic Frobenius is -I.
private def transfer (v : List Nat) : List Nat :=
  [(2 * v[4]!) % 3, (2 * v[5]!) % 3,
   v[0]!, v[1]!, v[2]!, v[3]!]

private def combination (a b c : Nat) : List Nat :=
  (List.range 6).map fun i =>
    (a * b0[i]! + b * b1[i]! + c * b2[i]!) % 3

private def belongs (v : List Nat) : Bool :=
  (List.range 3).any fun a =>
    (List.range 3).any fun b =>
      (List.range 3).any fun c => combination a b c == v

private def isSingleFactor (v : List Nat) : Bool :=
  (v[2]! == 0 && v[3]! == 0 && v[4]! == 0 && v[5]! == 0) ||
  (v[0]! == 0 && v[1]! == 0 && v[4]! == 0 && v[5]! == 0) ||
  (v[0]! == 0 && v[1]! == 0 && v[2]! == 0 && v[3]! == 0)

theorem mixed_kernel_isotropic :
    (basis.all fun v => basis.all fun w => pairing v w == 0) = true := by decide

theorem mixed_kernel_frobenius_stable :
    (basis.all fun v => belongs (transfer v)) = true := by decide

theorem mixed_kernel_rank_three_pivots :
    [b0[0]!, b0[1]!, b0[2]!,
     b1[0]!, b1[1]!, b1[2]!,
     b2[0]!, b2[1]!, b2[2]!] =
    [1, 0, 0, 0, 1, 0, 0, 0, 1] := by decide

theorem mixed_kernel_meets_no_factor :
    ((List.range 3).all fun a =>
      (List.range 3).all fun b =>
        (List.range 3).all fun c =>
          (a == 0 && b == 0 && c == 0) ||
          !(isSingleFactor (combination a b c))) = true := by decide

theorem q4096_scalar_parameter_count : (4096 - 4) / 12 = 341 := by decide
theorem q4096_scalar_model_count : 2 * 341 = 682 := by decide
theorem q4096_root_fiber_count : 1024 + 2730 + 341 = 4095 := by decide
theorem q4096_root_incidence_count : 2730 + 4 * 341 = 4094 := by decide
theorem q4096_pair_incidence_count : 12 * 341 = 4092 := by decide

end ScalarKernel
