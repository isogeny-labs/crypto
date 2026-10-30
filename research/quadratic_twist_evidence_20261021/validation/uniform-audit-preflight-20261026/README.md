# Independent uniform-yield implementation: compiler-only receipts

The source freeze is `6cd252b9c0ed4ffcc245d1c70281710f00fb3ab9`.
[Implementation addendum](../../UNIFORM_YIELD_20261026_IMPLEMENTATION.md),
[report and inspected PDF](../../reports/uniform-audit-preflight-20261026/README.md).
The October25 protocol and original producer remain byte-identical.

**Zero scientific executions.** Neither producer version, the independent
auditor, a corruption mode nor a new Lean proof has run. No expected value,
relation/rank/degree/state/scalar result, rejection count or improvement is
reported. The planned population remains96arms/896targets/210comparisons.
The faithful recurrence/probability interpretation and exact runner remain
unimplemented. Existing208public/269audited declarations and17identities
are unchanged historical evidence, not new proof-gate results.

| Action | Actual result | Raw receipt after extraction |
| --- | --- | --- |
| Versioned C++ producer compile/link | PASS; executable not run | `producer-compiler-receipt.log`, `compiler-commands.json` |
| Independent Rust release build | PASS; audit/corruption paths not run | `auditor-compiler-receipt.log`, initial/final build logs |
| Rust/backend test-harness compilation | PASS with `--no-run`; tests not run | `auditor-no-run-receipt.log`, `auditor-test-compile.log` |
| Fresh unchanged root release library |4479passed/119existing ignores/0failures | `cargo-lib-final.log` |
| Existing workflow metadata tests |4passed | `workflow-tests.log` |
| Draft uniform/dual cards and naming |both cards valid;0name problems | `backlog-final.log`, `naming-final.log` |
| Report/PDF |1page,32,083bytes; rendered/inspected; text equality and box/reference checks pass | `report-build.log`, `render.log` and versioned report |
| Shared Conductor |exits127; no reservation/replacement | `conductor.log` |

Exact compiler argv/cwd/exits are retained. Compilation receipts do not establish
the intended mathematical checks: Rust independently implements BigInt fractions,
breadth-first subspace closure and descending-rank recurrences; C++ uses GMP and
recursive recurrences. Runtime equality, witness/point/scalar checks, policy
frontiers and semantic corruption rejection still require actual execution.
Build-log elapsed times are diagnostic, not a performance measurement.

The parent-head observation at2026-10-26T04:13Z is retained separately:
`required-rules-parent.json`, `parent-rules.json`, complete parent checks,
workflow/status responses and the curated parent summary. Public active rules
require **gate,size,artifacts**, all absent on parent60b006333. Legacy protection
remains403. That parent snapshot is not validation of a later commit. Missing,
queued or review-job checks are not passes or submitted approvals.

The checkpoint-only upload question remains pending and the token is absent.
No auto-uploading gate was invoked after the resume became complete; no skip,
fake last-push, S3 upload, credential reuse or paid resource occurred. Read-only
review, independent implementation and ordinary compiler/library/metadata checks
advance while the dependent scientific launch remains blocked. The previous
[concrete checkpoint preview and scope](../../reports/uniform-yield-preflight-20261025/CHECKPOINT_UPLOAD_REVIEW.md)
are preserved. No repeated approval question is issued by this follow-up.

All34 raw files land in one deterministic `runs.tar.xz` (9,800bytes), not as
loose duplicate logs. It contains compiler/library/metadata/report receipts,
host/toolchain descriptions, source/binary hashes and public parent-rule/check
responses; it contains no binaries, study outputs, credentials, presigned token,
chat transcript or upload payload. `INPUTS.sha256` binds the compiled sources,
protocol/addendum and existing helpers/backends/Cargo inputs. Hashes were checked
after extraction and the extracted tree repacks byte-identically.

From a fresh directory:

```
tar -xJf /absolute/repository/path/research/quadratic_twist_evidence_20261021/validation/uniform-audit-preflight-20261026/runs.tar.xz
sha256sum -c /absolute/repository/path/research/quadratic_twist_evidence_20261021/validation/uniform-audit-preflight-20261026/RUNS.sha256
tar --sort=name --mtime='2026-10-26 00:00:00Z' --owner=0 --group=0 \
  --numeric-owner --mode='u+rwX,go+rX,go-w' --format=gnu -cf - runs \
  | xz -6 -T1 > repacked.tar.xz
```

The repacked SHA-256 must equal `runs.tar.xz.sha256`. All old sealed studies,
proofs, identities, reports, workflows, library/src/Cargo, registry/catalog/
benchmark and conflicted files remain unchanged. Keep PR2381 open/non-draft;
scientific, checkpoint, Conductor, exact-head CI, review and integration holds
are not discharged by these compiler receipts.
