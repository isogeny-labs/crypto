# Runner prerequisite and report-input CI repair

Three main failures were reproduced from their exact job logs:

- Autofix triage job 123215190821 exits 127 because `gh` is absent from the
  EKS runner. Ensure GitHub CLI before use in both triage and fix jobs, and
  add it to the maintained runner image.
- Backlog integration job 123209500309 cannot link `icprog`: the native
  linker cannot find `-lz`. Install `zlib1g-dev` when absent in both backlog
  jobs and include it in the maintained image. Validation commands and the
  cairn-required integration tests remain intact.
- Reports job 123228689046 cannot resolve an artifact remote because its
  sparse checkout excludes `.artifacts/config.json`. Include the config,
  provide the same pinned AWS CLI used by the other artifact consumers,
  configure the selected existing identity before hydration, and avoid empty
  per-step access-key overrides. The observed failing job selects keys, not
  an OIDC role. Both configured credential modes remain supported.

Report hydration receives exactly the uploader's original NUL-delimited
file selection. A pure Git fixture confirms ordinary-push and manual-run
selection, including paths containing spaces and unrelated research data.
The upload loop and selection algorithm are byte-identical to the originals;
only their order relative to hydration changes. An empty selection does not
invoke an unfiltered artifact pull. This avoids downloading unrelated research
archives and preserves all files the uploader previously selected.

Validation: actionlint and ShellCheck pass for all three workflows; YAML
structure confirms both authentication branches precede hydration and that
no access-key override shadows the configured identity. A container using
pinned Ubuntu 22.04 digest
`sha256:e0f80c829509ef4df9e76bd8944eb7b8ec25cbb379b112a7f9448b2c77a77c9b`
installs the declared packages, runs GitHub CLI, and compiles and executes
an actual C program linked with `-lz`. Raw outputs and controls are retained.
The complete runner image was not rebuilt here; existing-runner workflow
fallbacks do not depend on an image rollout. No runner assignment, credential,
IAM grant, trust policy, research threshold, test or job timeout changes.

Sources checked: failing job logs, the maintained ARC image, existing
workflow commands, artifact path selection and hydration implementation,
`docs/reports-s3.md`, and official GitHub workflow syntax. The authenticated
report upload is verified by the normal post-merge workflow, not this local
package or Git fixture. Current-head checks and fresh review remain required.

## Blocked-card verifier templates

The same main backlog validation reported unresolved `{artifact}` templates
in two blocked cards. Canonical `outputs.verify` uses shard variables such as
`{out}`; `{artifact}` belongs to the separate cairn-checker interface. Match
the existing run outputs exactly: the NIST ecbench session is `{out}` and the
volcano runner writes `{out}/records.private.jsonl`. Both cards remain blocked,
their frozen protocol bytes and all other card fields are unchanged, and the
independent cairn checker keeps `{artifact}`. Native validation now accepts
all 53 cards with the merged-source check, zero failures. The validator's
binary digest, exact commands, protocol blob identities and output are saved.
The former prime-base code-ancestry error is already fixed by its actual main
merge; no source pin or historical receipt was rewritten. Initial local
missing-problem-path errors were resolved by materializing the tracked problem
records before acceptance. No experiment, private-journal upload or checkpoint
action was performed.
