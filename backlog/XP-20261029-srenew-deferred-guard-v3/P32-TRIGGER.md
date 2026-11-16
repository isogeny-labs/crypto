# v3 §2 trigger: first-study P32 outcome

Recorded 2026-10-29. It was read only from the five gate files of `ic-dg-p32-20261029` (pod `cb4dfbae-e2f0-4931-87c3-e3eb7ced492c`), each checked against the job's `SHA256SUMS`. No first-study P32 `compare`, `band` or IC record had been opened.

| file | sha256 |
|:--|:--|
| job-exit.json | fd8aecaa97cdef10920c4aa5a46b2588bc81e518394f09080879ec57fe1ceb9d |
| audit.json | 68f2397c7fab5506535b3be5cc8c989937c5728cad592e690466f083da4ec5fd |
| l2-gate.json | b214132fc17e473f9b6fd09fa19cb19439d0264a7aa95ad0eb2af863d08da1f3 |
| input-spec.json | c91caf7b43725dca3dc45f6651414a4407b407dde18badba94617135a67d13f2 |
| build-info.json | 6a5baeea887ba260d6f0acdd166759c0323ed179f1c1f697bf2bd1200ed508cc |

## Facts

- **Exit:** `exit_code` 42, from a single pod.
- **Audit:** ok, session complete, 0 problems.
- **Spec:** `input-spec.json` is JSON-equal to the first study's `spec-P32.json`.
- **Build:** `build-info.json` names `be2969ce` and ecbench `88070375…412c`.
- **L2 gate:** `accepted` false.
  - 2,688 executions: 2,676 graded L2, 12 graded L1.
  - The 12 below L2 are 6 `rho-neg` and 6 `rho-neg-aa`. Each has one L2 blocker: `solve too short to admit wall time`.
  - `invalid_outcomes` is empty.
- **Rerun:** none. There was no host loss.

## Outcome

The first study's §6.1 ("the L2 gate admits every execution") fails at 32 bits, so 32 bits runs under this protocol.

TRIGGER: first-study P32 failed §6.1
