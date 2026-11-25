# Independent admission replay before expanding public targets

The frozen C/OpenSSL census returned the prescribed classification for every
control and certified all12recorded public targets across6prime-order curve
domains. All6domains passed the descriptive exceptional-class screen. This is
an implementation-independent replay of an existing method; it is unsigned
producer evidence, with independent reviewer execution and signed standing
still open. See the [exact producer receipt](evidence/ADMISSION-RESULT.json)
and attestation-bound PCC record in `pcc/claims/`. [pcc:unverified]

## Method and scope

The [C source](native/admission.c) consumes public parameters and coordinates
only. The input projection omits recovered scalars and timings. It checks
canonicality, nonsingularity and exact trial-division primality. OpenSSL3.5.7
supplies curve construction, point addition and doubling; the program asserts
that no group order or finite generator has been configured. Explicit binary
addition/doubling checks rG=O and rQ=O. It never calls `EC_POINT_mul`.
[pcc:unverified]

For a nonzero point killed by prime r, its order is r. Lagrange implies r
divides the positive full curve order N. Hasse bounds N by
p+1+floor(sqrt(4p)). If that upper bound is below2r, N=r. These are established
facts, already used by Rust `Curve::with_order_witness`. The official Std
lemma `Nat.eq_of_dvd_of_lt_two_mul` supplies the final natural-number
implication; this replay asserts no complete Lean refinement of OpenSSL or C.

Only certified domains receive trace, anomaly, supersingularity, j0/j1728
and embedding-degree1..99 flags. Passing the recorded screen does not certify
a deployment-standard curve or establish a cryptanalytic improvement.
[pcc:unverified]

## Frozen controls and observations

The [original draft](DRAFT-ADMISSION-PLAN.md) and
[pre-code addendum](ADMISSION-ADDENDUM-01.md) define the controls before the
first source/build/run. The sole census produced212JSONLrows, comprising199
controls,12real fixture rows and the summary. [pcc:unverified]

| Control class | Rows | Required outcome |
|---|---:|---|
| Integer square-root postconditions | 1 | All65536integers0..65535 satisfy both inequalities |
| Known composites | 3 | Reject with an actual nontrivial divisor |
| Small parameter cells | 144 | Match direct x/y enumeration and literal prime table |
| Literal anomaly and singularity | 2 | Certify-and-flag anomaly; reject singularity |
| Domain mutants | 48 | Exact specified rejection, not an arbitrary error |
| Oversized field | 1 | Unknown/resource before primality or point work |
| Recorded public targets | 12 | Full prime order and point certificate |

All required classifications passed. Small cells with insufficient interval
information remained `unknown_interval`; they were not called invalid curves.
The oversized input remained `unknown_resource` with zero arithmetic work.
The raw output and source/input/binary pre/post checks are archived without
alteration. Native phase exit0, sampled physical stop0, sampled whole-cgroup
peak13,795,000,320bytes under the fixed15,032,385,536-byte stop. The time limit
was60s inside a600s phase. No per-process memory or performance claim follows
from these diagnostics. [pcc:unverified]

The C11/O2 build used all prescribed warnings as errors. Before compiler
launch, the first copied wrapper failed its own filename check; the failed
wrapper and incomplete build directory are retained. A fresh wrapper/build
directory succeeded. No arithmetic census was rerun or overwritten.
[pcc:unverified]

## Reproducibility and next action

The [archive hash](runs.tar.xz.sha256),
[build qualification](evidence/BUILD-QUALIFICATION.json),
[native receipt](evidence/ADMISSION-RESULT.json) and
[diagram](admission.svg) bind source, input, binary and raw records.
Extract/rebuild commands are in [README](README.md). Repacking the extracted
raw tree uses the deterministic command in the repository's AGENTS§13a.
The raw JSONL is72,728bytes with SHA256
`e74b75bc015ad63448c0c0232e92763a0bea386e97f8581fef14f60ec2214d54`.
[pcc:unverified]

The next experiment must prospectively qualify new worker/auditor/collector
and export identities. Existing workers still restrict seeds101/202. Preserve
the original360-call dataset and324seed-table export findings; do not repair
old data retrospectively. The six curves and twelve deterministic public
targets are a bounded fixture set, not independent uniform samples. New-target
generation and solver comparison require their own frozen protocol and controls.
[pcc:unverified]

Sources checked: [SEC1v2§3.1.1.2.1](https://www.secg.org/sec1-v2.pdf),
[Sutherland18.783 Fall2025 Lecture7, Theorem7.3 and§7.4](https://math.mit.edu/classes/18.783/2025/LectureNotes7.pdf),
official Lean4.35 `Init/Data/Nat/Lemmas.lean:1485`, and frozen Rust worker,
constructor and point-auditor paths listed in README. KB MCP and wrapper were
unavailable; bounded primary-source reads are the fallback. No novelty clearance
is inferred. No performance leaderboard or chart is changed by this
correctness census. Classical/local only; S3 excluded. [pcc:unverified]
