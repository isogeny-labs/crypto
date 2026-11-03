# Complete cold execution boundary

This supplement is frozen before executing any cold timings. Earlier raw
reference, interrupted and resumed polynomial runs remain immutable.

Run all ten arms on each of the same 21 instances, including the inactive
column control. A target is one independent workload. Use the complete
frozen public coefficient stream, no hit counts or known scalar in the
candidate, and stop after a complete query first reaches full rank or at
the original stream cap. The capped control has an observed consumed cost,
not a cost to a successful solution.

The cold interval starts from public field/model/subgroup/eigenvalue and
G,Q parameters. It includes field setup, deterministic Tonelli--Shanks
lifting of every candidate's required original abscissa, sign orientation
to the public physical encoding, orbit-column construction and validation,
construction and validation of only the image factors that arm uses,
zero-triple handling and exact S4 symbolic derivation. Public model and
subgroup checks are included. It excludes input parsing, process startup,
known-answer target generation and writing the evidence after the clock.
No factor logarithm oracle, reference relation, planted scalar, precomputed
lift, precomputed orbit-column map or external solved system is available
inside the candidate interval.

Online work comprises five exclusive intervals: point queries, polynomial
decomposition including all peels, roots and group relation checks, matrix
insertion, back substitution, and independent group verification of the
target and all usable factors. Loop bookkeeping is charged to point
queries so these intervals sum exactly to online elapsed time. Cold is
setup plus online. Emit peak resident memory and allocation telemetry;
instrumentation and in-memory evidence construction are included. A direct
isolated elapsed interval includes symbolic work, normalization, roots,
hash/tree work and allocation, even when earlier multiplication counters
did not. Arithmetic counters retain their explicitly limited meanings;
do not manufacture a complete operation price from them.

Use a dedicated ephemeral c7i host, one Rayon thread and one measured
process at a time, under the repository isolation wrapper with a reserved
whole core including its SMT sibling. Freeze one warmup, two identical
non-timing replays, and five rounds of all ten arms per workload. Alternate
forward and reverse arm order by round; no adaptive arm selection. An
additional original-arm run in each round measures A/A spread. Apply the
same 8 GiB process cap and 1,200 s cap per arm, preserving every failure.
Use a 150-minute job TTL and retain partial runs on interruption. If this
does not finish, resume missing complete records without editing old ones.

Separately check cold relation sets, rank, answers and deterministic
counters against the sealed polynomial records and independent reference.
Report every per-target result, cost ratios, A/A spread and resource
verdict. A failure or censored arm cannot be treated as a successful cheap
solution. Isolated cold IC-to-IC ratios do not constitute an asymptotic,
fastest-external-solver or IC-versus-rho claim. The declared matched-rho,
external solver and complete operation-pricing obligations remain separate
and must be labelled with their actual execution status.
