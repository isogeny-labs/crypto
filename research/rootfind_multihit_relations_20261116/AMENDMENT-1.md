# Before-data correction to the multihit screen

Recorded after protocol commit `15a7782ef3` and before implementation or
measurement. The pair-collision census uses a different, full-sum builder:
its 27/99/201 candidates are not the torsion-batch table's 11/30/64
collisions. The source-pinned inline parity test is the control for this
screen's pair-key and collision counts. Seed rank is measured by the
modular row reducer and checked by full group equations and the planted
scalar, not asserted equal to the other builder's census.

The scan records every subtraction and every actual `PairTable::probe`.
As in the production loop, identity rests skip the probe. The first-hit
arm is charged through its first valid hit; the all-hit arm is charged
through every factor-base point on a target, even if rank becomes full
mid-target. This is a conservative per-target scan comparison. The
sample sizes, stream law, arms, positive control, and decision rule in
`PROTOCOL.md` are unchanged.
