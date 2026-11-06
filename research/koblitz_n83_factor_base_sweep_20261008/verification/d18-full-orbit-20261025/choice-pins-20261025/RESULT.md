# Complete-D18 planted-choice isolation

The [frozen protocol](../CHOICE_PIN_PROTOCOL.md) ran two serial constructed
controls against a fresh GET of the same 32,643-column, 5,418,738-point S3
archive. Both used clean source commit `d775b18dc427188e0279f4189c3c63b570f780fc`
and executable SHA-256 `0826acbe6a3e55fc747be480453cd1bca9c053adb584ce7e55548e13985644e4`.
The exact primary curve, four representative point indices `[0,166,332,498]`,
summand/intermediate x pins, solver bound, and original-field checks were
identical. Each arm imported the entire base anew. The [host receipt](host.json)
labels this as shared-host L0 evidence without a hard memory cgroup.

| Control | Free choices | Clauses | Import | SAT search | Conflicts | Whole worker | Supervisor max RSS |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |
| Phase zero pinned | Four selectors | 2,807,780 | 54.514 s | 0.239 s | 0 | 55.510 s | 3,845,980,160 B |
| Selectors pinned | Four Frobenius phases | 2,807,756 | 58.740 s | 16.589 s | 69 | 76.234 s | 3,944,513,536 B |

Both returned `PASS_CONSTRUCTED_CONTROL`, rank one, a matching planted row,
and a rejected altered row. The [independent Python auditor](audit.py), using
original GF(2^83) group arithmetic, verified fresh S3 GET checksum and source
bindings, four archive points and their subgroup orders, target sum, row
digest, binary/source identity, shape, phases, results and resource records.
Its [output](audit.json) is `PASS_TWO_INDEPENDENT_GROUP_AND_ROW_REPLAYS`.
Four [altered-receipt controls](negative-controls.json), run by
[`negative_controls.py`](negative_controls.py), were rejected: a false phase
status, changed selector row digest, false host-isolation label, and changed
provider checksum.
The raw `phase/` and `selector/` records, fresh-GET provider metadata, and
[SHA-256 inventory](sha256-inventory.json) are retained beside this report.
For replay, fetch the exact object through [AWS_RESUME.md](../AWS_RESUME.md),
build the example at the pinned source commit, install `blake3==1.0.8`, then
run `PYTHONPATH=<dependency-directory> python3 audit.py OBJECT BINARY`.

The selector-free, phase-fixed arm was fast. The phase-free, selector-fixed
arm took longer, while the earlier both-free shared-root constructed control
reached its 300-second cap. These are individual constructed observations;
they do not isolate a unique causal bottleneck or give a natural relation
yield. Neither arm ran linear algebra, descent or matched rho. The hard-cgroup
Linux free-choice and natural-query gates remain open, and
`selected_best_total_runtime` remains `null`.
