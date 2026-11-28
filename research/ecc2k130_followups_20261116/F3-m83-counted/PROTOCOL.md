# F3 protocol: a complete m = 83 counted IC row against matched rho

Lane F3 of `research/ecc2k130_followups_20261116/COMMON.md`. Frozen 2026-11-16, before any
smoke or probe data. All statements are `[pcc:unverified]`. Amendments are appended below
with a date. Nothing here launches paid compute.

## 1. Question and scope

L7 of the 2026-11-14 sweep (`research/ecc2k130_open7_20261114/L7-e2e-units/SUMMARY.md`,
"Residual gaps" 2) records that no complete m = 83 counted index-calculus (IC) row against a
matched rho exists, so the AGENTS.md §8a gate is unmet. This lane prepares that row. It does
not run it at m = 83: the container has 4 vCPU and no AWS identity, and the 2026-11-15 log
records the paid EKS round as blocked on fresh live budget.

Deliverables:
1. An audit of existing m = 83 material.
2. A frozen launch spec for the m = 83 row (`specs/launch-k0n83.json`).
3. A cost estimate for the IC arm and the matched rho, with uncertainty.
4. A local smoke at small degree in the identical spec shape, run on cores 0–2.
5. A list of what the owner must provide before a launch.

## 2. Instance (AGENTS.md §8a)

- Curve `E_0: y² + xy = x³ + 1` over `GF(2^83)`, modulus `z^83 + z^45 + z² + z + 1`, slug
  `icv1-f2m83-tm6151469093347-debefd74`, `#E = 4 · r`, `r = 2417851639230796216685689`
  (log2 r ≈ 81.0). ecbench builds it as `{"kind":"koblitz","a":0,"n":83}` (`Instance::Wide`).
- Smoke curves: the same `a = 0` family at n = 41, 53, 61 (`Instance::Binary`, one word). They
  are smoke sizes under §8a and do not discharge the gate.

## 3. Arms (identical at every size)

| arm | role | method |
|---|---|---|
| `rho-strong` | reference | `rho.signed_frobenius_strong` |
| `ic-pipeline` | candidate | `ic.pipeline`, `compact-orbit-scan:columns=c0,raw_x_cap=100000000`, `mitm-frobenius-counted:m=2`, `targets=walk`, `incremental-gauss` |
| `rho-strong-aa` | A/A control | `rho.signed_frobenius_strong` |

- The candidate is the recipe `ic plan` emits for a=0 Koblitz curves (planner `ic-planner/4`).
  It is the PX "pipeline" arm of `research/ic_planner_exhaustive_20261026`, the counted Koblitz
  arm nearest to rho at the largest measured r in L7's table (21.4× at n = 61, w = 0).
- Column rule, frozen: `c0 = round(0.6 · (r/n³)^(1/3))` (`KOBLITZ_COLUMN_KAPPA` in
  `src/cryptanalysis/ic_framework/planner.rs`). Values: 120 (n = 41), 313 (53), 537 (61),
  970249 (83).
- Workload: 8 public targets per curve, target seed 20261116301; measurement seed 20261116302;
  1 round, 0 warm-up, order `alternate`, isolation L0. Counts decide, so timing isolation is not
  needed; replays check the counts.

## 4. Controls

