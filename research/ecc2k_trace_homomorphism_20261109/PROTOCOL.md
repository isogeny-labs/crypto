# Pending matched trace-row comparison

This is a planned solver-stage study, not a completed experiment or an
end-to-end DLP result. No spending or job submission has occurred.

Question: does explicitly supplying the known trace equation reduce work in
the native Gröbner decomposition frontend on identical rational-point queries?
Competing predictions: nonzero rows may reduce search; rows already implied
at the admitted solver degree may add only setup cost; identically zero rows
cannot constrain search. No universal cost reduction is assumed.

Baseline: `groebner_decompose_with_trace(..., false)` at the tested source
revision. Candidate: the same function and inputs with `true`. The raw
system and full group-lift validation must be identical apart from the trace
row. Use the same engine, node cap, input points, subspace and target order.
Preserve failures and UNKNOWN separately from refutations.

Freeze a native Rust harness and public known-answer workloads before
execution: small E_0 exploratory degrees, degree 31 as primary exploratory
size, discovery seeds 17/937 and fresh holdouts 20261109/20261110. At least
five alternating paired rounds and a matched A/A control. Fix an explicit
subspace dimension and m in each workload manifest, record all field and
subgroup identities, and retain cases where the trace row is zero.
Cross-check every decided small case with enumeration; independently re-sum
all returned witnesses. A correctness mismatch rejects the candidate.

Primary stage metrics: counted Boolean solver work and search nodes, including
trace construction/preprocessing; retain encoding, solve and group-lift costs
separately. Native wall time is secondary and requires admitted EKS L2 and
paired intervals. Do not infer total scalar-recovery cost from these queries.
Any external-tool claim requires a matched export and same-host comparison;
Magma/msolve comparisons are pending, not silently supplied by a self-baseline.

Degree 83 and 131 are not silently routed through the one-word
FieldStructure. A future wide-field comparison must explicitly select an
exact encoder and retain the degree-83 confidence gate and full-DLP accounting.
The configured ECC2K-130 subgroup normal-basis weight-two base is a redundant
trace-row control, not a predicted cost reduction.

Before launch: immutable source/harness/protocol commit and publication receipt,
complete task-specific S3 checkpoint, exact EKS image/binary identity, live
capacity and admission checks, finite dollar ceiling, CPU/RAM, deadline and
retry allowance. Freeze resource values when access is established, before
any spend. Missing values block launch. No local performance fallback is
currently selected or authorized as a replacement for the prescribed route.

Acceptance: all decided group answers agree, with no dropped timeouts/failures;
report the paired costs and ratios even for a regression. A stage gain
requires lower total counted stage cost on equal completed workloads. Stronger
pipeline and ECC2K-130 conclusions require their separate measurements and
independent signed PCC/validation receipts.
