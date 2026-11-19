# ECBench phased curve panel

Status: **planned and unmeasured**. This directory defines a progression and
its gates; it contains no benchmark outcome and establishes no ECDLP speedup.
[pcc:unverified]

The panel starts with a deliberately broad 20-bit calibration phase, then moves
through nominal 30-, 60-, and 80-bit subgroup sizes. “Bits” always means
`ceil(log2(r))` for the prime subgroup order `r`; field size is recorded
separately.

## Contents

- `panel.json` — authoritative curve cells, exact identities, typed benchmark
  lanes, per-phase lane admission, ecbench curve fragments, target counts, and
  default gate constants.
- `panel.schema.json` — JSON Schema for the manifest.
- `phase1-cover-transfer-assessment.json` — typed transfer certificate,
  obligation ledger, and explicit unmeasured advantage fields for the exact
  Phase 1 cover route.
- `PROTOCOL.md` — execution, pruning, promotion, and claim gates.
- `REPORT.md` / `REPORT.pdf` — source-linked selection report and rendered PDF.
- `phased-panel.dot` / `phased-panel.svg` / `phased-panel.pdf` — editable,
  web-rendered, and PDF-rendered gate diagram.
- `validate.sh` — local identity/window/readiness checks against the committed
  curve registry and traits catalogue.
- `../../docs/ecbench/specs/panel-phase1-hyperelliptic-jacobian.json` and
  `../../docs/ecbench/specs/panel-phase1-cover-transfer.json` — executable
  32-public-target Phase 1 preregistrations.

## Phase summary

| Phase | Nominal subgroup bits | Accepted window | Curve cells | State |
|---|---:|---:|---|---|
| 1 | 20 | 18–23 | Koblitz `a=0`, Koblitz `a=1`, two general-binary field-structure cells, generic prime, `j=0`, `j=1728` | ready to preregister |
| 2 | 30 | 27–33 | Koblitz, general binary, P32a prime | ready to preregister |
| 3 | 60 | 56–62 | degree-73 Koblitz, B59, prime `j=0` | partially blocked |
| 4 | 80 | 76–84 | registered degree-83 Koblitz plus ECC2-79 and ECCp-79 anchors | blocked |

The curve ladder and algorithm/object lanes are orthogonal. Each lane promotes
independently; evidence in one row cannot satisfy another row's gate.

| Lane | Object and candidate track | Phase 1 | Phase 2 | Phase 3 | Phase 4 |
|---|---|---|---|---|---|
| `elliptic_algebraic` | elliptic subgroup; polynomial-system/SAT/algebraic decomposition | ready | ready | partially blocked | blocked |
| `elliptic_generic_decomposition` | elliptic subgroup; table, MITM, root-finding, or subtractive decomposition | ready | ready | partially blocked | blocked |
| `hyperelliptic_jacobian` | direct higher-genus Jacobian subgroup and same-Jacobian rho reference | ready | blocked | blocked | blocked |
| `cover_transfer` | typed source → cover/Jacobian → solver → recovery route | ready | blocked | blocked | blocked |

The two decomposition rows are intentionally separate tracks. Hyperelliptic and
cover-transfer rows now have exact executable Phase 1 entries only. The direct
lane pins `P1-JG2-18`, an 18-bit prime subgroup in an exact genus-2 Jacobian,
with `rho.negation` and `ic.hyperelliptic_v2`. The transfer lane pins
`P1-K0-23-G3-COVER`, the degree-three same-field pullback into a genus-3
Jacobian image, with source rho, destination rho/BSGS, charged source replay,
and nine exclusive cold wall phases. Phases 2–4 still fail closed. The Phase 1
specs are preregistrations, not completed benchmarks or speedup evidence.

Phase 3 is partially blocked because the degree-73 Koblitz cell uses the wide Koblitz path, where
the current ecbench implementation has rho/claw support but no matching
index-calculus arm. Phase 4 additionally requires local ICV1/EC1 imports for
the two Certicom anchors and generic wide binary/prime execution paths. A
blocked cell is not silently replaced or reported as measured.

## How to use the panel

1. Run `./validate.sh` at the exact source commit.
2. Select one phase and one lane only after that lane's blockers are empty and
   all of its mandatory objects support the candidate and strongest applicable
   same-group rho reference.
3. Before any probe, freeze candidate IDs and parameters, FB1/FBM1 and IC1
   identities, seeds, row cap, episode count, look schedule, error budget,
   target-rank definition, target list, timeouts, and resource envelope.
4. Create one `ecbench.spec/v2` per compatible curve/method family with
   `curve_policy` set to the phase and lane, for example
   `{"mode":"phase","phase":1,"lane":"elliptic_generic_decomposition"}`,
   and select each curve by exact cell ID such as
   `{"kind":"panel","cell":"P1-K0-23"}`. Direct Jacobian lanes instead
   select a pinned destination such as
   `{"kind":"panel_destination","cell":"P1-JG2-18"}`; a transfer lane
   also names its exact route in `curve_policy`. The native resolver reads
   the manifest's `ecbench.curve_spec`; do not copy that fragment into a phase
   spec. Do not combine arms that are inapplicable to some curve merely to
   obtain a rectangular table.
5. Run ecbench `plan` before computation. Preserve refusals and plan IDs.
6. Apply the candidate-pruning gate during relation probes, then the phase
   promotion gate in `PROTOCOL.md` after complete cold workloads and replay.

The manifest does not freeze candidate method parameters. Those belong in the
phase preregistration because freezing them now would preselect an algorithm
before Phase 1 calibration. Once frozen, they cannot be changed using the same
evidence; a changed candidate receives a new candidate ID and fresh error
budget.

## Validation

```sh
./research/ecbench_phased_panel_20261117/validate.sh
```

If `check-jsonschema` is available, the script also runs full JSON Schema
validation. Its built-in checks always enforce the four independent lane IDs,
per-phase lane coverage and blocker consistency, the exact Phase 1 destination
and typed route, both 32-public-target specs, the cover-transfer obligation set,
unique source/destination cells, exact phase windows, subgroup-bit membership,
local ICV1/EC1 consistency, trait-catalogue subgroup/cofactor equality, and
Phase 4’s blocked external-identity state.

## Evidence boundary

Passing a phase gate means the study is scientifically and operationally ready
to examine the next scale. It does **not** mean index calculus is faster than
rho. A speedup statement requires the separate claim gate: verified cold
end-to-end solves, complete phase costs, matched strongest applicable rho,
paired uncertainty below one at every required amortization weight, controls,
and independent replay. See `PROTOCOL.md`. [pcc:unverified]
