# Classical ECDLP precision component: resume checkpoint

Goal stays active. Classical algorithms only, local execution, Amazon S3 storage excluded.
This checkpoint is a conditional scalar proof result, not a complete counting or
ECDLP improvement certificate. Preserve the frozen84-case comparator protocol.

Native source/evidence commit: isogeny-labs/crypto
46249ffddaf4ec36dbf81310aa9693062c8ce46a, draft PR4996.
Shared model: aburan28/crypto-lean
3e47fb776a4eb5231de4c8aad2bb6e756c2f89a1, draft PR35.

Three Std-only theorems and seven scalar controls pass with Lean4.35.0.
PCC local proof verifier and seven-control gate pass; fresh-directory
reproduction uses no Mathlib. Producer claims remain unsigned. The two
interrupted native proof gates are preserved and yielded no verdict.

Next mathematical obligation: formally instantiate the finite call history,
using the exact source increments, positive scale, active valuation and strict
1/8 edge shrinkage. Compose the existing geometric-series laws with the shared
increment bound, then bind initial P0<=s and M0<=s to the native normalization.
The written proposed bound81*s*2^A is not yet a history theorem. General
P0<=C*s changes that coefficient. Keep symbolic-history arithmetic distinct
from extraction/refinement of actual successful Rust executions.

Then discharge scalar dimension absorption from actual source guards and
continue packet/mode, construction-precision, allocation, rational-width and
entry bit-work obligations. Reconcile the real optimized comparator sources
before the larger complete-solver study; no solver experiment ran in this
precision component. Do not turn a conditional bound into a scaling claim.

Reproduce focused component from native repo root, Lean4.35.0/Lake on PATH:

```sh
bash research/hashed_ecdl_precision_20261123/reproduction/check.sh NEW_DIRECTORY
```

Keep one Lean worker and the existing phase deadline/physical-memory guard.
Whole-library/current-root CI and independent execution provenance are separate.

External generated Mathlib caches were compacted reversibly. The two archives
are retained locally outside this small handoff capsule:

- /tmp/ecdlp-precision-audit-20261123/cache-compaction/mathlib-editor-indexes.tar.zst
  SHA256397490d1832c0493b5d047c013adb8c6ccf76ee036450a80da1f640f5d91fbee
- /tmp/ecdlp-precision-audit-20261123/private-cache-compaction/mathlib-private-cache.tar.zst
  SHA256ab140b503318036e9e30e50a2451370d883cbdf414eab5f4f8ed91b4ddb1abe3

Full lossless comparison/rehash receipts and restoration commands are retained
with cache manifests. Restore the needed dependency cache and preflight memory
before Mathlib work. Std-only reproduction needs neither archive. No owned
research source, raw output, native executable or earlier proof checkpoint was
removed. Earlier native and public-counter handoff capsules remain unchanged.

The original SCALAR-REVIEW-01 and62-input snapshot are immutable. Global
PROOF-MAP.md is restored exactly; later outcomes are in PROOF-UPDATE-01.md.
Publication review and subsequent outcomes are new addenda, never rewrites.
