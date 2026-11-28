# Reproduce the bounded compiler and proof qualification

Use fresh driver, target, log, proof and worker paths. Preserve all archived records. Shell scripts orchestrate compilation/limits and Rust performs mathematical tests; Python here only edits/copies source, checks metadata/hashes, or lays out recorded results.

Verify the current compilation manifest from the current source root. For the historical context, obtain the exact input manifest at the pinned Git commit and verify every byte/mode while materializing its files. The historical source helper has create-new and current-success guards; a prepared helper is not evidence that it ran.

`build-one.sh` records AS6; `build-one-v3.sh` records AS8; `build-one-v4.sh` records AS10 and the cache-recovery followup. All use the same opt3/LTOoff/codegen16/debug0/incrementalfalse flags, offline locked dependencies, CPU0/jobs1 and the declared numerical-thread settings. They never invoke root libtest. Existing raw paths are create-new protected. Copy drivers into a new directory and change only the absolute output/cache paths in that new source, with new pre-call hashes, before reproduction. Do not reuse supplied logs or silently retune compiler settings.

The source-root, manifest, stage and Cargo subcommand are explicit arguments:

```sh
bash NEW_DRIVER/build-one-v4.sh current CURRENT_SOURCE CURRENT_MANIFEST NEW_STAGE build --bin root-worker
```

Set the chosen source manifest in the new driver directory before compiling. Preserve Rust1.100.0 and all Cargo locks. The supervisor samples target usage/cgroup memory/free space every two seconds, stops its owned process group on refusal and records both compiler exit and source-postcheck exit. Compiler-budget seconds are operational limits, not relation-pipeline timing samples. All attempts, including failures and their dependency-cache state, count against the declared aggregate budget.

For cache recovery, inspect `CACHE_FILE_INVENTORY.json`, the exact backup hash, the file/mode audit, protected executable guards and the actual release outcome. These select only34 rebuildable cache files. The backup is RAM-local; the original compiler cache is not part of the source/equation proof evidence. Do not run release against a new cache using the old inventory. Create a fresh inventory and backup, verify every member and original byte/mode, check ownership/links/references, then require measured reclamation. Retain unknown process visibility rather than claiming a complete global scan. Restoration must use one member stream and verify every restored digest; its unexecuted path is not a tested recovery claim.

After a successful current worker build, the planned additional commands select only the worker's two existing units and the unchanged standalone native integration targets:

```sh
bash NEW_DRIVER/build-one-v4.sh current CURRENT_SOURCE CURRENT_MANIFEST NEW_UNIT_STAGE test --bin root-worker --no-run
bash NEW_DRIVER/build-one-v4.sh current CURRENT_SOURCE CURRENT_NATIVE_MANIFEST NEW_NATIVE_STAGE test --tests --no-run
```

Freeze the four actual executables and their hashes before calls. `freeze-workers.py` refuses failed compiler/source statuses and uses the declared terminal worker step. Its qualification-only pins whitelist only actual identities and the six old input digests; they cannot authorize the timing collector. `run-qualification.sh` invokes `--qualify`, unchanged native integrations, full independent parent replay and separate native auditors. Keep all raw failures and fresh proof trees. Compare original/proof/native/receipt bytes with the recorded source-specific opt0 reference. Do not compare incidental compiler/test durations or call them runtime gains.

Only complete current qualification and explicit memory/disk headroom allow historical source materialization and the symmetric historical compile/check. If a build fails or is refused, record later checks as unexecuted. Neither a compiler resource failure nor a proof-qualification pass establishes a15× speedup. [pcc:unverified]
