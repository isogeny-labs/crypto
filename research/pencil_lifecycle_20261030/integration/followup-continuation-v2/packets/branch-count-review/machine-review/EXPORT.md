Archive from this private root:

- REPORT.md, review.json, review-attestation.json, review-plan.md
- check.sh, prepare.py, verify-snapshot.py, finalize.py
- reconciliation.json, subject-bindings.json, dependency-ledger.json, expanded-dependency-ledger.json, full-dependency-snapshot.sha256, dependency-path.txt
- producer-binding-check.log, audit.stdout, audit.stderr
- target/*.lean (rename to .lean.txt without changing bytes), target/audit-readback.py, target/audit-fresh.py, target/readback-names.json, target/readback-audit.json
- All seven *-replay-* directories in full. These contain exact source snapshot, before.sha256, bindings.stdout/stderr, stdout/stderr, command.txt, started.txt, ended.txt, receipt.json. Include CountHelpers-replay-TBy7UDM2 infrastructure failure and ExpectedFailure-replay-izTuQSYc expected rejection.

Do NOT archive dependencies/ (multi-GB private copies), target/*.olean (generated binaries), or research/ (duplicated immutable producer archive). Producer inputs are bound to commit5342909fbcaffae8f8bce2b290bdc095d47eccb6 and may be linked there. Preserve all inventories/log bytes exactly; use the standard artifact store for large files. No chunking/truncation or cache upload is intended.