- **Positive control (pipeline end to end):** every smoke execution must recover its planted
  scalar (the record's `verified`), and `ecbench verify --replay-all` must reproduce every count. [pcc:unverified]
- **A/A:** `rho-strong-aa` must equal `rho-strong` count for count on every target (the walk is
  deterministic per seed). Any difference voids the session.
- **Negative control (the m = 83 gate):** `ecbench plan` on `specs/launch-k0n83.json` is expected
  to refuse the `ic-pipeline` arm ("no wide-field implementation"). A pass would mean the arm is
  runnable and this lane's blocker list is wrong.
- **Wide rate probe:** `specs/probe-k0n83-rate.json` runs the strong rho on the m = 83 curve with
  a 10^6-step cap (as `research/ecbench_wide_gf2m_20261004`). Every run must exhaust its cap.
  Its only output is ns per step on this host, labelled L0 and provisional.

## 5. Cost model for m = 83, frozen before the smoke

Notation: n field degree, r subgroup order, C columns, w the §5a weight.

- Rho: `G_rho = k_rho · sqrt(π r / (4n))` GAE, with `k_rho` the ratio of measured to modelled
  rho cost, fitted on the smoke and on the committed PX/FC a=0 sessions.
- IC (pair table modulo Frobenius, walked targets, incremental Gauss):
  `G_ic(C) = α · n · C² + β · r / (n² · C)`. The first term is the table build, the second the
  relation trials. α and β are fitted in log space on every committed row with known C (PX
  `pipeline` and `pipeline71`, FC `ic-compact`, all w = 0) plus the smoke rows.
- Predictions at m = 83 are reported for (a) the frozen rule C = c0, (b) the model optimum,
  (c) the largest C whose table fits 16 GiB, 128 GiB and 1 TiB (16 bytes per entry, worst case
  of the packed layout; entries ≈ n·C²).
- Wall time: GAE times ns per GAE. Rho uses the probe's ns per step. The IC arm has no wide
  implementation, so its ns per GAE is taken equal to rho's (an assumption, stated as such).
- Dollars: core-hours times a price per vCPU-hour. No live price is readable here. The
  estimate uses a bracket of 0.02–0.06 USD per vCPU-hour (Spot to On-Demand for c8a-class
  instances), labelled as an assumption to be replaced by a live readback.

## 6. Predictions and decision rule

- P1 (smoke): IC/rho at w = 0 rises with log2 r and lies in [8, 80]× at n = 61.
- P2 (smoke): every IC and rho execution verifies; A/A is exact. [pcc:unverified]
- P3 (gate): `ecbench plan` refuses the IC arm at n = 83.
- P4 (estimate): at m = 83 with C = c0 the model puts IC/rho above 100× and the pair table
  above 1 PiB.

Decision:
- **READY-TO-LAUNCH** only if P2 holds, P3 fails (the IC arm plans at n = 83), the estimated
  IC cost fits a budget ceiling stated by the owner, and a recorded live budget and route exist.
- **BLOCKED** otherwise, with every blocker named. A blocked outcome is not a negative result
  about index calculus at m = 83.

Falsifiers: P2 failing falsifies the claim that the launch shape works end to end. The P1
range failing at n = 61 falsifies the calibration the estimate rests on; the estimate is then
reported with the PX constants alone.

## 7. Budget ceiling and stop conditions for a future launch

- Ceiling: none can be set here; the owner sets it. This lane records the estimate and the
  smallest panel that would answer the question (section 5).
- Stop conditions frozen for the launch: stop at the owner's dollar ceiling counting every
  attempt; stop when the monthly or daily AWS budget remainder is unknown or exhausted; stop on
  any failed replay or A/A mismatch; preserve every censored or failed record.

## 8. Local resource rules

Cores 0–2 only (`taskset -c 0-2`), at most 2 GB address space per process
(`prlimit --as=2000000000`), `timeout` on every run, lane-private build directory, no commits.

## Amendments

### Amendment 1 (2026-11-16, before any smoke or probe data)

- Two lines (section 4 positive control, section 6 P2) gained a `[pcc:unverified]` tag because
  `pcc.py lint-prose` flagged them. No content changed. Hash at freeze: `5d14d9bb…2d2b`.
- Added after the freeze, before data: `requests/request-v1-rate-probe-l2.json` (the rate probe
  of section 4 as an L2 `isolab.eks.request/v1` job: rounds 5, warm-up 1, A/A, 600 s per
  execution, 1800 s job) and `requests/request-resumable-k0n83-ecbench-mode.json` (the launch
  spec wrapped in `isolab.eks.resumable/v1`, with placeholders where no image exists). The
  local probe keeps the frozen L0 spec.
- Build source: `HEAD` (`c3655407dc`) does not compile (`koblitz_index_calculus.rs:1646`,
  `lift_x_packed` returns `Vec::new()` where a tuple is expected). Every local run uses the
  newest first-parent commit without that code, `70b57c2b81`, exported with `git archive`.
