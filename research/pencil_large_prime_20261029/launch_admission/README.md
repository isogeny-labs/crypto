# Publisher trust repair proposal

This is a reviewable proposal, not an applied IAM change. The existing role trusts GitHub OIDC only for `aburan28/crypto` on `main`; the repository now resolves to `isogeny-labs/crypto`. The proposed policy replaces that exact subject, preserves the main-branch restriction, audience, federated principal and action, and changes no attached permissions.

The existing research authorization is scoped to the job-submitter role. Applying this administrative change requires separate authorization. Re-read the current policy and compare it structurally with `publisher-trust-current.json` before applying the reviewed proposal. A different policy requires a fresh review; do not overwrite concurrent changes.

This change alone would not deploy the updated recovery controller or qualify a runtime. The tested 8 GiB source change is PR #2871; its publication/deployment remains a separate action under the approved infrastructure path. No experiment is authorized by these proposal files.
