# Differential Kummer signed collector and paired pricing

Frozen 2026-10-22 UTC before collector runs. The transition law and
canonical-ratio partition are the tested recurrence in
`research/kummer_differential_walk_20261022/`; the signed Recover and all
exception diagnostics are in `research/kummer_recover_20261022/`.

Use the same 1,260 public targets and source orders. For each target, start
from `D=Q`, `G=P`, and pair `(K(D),K(D+G))`. Apply the two-branch quotient
recurrence, retaining exact coefficient labels. At each state, extract the
general Kummer `u` polynomial. Only if it splits into rational roots, recover
the signed divisor using the fixed signed anchor and `K(D+G)`, then insert
the resulting signed edge into the same `SignedGraph` rank collector used
by the native genus-two baseline. A complete candidate must return scalar
`k` and verify `kP=Q` on the source elliptic curve. Keep exceptional
Recover and xADD statuses and full phase-operation counts.

Pair each target by `workload_id` with its frozen, same-source rho record
from `sessions_projective_chart_v2.tar.zst`. Price 36 `(S/M,I/M,native)`
cells from the established grid. Record candidate and rho cost, ratio,
operation counts, full setup, relation processing and elimination; include
all wins and losses. Do not extrapolate from a pilot. If any gate or cohort
run fails, retain all raw failures and leave incomplete cost cells null.
