#set page(paper: "a4", margin: 16mm)
#set text(font: "DejaVu Sans", size: 10pt)
#set par(justify: false)
= Automatic Isolab experiment queue
Implementation and validation · 2026-10-27

The requested feature discovers reviewed, unrun experiments, submits bounded
jobs to the existing Isolab EKS cluster, and returns observations in a results
PR. The controller, GitHub workflow, two execution adapters, and recovery path
are implemented. Live cloud qualification remains outstanding.

#image("architecture.svg", width: 100%)

*Dispatch.* A registration binds an approved expspec record, prospectively frozen
protocol, immutable source and image, input hashes, independent verifier, and
resource envelope. An atomic GitHub tag prevents duplicate execution. Existing
claims only collect. CodeBuild provides the private-EKS route using native MCP.
The Actions runner executes no cryptographic workload.

*Return.* Job/Pod identity, observed image, request and output hashes, source,
protocol, export schema, and execution-cell coverage are checked. Native benchmark
L2, replay, input-spec, and A/A admission are retained. Compact public observations
are committed on a results branch; raw data stays in private S3. Failure and
invalid records remain observations pending independent scientific review.

*Validation.* The native test suite uses fake GitHub/AWS/MCP services and sealed
fixture observations. It exercises concurrent claims, uncertain submission,
restart collection, changed identities, literal argv, independent verifier
failure, canonical export compatibility, scoped PR publication, and queue
progress. Exact commands, counts, and failures are in `validation.json` beside
this report. Tests do not establish live EKS or IAM readiness.

*Deployment gaps.* This session has no AWS identity; no real EKS job was launched.
The workflow must land on main with OIDC/project/bucket variables and a private
canonical-schema reader token configured.
Hosted runner billing must permit job starts. The NIST filtering entry remains
blocked on published checkpoint inputs, a tested task image, a prospective EKS
and storage/budget amendment, and approval. Resumable/CRIU protocols are not
adapted by this ordinary-job queue.

*Sources.* Repository `queue-experiment`, `isolab-eks`, native expspec and remote
build verifier; canonical protocol/export schemas at crypto-autoresearcher
commit `6a69b2d02ecd53f1649bcf56e23e4ffe91d953ad`; AWS CodeBuild StartBuild;
GitHub Git references and workflow token event rules. The setup guide has links,
field requirements, credential boundaries, and recovery instructions.
