# Round16: explicit CM incidence and quartic quotient separators

[Report](REPORT.md) · [PDF](report.pdf) · [CM model](CM.md) ·
[cover sections](THETA.md) · [saturation](SATURATION.md) ·
[proof map](PROOF_MAP.md) · [protocol](PROTOCOL.md) · [ledger](ledger.json).

The level77 genus61 carrier has explicit affine F2 incidence circuits and
its diamond twist, with exact independent torsion controls through F32.
The primitive sign-projector denominator704 is not a unit at11. Four
polarized diagonal covers have explicit factor bases. A depth6 quartic
separator excludes all four old Q17 models, while four new Q22 survivors
keep full saturation and proper embeddings open. No IC efficiency verdict.

Raw successes, failed attempts and CI logs are in the deterministic
runs.tar.xz; replay evidence is in validation-runs.tar.xz. Extract and check:

```sh
cd research/koblitz_modular_ic_20261018/rounds/16
sha256sum -c runs.tar.xz.sha256 validation-runs.tar.xz.sha256
tar -xJf runs.tar.xz
tar -xJf validation-runs.tar.xz
sha256sum -c runs.sha256 validation-runs.sha256
./REPLAY.sh
../../native/target/release/koblitz-modular-carriers --round16-verify-sources .
```

REPLAY.sh rebuilds the native runner, repeats7 commands and compares7 JSON
outputs and5 generated Lean modules. It verifies5 source-bound IDC records.
The two manual Lean modules are compiled and audited by the full proof gate.
ARCHIVE_VALIDATION.txt records fresh extraction and byte-identical repacking.
Earlier frozen ledgers are checked against their own Git snapshots, not
against a later registration table; no old ledger is rewritten.

The visual source render.py reads native recorded values only; routes.svg
and its150dpi PNG are embedded in report.pdf. VISUAL_REVIEW.md records the
page inspection. Historical IC cost/rank graphs were checked and retain
their values; canonical frontier prose and source hashes are refreshed.
