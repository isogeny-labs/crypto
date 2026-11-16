# Line-layout tournament, v3 amendment 2 (protocol version 5)

Filed 2026-11-06, before any tournament data. Machine-readable record:
[`protocol-v5.json`](protocol-v5.json), `IC-LINE-LAYOUT-E2E-FP20-V3A2-20261106`.
It amends [`protocol-v4.json`](protocol-v4.json).

## Why

The version 4 qualification job, `ic-line-v3a1-qual-20261106`, ended at
03:02Z with `launcher_failed`, exit 127, on CodeBuild
`d6270339-9648-4adc-ab66-56f1374c9f3c`. The launcher's `run-request` could not
find `isolab-eks-mcp`. Since a34125841, `launch-remote` overrides the
buildspec with one that does not put the launcher's `bin/` on `PATH`.

No EKS job, pod or measurement existed, and no scientific seed was used.
Stopping rule 1 ended version 4 at that point. PR #3517 fixes the launcher.

## What changes

- **Launcher:** `run-request` with the PATH fix from #3517. The launch runs
  from a `main` checkout that contains the fix.
- **Job names:** `ic-line-v3a2-qual-20261106`, `ic-line-v3a2-s1-20261106` and
  `ic-line-v3a2-s2-20261106`, in [`launch-v3a2/`](launch-v3a2/). Apart from
  `job_name` and the provenance label, the requests are identical to
  `launch-v3a1/`.

Everything else is unchanged from version 4:

- the pinned image, ecbench binary, spec and seeds;
- the arms, controls, decision rule and `analyze.sh`;
- the USD 5 ceiling, which also covers the failed version 4 attempt;
- the claim's scope: the `ea8e9ef7` executable.

The stopping rules carry over: exactly one qualification job, then exactly two
scientific jobs, with no retries.
