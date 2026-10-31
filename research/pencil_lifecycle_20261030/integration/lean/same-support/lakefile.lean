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
