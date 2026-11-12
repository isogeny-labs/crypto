# Affine/quadratic supplied-basis collector — November 12, 2026

**Implemented:** native rational assignment collection for supplied basis
equations reducible by affine pivots and univariate quadratics. Each assignment
is checked against the actual input and normalized through the existing RR/F
extractor. The collector records both input and basis digests and unions signed
outputs. Twelve native development tests pass; claim `PCC-20261112-72ed05`
is unverified. [pcc:unverified]

## What completeness means here

`status:enumerated_supplied_basis` means all rational assignments of the supplied
equations were collected within the supported forms. It does **not** certify
that those equations generate the same ideal as the original input. In
particular, a forged unit basis must never establish original-input UNSAT.
The output's `binding.input_completeness` always says this is not certified.
Input/basis provenance and independent input qualification remain obligations.

The collector checks every emitted witness through the input equations, but
checking witnesses alone cannot detect a missing input solution. It does not
verify Buchberger criteria or ideal containment in both directions. No such
proof or signed verification is claimed.

## Interface and statuses

```sh
p1tool collect --metadata C.ms.plant.json --system-file C.ms --gb C.gb
```

The RR/F arm, prime field and exact variable order are checked before basis
enumeration, including the empty-result path. The supported metadata/input
format is the existing p1tool/msolve format. Input and basis bytes must be pinned
by the execution manifest. The parser reuses the existing msolve basis reader.

- Affine pivot equations are substituted exactly, with reverse reconstruction.
- A univariate quadratic is branched over all distinct prime-field roots.
  Nilpotent multiplicity is not counted as an extra rational assignment.
- Any nonzero constant equation proves the supplied system inconsistent. [pcc:unverified]
- Free variables, coupled residuals without a supported pivot, higher-degree
  residuals, or caps return `status:unknown`, null assignments and exit 3.
  A supported branch followed by an unsupported branch stays unknown.
- Extraction errors remain indexed in the output, with normalization status
  unknown and exit 3. They cannot silently become complete empty answers.

Bounds are 10,000 recursive nodes, 4,096 solutions and 100,000 terms per
polynomial. No prime-field scan or substitution of a different workload is
used. Exit 0 means the supported supplied equations were enumerated and the
assignments normalized; it does not establish input completeness.

## Checks and archived evidence

[Final native log](checks/collection-tests-final-20261112.txt) includes the prior
extraction/reduction tests plus collector fixtures. The algebra fixture compares
all affine/quadratic roots with exhaustive evaluation over a small prime field.
Separate fixtures distinguish contradictions, free/coupled/higher-degree cases,
solution/node caps, and a partially supported branch.

The archived F basis at
`../split_prime_ecdlp_20261107/followups/p1_rr_dickson/calibration/round2/dickson_smoke_f/p1151_m3_t4_s1_f.gb`
contains affine relations and a final quadratic. Its corresponding `.ms` and
metadata are reused as development inputs. The test removes the top-level
planted map, support x/y/w coordinates, separators and seed diagnostics;
only public curve, target and tower data are retained. Both rational assignments
are collected and normalized. No msolve computation was rerun.

Negative fixtures check mismatched basis headers and input hashes, missing or
unsupported RR/F arms even for empty basis results, and the absence of original-
input certification for a forged unit basis. Read-only review prompted the
metadata purge and empty-arm check. These are producer checks, not an independent
signed audit. [pcc:unverified]

## Remaining route to the original comparison

The first admitted panel must verify that every actual solver output falls
within a supported collector case (or preserve it as unknown), bind the basis
to the exact source-built solver run, and compare complete normalized input
sets with an independent reference. General higher-degree collection and
excluded-chart coverage remain open. This development addition neither measures
nor predicts retained F4 degree, smaller matrices or lower complete cost.

The existing checkpoint upload and live budget/pool dependencies are unchanged;
the environment has no configured transport/identity. A nonsecret existing
runner/connection name was requested while native work continued. Repository
instructions require a successful S3 checkpoint and known budget headroom before
launch. No cloud build or paid job has been dispatched. The append-only ledger
merge conflict also remains unresolved, with both histories preserved in PR 3835.

## Sources checked

- The archived P1 basis files and p1tool's `check-gb`, basis parser, reconstruction
  and extraction code; these define the concrete supported output format.
- [Official msolve README](https://github.com/algebraic-solving/msolve): `-g 2`
  returns a reduced grevlex basis; it is not a list of rational assignments.
  This supports separating basis computation from assignment collection.
- The previously reviewed P1 protocols/results, round7_rr and PKM paper, linked
  in the study README. No new mechanism or novelty is claimed.
- KB remains unavailable (no loaded MCP and repository wrapper has no executable).

[Diagram](collection-flow.svg), [PDF](REPORT-COLLECTION-20261112.pdf),
[editable PDF source](REPORT-COLLECTION-20261112.typ).
