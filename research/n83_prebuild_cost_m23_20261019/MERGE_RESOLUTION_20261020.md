# Parent solver-cache merge resolution

The base at `124ab188cc39d52ee99e979da95b501a3ca34f88` was merged into
the branch without rebase or force-push. The only conflicted path was
`src/cryptanalysis/sat.rs`. The 01:55 UTC Copilot request received no
response or resolution for more than three hours; the author took over
under the repository fallback rule and recorded the reason on PR 2271.

Both explicit policies remain available and default off:

- `set_xor_decided_cache`: the boolean row cache measured in this study,
  retaining its separate checked/skipped counter semantics.
- `set_decided_xor_row_cache`: the parent's generation-based cache,
  retaining generation invalidation and its existing callers/tests.

Enabling either disables the other and clears the other policy's cached
certificates. Switching invalidates the XOR propagation epoch. Matrix
clear/rebuild and backjump maintain each policy's invalidation contract.
New directed tests cover switching on an existing matrix and generation
wraparound. All 36 standalone native SAT tests pass, including both
branches' exhaustive formula checks and fixed search-trace tests:
[`merge-sat-direct-tests.log`](merge-sat-direct-tests.log).

The first release-library build rejected the new tests' incorrect
backtracking method name before execution. That log is retained; the
tests now call the solver's actual `backjump` method. The complete
release-library and affected-example gates are running separately and
must pass before push. Their eventual status is recorded in
`merge-release-lib-20261020-2.log` and
`merge-release-examples-20261020.log` with separate exit files.

The running full-base experiment remains pinned to source
`d9ef42301cc3d22f437e1f89304e673836afb6a3` and its frozen worker hash.
No merged implementation is substituted into its running card or
historical performance attribution. All parent research artifacts merge
without conflict or content selection; no frozen evidence was rewritten.

The independent catalog assessment repair is a subsequent change. Its
exact membership audit, arithmetic checks, report regeneration and CI
status remain distinct from this solver resolution.
