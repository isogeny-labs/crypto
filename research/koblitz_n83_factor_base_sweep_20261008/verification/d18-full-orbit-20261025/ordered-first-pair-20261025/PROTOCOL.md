# Full-D18 exact first-pair symmetry gate

Freeze one clean source commit, one release binary, the primary `a=0`
GF(2^83) curve, and the complete direct-subgroup D18 S3 base with 32,643
signed-Frobenius columns and 5,418,738 point rows. Freshly GET its object
and wrapper with provider full-object checksums, verify compressed SHA-256
`eaef94ef45b3d2d4cd2ec2f40c1173b4cdb2a437b7e618a858c73fb1caf3902b`,
wrapper SHA-256
`1b93351febd47dac4de8c5dd39dbf58564d0057a044054baf448d978f6c0d616`,
and producer-manifest binding. Use the unchanged `pilot-01` public corpus,
first `(a=0,0)` target and `m=4`, identity mask zero.

Both arms use the exact shared-root signed-Frobenius domain with no selector
or phase pins. Branch on the four summand x blocks first, then Frobenius
phase wires initially false. After domain construction, call the existing
`order_first_pair` constraint on the first two x blocks. They share an
identical domain, and their first S3 node is symmetric, so exchanging those
two inputs preserves the node and every downstream constraint. Lex ordering
keeps one orientation for every satisfying pair. This arm imposes no order
on later summands and does not change the field equation or factor base.
The source adds 82 comparator variables and 247 clauses, so the frozen
expected shapes are 329,808 variables, 2,807,999 positive or 2,807,501
natural CNF clauses, and 3,901 XOR rows. A different shape is an identity
failure, not a new empirical arm.
Small-system tests already cover exact first-pair-swap behavior; the positive
full-D18 control below checks it on this source and object.

Run the positive constructed arm first under a 300-second whole-worker cap
and independent 310-second process watchdog. Require native SAT, original
field-equation verification, a checked four-point group lift, signed-orbit
row, rank-one insertion and altered-target rejection before natural search.
Only then run one unpinned public natural query under a 2,400-second
whole-worker cap and independent 2,410-second watchdog. Each solver uses a
100,000-conflict cap. Save the exact command, source/binary/S3 identity,
phase records, raw stdout/stderr, terminal exit, wall and memory usage, and
host load for each arm. Keep the L0 shared-host isolation label.

An independent auditor must replay the positive relation over the original
field and reject an altered target. If a natural SAT result appears, it must
replay the exact point indices and signed-Frobenius row against the S3 object
and public target before admitting one useful row. If a cap, kill or missing
result occurs, classify it as censored or producer failure from the raw
terminal evidence. A single positive result or single natural query cannot
estimate yield or a total-runtime ordering. Preserve all stage charges and
keep `selected_best_total_runtime` null until natural relation rate, rank,
factor-base logs, unseen-target descent and matched same-target rho pass.
The Python original-field auditor uses the pinned `blake3==1.0.8` package
to recompute the native wide-row digest.
