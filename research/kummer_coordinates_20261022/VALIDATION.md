# Validation and boundary

The generator uses the published generic Mumford projection and fast
Kummer xDBL/xADD from
[Chung, Costello, and Smith §§7.2–7.3](https://www.microsoft.com/en-us/research/wp-content/uploads/2016/02/recovery.pdf).
It transforms the original Scholten divisor by the exact affine Rosenhain
map and separately checks each group sum in the new Jacobian. The
two-torsion actions are fitted as 4-by-4 projective linear matrices from
11 generic pairs/model and then checked on all 1,260 public targets and
every recorded model exception. They are a validated computational map
on this finite cohort, not a symbolic formula for all curves.

The independent `validate` binary recomputes xADD from every recorded
coordinate witness, checks projective equivalence or a zero output, verifies
the Mumford expected degree, and recomputes every count from raw JSONL.
It matches each public workload, seed, scalar and seven labels against the
prior independently run reference quotient gate in
`research/kummer_walk_20261022/results/target-gate-*.jsonl`.
The frozen public source scalar and all label checks passed. The 175
exception rows are retained in a separate raw file; seven target rows have
a zero coordinate in the difference point and yield a zero vector from
the projective xADD formula despite a valid expected Jacobian class.

The two-class signed test checks that K(T) and K(T+G) distinguish T from
-T by using the Jacobian oracle. It does not implement the published fast
Recover operation, nor prove a general label-carrying arbitrary-jump
collector. No operation timing or candidate/rho cost ratio was measured.
