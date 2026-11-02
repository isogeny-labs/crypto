# R73 claim map

The accepted native run supplies the complete `F32` square columns,
relative Frobenius matrix, two stable-image iterations, all 32 fixed
dimensions and degree-profile reconstruction. R52 supplies the full
source table, R61 the source unit, R70 normality and R65 the trace
Gram discriminant. `results.json` binds native and Lean attempts to
their frozen bytes, including three rejected attempts.

The geometric deduction that the degree-five fiber is tame uses the
standard different lower bound, equality criterion and discriminant
identity. The three Lean theorems prove only the stated arithmetic
steps under explicit premises; they do not import or replace the
geometric theorem or native source identities.
