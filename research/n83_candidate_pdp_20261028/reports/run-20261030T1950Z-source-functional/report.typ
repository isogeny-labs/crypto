#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
#set heading(numbering: none)
= n83 source-bound terminal functional panel
Observations only; this report does not promote a mathematical claim.

The retained prime m3–9 panel contains 42 cases: original, baseline and spike40, each with planted and natural targets. It contributes to the wider three-family m2–9 fixed-base study. Inputs and factor bases were unchanged.

== Execution and retained failures
EKS run n83-source-functional-20261030-r1 reached its deadline after unschedulable Pod observations. No worker start was observed and no raw object was published. This is an infrastructure outcome, not a refutation. Its request, Job/Pod identities, statuses and empty raw listing are preserved.

A separately frozen version 2 ran n83-source-functional-20261030-r2 on the existing host with the same native binary and 42 input/source/base identities. Producer and fresh replay completed; restored S3 bytes passed a further native replay. Each phase had a 120-second limit and 2 GiB virtual-memory limit. No retries; no timing claim. Per-case wall/CPU times were not measured.

== Recorded results
All 42 source and retained-factor checks passed. Nonzero pairings: 42/42. Recorded coefficient degrees: [14]. Recorded nonzero term counts: [69632]. These are degrees/counts of C, not degrees of regularity.
#image("pairings.svg", width: 100%)
The bars count recorded nonzero pairings for two targets per arm/m in run r2. Deterministic exact arithmetic; no statistical interval is asserted.

== What was checked
The native instrument reconstructs each complete source, verifies input/base/certificate hashes, recomputes A0/A1/A2 and C, and compares the direct complement pairing to the exact scalar formula. The pinned image passed 22 development test invocations: six new tests plus eight unique donor tests imported twice. Mutation and controller success/failure/termination controls passed. Production source commit: d0f98fafe1df2b06ad5df6ca6f033178cfa7dc02.

== Claim limits and custody
Strict filtered fall remains unverified. All LFD and regularity fields remain null. V16 was not computed. The separate syzygy and filtered-space argument still needs independent verification. Candidate x-domain equations do not enforce the expanded liftable-point lookup ideal. No elite-base advantage or speedup is established.

Raw archive: 25122846 bytes, 398 verified payloads. SHA256:
#text(size: 7pt)[31fb56ec3a98c0e9890ee35a593d2d2bd1aab2fad4db1e3df7845ee4f21e777a]
See evidence/source-functional-r1 and -r2, both experiment-source-functional exports, protocol-source-functional(-local).json, and SOURCE_FUNCTIONAL_RUN_FORMAT.json. Full coefficient polynomials and exact executable are retained in S3; Git holds case records, hashes, formats and this report.
