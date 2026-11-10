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
