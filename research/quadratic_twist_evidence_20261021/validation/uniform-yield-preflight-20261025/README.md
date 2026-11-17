# Uniform-yield preflight validation; no scientific execution

Protocol c6cadc491 and pre-execution GMP correction7e9a48bb8 are committed.
The prepared producer compiles; no uniform experiment, solver response,
collection-state result or source recovery has run. Independent uniform Rust
and Lean recurrence implementations remain pending. No result ledger row,
identity certificate or new proof claim is issued.

| Preflight/ordinary validation | Outcome | Raw evidence after extraction |
| --- | --- | --- |
| First C++ compile | fails: Boost headers absent | `producer-attempt01.cpp.txt`, `producer-build-01.log` |
| Second compile using existing GMP | fails: missing functional include | `producer-attempt02.cpp.txt`, `producer-build-02.log` |
| Final compile, -lcrypto -lgmpxx -lgmp | passes; binary never executed | `producer-build-03.log`, source `INPUTS.sha256`, binary hash/size |
| Safe official checkpoint gate before RESUME init | exits2, no upload path reached | `checkpoint-gate-before-init.{log,exit}` |
| Complete local RESUME/status | READY locally, no successful S3 checkpoint | `checkpoint-status-local.log`, `RESUME.review.md` |
| Checkpoint-only access | env/file token absent; no values captured | `checkpoint-access.json` |
| Fresh unchanged root `cargo test --release --locked --lib --quiet` |4479passed/119existing ignores/0failed | `cargo-lib-final.log` |
| Workflow metadata tests |4passed | `workflow-tests.log` |
| Draft cards | both validate; no Cairn/main-ancestry promotion | `backlog-validate.log` |
| Curve naming metadata |0problems | `naming-full.log` |
| One-page status PDF | rendered/inspected, final extracted text equal | final build log, report/PDF |
| Artifact policy/size/diff | no selected files | receipts/committed checks |

These builds/tests are ordinary implementation validation, not execution of the
pending research protocol. The new launch gate is honored: the official script
5ee30642c1af68af40d530d85aaf160695b19559 is copied unmodified from main into
scratch; its content and the exact new AGENTS diff are archived. Current-main
AGENTS7d00a85eb39b41a7ee5f12bff2ac48b31b9e0e59 requires a successful S3
checkpoint before experiments, and the gate auto-uploads after RESUME is complete.
It is not invoked after completion without new user upload authorization/token
access. No skip, fake last-push, attempted extra upload or previous credential
reuse. A local preview is not reported as an S3 checkpoint.

The report initially failed PDF text equality on Graphviz's fi ligature; exact
source/before-after text/log retained. Replacing the diagram label and wrapping
the checkpoint prefix fixes extraction/legibility. The final one-page PDF is
33,573bytes and the raw archive21,684bytes, both below64KiB. No experiment
outcome is hidden behind that formatting correction. All prior frozen studies,
proofs/17certificates, library/src/Cargo,registry/catalog/benchmark/conflicted
files remain unchanged. Conductor127/no reservation remains an independent hold.

Restore with:

```
mkdir FRESH_DIR
tar -xJf research/quadratic_twist_evidence_20261021/validation/uniform-yield-preflight-20261025/runs.tar.xz -C FRESH_DIR
(cd FRESH_DIR && sha256sum -c /absolute/repository/path/research/quadratic_twist_evidence_20261021/validation/uniform-yield-preflight-20261025/RUNS.sha256)
```

All27 raw file hashes and deterministic repack verify. The archive contains no
credentials, presigned URL/token or chat transcript. Initial checkpoint content
and the exact branch-scoped authorization proposal are linked from the report.
The broader goal is blocked at the new launch authorization/access gate, not
proved impossible. Existing exact-head CI/review/integration/math holds stay.
