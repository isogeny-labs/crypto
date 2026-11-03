#set page(paper: "a4", margin: 17mm)
#set text(font: "DejaVu Sans", size: 9pt)
#set par(leading: 0.5em)
= Automatic CRIU / EBS recovery on EKS
Iso Lab · 21 October 2026 · native harness qualification

The AWS-only native harness survived a stopped controller and the deliberate
loss of its own Spot worker. Test E preserved a 1,140-record checkpoint prefix
and completed 1,806 records after restoration. Its separate fresh epoch passed
*1,806 L2 observations, 1,800 native replays, 600 A/A pairs and 19 S3 payload
hashes*. A duplicate terminal submission created no second epoch.

#image("flow.svg", width: 100%)

== Scope and frozen workload
This qualifies infrastructure and correctness, not an algorithm speedup or
arbitrary workload recovery. The public fixture uses prime_search bits 32 / seed
59297; two targets / seed 20261002; rho.negation, rho.plain and matching A/A;
300 measured rounds, one warmup. Test C used five rounds and completed before
its first checkpoint. All changes were frozen before launch. Every resumed
computation timing sample is invalid/L0; only the distinct fresh epoch earns L2.

The supported recovery class is c8a.2xlarge, AMD family 26/model 2/stepping 1,
AL2023 kernel 6.18.54-121.166.amzn2023.x86_64, containerd 2.3.5+unknown and CRIU
4.2.1. Source, binary, runtime and request hashes bind every attempt. The original
absolute deadline is never extended. Capacity stayed within the existing
96-vCPU Standard Spot quota and 64-vCPU lab cap.

#table(columns: (34%,66%), inset: 4pt,
 [Test E receipt], [Observed value],
 [Original → replacement worker], [#raw("i-0ad9c1eaaaf9033a7") → #raw("i-0ae32d58bdccafdf4")],
 [Encrypted snapshot], [#raw("snap-0af1b03e30d462fce")],
 [Checkpoint readback], [45,844,480 bytes; full versioned S3 SHA-256 verification],
 [Original byte prefix], [5,096,143 bytes / 1,140 records, preserved in final 1,806],
 [Restored process tree], [Root PID 531 and child PID 5133; stopped credential checks before SIGCONT],
 [Fresh A/A interval], [95%: 0.999546254387097–1.0006296636908705; 600 admitted pairs],
 [Fresh Pod UID], [#raw("dffc2fc4-fb8f-48f7-92a6-e2d79d0e8178")],
)

== Failures, controls and remaining limits
A failed in bootstrap before computation; B was cancelled for a deployment
provenance mismatch before any payload start; D was rejected by CodeBuild's
single-controller limit. C passed the zero-restore path (36 L2 / 30 replays /
10 A/A / 19 hashes). These attempts remain recorded; none is a retried failed
L2/A/A measurement. E's computation replay verifier ran under the supervisor;
F passed the final runtime with replay observed at UID/GID 1000, empty groups,
zero capabilities and no_new_privs. Original and restored payloads already ran
with those restrictions. F preserved 1,147 records / 5,128,482 bytes and completed
1,806 computations across us-east-2a → us-east-2c. Its fresh epoch passed
1,806 L2 / 1,800 native replay / 600 A/A / 19 S3 hashes. A/A 95% interval:
0.9998302217833182–1.0003545982056818. The replacement exposed eight physical cores
and eight logical CPUs, with no SMT. Its fresh Pod UID is
#raw("8a5f3da5-38b1-42d0-8ed5-1a51ef71720d").

The supervisor remains privileged and is for trusted workloads. Ordinary
benchmark Pods remain Restricted; containerd CRI restore-via-create is disabled.
Sixteen native tests cover identity/deadline/fencing, incomplete generations,
corrupt/truncated archive readback, credential checks and failed-epoch refusal.
Live corrupt-snapshot multi-generation fallback, arbitrary commands, GPU/ARM and
cross-family recovery are not qualified. The full main-only commit-building
GitHub Action remains pending review/CI/merge; this AWS path does not prove it.

Source-linked details, final-runtime F results and all exact identities are in
#raw("REPORT.md") and #raw("runs.tar.xz") (verify #raw("SHA256SUMS") before
extraction). The archive contains public records and receipts; raw process RAM
stays in private encrypted S3. Canonical performance graphs and leaderboards
remain unchanged because no speed claim is made.

The lab host MCP/client upgrade was also checked through a fresh initialized
connection. Re-submitting completed F through MCP succeeded with the same
request, attempt and benchmark UID. Disposable PVCs and the temporary admin
grant were removed; encrypted checkpoints remain private.
