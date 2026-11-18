# H114 run blocked before measurement

Status: infrastructure-blocked; candidate outcome unknown; no scoring run has
started.

## Frozen identities

- Crypto producer PR #4622 revision:
  `4ceb5c032917a5f72a15046eee5aa4aad327a93c`.
- Cryptanalysis verifier PR #1736 revision:
  `f18430232e2814922d95f67e97abdda5d6ff44f6`.
- Machine protocol SHA-256:
  `8d89092331a64a00d51ce7caa1602b035ff582835f8b6f7287912d07590ed51c`.
- Producer source SHA-256:
  `795a80d7cd527ac878e54087664aaa1ccf6c46322f7952c7dd213df0a8b90d57`.
- Verifier source SHA-256:
  `307589c02d437d7d13377c6fce194b441f37579d0b1879263fbe925591a26650`.

## Observed gate failure

On 2026-11-17 a temporary completed `.resume/RESUME.md` passed
`scripts/checkpoint.sh status`'s content check, but no checkpoint transport was
available. `aws` was absent, `CHECKPOINT_UPLOAD_TOKEN` was unset, and
`/home/agent/.config/checkpoint/upload-token` did not exist.

Both `scripts/checkpoint.sh push` and `scripts/checkpoint.sh gate` exited 1:

```text
checkpoint: no way to upload: no AWS profile 'ecc-challenges-new', no CHECKPOINT_UPLOAD_TOKEN, no /home/agent/.config/checkpoint/upload-token
checkpoint: cannot upload, so nothing was checkpointed
```

No immutable image digest or EKS Job receipt exists. Neither `--quick` nor the
full producer mode was invoked, so there are no raw candidate records, scores,
selected predicates, decoder outputs, timings or speedup observations.
The exact branch-specific resume content is retained in
[`CHECKPOINT_RESUME.md`](CHECKPOINT_RESUME.md); the shared tracked resume file
was restored after the failed attempt.

## Recovery condition

Use an authorized existing upload token or AWS identity; do not place a secret
in Git or a PR. From this exact branch, run `scripts/checkpoint.sh push` and
require `scripts/checkpoint.sh gate` to exit zero. Then build the exact producer
revision into a digest-pinned Isolab EKS image, confirm the frozen USD 1.00 and
1,800-second limits, and launch one exhaustive run with zero scientific
retries. The exact output bytes must pass verifier normal mode and all five
tamper rejections before interpretation.

The canonical recurring-access problem is tracked in
[`problems/arity-completion-launch-gates.md`](../../problems/arity-completion-launch-gates.md).
Recording this failure does not resolve the requested experiment or provide
evidence for or against the H114 hypothesis.
