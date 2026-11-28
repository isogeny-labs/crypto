#set page(paper: "us-letter", margin: (x: 0.72in, y: 0.67in))
#set text(font: "New Computer Modern", size: 9pt)
#set heading(numbering: none)

= K13 admission probe: Amendment 1

*Before-data correction.* The original two-arm probe was committed before any
EKS submission, but the native EKS benchmark contract rejects it: an L2
specification needs at least five measured rounds, one warmup, a reference
arm and a matching A/A control. The v1 protocol and spec remain preserved.
`taskq/deploy/eks-karpenter/mcp/src/main.rs` is the checked source of this
requirement. The live `isolab` Job list contained no matching K13 probe Job.

#figure(image("admission_v2.svg", width: 57%), caption: [The corrected probe tests admission before the separately frozen comparison.])

*Corrected fixed cell.* `SPEC-K13-ADMISSION-v2.json` retains all eight frozen
Koblitz E0 n13 targets from seed 202611151301, factor-base dimension five,
algorithm seed 202611151302 and the exact batch-m3 method. F6-IC is the
reference method; the batch duplicate is the A/A control. The only F6 method
change from the full panel is role metadata. One warmup plus five measured
rounds across three arms gives 144 fresh processes, 120 measured and 40
batch-m3 measured rows. Every measured row must verify its answer, carry
complete clocks and phases, earn L2 or L3 and have a solve duration of at
least 1 ms. A pass permits the unchanged 560-execution panel after its own
gates. It is not a solver speed result.

*Execution state.* No image or Job was created. The correct account exceeds
its daily and monthly EKS limits, and another Conductor task holds the
checkpoint path. The v2 request must pass native benchmark validation,
source-bound `ecbench plan`, source/image publication, budget and checkpoint
gates before execution. No automatic retry is authorized.
