# Current formal-file digest, 27 October 2026

This is a file-integrity inventory for PR #2304 at source commit
`12744b3ffe50f9e5fd2b756f25bfb6e57b38f10c`. The companion
[`CURRENT_SHA256SUMS_20261027`](CURRENT_SHA256SUMS_20261027) lists 119 present
files and has SHA-256
`1dcc25aa8e749ffd79ef2e31207d7d7cb647c9b21824393ea9b8cf9baf21e07b`.
It includes every path from the historical N9 `SHA256SUMS`, the new branch
and cross-chart Lean files, the current weighted-quotient PDF/graph/CSV, the
proof audit, and the historical manifest itself. It does not include itself
or this note.

The historical N9 `SHA256SUMS` is preserved. Its hashes no longer describe
the later checkout for 13 paths: `.artifacts.json`, `.gitignore`,
`Common22InfinityChart.lean`, `INFINITY_CHART.md`, `LOCAL_VALUATION.md`,
`README.md`, `RuntimeSemantics.lean`, `coefficient-slice.json`,
`cross-scale22.json`, `infinity-chart-build.log`, `runtime-bindings.json`,
`source-bindings.json`, and `verify.py`. The dated inventory records current
bytes without changing that earlier snapshot. A digest match is file
integrity evidence; it is not a substitute for compiling or reviewing a
theorem.

From this directory, verify the present files with
`sha256sum -c CURRENT_SHA256SUMS_20261027`. The formal `python3 verify.py`
also passes after the written-audit binding was refreshed. Direct pinned
Lean compilation of `Common22BranchDomain.lean` and
`WeightedProjCrossImage.lean` exited 0, and their `#print axioms` output
lists only `propext`, `Classical.choice`, and `Quot.sound`. The
branch file imports a separately checked module that is not a default Lake
target; a fresh checkout can reproduce the direct compile with:

```sh
lake env lean -o .lake/build/lib/lean/Common22BranchValuation.olean Common22BranchValuation.lean
lake env lean Common22BranchDomain.lean
lake env lean WeightedProjCrossImage.lean
```

The [weighted-quotient report](WEIGHTED_QUOTIENT_COMPONENTS.md),
[PDF artifact pointer](.artifacts.json), [graph](weighted-quotient-components.svg),
and [status CSV](weighted-quotient-components.csv) state the theorem scopes
and the remaining gluing and Picard-class obligations.
