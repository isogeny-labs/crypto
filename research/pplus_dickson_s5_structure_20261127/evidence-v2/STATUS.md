# V2 evidence status

Status: `STRUCTURE_ONLY_STOP`.

The repaired verifier matched every frozen channel, kernel, chart, zero and
signed-witness field and reported complete coverage. The analyzer found the
declared low-rank structure but no exact subquadratic zero oracle, so production
and m=7 remain unauthorized. See `REPORT.md` for scope and next work.
[pcc:unverified]

For Git-only custody without the unavailable artifact store, `candidate.txt`
is stored losslessly as deterministic `candidate.txt.gz`. `SHA256SUMS` retains
the original uncompressed digest; `PACKAGE_SHA256SUMS` binds the stored bytes,
and `./verify_evidence_package.sh evidence-v2` checks both layers from the study
directory.
