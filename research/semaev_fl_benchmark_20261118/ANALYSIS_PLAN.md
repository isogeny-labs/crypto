# Analysis plan

The comparison is paired by workload, target, round, and deterministic arm
position. Warmups are excluded. A timed-out, interrupted, malformed, or
verification-failing pair is retained as a failure record and contributes no
timing observation.

For each admitted pair, calculate `candidate / full_width` for logical word
XORs, performed word XORs, solver wall time, whole-run wall time, and peak
bytes. Summarize ratios with their geometric mean and a deterministic paired
bootstrap 95% interval. Also report medians and every raw observation; the
small sample does not support a distributional generalization.

The two duplicate arms produce separate `full_width / full_width-aa` and
`fl / fl-aa` timing ratios. A timing claim requires the candidate interval to
be wholly below one and its effect to exceed both A/A intervals. Exact work
counters are deterministic controls, not noisy timing observations.

The fixed-system harness is analyzed separately. Its input and reduced-basis
SHA-256 values must match across arms. The m=2 and interleaved m=3 cases are
different cells and are never pooled. Local harness timings are diagnostic;
only native-L2 EKS timing is admitted.

Rho is contextual and must not be described as a competing Groebner-basis
implementation. An external Groebner reference is included only if its exact
binary is present and pinned before execution. Absence is reported as such.
