# Independent final-008 committed-source archive review

Reviewed driver `b784714e7f1d311ddccf9f90d9611b8922a66b3afd75e052cd0e6517d073d8dc`, incremental patch `ce6f6db6109d819cfc4c7d3085189afbda50869c3adfe78dbdae77b34607dd2c`, and template `2d186e455351eef95d6488abe46d0c840c4d6df41260a8a126fcdbb3871a0bd9`; all hashes recomputed. AST comparison with final007 confirms gates is the only modified existing function, with two new archive helpers. Template comparison found exactly three driver/analysis path changes, with no workload, limit, phase, or acceptance changes.

## Review conclusion

No blocker identified in the examined source-verification replacement. HEAD/tree/clean checks through source_gate and current regular/non-symlink file hashes remain. The snapshot's exact expected file set is used as literal pathspecs in a single git archive invocation against the full commit ID; working-tree contents cannot substitute for that named commit. Git export attributes that change or omit content would fail the exact hashes/member set rather than silently pass.

The unmodified Runner.command captures exact arguments, NUL arguments, binary/protocol before/after hashes, raw archive stdout hash, stderr hash, process outcome, times, and cleanup. The successful receipt precedes parsing. Existing per-command and whole-driver deadlines apply. Output is retained on disk rather than passed through a stdout/stderr pipe cycle. No extraction occurs.

Archive parsing requires canonical relative effective member names, no traversal/absolute paths, no duplicate normalized member names, only expected regular files and zero-sized ancestor directories, exact expected file sizes and SHA256s, and exactly the expected completed file set. Symlinks/hardlinks/other non-file entries cannot satisfy a file. Reading from the parser's end offset rejects any nonzero trailing archive data, including a second hidden archive. Empty or missing expected content cannot pass. Tar metadata is interpreted by tarfile, but effective file identities and logical content still face the exact checks; no metadata is materialized on disk.

All final008 before/after artifact hashes match current files. The eight zero-exit command logs match receipt hashes and contain 122 test invocations. The real temporary-Git test uses the actual Runner, verifies literal arguments and receipt/raw-output hashes, includes spaces and binary contents, and confirms subsequent working-tree changes cannot alter retained committed content. Malformed controls cover missing/extra files, links, duplicates, traversal, hash changes, unexpected directories, truncation and nonzero trailing bytes. These are guard tests, not dataset generation.

The saved official template-validation record has zero schema/lint findings and binds the reviewed template; it remains unfrozen. The original 46 instance IDs, 92 phase observations, failure handling, nonduplication of counters, restored-verification semantics and success export binding are unchanged. Current-file checks remain sequential, not an atomic filesystem snapshot; existing ownership/source gates are still required.

No source edits, build, crypto CLI, solver, cloud action or original-prepare mutation occurred during this review. Code and retained evidence were inspected and hashed; guard suites were not independently rerun.
