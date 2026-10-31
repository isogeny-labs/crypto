import Lake
open Lake DSL

package same_support

require mathlib from git
  "https://github.com/leanprover-community/mathlib4.git" @ "d13f23b723b8a846827a245b89c10fc7d3f11612"

@[default_target]
lean_lib ProjectiveLawBoundary

@[default_target]
lean_lib RuntimeSemantics

@[default_target]
lean_lib SameSupport

@[default_target]
lean_lib SameSupportClassifier

@[default_target]
lean_lib WeightedSameSupport

@[default_target]
lean_lib CanonicalResidue

@[default_target]
lean_lib UInt64Residue

@[default_target]
lean_lib InvariantBranches

@[default_target]
lean_lib NatClassifier

@[default_target]
lean_lib WordClassifier

@[default_target]
lean_lib CrossScaling

@[default_target]
lean_lib WordErrorExclusion

@[default_target]
lean_lib Constructor

@[default_target]
lean_lib Add11
