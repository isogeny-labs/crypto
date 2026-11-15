# Amendment 3: projective-incidence census and finite density bound

14 November 2026, before execution. This is a follow-up to the q=8
falsification in Amendment 2. It retains the same fields, moduli, cubics,
and native implementation language. The question is whether the overlap
of projective first-factor search sets admits a useful second-moment bound.

For each `q in {2,4,8}` and each of the two cubic spaces `V`, form one set
`S_r = {r*(w+c) : w in V*, c in k*}` for every projective representative
`r in P(V)`. Count the number `t_z` of sets containing each nonzero field
element `z`, once for a single cubic and once for all representatives of
both cubics. Record the histogram of positive `t_z`, the sum `M=sum t_z`,
the second moment `E=sum t_z^2`, the union size, and the integer Cauchy
lower bound `ceil(M^2/E)`. Require the union size to equal the independently
recorded direct-product W1 union from the earlier runs and require every
member to have relative trace zero.

This finite census can falsify simple incidence conjectures and provide
exact finite lower bounds. It does not by itself prove an all-q density
formula or establish ECDLP speed. Stop after ten minutes, assertion failure,
or memory exhaustion. Retain stdout and run-time/source/binary metadata in
a new write-once run directory. Use `scripts/checkpoint.sh gate` before run.
