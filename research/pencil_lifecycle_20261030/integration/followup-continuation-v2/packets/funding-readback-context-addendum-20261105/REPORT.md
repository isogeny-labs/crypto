# Existing MCP context readback addendum

At 2026-11-05 16:19 UTC the explicitly selected existing MCP kubeconfig and context reached the Kubernetes API as the same current WorkerRole. NodePool list and the isolab/jobs ResourceQuota get were both denied by Kubernetes authorization. Exact commands, error text, exits and hashes are retained. No credentials, kubeconfig contents, new role or cluster mutation are included.

This is a different observation from the historical default-context STS failure preserved in [the funding packet](../funding-readback-20261105/REPORT.md). The explicit MCP context bypasses neither RBAC nor a spending limit: live enforcement remains unverified. The existing [PDF](../funding-readback-20261105/report.pdf) remains a dated record of the earlier context; this additive record supplies the current authorization-layer distinction.

The same observation window also confirmed GitHub's remote branch at 763a024c40c405817e1447f0ad9ecae09c1219b8 while refs/pull/3471/head and the PR API still reported e09d78d62447a12409e0eafe8917094d548db4b8. The repository and branch identities match. The pull merge ref was absent. The server-side reason for delayed synchronization is unknown; no retry, close/reopen, empty commit or push was performed for this observation.
