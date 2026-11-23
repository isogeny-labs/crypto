# Frozen model and arithmetic proof contexts

PCC-20261116-76dc04 binds complete `ScalingAudit.lean`,
`ParameterArithmetic.lean` and `lake-manifest.json` bytes, plus the ten
named declaration bindings and the maintained gate audit. Preserve them;
future results use new versioned modules/contexts and claims.

The gate audits twelve declarations: ten named endpoints and two generated
definition-equation theorems. It reports zero failures and zero trusted-native
exceptions. Rational endpoints use only propext, Classical.choice and
Quot.sound; the four Core natural-number endpoints use no axioms.
No actual solver, distribution, real-power asymptotics, point count or
growing-family admissibility is certified by these proofs. [pcc:unverified]
