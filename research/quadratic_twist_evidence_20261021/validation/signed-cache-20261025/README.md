# Signed-cache comparison: execution and validation

Run `signed-cache-20261025` extends e1e0b9cbf0821e5c89639245267c1c2986aeba12.
Protocol b605b7877 and compiled source/proof freeze a150b288c precede the first
native execution. Both initial and fresh final runs pass, with no intervening
native arithmetic or counter changes. Full commit hashes, commands, environment,
builds and exact raw controls are retained in the archive.

| Check | Executed outcome | Restored evidence |
| --- | --- | --- |
| `bash research/quadratic_twist_evidence_20261021/run-signed-cache.sh FRESH_DIR` | PASS;80 paired setups/800 separate scalars, exact case/audit/certificate bytes | `runs/final/` |
| Independent arithmetic/request replay | PASS; every ordered lift, source answer,36-component vector and setup/online/cold delta | canonical `native/signed-cache-20261025/audit.json` |
| Separate batch-RREF/witness semantic checker | PASS;100 complete solver certificates | `runs/final/audit.log` |
| Seven rehashed semantic controls | each exit101/no receipt | exact altered case/index and logs in `runs/final/controls/` |
| Source-bound coordinate polynomial | IDC1h202bb532af10c977;32+16 points, false identity/source rejected | `runs/final/certificate/`, `certificate.log` |
| Unmodified scoped repository proof gate | PASS;191 public named theorems /249 audited declarations;0 failing/extra trusted | `runs/lean-gate-final/`, `runs/local/proof-gate-final.log` |
| Fresh root `cargo test --release --locked --lib --quiet` |4479 passed,0 failures,119 existing ignores | `runs/local/cargo-lib-final.log` |
| Isolated auditor backend tests |22 passed | `runs/final/backend-tests.log` |
| Workflow YAML metadata tests |4 passed | `runs/local/workflow-tests.log` |
| Executed/draft backlog cards | both validate; no Cairn-state promotion | `runs/local/backlog-validate.log` |
| Full curve-name metadata check |0 problems | `runs/local/naming-full.log` |
| Current-main official artifact policy | no selected new files | `artifact-policy-check.json` |
| Report build/text equality/no overfull/reference errors | PASS; both final pages rendered and inspected | `runs/local/report-build-final.log`, report sources/PDF |

All native commands have600-second caps; none is reached. Existing local host
only, two build workers, no paid compute. Diagnostic build/test times are not
research timings. The old binary loop includes unused final doublings, so these
algorithm requests are not a calibrated cost or hardware instruction count.
New cache storage/point checks/reads and field tables, sorting, traversal,
allocation, hashing and audit remain unpriced. No speed, GAE, S, rho or
external-fastest result is admitted.

## Proof and immutable inputs

Thirty frozen source cases are pinned in
`native/signed-cache-20261025/source-inputs.sha256`:ten reduced-S3, ten adaptive
and ten checked-root records. Every new reference equals the old checked-root
candidate after removing four added zero counters. Each candidate full solver
certificate equals its frozen adaptive certificate by actual x coordinate.
`INPUTS.sha256` binds final producer, existing helpers, auditor/Cargo, protocol,
Lean source and existing identity/hash backends. Library/Cargo, registry,
catalog and benchmark inputs remain unchanged.

Nine public Lean theorems and two generated helpers are newly audited. The
negative entry is derived from C+P=O, not assumed equal to −P. Ordered filtered
lists preserve duplicates/order; arbitrary/repeated columns are covered.
Actual F43 cache points, inverse checks, base logs5/7 and source recovery are
kernel checked. The first compiled draft had fixed columns and exposed base
logs; it passed. Before native execution the theorem was strengthened to
arbitrary columns and the actual base logs were proved. Both sources/build logs
are in `runs/proof-drafts/`; no failed build is hidden or relabeled. All prior
fifteen study identities/sources remain unchanged; the sixteenth checks the
coordinate polynomial only. Lean4.34.1/Mathlib
d13f23b723b8a846827a245b89c10fc7d3f11612 are pinned. Only
propext/Classical.choice/Quot.sound occur; no external lean4checker replay,
trusted native evaluation or universal native-algorithm formalization is claimed.

Seven controls rehash their index after altering a cached sign, point, missing
entry, read demand, source binding, failed query or scalar. The false cached
point remains on the actual curve; its inverse/source slot is wrong. This does
not introduce invalid-curve scope. Exact changed JSONs and failure logs remain.

The profile probe still reports missing gawk and produces no profile/cost
evidence; its exact tool source hash and diagnostic are retained. Actual
Conductor exits127, no task lease or replacement coordinator is claimed.
No credentials or chat transcripts are task metadata. No upload or credential
reuse occurred. Shared coordination, exact-head CI access, reviews, integration
and broader science remain completion holds.

## Restore the raw tree

```
bash research/quadratic_twist_evidence_20261021/validation/signed-cache-20261025/restore-runs.sh FRESH_DIRECTORY
```

Canonical cases stay loose because replay/source bindings name their paths.
Only identical final/control case copies are dehydrated from `runs.tar.xz`;
every changed payload and rehashed index remains exact. Restoration hydrates
those copies, overlays the archive and checks **173 original file hashes**.
The compiled producer is omitted; its hash/size, exact source and build command
are retained. No old frozen file is changed. `OUTPUTS.sha256` binds all new
canonical records and certificate files.

The archive is24,516 bytes and the inspected PDF56,200 bytes, below64KiB;
all canonical case JSONs remain below1MiB. Full restoration passes and extracting
without hydration then repacking gives identical archive bytes:

```
tar --sort=name --mtime='2026-10-25 00:00:00Z' --owner=0 --group=0 \
  --numeric-owner --mode='u+rwX,go+rX,go-w' --format=gnu -cf - runs | xz -6 -T1 > repacked.tar.xz
```

The next direct-sign reference card remains an unexecuted draft. It preserves
both old arms, adds direct uncached signs and charges setup/read work before
interpreting cache benefit. This round supplies a verified request change
against a named reference, not a full-work performance verdict.
