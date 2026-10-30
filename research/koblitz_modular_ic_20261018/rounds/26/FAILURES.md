# R26 failures

CHAR26-01 is rejected: ord is a reserved Singular identifier, and ++ cannot
increment an intvec entry. Only point/line negation and CRT checks pass;
the character fibers and subgroup norms are not accepted. Rename subgroupSize
and use explicit indexed assignment before02. Native exit0/late marker does
not pass. Preserve the exact failed source and all diagnostics.

CD26-01 is a mathematical failure of the proposed unshifted Z chart:
constant numerator is not divisible by t^14 at C/D; e is not the asserted
unit at C, and unramified residues fail. Some tame residuals vanish but no
proper fiber is accepted from this run. Do not silently reuse the O Z chart.
The prospective alternative transports O through the existing diamond2 map.

Lean01 is rejected: norm_num does not close these ZMod literal equalities,
abel was not imported, and field_simp already closes both field goals before
ring is invoked. Use kernel decide for the finite assertions, neg_sub for
the group identity, and remove redundant ring. Failed source/log retained.
