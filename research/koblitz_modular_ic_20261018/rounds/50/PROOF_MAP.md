# R50 proposition-to-claim map

Accepted builds: lean50-01 (Rational50,6), separation50-03 (Separation50,1),
local-lean50-01 (Local50,3), marked-lean50-02 (Marked23_50,2). All have
compile.exit0 and audit.exit0:12 unique mathematical theorems,zero failing
or trusted declarations,no native_decide. Lean4.34.1/mathlib
d13f23b723b8a846827a245b89c10fc7d3f11612,-j1,-M4096,1200s per phase.

| Lean theorem | Use | Concrete hypotheses outside the theorem |
|---|---|---|
|Rational50.quadratic_norm_zero_iff|Quadratic elimination|Actual A,B,nonzero B and source-field map|
|Rational50.linear_coordinate_numerator|Clear n's denominator|Actual C,D and source relation|
|Rational50.zero_denominator_counterexample|Need a nonzero denominator|Generic counterexample,not source data|
|Rational50.bezout_norm_inverse|Norm-quotient inverse|Actual native Bezout arrays and characteristic2|
|Rational50.field_map_bijective_of_equal_finrank|Generic map bijectivity|Compatible maps,source field,target algebra and dimensions|
|Rational50.overlap_coordinate_roundtrip|Scaled overlap coordinates|Actual chart equations and inverse-unit identities|
|Separation50.evaluations_exclude_image|Integral element outside plane image|Actual separated degree2 evaluations|
|Local50.scaled_quadratic_identity|Monic integrality of Bprime*v|Actual source E and Bprime|
|Local50.bezout_intersection_reconstruction|Constructive CRT inclusion|Actual unit lift and module presentation|
|Local50.exact_eighth_power_nilpotency|Exact exponent8|Actual quotient,power normal forms and cyclic block|
|Marked23_50.binary_prefix23|Native binary prefix equals23P|Abstract commutative additive monoid,no coordinates|
|Marked23_50.exact_order23|Exact prime order|Actual nonsingular curve,group law,point and scalar identities|

This is a local scope map,not independent semantic review. Native arrays,
scheme gluing,complete pole/RR data,RR module generation and multiplication
tables are not formalized by these statements. Their native checks and
written implications are in the geometry notes. Rejected compiles
separation50-01,separation50-02,marked-lean50-01 have no successful audit and
add zero accepted theorems. Successful compiler/linter warnings are retained.
