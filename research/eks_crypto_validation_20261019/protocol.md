# EKS crypto correctness and S3 persistence check

Requested: execute a real crypto task through the EKS harness and verify that its results are saved to S3.

Frozen workload: the unchanged `docs/ecbench/specs/smoke-prime.json` bundled in the deployed image. SHA-256 `684c9aebbc0afd0ae89b01cec9439d3330898719cb3d56b30155544d6aa79cc2`. Curve constructor: prime_search, 20 bits, seed 59297; two public synthetic known-answer targets, target seed 20261002. Nine solver arms, one warmup and three measured rounds per target/arm: 72 planned executions, including 54 measured runs. Reference and control are rho.negation; all arms and algorithm seeds come from the frozen spec/plan.

Image: `104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:67eea58f5d4a7a8882b55f089c6063bb49fad58d40260ee9b0ceb349fc0e4581`. Native ecbench source revision `cb2ec6873f30b683f7885b53c410d69965f02c20`; binary SHA-256 `583e62f0836eae43330c4b36bbe3d145dacb01de3132436c71f6924445c6a5a9`.

The question is whether the existing native crypto solver completes its actual computations in an EKS pod, verifies the recovered scalars, replays every measured deterministic record, and persists all evidence to S3 with matching hashes. Success requires a completed Job, all planned records verified, a passing full replay audit, successful S3 download, and a match for every file in the pod's SHA256SUMS. A second audit runs on the downloaded session using the same frozen binary outside the worker pod.

Limits: 2 requested vCPUs, 4 GiB memory, 1,200-second Job deadline, zero retries. Each solver execution retains the spec's 120-second limit. `--cpus inherit` uses the pod placement; `--allow-busy` retains observations without requiring a quiet host. Results retain their actual isolation grades. No timings, method rankings, speedups, scaling claims, or deployed-curve security conclusions are made. This is a public fixture correctness test; it is not recovery of a real-world secret.

The exact MCP arguments and pod command are in `request.json` and `run.sh`. Failure artifacts are uploaded by an exit trap when possible; Spot termination or a hard kill can still prevent upload. A failure is retained and reported, never replaced silently. Session files are immutable; reports and receipts stay outside the sealed session.
