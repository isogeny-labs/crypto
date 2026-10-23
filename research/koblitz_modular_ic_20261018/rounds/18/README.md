# Round18: canonical characters and the next integral construction gates

[Report](REPORT.md) · [PDF](report.pdf) · [integral maps](CM.md) ·
[theta descent](THETA.md) · [saturation](SATURATION.md) ·
[proof map](PROOF_MAP.md) · [protocol](PROTOCOL.md) · [ledger](ledger.json).

This round evaluates the diamond-2 correspondence, identifies the common
residual Hecke system and exhausts an affine-linear elliptic-map ansatz.
All102 nonconstant finite fits are independently rejected. The actual
integral factor inclusion remains open. Symmetry determines the canonical
theta characters under the written pullback-class bridge. The selected
cover has an integral rotation obstruction; an explicit scalar refinement
commutes with R but still needs mixed actions and invariant sections.
Q24 and Q26 points are excluded; four Q28 models keep full saturation open.

The native ledger records requirements and failures without an IC assessment.
There are94 named Lean results; all formal scope is in PROOF_MAP.md. Raw
attempts are frozen as runs.tar.xz, with native SHA256 member lists. Fresh
extraction, byte replay and source-pin receipts are in validation-runs.tar.xz.

```sh
cd research/koblitz_modular_ic_20261018/rounds/18
sha256sum -c runs.tar.xz.sha256 validation-runs.tar.xz.sha256
tar -xJf runs.tar.xz
tar -xJf validation-runs.tar.xz
sha256sum -c runs.sha256 validation-runs.sha256
./REPLAY.sh
../../native/target/release/koblitz-modular-carriers --round16-verify-sources .
```

The generic source-verifier CLI retains its Round16 name. Frozen Round17
pins are checked at76e409c, not rewritten for the new registrations.
render.py produces the editable visual, SVG,150dpi PNG and inspected report
PDF from native recorded values only. Canonical frontier prose and dashboard
source hashes are updated; historical cost/rank graphs retain their values.
