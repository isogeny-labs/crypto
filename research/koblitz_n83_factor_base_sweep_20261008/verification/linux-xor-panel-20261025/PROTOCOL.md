# Resume gate for the primary N83 Linux XOR panel

This directory records a blocked AWS admission, not a Linux solver result.
The intended comparison is a separately frozen, single-host Linux experiment
on the four source-pinned hash/Gray 5+1 and 4+2 XCNF inputs in
[`admission.json`](admission.json). Its four inputs are already in S3 and
were fetched back and byte-replayed. The source curve is the primary
`a=0`, 81-bit subgroup; do not mix it with the diagnostic `a=1` curve.

Before allocating compute, require an actual provider admission probe. AWS
`RunInstances --dry-run` returned `DryRunOperation` while the subsequent
real call was blocked at account admission. A dry-run alone is insufficient.
The precreated instance profile `n83-xor-panel-20261025` is limited to
the exact input/output prefix and SSM; its [policy](s3-policy.json), [trust
document](ec2-trust.json) and [90-minute shutdown](ec2-user-data.sh) are
committed for review. No instance was allocated in the blocked attempt.

On an admitted host, perform these gates in order:

1. Pin the image, instance type, isolation level, cgroup limits, binary
   version and SHA-256, architecture, CPU quota, memory, source revision,
   source archive, S3 checksums, exact formulas, host charge and cold-start
   cost. Fetch the four named `inputs/*.xcnf` objects and run
   `audit_admission.py --downloads-dir DIR` before any solver call.
2. Freeze a new Linux protocol, including all flags, seed, solver caps, arm
   order, adjacent A/A controls, and a stop rule, then commit/push it before
   timing. Linux binary hashes must not be substituted into the Darwin
   protocol. The matrix-column-limit contrast is a separate factorial axis
   because the admitted XOR matrices change.
3. Run matched arms sequentially on one isolated host with independent
   process and CPU time, peak memory, exact conflict count, stdout/stderr,
   exit code, and raw-output hashes. Store each process receipt and raw
   stream under the dedicated S3 `outputs/` prefix using write-once keys.
   An `INDETERMINATE`, watchdog, OOM or malformed output is censored.
4. Download and independently replay every S3 output byte. For any
   `SATISFIABLE` model, run the exact worker's `verify-cross-xcnf`
   command against the original S3 archives and exported formula. A
   group-rejected model requires blocking and continued search; it is not
   useful relation yield. Only independently verified natural partials
   enter graph/rank accounting.

No solver-only contrast can select the factor base. The full objective still
charges construction, import, relation failures, partial graph closure,
linear algebra, unseen-target descent, artifact I/O and matched same-target
rho. Keep `selected_best_total_runtime` null until comparable complete
runs establish a winner.
