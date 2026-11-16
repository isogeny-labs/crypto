# SIBLING-POINTS Stage 1 on Iso Lab EKS: sibling pools and `cv=0` at k = 28 and 32

Status: frozen before any data of this panel (2026-11-10). Every number below is a
pre-registered expectation or a prior measurement, labelled `[pcc:unverified]`.

## Why these leads (selection, 2026-11-10)

Under the success criterion (measured improvement over the current index-calculus
baseline), the two strongest open leads in the repository on this date are:

1. **Sibling points** (`wwalk sib=8,pool=64`, PR #3847, `../SMOKE.md` second smoke): total
   bit-op ratio to `tw2cj-lazy` 0.775 at k = 20 and 0.691 at k = 28 at w = 0 on one curve,
   12 targets x 2 rounds, L0, local; unresolved inside the §5a band (loses at w = 1)
   [pcc:unverified]. The smoke's own open question: does the mechanism pay once the
   fixed pool set-up is a small share (k = 32)?
2. **`cv=0` target walk** (`research/curve25519_engine_items_20261110`, Addendum 1):
   twcj-cv0 / twcj 0.879 at k = 20 and 0.950 at k = 24 [pcc:unverified]; never measured
   on the two-seed `tw=2` walk, which is now the ratcheted incumbent family (rounds 36/37).
   `cv=0` is exclusive with `lazy=1` (plugin text), so the comparison is `cv=0` against
   `lazy=1` on the same walk.

The 2026-11-10 lead sweep (`research/lead_sweep_20261110/SWEEP_REPORT.md`) produced no
signal that outranks these (§2-§3 there: no below-sqrt(p) evidence; both flags read as
artefacts). Knowledge base: the `research-kb` server and CLI are not available in this
session (as for the 2026-11-10 desk note); the selection rests on the repository's own
records cited above.

## Design

- Specs `spec-k28.json`, `spec-k32.json`: the five SNC-panel curves of
  `../../snc-panel/spec-k{28,32}.json` (same explicit curves, new target and measurement
  seeds), public targets, 8 targets per curve, 6 rounds, 1 warmup, alternate order,
  per-execution timeout 600 s, isolation L2.
- Arms: `rho-neg` (reference), `rho-neg-aa` (A/A control, same method), `tw2cj-lazy`
  (baseline, SNC-panel arm), `tw2cj-lazy-sib8-p64`, `tw2cj-lazy-sib8-p256`, `tw2cj-cv0`.
  (`tw2cj-cv0-sib8-p64` was dropped before any data: a local development smoke of the
  first draft returned `wwalk: sib with cv=0 is not supported` for that arm.) Oracle strings are in the specs; factor base
  `prime-projected-abscissa:size=2,scan_cap=32` for every IC arm.
- Execution: Iso Lab on EKS Spot (`isolab-experiment.yml` dispatched on `main` with the
  frozen commit as `source_revision`), one job per k, c8a.2xlarge, default deadline. The
  container's AWS keys are rejected by STS on 2026-11-10, so the GitHub OIDC route is used.
- Analysis: `ecbench verify --replay 8`, `ecbench compare` (paired bootstrap 95 percent
  interval, ratio of means) and `ecbench band --ranks` (w = 0, 0.1, 1), as in the SNC panel.

## Pre-registered expectations [pcc:unverified]

- E1. Total bit-op ratio `tw2cj-lazy-sib8-p64` / `tw2cj-lazy` at w = 0:
  0.69 [0.62, 0.76] at k = 28 and 0.66 [0.58, 0.74] at k = 32.
- E2. `tw2cj-lazy-sib8-p256` / `tw2cj-lazy` at k = 32 below 1 at w = 0 (set-up share
  falls with sqrt(r)); at k = 28 reported only.
- E3. `tw2cj-cv0` / `tw2cj-lazy`: no direction pre-registered; reported with its interval.
- E4. Ranking against `tw2cj-lazy` and against `rho-neg` is stated only where it holds at
  w = 0, 0.1 and 1 (§5a); otherwise "unresolved in the band". Distance to `rho-neg` is
  reported as distance remaining.

## Admission (a failed item voids the job's comparisons; it is reported, not re-run under the same name)

- L2 gate and A/A: `rho-neg-aa` / `rho-neg` interval contains 1.
- Every record verified; `verify --replay 8` reproduces.
- A refused arm (plan error) is reported as refused; the remaining arms stand only if the
  runner completed the session.

## Not claimed

No leaderboard or `benchmarks/README.md` row: that needs the validation gate (replay
certificate, validator report, VALIDATION.json). No security claim about any standard curve.

## Amendment 1 (2026-11-10T23:20Z, before any data of this panel)

Both EKS jobs failed before a pod ran: workflow runs 42913750091 and 42913756188, Kubernetes Jobs
`gh-42913750091-1` (UID `cd697db9-...`) and `gh-42913756188-1`, state `failed`, `pod_uid` empty, launcher
exit 23, about 3.5 min after submission. Image `isolab/tasks@sha256:54dea54b...`, ecbench SHA-256
`3e0cde94...` built from `a4db61b2e1`. Receipts are in the lab S3 bucket, which this session cannot
read (no AWS identity). Every non-publish dispatch of this workflow since 2026-11-01 has failed.

Change: run the same panels on this session's host (4 vCPU cloud VM, no PMU) from
`spec-k28-local.json` and `spec-k32-local.json`. These are byte-for-byte the frozen specs except
`isolation_required` L0, the label and the description. Expectations E1 to E4 and the analysis are
unchanged. Admission becomes: every record verified, `verify --replay 8` reproduces, and the A/A
interval contains 1. There is no L2 gate. Local L0 rows are development evidence (as in the SNC
panel), not an EKS measurement. The EKS specs stay frozen for a re-launch once the route works.
