# H115 run blocked before measurement

Status: infrastructure-blocked; candidate outcome unknown; no scoring run has
started.

## Frozen identities

- crypto producer draft PR #4625 source revision: `6d8def5473`;
- producer source SHA-256:
  `f0644129c8d7c0161cbddac3283f94453983d25b6f6cdeaf327b170c1fc579bd`;
- cryptanalysis verifier draft PR #1739 revision: `ed206d8d`;
- verifier source SHA-256:
  `e625542931d0b8b6b6b753ce19b495a03b9640c7f1cdb6ba06b3d7d9196e1a37`;
- machine protocol SHA-256:
  `c8412d94f3e6cc928c13c93572a8796cea3a983b5fcd4e6b043a75bc5b1a2363`.

## Observed gate failure

On 2026-11-18, `.resume/RESUME.md` passed the checkpoint content check and
the source branch had zero commits absent from remote refs. Both
`scripts/checkpoint.sh push` and `scripts/checkpoint.sh gate` then exited 1:

```text
checkpoint: no way to upload: no AWS profile 'ecc-challenges-new', no CHECKPOINT_UPLOAD_TOKEN, no /home/agent/.config/checkpoint/upload-token
checkpoint: cannot upload, so nothing was checkpointed
```

No checkpoint, immutable image digest or EKS Job receipt exists. The producer
was compiled and run only in `--selftest` mode before this gate; its `--run`
path was not invoked. There are no candidate records, scores, selected IDs,
decoder outputs, timings or speed observations.

## Recovery condition

Use an authorized existing upload token or AWS identity without placing a
secret in Git or a PR. From this exact branch, run `scripts/checkpoint.sh push`
and require `scripts/checkpoint.sh gate` to exit zero. Build the bound source
into a digest-pinned Isolab EKS image, retain the frozen USD 1.00 / 1,800-second
limit and zero scientific retries, and launch exactly one exhaustive run. Its
exact bytes must pass cryptanalysis verifier revision `ed206d8d` plus all eight
one-at-a-time copied-packet tampers before interpretation.

The recurring access dependency remains tracked in
[`problems/arity-completion-launch-gates.md`](../../problems/arity-completion-launch-gates.md).
This infrastructure failure is not evidence for or against the H115 moment
hypothesis.
