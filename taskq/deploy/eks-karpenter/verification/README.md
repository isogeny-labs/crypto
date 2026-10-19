# Deployment verification — 2026-10-19

All observations below are from account `104186811093`, region `us-east-2`.
This is infrastructure validation, not an experiment or performance claim.

| Requirement | Observed result |
|---|---|
| EKS cluster and prerequisites | `isolab` ACTIVE, Kubernetes 1.36, two Ready system nodes, dedicated VPC; see `cluster.json` |
| Cluster add-ons | VPC CNI, CoreDNS, kube-proxy, EBS CSI and metrics-server ACTIVE without health issues; see `addons.json` |
| Karpenter | Helm 1.15.0 release deployed, controller 2/2 available, EC2NodeClass and NodePool Ready |
| Pod-driven Spot provisioning | With zero workers, `smoke-20261019-175032` caused Karpenter to launch `c8in.xlarge` Spot instance `i-080b2a06efdab61ac` |
| CPU allocation | Guaranteed QoS, `Cpus_allowed_list: 1,3`; live kubelet reports static CPU manager, full physical cores, single NUMA node; see `worker-kubelet-config.json` |
| Pod IAM and durable output | Pod assumed `isolab-job`, uploaded its result to S3, and the deploying client read it back; see `spot-smoke-result.txt` and `spot-smoke-pod.json` |
| Failure propagation | Expected exit 7 produced Failed Job and launcher exit 1; see `expected-failure.json` |
| Automatic idle cleanup | Karpenter deleted the first NodeClaim and AWS reported that instance `terminated`; see `first-worker-terminated.json` |
| Native crypto image | Freshly compiled ecbench from `cb2ec6873f30b683f7885b53c410d69965f02c20`, no custom Rust flags; private ECR digest in `../image.json` |
| Private image → pod → S3 | `ecbench-plan-20261019` completed on a newly provisioned worker; its S3 plan and build-info files matched local image outputs byte for byte |
| Harness RBAC | Live impersonation check: create Jobs in `isolab` allowed, read Secrets denied |
| Local checks | ShellCheck, Bash syntax, cfn-lint, eksctl dry-run, exact-version Karpenter CRD schemas, live Kubernetes admission validation passed |
| Repository checks | `cargo test --release --lib`: 4,405 passed, 0 failed, 118 existing ignored tests; no tests were disabled |
| Native MCP connector | Six tools loaded into Codex; Rust unit tests (5/5), Clippy with warnings denied, formatting and ShellCheck passed |
| MCP live smoke | `mcp-plan-20261019-183708`: stdio submission, duplicate rejection, successful private-image plan, logs, downloaded S3 hash, intentional exit-7 failure and cancellation passed; see `mcp.json` and `mcp-artifacts.json` |

The first full library run lacked eight runtime fixture inputs in the sparse
checkout. The tracked fixtures were materialized, and the full suite was rerun
successfully. No Rust source was changed. Both logs are retained in S3.

Native plan SHA-256:
`e37b221bfe4d4460099c3129dc42bc63cd217eff31cf38cee928c5da6b5a7e84`.
The plan validates executable/container/input plumbing; it runs no timed
cryptanalytic experiment and establishes no speed or fidelity claim.

`artifacts.json` records the size, SHA-256 and durable S3 URI of full logs and the
native plan. Fetch with `aws s3 cp URI FILE` and verify the hash. Compact cloud
receipts are committed beside this file. The smoke commands are in `../smoke.sh`
and the native plan example in `../README.md`.

The native `isolab_k8s_*` tools provide the Kubernetes harness path. The existing
Iso Lab NATS/MCP execution contract has **not** been migrated to Kubernetes.
A restricted pod on an EC2 VM does not meet its strict/bare-metal
fidelity policy. GPU pools, checkpoint/restart integration and live PVC
provisioning tests were not part of this verification. The installed EBS driver
and its IAM role are healthy; durable output was verified through S3.
