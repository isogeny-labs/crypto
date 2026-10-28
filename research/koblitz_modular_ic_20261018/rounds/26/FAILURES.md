# R26 failures

CHAR26-01 is rejected: ord is a reserved Singular identifier, and ++ cannot
increment an intvec entry. Only point/line negation and CRT checks pass;
the character fibers and subgroup norms are not accepted. Rename subgroupSize
and use explicit indexed assignment before02. Native exit0/late marker does
not pass. Preserve the exact failed source and all diagnostics.
