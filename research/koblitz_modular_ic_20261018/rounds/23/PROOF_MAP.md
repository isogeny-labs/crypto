# Round23 claim-to-proof map

Pinned Lean4.34.1,mathlib d13f23b723b8a846827a245b89c10fc7d3f11612.
REPLAY_LEAN.sh recompiles every transitive project import from this checkout
before the repository AuditAxioms checker. Only propext,Classical.choice and
Quot.sound are allowed;no custom axiom,sorry/admit or trusted native evaluation.

| Claim | Declaration/native certificate | Exact scope |
|---|---|---|
| A unit colon element removes the extra source equation | mem_of_unit_mul_mem,mem_of_colon_unit,redundant_equation_of_colon_unit | Any commutative ring and ideal,with explicit unit/membership hypotheses. The actual h and its three coefficient polynomials are in SS23-certificate.sing;compact native replay verifies the original source identity. |
| Recover Tate from its cofactored residual | tate_equation_of_unit_cofactor | Explicit unit cofactor hypothesis;actual SS5 cofactor1 is checked in the first-host chart run and recorded there. |
| Actual SS5 inverse | ss5_unit_integer_lift,ss5_explicit_inverse,ss5_unit_is_unit | Literal universal integer identity and its specialization to every characteristic2 commutative ring satisfying the exact minpoly and two original quartics. No finite sampling or native_decide. |
| Relative derivative assembly | source_relative_jacobian_factor | Explicit base and QR derivative hypotheses. Compact native replay checks both against the original circuit. |
| Degree exhaustion | degree_exhaustion,ss5_degree_accounting,z5_degree_accounting | Arithmetic exhaustion,with the proper-curve degree formula supplied in CM.md. Not a formal scheme/DVR theorem. |
| SS5 denominator cancellation | ss5_denominator_value,quotient_unit_cancellation | Integer valuation consequences with values exposed as hypotheses. Native unit checks and the written smooth-curve argument supply the actual valuation bridge. |
| Z5 ramification and denominator values | z5_ramification_from_lengths,z5_separator_values | Conditional numerical implications. Z5-components gives exact local lengths;Z5-smooth gives unit minors of the actual closure. The smooth-curve/DVR length interpretation remains a written proof,not a formalized normalization. |

No Lean theorem asserts a modular-factor map,full normalized charts,complete RR,
node187 polynomial membership,complete quotient saturation or an arbitrary-divisor
reverse evaluator. First-host raw verification receipts were lost at migration;
the new source-bound replay/audit provides the retained verification status.

The retained second-host build/audit checks15 theorems,0 failing,0 trusted-native.
Its printed KoblitzRound23Local source SHA256 is
e6b9032ca95652ec4e06f2f454a9f6e0be44d183f4c70becf44f97352c6c436b,
which equals the committed module. A separately invoked leanchecker --help
probe did not return help and was stopped;no independent fresh-checker pass
is reported. The kernel compilation and transitive axiom audit did complete.
