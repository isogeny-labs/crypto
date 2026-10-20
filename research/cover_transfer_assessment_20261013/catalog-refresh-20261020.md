# Catalog assessment refresh, 2026-10-20

## Exact change

The catalog moved from commit
`ff48e2800f62b45c746b3f391540c07b16af4e75` to
`8e4c99759fd8d9594ed207d793b07b59ebaf19f7`. The assessment generator had
already moved to the new population, while the independent verifier,
standalone binary population assertions, and report still expected the old
population. This repair reconciles those expectations with an explicit
native inventory; it does not change template statuses or promote transfer
or performance claims.

The Rust auditor in
[`examples/cover_catalog_transition_audit.rs`](../../examples/cover_catalog_transition_audit.rs)
reads both pinned Git snapshots and requires the current registry, cover
report, and graph bytes to equal the new snapshot. Its
[versioned inventory](catalog-transition-20261020-v2.json) records:

| Identity inventory | Retained | Added | Removed | Current total |
| --- | ---: | ---: | ---: | ---: |
| Exact model identities | 2,098 | 474 | 2 | 2,572 |
| EC1 representations | 664 | 313 | 0 | 977 |

All retained model JSON, slug and ICV1 labels agree. All retained EC1 curve,
field, generator and model bindings agree. The 977 current representations
belong to 972 models; five models have two representations.

The removed models are `icv1-fp21-tm700-56b6783a` and
`icv1-fp21-tm700-7ee99c50`. Neither had an EC1 representation. Both old
records remain available at the pinned historical revision and in the
inventory. The transition is not append-only.

The [first inventory](catalog-transition-20261020.json) is retained.
Version 2 adds the distinction between EC1 representations and
EC1-bearing models; both versions report the same retained, added and
removed identities. An earlier append-only assumption was rejected
when the two removed models were encountered; the explicit removal
inventory replaces that assumption.

The inventory also lists the complete before/after semantic sections.
`claim_contract`, `profiles`, `global_cost_accounting`, template keys and
template statuses are unchanged. The reviewed text changes in
`result_templates`, `controls`, and `open_obligations` update population
descriptions and distinguish historical release artifacts from current
source checks.

## Independent checks

The updated independent verifier reports **zero problems** for all 2,572
maps, 2,570 cover identities and 977 EC1 bindings. It reconstructs the exact
binding sets and replays 165 binary, 1,083 prime-field and 1,324
extension-field constructions, including 30 normalized targets. The
generator's byte-exact check also passes. Receipts:

- [Independent verifier](../n83_prebuild_cost_m23_20261019/catalog-refresh-verifier-4.log).
- [Deterministic generation check](../n83_prebuild_cost_m23_20261019/catalog-refresh-generation-5.log).
- [Native cover checker](../n83_prebuild_cost_m23_20261019/catalog-native-cover-check-20261020.log):
  2,572 verified, zero unsupported, zero invalid input.
- [Native inventory regeneration](../n83_prebuild_cost_m23_20261019/catalog-transition-replay-20261020.log)
  and byte comparison against the retained version 2.
- [Native library validation](../n83_prebuild_cost_m23_20261019/merge-release-lib-20261020-2.log).
- [Native binary and example validation](../n83_prebuild_cost_m23_20261019/merge-release-examples-20261020-2.log).

The release-library command passed: **4,401 passed, zero failed, 118
ignored**, exit zero. The affected binary/example command also passed:
**77 passed, zero failed** across twelve targets, exit zero. That includes
all 13 standalone hyperelliptic-cover tests, the 11 cover-checker tests,
both native transition-auditor tests, and the N83 worker/replayer/auditor
gates. The native transition regeneration is byte-identical to version 2.
The prior failed compilation is retained separately; it was a method-name
error in new SAT cache tests and was corrected before the current run.
The first example build also stopped on a tracked source omitted by this
sparse checkout. Materializing
`research/n83_xor_decided_cache_20261019/worker.rs` restored the parent's
include path before the second example build; no source shim was added.

The certificate CI job now regenerates the native transition inventory
from both exact catalog commits and the full pre-refresh ledger commit
`05bb1e403a13ca609a2a09f0635dfbe46772e27f`, then compares its bytes with
version 2. The auditor's regression tests run in that same job.

## Visual and PDF verification

Both editable diagrams and their SVG/150 dpi PNG renders now use the
current population. The report retains the historical 2,100-model release
receipt and its earlier allowlist boundary as historical evidence. All 13
pages of the rebuilt PDF were visually inspected. A stale 2,100-map diagram
label found during inspection was corrected and the affected final page
was inspected again.

The PDF's exact coverage marker is preserved. Tectonic stores its Info
dictionary in a compressed object stream, so the independent verifier
uses the native `pdfinfo` reader when the older literal parser cannot
read that representation. It still requires the identical decoded
Subject, PDF signature and artifact digest, and fails if decoding fails.
The CI job installs `poppler-utils` for that path.

Reproduction commands remain in the [main report](README.md#reproduction).
The native inventory is written with create-new semantics:

```sh
cargo run --release --example cover_catalog_transition_audit -- NEW_INVENTORY.json
node scripts/build_cover_transfer_assessments.mjs --check
node research/cover_transfer_assessment_20261013/verify_assessment.mjs
```

This is a catalog and CI repair. No measured performance value changes;
the index-calculus cost leaderboards and boundary ledger remain unchanged.
The N83 full-size experiment retains its original frozen worker revision
and binary independently of this repair.
