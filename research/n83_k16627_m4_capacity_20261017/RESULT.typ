#set page(paper: "a4", margin: 17mm)
#set text(font: "New Computer Modern", size: 10pt)
= N83 retained-base capacity result

*2026-10-17 UTC.* The primary `GF(2^83)` adapter accepted the retained
K = 16,627 signed-Frobenius base and built an m = 4 identity-mask-zero
orbit-domain model under one CPU, 4 GiB hard memory and zero swap.

#image("capacity-diagram.svg", width: 100%)

The full worker completed with 201,598 SAT variables, 5,915,238 clauses
and 3,901 XOR rows. Its cgroup peak was 2,661,126,144 bytes; exit 0,
no OOM. The preregistered one-variable negative control returned
`UNKNOWN_variable_cap` after exact-base import and preflight, also exit 0
without OOM. The input, binary, source and result digests are bound in
`BUILD_RECEIPT.md`, `source-attestation.json`, `negative/worker.json` and
`full/worker.json`.

This is a model-capacity result at the specified base and domain. SAT
search, natural relations, useful rank, linear algebra, target extraction
and complete cold cost remain open. The next gate is a degree-83 planted
relation on this exact imported base. Full accounting and the baseline/rho
gap are unchanged. See `RUN_REPORT.md` for hard caps, timings and limits.
