#set page(paper: "us-letter", margin: 0.72in)
#set text(font: "New Computer Modern", size: 10pt)

= N83 Colima build admission

#text(size: 9pt)[Attempts 2026-11-16T00:25Z and 00:29Z · Local Linux ARM64 · No K600 solver work]

*Observed result: attempt 1 invalid; attempt 2 passed.* The first no-work container reported 12 GiB hard memory, zero swap, four CPU quota and 512 PIDs but printed a Git warning because the VM lacked the checkout's alternate object directory. Its exit 0 was rejected as an invalid source pass. The Colima profile was stopped and restarted with that exact alternate directory mounted read-only. The new control verified the `HEAD` commit object, committed plan blob, clean source, exact input hashes, frozen cgroup limits and empty Git stderr. It exited 0 with no residual named container.

#figure(image("admission-flow.svg", width: 100%), caption: [Observed failure, mount correction and source-bound retry. Dashed edge is pending work.])

== Evidence and boundary

The source-linked result is `verification/k600-colima-charged-build-20261116/ADMISSION_RESULT.md`. Attempt 1's 249-byte log has SHA-256 `f6e644323e344e7843d157a8c5d92cf3f7d0c45ea96dedb3a08d2c15e7866767`; its child/wrapper exit was 0 but not accepted. Restart stop/start receipts both exited 0, and VM `findmnt` showed `virtiofs ro,relatime` for source and alternate. Attempt 2's 142-byte log has SHA-256 `493b0e6a25e949fe8e5e04860aa86fb9259188506a0e3fcf63f9fffeacc26b3f`, with child/wrapper exit 0 and no signal. Raw logs, receipts, scripts and source revision are retained alongside the result.

This validation clears local source/mount/cgroup admission only. Native build-producer bootstrap, the five Rust binaries, CryptoMiniSat, the labelled K600 runtime image, zero-trial worker preflight, constructed and natural relations, matrix rank, target descent, matched rho and full cold one-target IC runtime remain pending. Canonical IC charts are unchanged because no algorithm measurement was produced.
