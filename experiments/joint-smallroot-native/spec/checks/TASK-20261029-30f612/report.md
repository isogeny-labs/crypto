# TASK-20261029-30f612 — source-only Q4 preparation

Exactly one existing native `cache-preparation --seal` invocation finished with
exit 0: 80 candidates, 80 accepted, zero excluded, zero failures, 4,557,159,684
accepted bytes. Every accepted current SHA256 equals its frozen historical
expectation. No cache advice, container creation/start, guard driver/worker,
allocator, qualification case, or scientific invocation occurred.

| Requested requirement | Status and evidence |
| --- | --- |
| Actual source-only admission, published snapshot, current claim and maintained ownership before writes/seal | Verified: `preparation-authority.json`, `dispatch-readiness.txt`, and pre/post-seal lease receipts. Admission SHA256 `2313d2447253655a7588a8a641397c9dd46cd8ef5d45d4350653007574041b78`; exact snapshot `e6d1de8f1a384f309e52990b1c9682cbd20cb744` bytes independently checked. |
| Original C1 sources/binaries, unchanged production, actual libraries/toolchain lineage | Verified current hashes in `source-binary-library-manifest.json`; original C1 source retained in `source/original-C1/`; ELF interpreter/NEEDED/RUNPATH inspected with `readelf`, without invoking the driver/worker. No rebuild/substitute. |
| Exact pinned image and allowlisted runtime metadata | Verified image identity/digest and daemon metadata in `runtime-metadata/`; no Config.Env or Docker configuration read. Retained original image-library bytes match the frozen hashes; current image contents are bound by immutable image identity, without a fresh extraction/container. |
| One source-only native seal over frozen owned80 inventory | Verified `cache-seal-input/`, raw `cache-seal/`, and historical-hash-filtered `cache-eligible/`. No extra candidate or rescan/substitution. |
| Strict new Q4 launcher with fixed resources and new namespace | Implemented, shell syntax validated, operationally unexecuted: `launcher.sh`. Only exact Q4 accepted; original driver/worker, image, 16 GiB parent/8 GiB worker/effective swap0, 2 CPUs, pids64, private cgroups, no networking, read-only rootfs and exact read-only binary/library mounts. Original production 1800s timer; outer1950s wait protection. Partial outputs and owned-container removal retained. |
| Expected/runtime/source/native-helper/eligible seals and complete byte manifests | Verified `expected-checks.json`, `runtime-source-seal.json`, `source-binary-library-manifest.json`, eligible allowlist and `owner-stage-manifest.json`; manifest's self digest is external to avoid recursion. |
| Ancillary library failures and replays retained intact | Exact25-file `ancillary-library-validation/` copied and member hashes verified; manifest SHA256 `3036b6fb1bbc5dc2e477d3d04a11271c32b248a1f83502b78f2977797be03a47`. Original full run: 5198 passed,8 failed,123 ignored,exit101. Each missing-input failure separately passed on same-binary replay after tracked restoration. No full-suite PASS. |
| Actual owner preparation publication and programme four-file outputs | Root owns publication. Owner stage is frozen for root commit/push; final programme files bind only the actual returned owner commit. No future commit is asserted here. |

The source review used both AGENTS contracts, the owner's report-evidence skill,
the task's actual continuation and two prior decisions, original C1 driver/worker,
the unchanged original runtime wrapper, retained cache helper source/build command/
compiler/runtime records, and managed cloud runtime Docker/networking guidance.
These primary software sources establish the frozen argv, hash/identity semantics,
native headroom predicate and failure retention. No new mathematical technique or
novelty claim is proposed; external scientific literature was not resurveyed.

`prepare-metadata.py` is single-use control metadata: native `sha256sum` hashes
software metadata inputs, and the unchanged existing native helper alone reads and
hashes all80 cache candidates. Python performs no cryptographic, scientific,
performance or harness computation. Its metadata comparison excludes hash
mismatches before any possible future advice; there were none. Dynamic-loader
override selectors were observed unset/empty before invocation; no environment
dump was retained.

Future operation requires actual published owner stage plus the existing6cad2d
programme snapshot and separate live T6/case claim/admission. The unexecuted
launcher requires fixed `prelaunch-admission.json` verdict `admitted_Q4`, exact
task6f4f28, absolute owner seal/manifest/expectation hash references, actual
publication commit bindings, and the fresh matching safe T6 proof. It has no
cache branch. Conditional advice remains separately admitted, at most one pass;
the source-only seal neither performs nor authorizes it. Current headroom is
ordinary timestamped metadata only; no future H or OOM outcome is inferred.

Full Rust raw log custody remains incomplete: the unchanged464884-byte local
raw log has SHA256 `484e23d62444cf64ab020ecda783475bf28344d0370029730307b1c3b23e87cb`;
canonical upload is false. The compact package contains excerpts/replays,
not that full log. No large log was split or added to Git; no ELF is included.

Managed native session: `gpt-6.1-sol`, effort `high`, session
`native-q4prep-20261029`, actually selected as reported by root. API model probe
remains unverified; fallback false. The original scientific24-call panel remains
zero executed, reach/cost and verified prime-field IC gain unresolved.
