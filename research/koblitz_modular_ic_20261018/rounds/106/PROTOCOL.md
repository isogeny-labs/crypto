# R106: explicit local base uniformizers at x=1

Bind the exact R105 full f-support JSON by SHA-256. In each cluster put
`t=x+1`, `z=y` or `z=y+1`, and `s=t/z`. Substitute `t=s*z` into the complete
R105 f support, divide by the largest common z power (expected 2 and 3),
and check that the resulting polynomial H(s,z) has H(0,0)=0 and
`dH/dz(0,0)=1`. Lift its unique zero-constant branch `z(s)` over F2 to
precision `s^32`, set `t(s)=s*z(s)`, and verify the original f vanishes
modulo `s^32`. Check initial valuations `(v_s(z),v_s(t))=(2,3),(3,4)`.
Fresh replay must match exact JSON; changed R105 hash must fail.

This produces complete base-field local coordinates to finite precision,
not yet the lambda extension's normalized branches. Commit source and
protocol before data, retain failures. Deterministic exact F2 arithmetic,
one local process, no cloud resources or statistical comparison.

Sources: R51 donor via R105 hash, R105 Newton edges,
[Stacks complete-local Hensel lemma](https://stacks.math.columbia.edu/tag/04GE),
and [Stacks normalization of one-dimensional schemes](https://stacks.math.columbia.edu/tag/0C44).
