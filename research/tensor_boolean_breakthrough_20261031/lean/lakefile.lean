import Lake

open Lake DSL

require CryptoLean from git
  "https://github.com/aburan28/crypto-lean" @
    "148f5dc389b341401e06d76abe7783c4ccf42f2e"

require atlas from git
  "https://github.com/facebookresearch/atlas-lean" @
    "a4557099d45c620a038e90a9c4b4d9ab08294b81"

require ECDLP from git
  "https://github.com/kaarelkaarelson/fast-ecdlp" @
    "d1deb5abe008e981206ed58cbd83012b6afc3740" / "lean"

require theorem_refutations from "../../theorem_census_20261109/lean"
require semaev_symmetry_audit from "../../semaev_symmetry_audit_20261113/lean"

require mathlib from git
  "https://github.com/leanprover-community/mathlib4" @
    "cfc55fc58714a59dc06e6e79edc6d1d95140d3c2"

package tensor_boundary_bounds where
  version := v!"0.1.0"

private def zsmulOverlayFiles : Array System.FilePath := #[
  "NumberTheory/EllipticDivisibilitySequence.lean",
  "AlgebraicGeometry/EllipticCurve/DivisionPolynomial/Basic.lean",
  "AlgebraicGeometry/EllipticCurve/Universal.lean",
  "AlgebraicGeometry/EllipticCurve/DivisionPolynomial/Omega.lean",
  "AlgebraicGeometry/EllipticCurve/DivisionPolynomial/ZSMul.lean"
]

private def installZsmulOverlay (projectRoot : System.FilePath) : IO Unit := do
  let sourceRoot := projectRoot / ".." / "lean-zsmul-overlay" / "Mathlib"
  let targetRoot := projectRoot / ".lake" / "packages" / "mathlib" / "Mathlib"
  if ← targetRoot.pathExists then
    for relative in zsmulOverlayFiles do
      let source := sourceRoot /
        System.FilePath.mk (relative.toString ++ ".overlay")
      let destination := targetRoot / relative
      match destination.parent with
      | some parent => IO.FS.createDirAll parent
      | none => pure ()
      IO.FS.writeFile destination (← IO.FS.readFile source)

/-- Install the rebased division-polynomial sources before every Lake action. -/
initialize installZsmulOverlayOnLoad : Unit ← do
  installZsmulOverlay (← IO.Process.getCurrentDir)

/-- Install the same sources after dependency materialization on `lake update`. -/
post_update pkg do
  installZsmulOverlay pkg.dir

@[default_target]
lean_lib TensorBoundaryBounds where
  roots := #[`TensorBoundaryBounds, `DilationScaleSpecialization,
    `DilationScaleDegree, `DilationScaleExplicitDegree,
    `RecursiveSemaevNaturality, `FiniteFieldSemaevVanishing,
    `DilationSemaevBridge, `DilationNonzeroBounds, `SemaevRelationSigns]
