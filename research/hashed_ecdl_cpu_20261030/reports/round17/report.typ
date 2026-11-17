#set page(paper: "a4", margin: 15mm)
#set text(size: 9.5pt)
= CPU Curve reciprocal candidate: scheduling failure
Observations only; no algorithm measurement, speedup or hypothesis promotion.
Job `hashed-ecdl-cpu-round17-20261030`, October 30, 2026 UTC.

The candidate reuses the existing reciprocal field-reduction context in
Curve membership checks, the curve equation and checked affine arithmetic.
Modular sums use bounded additions; larger fields retain wide remainders.
The inline context adds two u64 values (16 bytes), with setup charged.
Both complete certificate replays remain. The previous inverse cache remains.

*Candidate source:* `23ff0e6bda635d428417024a8ab17e233f7d9af4`.
Reference: `5d7dcb67616c`; original baseline: `2bd4588fba60`.
Software validation:35 release tests, strict all-target Clippy and formatting
pass. Independent tests use u128 remainders and Fermat inversion, bypassing
both production kernels. This does not establish performance.

#image("execution.svg", width: 100%)

#table(columns: (2fr, 1fr), inset: 5pt,
 [Protocol-planned executions], [150 (135 measured + 15 warmups)],
 [EKS submissions], [1],
 [Recorded algorithm executions], [0],
 [Retained certificates / timing rows], [0 / 0],
 [Retry submissions], [0],
 [Final job state], [Failed: DeadlineExceeded])

The job was created at16:05:42Z and reached its300-second deadline at16:10:42Z;
Kubernetes recorded Failed at16:10:43Z. Last observed pod was unschedulable,
with no node assignment: one node violated pod anti-affinity, two did not
match affinity/selector, and two had untolerated taints. The terminal status
has no retained pod. This is an infrastructure outcome, not a solver timeout
or evidence of a slow algorithm. No measured CPU model or algorithm time exists.

Expected results-object HEAD returned404. No native analyzer ran and no timing
or certificate archive was returned. The frozen input bundle and source/binary
identities are retained. No synthetic executions fill the missing cells.
The empty export records zero observed executions; the frozen protocol still
plans150. The completeness checker reports eight findings: missing arms,
missing planned rows, five per-arm instance shortfalls and unperformed masked
analysis. The export is an incomplete failure record, not a passing panel.

The first verifier-region receipt accidentally hashed an empty slice. Before
measurement submission, the endpoint was corrected and the script required a
nonempty region containing verify. The actual3033-byte region is identical to
the preceding source. This correction changed metadata only and is preserved
in repository history. Remote branch advances were merged before publication;
all frozen candidate source hashes still matched after reconciliation.

Protocol: `d5f974403ab76e04927ff53b75cf89e6fcd42b9011921b9a3c88a8a2022fa084`.
Job UID: `008f64ff-e845-4bed-94c4-f475f6788e04`.
Raw status, input receipt and failed export: `round17/`.
Raw evidence was pushed as `90d1adb048be` before this report.

The further10x goal remains unproven. Round16 timings apply only to the prior
candidate, not this one. Candidate end-to-end timing, strict isolation,
independent scientific review, external/rho comparisons and larger-scale CPU
recovery remain open. No automatic retry or replacement job was launched.
