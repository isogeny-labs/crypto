# Native end-to-end continuation: observed status

Observed on 2026-10-26. This is a preparation/access record, not an experiment
result, a frozen protocol, or a completed native implementation.

Update, 2026-10-28: the supplied lab-account AWS credentials pass the official
checkpoint gate; downloaded resume bytes match. The
[new access receipt](access/20261028-aws.json) records that success and the
remaining `codebuild:BatchGetBuilds` denial. All five historical artifacts are
now [published and readback-verified](../ARTIFACT_PUBLICATION_RECEIPT.json).
The observations below remain the original 2026-10-26 failure record.

The cloud environment is running again. Three independent prerequisites remain:

| Prerequisite | Observation | Consequence |
| --- | --- | --- |
| S3 resume checkpoint | Official `checkpoint.sh gate` exits 1: no upload token, token file or configured fallback AWS profile | No new experiment may launch under AGENTS.md |
| Compatible isolated runner | NATS/EKS read-only checks exit 1 because their clients are absent; documented connection bindings are also absent | No authenticated worker preflight or qualified placement has been established |
| Complete native instrument | Native source controls and target-stage solvers exist; native forward construction, Weil-function calibration and inversion do not | Existing stage adapters cannot deliver the requested cold end-to-end rows |

## Checkpoint evidence and policy

The checked-in [gate output](access/checkpoint-gate.txt) records the failure.
The official gate source was read from `isogeny-labs/crypto`, Git blob
`9ec8e848cb3cbcd64a6342b372c3b15b1daa31cf`. It was invoked as:

```sh
bash /workspace/cubic-followup-context/checkpoint.sh gate
```

Working directory: the earlier `/workspace/crypto-pr` continuation checkout.
`.resume/RESUME.md` was populated first. Nothing was uploaded or checkpointed.
The complete resume note was then copied to the proper sparse continuation
checkout `/workspace/crypto-e2e`; it remains ignored by Git.

[AGENTS.md](../../../AGENTS.md#resume-checkpoints-in-s3-before-every-run) says:
“Other runtimes (Codex) run `scripts/checkpoint.sh gate` before launching and do
not launch if it fails.” The preparation files do not override that rule.
An owner-provided `CHECKPOINT_UPLOAD_TOKEN` binding or the documented AWS profile
must make the official gate succeed before correctness runs or measurements.
Credentials belong in environment settings, never in this record or the PR.

## Runner and host evidence

- Presence-only checks found no `ISOLAB_URL`, `ISOLAB_TOKEN`, `ISOLAB_CREDS`,
  `ISOLAB_BIN`, `KUBECONFIG`, `ISOLAB_KUBE_CONTEXT` or `ISOLAB_MCP_BIN` binding.
  This does not prove no compatible runner exists outside this environment.
- The documented runner helper was read from the repository. Its read-only
  `check nats` and `check eks` calls each returned `client missing`, exit 1.
  Neither check reached an authenticated worker. No job or capacity was created.
- This host still exposes neither CPU nor memory PSI through the documented
  `/proc/pressure` or cgroup paths. It cannot qualify the requested local timings.
- The EKS/CodeBuild bridge in
  [HARNESS.md](../../../taskq/deploy/eks-karpenter/HARNESS.md) runs a deployed
  digest-pinned executable. It does not build caller source from a revision
  supplied as provenance. Dispatching its existing fixed validation job would
  not benchmark the missing native transfer.
- Reading repository Actions secret names returned HTTP 403, `Resource not
  accessible by integration`. No secret value was requested, retrieved or
  printed. The failure provides no evidence about whether remote bindings exist.
- The repository workflow listing visible to this identity contains no
  checkpoint/isolab continuation workflow. No workflow was dispatched.
- `conductor check` returned exit 127, `conductor: command not found`.

## Continuation order

1. Configure an existing compatible runner connection and checkpoint binding.
   For NATS, use the documented hub URL and token or credentials-file binding.
   Confirm the official checkpoint gate succeeds before launching any experiment.
2. Materialize needed workspace paths in the sparse checkout; port the full map
   from the archived contracts using native execution. Pass the p=157 two-step
   fixture and the p=109 fixture, source/target equality and exceptional-case
   controls. Restore/update the resume checkpoint as code and plans change.
3. Extend the common native measurement interface, freeze the final executable,
   canonical protocol and budget, then preflight the actual compatible worker.
4. Run the [matched cold comparisons](REQUIREMENTS.md), retain every attempt,
   audit answers, price all work and perform the required independent replay.

No new cryptographic correctness run, timing run, measurement row or performance
claim was produced by this continuation. Root release tests were not run:
the mandatory experiment checkpoint remains unavailable. Documentation and Git
diff checks do not substitute for those tests. The earlier 58 diagnostic
recoveries remain historical evidence with their original limitations.
