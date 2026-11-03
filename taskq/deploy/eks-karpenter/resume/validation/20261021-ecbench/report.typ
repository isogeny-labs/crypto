#set page(paper: "a4", margin: 17mm)
#set text(font: "DejaVu Sans", size: 9pt)
#set par(leading: 0.5em)
= Native ecbench CRIU / EBS qualification
Iso Lab · 21 October 2026 · operator-controlled validation

*Observed:* a native crypto process tree resumed on a replacement Spot worker
and completed 36 records. Independent native replay reproduced all 30 measured
records. These computations remain *L0, with invalid benchmark timing*. A distinct
fresh epoch earned 36/36 L2 observations, 30/30 replays, 10 admitted A/A pairs and
19/19 verified S3 payload hashes. This run does not certify automatic recovery.

#image("flow.svg", width: 100%)

== Frozen workload and failures
The supported workload is a public 32-bit prime_search fixture (seed 59297), two
targets (seed 20261002), rho.negation, rho.plain and matching A/A; five measured
rounds and one warmup. The original UTC deadline was 2026-10-21 06:25. All
computation and fresh-epoch execution finished before it; later evidence
collection and independent replay do not represent new admitted measurements.

The first 36-bit request was rejected before a child started. A supported measured
process then failed CRIU dump because it held a perf_event descriptor. Both failures
are retained. A native launcher forced L0, denied performance-counter creation,
and retained the original math workload. Its actual parent and active child
checkpointed with CRIU 4.2.1 on AL2023 kernel 6.18.54-121.166.amzn2023.x86_64.

The first restore inherited supplementary group 0. After that copy completed and
its Pod was removed, a second nonconcurrent restore cleared the supervisor's
groups. Its parent and child then had UID/GID 1000, empty supplementary groups,
zero capabilities, no_new_privs and seccomp. That copy also completed 36 records
and passed 30 independent native replays. This establishes the tested trusted
process procedure, not isolation for arbitrary untrusted images.

#table(columns: (36%,64%), inset: 4pt,
 [Check], [Recorded evidence],
 [Original worker], [#raw("i-0ab01e249ab5475f7"); c8a.2xlarge, us-east-2c],
 [Replacement worker], [#raw("i-0ab0f5f4afc72d5d6"); c8a.2xlarge, us-east-2a],
 [Writer fence], [Original EC2 confirmed terminated before replacement Pod creation],
 [Committed filesystem], [#raw("snap-0ead1fd75c8add942"); encrypted, complete; 52 file hashes],
 [Private archive], [Versioned S3 archive passed complete download / byte comparison],
 [Fresh A/A], [10 pairs; recorded 95% ratio interval 0.9997380265–1.0011447800],
 [Fresh results], [36 L2 observations, 30 native replays, 19 S3 payload hashes],
)

== Identity, scope and evidence
Source: #raw("cb2ec6873f30b683f7885b53c410d69965f02c20").
Native SHA-256: #raw("583e62f0836eae43330c4b36bbe3d145dacb01de3132436c71f6924445c6a5a9").
Fresh Job: #raw("criu-ecbench-20261021-epoch-1").
Fresh Pod UID: #raw("848af771-ccde-46ce-aef5-7e73e2de357e").

The fresh job used the maintained AWS/CodeBuild harness with a pinned deployed
image. It was not the pending PR 2342 full main-branch commit-building Action.
A transient Spot quota/capacity error cleared within the unchanged 96-vCPU account
quota and 64-vCPU pool cap. No failed L2/A/A epoch was selectively retried.

Full identities, protocols, exact commands, private archive version and source
hashes are in #raw("REPORT.md"), #raw("verification.json"), #raw("COMMITTED.json"),
#raw("protocol-v3.json"), #raw("groups-audit.json") and the fresh-epoch artifacts.
Raw memory stays private and is absent from Git. Canonical performance graphs and
leaderboards remain unchanged: this is infrastructure qualification, not an
algorithm speed comparison. The diagram records execution and validation counts.
