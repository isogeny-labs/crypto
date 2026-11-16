#set page(paper: "a4", margin: 17mm)
#set text(size: 9pt)
#set par(leading: 0.5em)
= Pre-merge commit-image integration
#link("https://github.com/aburan28/crypto/actions/runs/39527210297")[GitHub Actions run] · #raw("pr2342-abe6b1b52f-premerge-20261021")

Verified at #raw("2026-10-21T02:19:47.328510+00:00")

#image("flow.svg", width: 100%)

== Observed execution
GitHub Actions built the selected repository commit. The reviewed publisher ran from the authenticated harness against a temporary EKS Docker daemon, then invoked the VPC launcher through CodeBuild. No source binary ran during publication. This pre-merge check is distinct from the still-required full main-branch GitHub Action and does not establish its OIDC publishing step. The native executable was built as static x86-64 musl with Rust 1.99.1, without default Redis TLS features. The native build job had no AWS identity.

The public 20-bit validation fixture ran on an M8a Spot worker: eight logical CPUs, eight physical cores, one sibling per core. The pod had four CPUs, 4 GiB memory and a 900-second deadline including provisioning. GitHub build, harness publication and CodeBuild startup were separate setup costs. No EKS measurement was automatically retried.

The fixture uses curve seed 59297 and target seed 20261002, two targets, five measured rounds and one warmup. Its nine arms are rho-neg, rho-plain, rho-frozen, bsgs, bsgs-il, bsgs-neg, kangaroo, ic-mitm and rho-neg-aa. The frozen input-spec.json records every parameter.

All *108 executions earned L2* and verified. All *90 measured native replays* matched. The native A/A control was admitted. This is a check of integration, isolation and correctness; no algorithm speedup or external-tool ranking is claimed.

A/A pairs: #raw("10"); median wall ratio: #raw("1.0148534868574546"); recorded 95% interval: #raw("[0.9951100498878953, 1.0324926862547843]").

All *19* S3 manifest payloads were downloaded and hash-checked. The executable downloaded from GitHub matched the S3 binary SHA-256. The actual pod image matched the published ECR digest, and the measured input matched the requested spec.

== Identity and sources
Tested source: #raw("e862ee96a5d1109ef284a8ddf8e21528d2f988af")

Build workflow revision: #raw("c317f4b344d7fe49dd296e3e50ae9f429e9187ec")

Reviewed publisher revision: #raw("abe6b1b52fe976a05b9382655be21b2db2af2034")

Native SHA-256: #raw("3958d37e00925c52f69036f908086fd48cb5216f6ee66272a225e3f674b1472b")

Job UID: #raw("c86beb9c-058a-4914-b8d2-68106f1914d0")

Pod UID: #raw("f7ce31c7-60aa-41d9-90b4-f10832f906c8")

Source evidence: ../protocol.json, ../verification.json, ../publisher-pod-final.json, ../published-image.json, ../client/result.json, ../launcher/ and ../artifacts/ (retained in ../artifacts.tar.gz for the repository copy). Exact S3 prefixes and image digest are in verification.json.

The earlier dispatch 39462747363 returned HTTP 500, had no jobs when observed, and rejected cancellation; it supplied no measurement. This additional pre-merge run has its own frozen protocol and identities. The main GitHub Action remains a separate required validation after passing CI/review and merge. Algorithm leaderboards and canonical curve graphs are unchanged. A process diagram is used because this integration check reports one configuration without a performance comparison.
