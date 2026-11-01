# Three-arm direct-sign reference: executed checks

Run `direct-sign-20261025` extends cc23bec37424c50d048e6ea4bdc3bfacd3f0f2e4.
Preregistered protocol5e270b8a2 and compiled source freeze d303efaee precede
the first native execution; full hashes and commands are archived. Initial and
fresh final runs pass without native source/counter changes or a native timeout.

| Check | Executed outcome | Evidence |
| --- | --- | --- |
| `bash research/quadratic_twist_evidence_20261021/run-direct-sign.sh FRESH_DIR` | PASS;120 setups/1200 separate source scalars, exact canonical bytes | restored `runs/final/` |
| Independent Rust arithmetic/request replay | PASS; all38 counters, ordered tested operands/sums including rejected signs, complete roots/lifts/certificates/rows/degrees/logs/answers | canonical audit and `runs/final/audit.log` |
| Separate batch-RREF/witness semantic audit | PASS;100 complete certificates | `runs/final/audit.log` |
| Seven rehashed semantic controls | each exit101/no receipt | exact altered payloads/indexes/logs in `runs/final/controls/` |
| Existing coordinate IDC1h202bb532af10c977 | PASS;32+16 points, false identity/hash rejected; no new identity | `runs/final/certificate.log`, unchanged signed-cache certificate |
| Unmodified scoped repository proof gate | PASS;191 public named theorems/249 audited declarations;0 failing/extra trusted | `runs/lean-gate-final/`, `runs/local/proof-gate-final.log` |
| Fresh root release library suite |4479 passed,0 failures,119 existing ignores | `runs/local/cargo-lib-final.log` |
| Isolated auditor backend tests |22 passed | `runs/final/backend-tests.log` |
| Workflow metadata tests |4 passed | `runs/local/workflow-tests.log` |
| Three executed/draft backlog cards | validate, no main-ancestry/Cairn promotion | `runs/local/backlog-validate-final.log` |
| Curve naming |0 problems | `runs/local/naming-full-final.log` |
| Official current-main artifact policy | no selected new/modified files | `artifact-policy-check.json` |
| Two-page PDF/text equality/no overfull/reference errors | PASS; both final pages rendered and inspected | `runs/local/report-build-final-03.log`, report sources/PDF |

## Inputs, interpretation and proof scope

Both old arms equal frozen signed-cache records after removing only two new
common counters and the tested-sign trace hash. `sign_operand_reads` and
`sign_dispatches` equal2L in every arm; the old cache-only read counter is nested,
not a second equal-cost read. Independent Rust recomputes the complete ordered
`(i,j,si,sj,A,B,sum,accepted)` stream, all counters and mathematical results from
separate field/group/polynomial arithmetic. Hashes do not provide solver answers.
The source labels remain outside recovery; all1200 synthetic source scalars
verify under the actual F43 base-field isomorphism and explicit source order.

Direct signs remove the binary sign loop and are exactly the existing faithful
`SignedCache.signedPoint` branch. Existing signed-zsmul, ordered-lift,
cache-validation and actual source-recovery proofs are reused unchanged.
No new theorem, duplicate identity or universal algorithm formalization is
claimed. All16 identity records and Lean sources remain unchanged. Lean4.34.1
and Mathlib d13f23b723b8a846827a245b89c10fc7d3f11612 are pinned; only
propext/Classical.choice/Quot.sound occur. No external lean4checker claimed.

All400 direct cold records use two fewer additions than cached; all400 cached
records use fewer negations. The first prefix Q=P main binary/direct/cache
addition requests are125/45/47, twist136/40/42. Cache validation is included.
All roots, ordered lifts, rows/ranks, solver/expanded witness degrees, failed
shifts and source answers are equal. This stronger reference exposes a tradeoff,
not a complete-work ranking. The binary loop includes unused final doubles and
identity additions that a compiler may remove. Differently weighted/nested
requests are not retired instructions, measured time or equal-cost units.
Storage/point validation/reads, tables, hashing/trace work, allocation, sorting,
traversal, exponentiation and audit remain unpriced. No speedup, GAE/S/rho,
fastest-external or intrinsic-regularity claim is admitted.

Controls alter a direct trace, direct demand, on-curve cached sign-slot point,
missing arm, missing failed query, scalar or source, then rehash the index.
The false cached point remains on the actual curve; no invalid-curve scope.
The first PDF text-equality failure (large fraction/bracket extraction after
Ghostscript) is retained with exact source/log/before-after text. Flat coordinate
notation and a standard-font proposed-route layout preserve final result text;
final extraction matches. No failed native run is hidden. All native commands
have600s caps, existing local resources and two build workers; none hits a cap.
Build/test elapsed numbers are diagnostics, not research timings. No profiling
was rerun; historical missing gawk supplies no cost evidence. Conductor exits127;
no shared reservation, replacement coordinator, paid compute, upload or
credential reuse is claimed. Library/Cargo/registry/catalog/benchmark and all
prior frozen files remain unchanged.

## Restore exact raw records

```
bash research/quadratic_twist_evidence_20261021/validation/direct-sign-20261025/restore-runs.sh FRESH_DIRECTORY
```

Canonical cases remain loose because replay bindings name them. Only exact
canonical duplicate case copies are dehydrated; every altered payload and
rehashed index remains in the archive. Restoration overlays the archive on
hydrated cases and checks **168 original file hashes**. The compiled producer
is omitted, with hash/size, source and build command retained. `INPUTS.sha256`
binds sources and existing proof/certificate backends; `OUTPUTS.sha256` binds
all new canonical outputs. Full restoration and deterministic repack both pass.
The archive is34,052 bytes; the inspected PDF57,795 bytes, below64KiB, and
canonical JSONs remain below1MiB. No historical evidence was overwritten.

The next selected F13 degree-two twist route is an unexecuted validated draft:
three exact models already registered, explicit exceptional map/subgroup checks
required before useful-row/degree comparison. Registry lookup/atlas receipts are
metadata, not a reproduced new reduction. The j0 cubic alternative remains a
separate deferred draft. Broader mathematical, complete-cost, coordination,
exact-head CI/review and integration gates remain open.
