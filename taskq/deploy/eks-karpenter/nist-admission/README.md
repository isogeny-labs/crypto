# NIST read-only admission bridge

This main-only manual workflow uses the existing `ISOLAB_TRIGGER_ROLE_ARN`
OIDC trust. It performs only STS identity and three fixed AWS Budgets reads.
It does not build, publish, submit, restore or upload a scientific payload.
No IAM, repository-variable or credential changes are part of this component.

The user specified no run-specific spending limit on November 16. This removes
the unanswered total-cap question. The shared account controls in
`../COST_CONTROLS.md` remain: one USD200 daily allowance and USD1000 monthly,
including committed/reserved work, with finite per-attempt resource/deadline
limits. The two daily budgets are never added together.

The report deliberately always says `launch_admitted:false`: AWS Budget charged
spend is delayed and does not establish outstanding reservations or live capacity.
Only allowlisted booleans are retained. Private amounts and raw API errors remain
in an owned temporary directory removed on exit; they are not uploaded as Actions
artifacts. An unavailable permission or malformed spend read fails the check.

Before dispatch, this workflow must land on main through normal checks/review.
The GitHub client can use the existing repository login even when the local
Cloud environment has no AWS profile; the main-only role cannot be borrowed as
a local login. The native sampler still needs matching source/image, successful
allowlisted checkpoint, actual recovery admission, separate cost pilot and
canonical execution freeze. The full target remains60000 accepted curves,
10000 in each fixed field, with all ten percentiles and all attempted indices.

Local synthetic controls cover available reads, missing identity, wrong account,
denied budget access and missing spend fields. These are administrative controls,
not native/scientific validation. Existing November7 source/evidence/claim subjects
stay unchanged. No frozen Python research instrument or unrelated job is run.
