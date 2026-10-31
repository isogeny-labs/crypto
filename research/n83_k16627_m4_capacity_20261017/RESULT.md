# Retained N83 K=16,627 base: m=4 model-capacity result

The native primary adapter now accepts m=4 on its guarded chained-S3
paths. Under a one-CPU, 4 GiB, zero-swap, no-network container, the exact
retained base with **16,627 signed-Frobenius columns** and **2,760,082 point
records** imported and produced a four-summand orbit-domain model with
**201,598 SAT variables, 5,915,238 clauses and 3,901 XOR rows**. The
container exited 0 without OOM; peak cgroup memory was **2,661,126,144
bytes**. The required negative-cap control returned
`UNKNOWN_variable_cap` after checked import and preflight. Exact inputs,
commands, receipts and limits are in the [run report](RUN_REPORT.md).

This is a checked capacity result for the specified object and model.
Solver search, accepted relations, useful rank, linear algebra, target
extraction and complete cold cost were not measured. The next gate is a
degree-83 planted relation on this exact imported base, then matched
natural-query/rank measurements against the retained K=2,048 and
2,001-column structured bases. The IC scoreboard remains unchanged.

| Requested dimension | Status after this run |
| --- | --- |
| Prebuild and retain an N83 symmetric base | verified for the exact K=16,627 object in the preceding construction PR |
| m=4 native adapter and hard-cap model construction | verified for the identity-mask-zero orbit-domain model; negative cap and full arm both completed as frozen |
| m=4 quality and end-to-end cost | open: planted control, natural relation yield, useful rank and complete cold accounting remain |
| m=3 and m=2 quality | not attempted in this capacity experiment; previous support thresholds do not replace measurements |
