# Explicit projector follow-up, frozen before code

The field-degree gate is terminal: positive genus2/3/4cases need degrees8/13/80
respectively. Preserve ORDERS.txtand prior trace outputs. For EACHof the same
three positive regular cases, its characteristic polynomial mod3is f*g with
distinct dual irreducibles. Compute e_f=g*(g^(-1)modf)mod(f*g), e_g=1−e_f.
Verify both idempotent, complementary, orthogonal; f*e_f=0andg*e_g=0modchi.
Emit explicit coefficient vectors for both. This is exact Chinese-remainder
algebra, not a novel isogeny algorithm or constructed Mumford kernel point.

These operators apply ONLYto J[3]. Applying the same integer polynomial in
Frobenius to arbitrary target/subgroup points is not a3torsion projection.
A separate source of3torsion over the certified extension remains required;
no public DLPscalar is used or inferred. All three cases retained, no new
curve selection, point-count rerun, timing, cloud work or benchmark.
Commit/checkpoint source before algebra execution; write separate projector
raw output instead of replacing ORDERS/CHECKS. Report in same native study.
