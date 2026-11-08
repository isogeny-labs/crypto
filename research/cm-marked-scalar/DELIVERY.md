# Delivery reconciliation, 2026-11-08

PR #3674 was opened before implementation/execution and carries the five-curve construction, native qualification and cost/report records. Scientific source/report commit: 73275020925ce002834e3422c9012b7ef067d78e. The counted source/inputs were published earlier at 7e8109ea2a65; their original receipts remain unchanged.

Upstream advanced to 2e5584c88a5c while the PR was open. A provenance-preserving merge, without rebase, conflicted only in problems/README.md because both sides inserted a row after the table separator. Comparing merge-base to our side showed exactly our single added CM admission row. The resolution keeps the full current-main index, including the new P3 census problem, and adds our row. The native study files and all evidence hashes are unchanged.

A broad `git diff --cached --check` against the old branch HEAD began fetching/checking the many unchanged upstream records. That read-only check was terminated (143) before commit; it is not represented as a passing check. The final check compares the PR's own paths against the fetched current main. No inherited scientific record or source was rewritten to satisfy that check, and no experiment was repeated for the merge.

The full runtime-improvement objective and the admission conditions remain open. The merge and PR metadata do not promote a scientific claim or convert the unfrozen timing draft into an executed experiment.

The PR-wide whitespace check reports a final blank line in two already committed renderer-warning logs: reports/native-five-curve-20261108/chart-render.log and render-v1.log. Their original bytes are preserved. The remainder of the PR passes the same whitespace check with only those two archived logs excluded; the unrestricted check is not claimed to pass. Report/input/raw-data hash checks pass. The new CM index row is placed after the existing table rows, preserving the current-main P3 entry and reducing repeat conflicts at the first row.
