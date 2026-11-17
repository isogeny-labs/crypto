# Amendment 33: retain the homogeneous kernel for affine target recovery

Trial38 replays all six immutable graph matrices from37, rather than choosing
only its observed best arm. Exact source/input SHA bindings are frozen in
`quotient-plan.json`. This is a saved-matrix component experiment; no new
DLP sample,37end-to-end repricing, cost composition or rho result.
[pcc:unverified]

The current collector computes a homogeneous RREF rank then discards its
basis. The later affine solve starts from scratch. Here BOTH arms pay for
the original homogeneous checkpoint. The baseline then runs the original
full affine RREF. The candidate extracts the checked codimension-one
kernel h (one free coordinate fixed to1), writes factor-base logs as h·t,
and maps each affine row to (a·h)t+c_Qd=rhs. It solves for the actual target
d using a two-column RREF. Existing zero-normalization behavior is preserved
to isolate reuse from36. No answer/reference enters either solver.
[pcc:unverified]

Include homogeneous arithmetic, baseline full solve, candidate kernel
negations, dot products/additions, two-column solve and emitted known new
state events. Remaining RREF/Euclidean/fixed-factor/container/control work
stays explicitly incomplete. Native gaps keep complete totals, bands,
ratios and comparative eligibility null/false. Diagnostic state hashes and
validation instrumentation are outside algebra counters and inside wall;
wall is practicality only, without a runtime-speed assertion.
[pcc:unverified]

The separately copied Rust qualifier uses column-keyed/u128 RREF with Fermat
inversion, without the driver's Shoup/support-list helper. Independently
check every affine RREF state and semantic operation counter, homogeneous
rank/state/counters against the saved checkpoint, the unique normalized
kernel, every projected row, target, known new event conservation and
primitive repricing. Reject changed counts, projections, kernel/scalar,
protocol, input/source binding or claimed completeness. Preserve every
failure/cap/timeout without weakening criteria. [pcc:unverified]

One native worker processes six paired replays, zero warmups/retries,
60s worker/180s whole/900s runner/8GiB. Dedicated Runpod CPU4vCPU/8GiB at
<=0.40USD/hour, pinned official Rust image, first allowed CPU. Finish builds
and publish clean source/spec/source+binary+input manifests before sole
dispatch. Copy immutable worker/qualifier/wrapper/source/inputs, retain exact
argv/flow/status/raw/hash custody, collect before immediate owned-pod
delete/absence check. Native actual-wrapper raw/create-new/stdout controls
precede dispatch. [pcc:unverified]

Sources checked: actual homogeneous_rank/CountedRelationSystem/final solve,
JV primary §3.3 structured elimination context,37certified checkpoint
matrices. KB unavailable: validation/quotient-kb-{exact,negative}.log.
This is standard nullspace substitution, without a novelty, general-curve
or asymptotic claim. After the component check, production integration and
a fresh full pipeline measurement remain separate obligations.
[pcc:unverified]
