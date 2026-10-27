#set page(paper: "a4", margin: 17mm)
#set text(size: 9pt)
#set par(leading: 0.5em)
= Periodic recovery qualification B
*Overall result: failed. Recovery passed; fresh L2 admission failed.*

#raw("ecbench-periodic-20261022-b")

#image("grades.svg", width: 100%)

== Frozen scope
Public 32-bit fixture, curve seed 59297, two targets (seed 20261002), rho.negation reference, rho.plain baseline and matching negation A/A control; 600 rounds plus one warmup. Checkpoint interval 30 seconds, original deadline 3,600 seconds, at most two restores, zero measurement retries. Account quota 96 vCPUs, lab cap 64. Runtime source: #raw("5499eeb9d72e6e777cb0d28b0ade41afbfff90b7"). Full immutable request and identities: protocol.json.

== Recovery observed
Two encrypted, completed snapshots committed with full versioned archive readback. After the original controller stopped and its dedicated worker terminated, generation 2 restored on a replacement c8a.2xlarge. The first 2,486 records / 11,113,922 bytes remained identical; computation completed 3,606 verified records and 3,600 matching native replays. All computational timing remains invalid.

Both restored processes were stopped with UID/GID 1000, empty supplementary groups, zero capabilities, no_new_privs and Seccomp 2. Both checkpoint captures succeeded initially: live zombie recapture was not forced, although native regression tests cover it. See restored-control and g1/g2 metadata.

== Fresh epoch rejected
The distinct fresh Job completed native verification, but sequences 676, 1406 and 1826 each recorded three excess busy ticks on CPU 4. All had zero steal and no SMT siblings. The retained counters do not identify a noise source or prove a kernel-accounting defect. No grade or threshold was changed. L2 exited 42 before A/A, which remains unevaluated. The logical run and restarted CodeBuild controller failed; no benchmark retry occurred.

All 18 emitted S3 payload hashes matched after download. The frozen acceptance required all 3,606 L2, admitted A/A and 19 payloads; those criteria were not met. Earlier positive F evidence does not turn this run into a pass.

== Evidence and remaining work
REPORT.md links the exact private S3 result prefix and logical receipts. public-evidence.tar.xz retains the full records, audits and identities; verification.json and verify-failure.sh document checks. Checkpoint memory stays in private encrypted S3, never Git. Both owned PVCs and all three temporary operator access associations were removed after preservation; committed snapshots remain retained.

The runtime pin now matches the deployed checkpoint repair with this limited qualification. The separately frozen main-only GitHub Action remains unverified and retains its original acceptance criteria. No identical B rerun is scheduled. This is infrastructure evidence, not a timing or algorithm speed claim; canonical performance graphs and leaderboards are unchanged.
