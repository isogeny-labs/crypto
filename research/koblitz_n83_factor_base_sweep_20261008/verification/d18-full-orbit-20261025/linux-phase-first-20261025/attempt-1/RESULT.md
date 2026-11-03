# Linux phase-first attempt 1: S3 access gate

GitHub Actions [run 40220890684](https://github.com/aburan28/crypto/actions/runs/40220890684)
started from exact source and protocol commit
`2b7b04e1d613debe9d71c6403afba09ac3d9b289`. Checkout, source/image
identity, and AWS credential configuration passed. The fresh S3 GET stopped
on `AccessDenied` for the complete D18 `manifest.json`: the configured
GitHub principal `rho-gh-snapshot-readonly-20261024` has no identity-based
`s3:GetObject` allow for that key. The binary build, hard-cgroup preflight,
constructed arm and natural arm were all skipped. This is an access result,
not a relation-yield observation.

Provider artifact `12608973073` is retained here as [`provider.zip`](provider.zip):
2,191 bytes, SHA-256
`2f417b24cf499cc9d6f30ddb28c05972932d867962fcdb32ac768f6fd7dd5b81`.
The [independent audit](audit.json) checks the run's terminal job and step
state, source commit, all eight ZIP members, empty manifest GET metadata,
exact AccessDenied resource, masked credential fields, and absence of any
build or worker output. Raw provider metadata and the failed step log are
committed beside it.

The [proposed policy](exact-read-policy-proposal.json) is **not applied**.
It grants only `s3:GetObject` on the exact D18 manifest and compressed
object, without bucket listing or write actions. Applying it to the existing
GitHub principal would allow a hosted runner to fetch those private S3
objects; see [the review note](IAM_READ_PROPOSAL.md). Do not rerun the same
unchanged S3 access gate until that external permission is explicitly
authorized and verified. `selected_best_total_runtime` remains `null`.
