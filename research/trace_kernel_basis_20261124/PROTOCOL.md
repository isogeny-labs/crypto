# Trace-kernel basis support protocol — 2026-11-24

Status: implemented native construction; research panel not executed. The
local Cargo tests are development qualification, not this panel's evidence.

## Request and scope

Continue the ECDLP relation investigation and deliver the code and results in
a PR. This component keeps the factor-base leaf domains and the complete
auxiliary trace hyperplane fixed. It tests an auxiliary representation choice,
not a change of factor base or an additional subgroup filter.

## Construction

Use y^2+xy=x^3+x^2+c in polynomial-basis fields with (n,modulus,c):
(7,0x83,3), (9,0x211,2), (11,0x805,22). These are the earlier synthetic
fixtures, not the repository's reference family. n=9 has composite degree.
For each n choose the standard trace chart, tau=1, and let ell=ceil(n/3)-1.
Every leaf is offset plus the span of the first ell kernel vectors. Use the
same ordered three leaf blocks in both equations S3(x1,x2,u), S3(u,x3,t).
The auxiliary spans the WHOLE affine trace-one hyperplane in every arm.

Enumerate nonzero x in increasing encoded order. Retain x with Tr(x+1)=0
and Tr(x+1+c/x^2)=0. Training uses the first two; held-out uses the next
eight, disjoint by construction. These are actual rational point coordinates;
the component does not count or recover group relations.

Arms:

- Standard: polynomial coordinates with the trace pivot eliminated.
- Random: full-rank kernel basis by rejection sampling with xorshift64,
  seed 20261124+n, accepting only a rank-increasing vector.
- Optimized: score every nonzero trace-zero field element by its auxiliary
  column support over BOTH training chains; sort by (weight,encoded value),
  then accept rank-increasing vectors until rank n-1.

The score counts each coefficient one in the polynomial-basis Weil descent
after Boolean reduction. Include constants and all terms when reporting
whole-chain support. The fixed offset is identical across arms. No row
reduction, solution-dependent filtering, timing, or solver feedback enters
the score. Candidate enumeration and scoring are exponential in n; disclose
that preprocessing rather than presenting the selector as scalable.

## Acceptance and falsification

Require identical spanned kernel and offset, full rank, disjoint target
coordinates, and exact additivity of the reported training support. The
optimized TRAINING support must be no greater than either full-rank control.
This follows from the stated finite minimum-weight-basis problem; violation
is an implementation or objective error, not a negative research outcome.

Report every held-out support, including regressions. A lower training
support is NOT an acceptance criterion for lower solving time. The scientific
success criterion of lower total cost per independent relation remains
unresolved until actual same-target solving/collection and preprocessing
costs are charged. Do not fill those fields from support counts.

## Execution and custody

Exact commands, after source publication and repository admission:

```sh
cargo build --release --locked --manifest-path research/trace_kernel_basis_20261124/native/Cargo.toml
research/trace_kernel_basis_20261124/native/target/release/trace-kernel-basis --inspect 7
research/trace_kernel_basis_20261124/native/target/release/trace-kernel-basis --inspect 9
research/trace_kernel_basis_20261124/native/target/release/trace-kernel-basis --inspect 11
```

One ordinary CPU, 1 GiB RAM, 120 seconds maximum execution, zero automatic
retries. The paid budget and current remaining balance are unset; that is
not permission to spend. Before any research launch, publish source and
protocol, obtain the required S3 checkpoint, and pass the approved EKS
resource/budget route. Preserve request/source/image/binary identities,
stdout, stderr, status and outputs. Do not reroute an unadmitted run locally.

This record does not authorize an unbounded solver sweep. A subsequent
solver-cost comparison needs a separate frozen protocol and the matched
trace-elimination baseline, with full failure, verification, rank and setup
accounting from the original objective.
