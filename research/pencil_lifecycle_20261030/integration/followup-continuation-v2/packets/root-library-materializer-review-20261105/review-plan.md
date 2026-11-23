# Prospective independent source review: root-library materializer

Written before reading producer materializer-v1 source. Reviewer did not author this producer implementation. The requested design and historical durability failures are known; the implementation is unread. Await the exact published commit supplied by root before opening producer files. This is a source-review plan, not an execution authorization. [pcc:unverified]

## Claim and prior

The proposed materializer should construct a fresh, bounded source/vendor/cache input set for the root library at pinned revision29861c9, preserve include/path dependencies, and produce new hash-bound evidence without relabeling an old manifest. Prior: the construction is plausible, but path resolution, cache provenance, limits and failure cleanup are load-bearing. No runtime qualification is presumed. [pcc:unverified]

## Review joints and worked attacks

1. Revision and source identity. Expand29861c9 to its full immutable commit and verify against the published binding. Attack: a dirty checkout or branch tip supplies different include bytes while a receipt repeats29861c9. Every copied source and included data file must derive from the pinned tree or an explicitly bound separate input. [pcc:unverified]

2. Path/include closure. Trace include_bytes/include_str, build-script inputs, Cargo manifests/workspace members and local path dependencies. Attack: an omitted fixture or symlink escapes the selected root, or an absolute include resolves to an unrelated host file. Require explicit resolved-path containment, file-kind handling and byte/hash bindings; missing required inputs fail closed. [pcc:unverified]

3. Vendor and cache provenance. Check Cargo.lock and package checksums, source replacement configuration, registry/path/git dependencies and copied cache bytes. Attack: a matching directory name hides a different package version or a cache entry silently substitutes unbound executable artifacts. Distinguish content-addressed data from compiled cache products and preserve every provenance assumption. [pcc:unverified]

4. Fresh manifest and race safety. Follow metadata-before/hash/metadata-after or equivalent stable-copy checks and final verification. Attack: rewrite a file during copying, preserve its nominal filename, then relabel an old manifest as fresh. The new receipt must identify actual materialized files, sizes, hashes, source revision and verification phase; old frozen evidence stays unchanged. [pcc:unverified]

5. Resource admission and deadline. Trace finite disk/output, CPU, RAM, concurrency, runtime, cumulative allowance and retry bounds before side effects. Attack: a blocked filesystem copy or subprocess exceeds its phase while an unbounded parent waits. The existing owner dollar-accounting waiver must be explicit and narrowly applied; it does not change the46,800-worker-second campaign or2026-11-06T04:00Z deadline, reset previous charges, or waive resource/ownership constraints. [pcc:unverified]

6. Attempt ownership and cleanup. Trace fresh-directory admission, markers, process identities and exception/signal cleanup. Attack: an existing output path causes finally to overwrite another attempt's receipt, or an ambiguous unit identity causes unrelated cleanup. Require owned-only writes/termination, terminal/cgroup confirmation before successors, and bounded cleanup charged to the same attempt. [pcc:unverified]

7. Durable failure and acceptance. Inspect disk-full, missing file, malformed manifest, checksum mismatch, child failure and interrupted-copy paths. Attack: a partial manifest or success marker survives an error and is mistaken for complete materialization. Require failure receipts with actual completed counts, no success promotion from declared totals, and an explicit complete acceptance conjunction. [pcc:unverified]

8. Scope preservation. Attack: materialization quietly builds/tests, fetches mutable dependencies, launches scientific payloads, changes IAM or expands retries. Source review and any later bounded materialization must remain distinct from compiler/runtime qualification and the mathematical experiment. [pcc:unverified]

## Independent derivation and acceptance boundary

A valid materialization receipt needs an injective mapping from each declared source input to its fresh destination, a pinned origin and verified byte equality or explicitly recorded content transformation. A list of paths alone proves neither completeness nor equality. Every required include/dependency must be covered, and every executed tool must itself be identified. This is the review's initial acceptance criterion, before implementation inspection. [pcc:unverified]

Bind producer commit, source hashes, proposed commands and resource policy first. Inspect local source/reference evidence supplied in the published packet; consult primary Cargo/checksum documentation where package semantics are uncertain. Record concrete file/line findings and distinguish inherited prerequisites from new defects. No producer file has been read for this plan, no compiler/materializer is executed, and no positive runtime verdict follows from source consistency. [pcc:unverified]
