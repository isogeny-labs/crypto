# CI tracked-path output handling

Four Rust jobs at2f5e332 failed before compilation in ci-scope.mjs: git ls-files -z exceeded the16MiB stdout buffer. Pinned base path bytes16604789; PR head17277485. New evidence files crossed the limit. Source classification and compiler gates are unchanged. Git stdout now uses a private temporary file, read in full with its NUL delimiters preserved; stderr remains bounded and process failures propagate. Temporary output is removed on success and failure.

A real Git index with80000 archive paths plus two source names (including UTF-8 and newline) reproduces ENOBUFS under the old helper and passes under the correction. All14 scope tests pass. Read-only discovery on the actual checkout finds2164 source files and196 embedded inputs without materializing files. No native Rust implementation changed; existing library/GPU/Clippy/example checks retain their source bindings.

Source review: existing ci-scope.mjs and test-ci-scope.mjs; official Node child_process documentation https://nodejs.org/api/child_process.html describes maxBuffer termination and descriptor-backed stdio. CI job links and extracted error lines are in ci-observations.json. Full logs remain at those jobs and private hash-bound paths in raw-log-custody.json. The local node executable was initially absent; no tests ran until the exact CI version22.23.4 was downloaded privately and matched to the official SHA256 manifest.

Remote CI for the corrected revision remains pending publication. No scientific execution, quota extension or measurement result is inferred.
