# First complete run on the tree of the pull request (binary A), superseded

Everything here ran to the end (build, unit tests, graph, fidelity, counts, verify, reg, clippy) with the binary whose
SHA-256 is in `binary_hash.txt`.  Two defects were found in its records, both in bookkeeping, neither in the search:

1. In `--all` (every model) runs, `found` was computed from the solver's final outcome, which is `Unsat` after the last
   blocking clause, so every target that has roots was marked `ok: false` for the four shell arms (`failures.log`: 16 + 3 verify
   runs and 4 regression-verify runs, exactly the targets with roots); the root lists themselves are equal to the corpus' (the
   `roots` counts agree with `a1`).  Fixed by taking `found` from the number of models.
2. `summarize` lost the kind of leaf (the record's `leaf` key holds the counts) and counted the bare-guard arm's learned clauses
   as solver conflicts.  Fixed in the summarizer.

The rerun with the corrected binary is in `../`.  Kept because a run that exposed a defect is evidence; no number in the README
comes from this directory (the deterministic counts of the two runs are compared in the README, section "Reproduction").
