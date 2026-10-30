# Round17: Curve reciprocal-context comparison

The original baseline 2bd4588fba60 remains unchanged. Candidate 23ff0e6bda63
reuses the existing field reciprocal context in membership and affine
arithmetic. Reference 5d7dcb67616c includes the prior bounded inverse cache.
Five arms, three fixed fixtures, one warmup and nine repetitions:150 executions.
Both complete certificate checks, setup, discarded trials and serialization
remain charged. All output bytes and corrupted-scalar controls must pass.

Candidate adds two u64 context values to Curve (16 inline bytes). Both
candidate/reference retain the previous inverse table. These are auxiliary
Curve storage, outside the retained sum-table metric. No memory equality or
peak-RSS claim is made. Larger fields retain the wide remainder reducer.

One EKS CPU Spot job,2vCPU/4GiB,300 seconds,no retries. Ordinary jobs have no
checkpoint resume; interruption invalidates completion and clocks. L0 timings
are diagnostics only and cannot establish further10x, external/rho advantage,
or largest feasible CPU scale. T-939 owns this panel and report.

Before measurement, a verifier-region hash script was corrected because its
initial endpoint preceded the region. The corrected nonempty comparison and
its history are documented in curve-reciprocal/validation.json. Solver source,
35 release tests and all strict Clippy checks were unaffected by that fix.
