#set page(paper: "us-letter", margin: 0.72in)
#set text(font: "Helvetica", size: 9pt)
#set par(leading: 0.55em)
= K13 native launcher argument check

*Local validation only.* The source-compiled `isolab_k8s_benchmark` argument
builder accepts the frozen v3 K13 admission spec. The test invokes the
validator and command builder without submitting an image or Job.

== Source-bound result

Rust 1.99.1 release tests: *8 passed, 0 failed*. The new test embeds the v3
spec (SHA-256 `ef890a6193c6b7c45d09775a27dede3c3eb16d7ab2ffaac8b359d18e976ef308`),
asserts the `c8a.2xlarge` and `l2` arguments, and decodes the embedded command
spec to check exact equality. The five measured rounds, warmup and L2
requirement remain intact. Raw outputs and source hashes are in `RESULT.json`.

#figure(image("flow.svg", width: 100%), caption: [Local argument acceptance and the remaining execution gates.])

== Remaining gate

Both budget-readback calls returned `UnrecognizedClientException` for the
current AWS profile. Available account headroom is unknown, and the prior
monthly readback exceeded the ceiling. The checkpoint path remains reserved.
No build or Job was launched. A current same-account budget readback and the
branch checkpoint are required before EKS L2 admission and independent replay.
Full scope: `REPORT.md`.
