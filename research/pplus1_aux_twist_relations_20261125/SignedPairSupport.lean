import Init

/-! The nine formal signed-pair classes for two factor-base columns.
    The point-level distinctness check is in native/src/verify.rs. -/

private def signedIndices : List Nat := [0, 1, 2, 3]

private def signedCoefficient : Nat → Int × Int
  | 0 => (-1, 0)
  | 1 => (1, 0)
  | 2 => (0, -1)
  | _ => (0, 1)

private def signedPairCoefficients : List (Int × Int) :=
  signedIndices.flatMap fun i =>
    signedIndices.map fun j =>
      let u := signedCoefficient i
      let v := signedCoefficient j
      (u.1 + v.1, u.2 + v.2)

theorem signed_pair_coefficient_classes :
    signedPairCoefficients.eraseDups.length = 9 := by
  decide

#print axioms signed_pair_coefficient_classes
