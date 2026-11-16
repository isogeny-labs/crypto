
## 2026-11-16T16:00:12Z — NIST read-only OIDC admission bridge

### What was done (commands, files, features)

Prepared a main-only manual workflow using the existing lab OIDC role. Its only
AWS operations are STS identity and three fixed budget reads; it creates no build,
worker, image or scientific upload. Five local synthetic administrative controls
completed, including wrong-account refusal and denied/missing budget cases.
PCC-20261116-50d4da remains an unsigned statement. [pcc:unverified]

### Decisions made (and why)

The user set no run-specific spending limit. Stop asking for that cap; retain the
shared USD200/day and USD1000/month controls and finite paid-attempt limits.
Local AWS credentials are absent, but the existing GitHub client works after a
focused sandbox-network retry. The current experiment workflow builds ecbench;
do not dispatch it as the NIST sampler. Add a separately reviewed read-only
bridge to establish real identity/budget access through existing main OIDC.
Only booleans are exported; private financial amounts/raw errors remain temporary.
A successful diagnostic never admits launch while reservations/capacity are unknown.

### Open threads (unfinished, pick up next time)

Land this workflow through actual required checks/review, then dispatch it on main
and read its allowlisted report. The native source/image, isolated checkpoint,
worker-loss admission, separate cost pilot and canonical primary freeze remain
necessary. The target remains60000accepted, with ten percentile outputs; no study
nonce, sample, paid EKS job, policy/identity/quota change or new keys occurred.
Parent3610 and its historical evidence are not edited by this tooling component.

### Errors (what went wrong, how it was handled)

Initial GitHub request failed on sandbox socket permissions; the same read worked
outside the sandbox using existing login. No-checkout sparse clone initially had
an empty index; explicit read-tree materialized only selected paths. Conductor is
absent, no reservation claimed; documented warning fallback applies. Lessons brief
read for isolab-eks (including main-before-log and prose-lint lessons). KB MCP is
not loaded; current ACCESS/COST_CONTROLS and trusted workflow source provide the
administrative fallback. Initial claim command lacked the new claim-type field;
used the canonical data classification after reading help. No gate was weakened.
