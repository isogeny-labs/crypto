#set page(paper: "a4", margin: 15mm)
#set text(font: "DejaVu Sans", size: 9pt)
#set par(justify: false, leading: 0.6em)
#heading(level: 1)[Real crypto on EKS: correctness and S3 persistence]
2026-10-19 · Session ECBS1hc931106fd275

*Result:* 72/72 elliptic-curve solver executions verified. All 54 measured runs reproduced in both the pod audit and a second audit of the S3 download on the harness host. Neither audit reported a problem.

#image("../../verified-executions.svg", width: 100%)

*Workload.* Existing public synthetic 20-bit prime-field fixture; modulus 563153, subgroup order 562427. Two targets, three measured rounds and one warmup across nine arms. Every arm recovered the known scalars 330146 and 138031. Verification checked each equation [k]G = Q.

*Provenance.* Codex MCP Job ecbench-real-20261019-1922 in EKS isolab, us-east-2. Spot c8in.xlarge, Intel Xeon 6975P-C; 2 requested vCPUs, 4 GiB. Native ecbench source cb2ec6873f30b683f7885b53c410d69965f02c20. Full image digest, inputs and exact commands: ../../protocol.md and ../../request.json.

*Persistence.* All 17 S3 objects were downloaded. Every one of the 16 payload files matched the SHA-256 manifest produced inside the pod. The sealed session was then fully replayed outside the worker pod using the same frozen native binary.
#raw("s3://isolab-results-104186811093-us-east-2/\nruns/ecbench-real-20261019-1922/\n5319a3b0-ba85-44cc-be19-02d123364899/", block: true)

*Evidence.* ../../artifacts/session/records.jsonl; ../../artifacts/audit.json; ../../host-replay.json; ../../cloud-receipt.json. Graph data: ../../counts.json (verified records grouped by arm). Counts are complete observations; no uncertainty interval applies.

*Second audit SHA-256:*
#text(size: 7.5pt)[#raw("386d52abd0a8a36f32f787051256d5c88e5e7629bbfba595e07085b14a90ea82")]

*Limits.* All records earned L0: the quiet-start and host-eviction requirements for stronger isolation were unmet. No timing comparison, speedup, deployed-curve security claim or strict-fidelity result is reported. The second audit is a separate execution of the same implementation. This check adds no new curve or performance finding; canonical scoreboards and curve diagrams are unchanged.
