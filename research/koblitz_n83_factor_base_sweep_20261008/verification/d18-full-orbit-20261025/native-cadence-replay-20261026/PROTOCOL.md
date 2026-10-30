# Native D18 cadence-32 independent replay and bounded local pair

Freeze this addendum before launching either worker arm. It continues the
[source-bound cadence protocol](../xor-cadence-20261025/PROTOCOL.md) on the
primary `icv1-f2m83-tm6151469093347-debefd74` Koblitz `a=0` subgroup,
order `2417851639230796216685689`, with the exact full-D18 S3 generation.
The complete object is 160,747,529 bytes, SHA-256
`eaef94ef45b3d2d4cd2ec2f40c1173b4cdb2a437b7e618a858c73fb1caf3902b`;
the committed producer manifest SHA-256 is
`1d3650eabc4662d8f2597498d7ac92b7cd0a4134ac5f8155aaa49c7af3407dce`.
The distinct S3 custody wrapper `s3-manifest.json` has SHA-256
`1b93351febd47dac4de8c5dd39dbf58564d0057a044054baf448d978f6c0d616`
and binds that producer manifest to the object key, hash, size and curve.
The source tree and native worker/verifier binary hashes must be recorded before
timing. Fetch both S3 objects freshly with provider GET receipts and verify full
local hashes. Use the same `pilot-01` constructed target and first public
natural target as the prior cadence attempt.

## Hypothesis and reference

With a recorded lower host load than the prior 100.818 one-minute average,
the unchanged cadence-32 worker may finish import and the constructed SAT
control inside its 300-second internal wall cap. The prior attempt was
`CAPACITY_CENSORED_BEFORE_MODEL` at 303.192 seconds of supervisor wall;
it is the capacity reference, not a paired algorithm timing baseline. The
ordered cadence-one positive passed under different host conditions and cannot
be combined with this run into a speedup. A complete cold one-target IC
baseline for this primary subgroup is not yet verified. The diagnostic
`a=1` K1107/root11 result belongs to a different subgroup.

## Native execution and audit

Use the existing Rust worker subcommand
`primary-full-d18-m4-l0-ordered-xor-cadence32 positive|natural` from one
release binary. Shell commands may only fetch, hash, time, and cap the native
process. Use `/usr/bin/time -l` and an independent `gtimeout` outer limit of
310 seconds for positive or 2,410 seconds for natural. Keep stdout, stderr,
exit code, wall/CPU/RSS, launch load and complete phase/result/cap records in
fresh write-once directories. An internal exit 124 with a checked
`UNKNOWN_wall_cap` receipt is censored; an outer timeout or missing/malformed
receipt is a producer failure. The thin shell launcher writes the observed
exit code to `<result.json>.exit.txt`. The native verifier checks this file,
the cap receipt, frozen binary/source hashes and the exact ordered phase
prefix. It classifies a cap before import, during model construction or during
search separately and rejects a cap after a terminal solver phase.

The new separate Rust verifier checks worker schema, exact object and manifest
hashes, source/binary binding, frozen formula shape, target, selected row
indices/labels, representative Frobenius action, original-curve group sum,
signed-orbit coefficient row, row digest, changed-target control and rank-one
condition. It uses generic multi-limb curve arithmetic rather than the worker's
SAT model or its factor-base loader. Verify the constructed result and run
changed-target and changed-row-digest negative controls before launching
natural. The worker internally checks its original CNF/XOR assignment before
reporting SAT; independent full-assignment replay remains a separate gate if
the model bits are not retained. A natural SAT result needs the same native
group/row replay and an independently retained model certificate before it
can be promoted to a relation-yield observation.

## Stop and accounting

Launch only if one-minute host load is at most 24, swapouts remain zero, and
the scratch volume has at least 2 GiB free. First run one positive arm. If it
caps before import, stop and record capacity censoring. If import completes
but the positive gate fails or its native replay rejects, stop; do not launch
natural. If positive replay passes, run exactly one natural query at the
frozen 2,400-second internal and 2,410-second outer caps. Preserve failed
attempts, process charge and host conditions. Do not infer a solver cadence
gain from a single different-load comparison. Do not set
`selected_best_total_runtime`: natural multi-query yield, graph closure,
useful rank, full-width logs, unseen-target descent, complete cold cost and
same-target rho remain separate requirements.

The later, prelaunch [retained assignment amendment](ASSIGNMENT_CERTIFICATE_AMENDMENT.md)
adds a SAT sidecar and separate native CNF/XOR/field replay to this protocol.
The prior capacity preflight remains a source-bound record of the earlier
verifier build; it is not rewritten as an assignment replay result.
