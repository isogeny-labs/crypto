# V1 evidence status

Status: `COVERAGE_INVALID_STOP`.

The raw candidate, verifier and analyzer files and their hashes are immutable.
They passed the comparisons implemented in v1, but `AMENDMENT_1.md` records
three frozen protocol obligations that v1 did not emit or independently check.
Accordingly, `analysis.txt` is not the final protocol verdict. [pcc:unverified]

For Git-only custody without the unavailable artifact store, `candidate.txt`
is stored losslessly as deterministic `candidate.txt.gz`. `SHA256SUMS` retains
the original uncompressed digest; `PACKAGE_SHA256SUMS` binds the stored bytes,
and `./verify_evidence_package.sh evidence` checks both layers from the study
directory.
