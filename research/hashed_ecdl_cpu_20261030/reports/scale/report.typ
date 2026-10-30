#set page(paper: "a4", margin: 18mm)
#set text(size: 11pt)
= CPU recovery ladder: scheduling deadline
Observations only. This attempt produced no larger-field recovery evidence.
The existing completed CPU comparison is separate and remains valid within its
own fixed fixtures and source identity.

Job `hashed-ecdl-cpu-scale-20261030` was submitted October 29, 2026 at 20:34:24 PDT
and reached its 1500-second deadline at 20:59:24 PDT. The last observed Pod state
was Pending/Unschedulable: the available CPU host was occupied by another
experiment with exclusive-host scheduling. The other workload was left intact.
The terminal job condition is `DeadlineExceeded`; no recovery rows, certificates,
workload logs or result archive were produced. No automatic retry was submitted.

#image("pipeline.svg", width: 100%)

The frozen plan contains 24 executions: six exact prime-field curves from 20
through 40 bits, two public point searches per curve, and two identical CPU arm
invocations. Each completed recovery would have been replayed in a separate
process and would have rejected a corrupted scalar. The intended limits were
120 seconds per recovery,60 seconds per external replay,20 million retained tuple
rows and 16384 columns. One job reserved 2 vCPUs and 4 GiB. These are requested bounds,
not measured memory usage or performance.

Targets are constructed by searching public x coordinates from fixed seeds 101
and 202, without constructing a target logarithm. The source uses eight ordered
factor-base slots with tau equal to twice the integer ceiling of r^(1/8), plus 2.
The native executable and independent archive verifier were compiled and hashed
before submission. The standalone source is `a09a3b1bf0af`; the runtime image is
pinned separately to the main-built image recorded in `scale/results/build.json`.

Protocol SHA-256:
`91e3449df0ae301d9d882db910f334a1e4ffa9e93cbd1ba93630f9d3b16f7510`.
Job UID: `2c2bb11e-f38b-47b5-9e9e-3e069fbbfa24`.
The terminal receipt and incomplete export are `scale/results/status.json`
and `scale/results/export.json`. Protocol plan: 24 cells; export contains 0 execution rows and 0 completions.
The strict completeness checker rejects the empty export against that plan;
`scale/results/export-check.log` preserves its findings. No rows were invented. The native analyzer is
identified in the frozen contract but did not run.

The 1500 seconds are scheduling elapsed time, not algorithm runtime. No field-size
frontier or largest feasible CPU scale can be inferred. A fresh bounded run
needs available CPU capacity and its own immutable run record. The required
larger-scale validation remains open; the implementation and fixed-workload
comparison are reviewable separately.
