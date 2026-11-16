# Fresh native decoder integration contract

One native rustc build (120 s), one filtered recovery_tests invocation (30 s),
one attempt. Three methods: recover eight saved public points (scalar labels
0,1,2,25,173,292,440,478 only in checker); verify that removing repaired chi(X+15)
term makes recovery fail the point check at173, reject malformed packets and point;
independent affine verification checks zero, base, double, negation, subgroup
order and all eight saved points. Production uses fresh16-sign shared windows.
No supplied signs, scalar input, source labels, search fallback or original
cohort substitution. Record logical method counts only, not complete field costs.
Full4563-case plan is unchanged and this check is not a performance experiment.
