# Reconstructing the retained metadata merge inputs

`summary.json` records both exact parent commits and the SHA-256 of each
registry input. The input bytes remain in Git; they are not duplicated as
large loose JSON files here. The original stage captures and failed/corrected
helper stdout remain in the durable scratch merge directory. No scientific
arithmetic or frozen relation data was merged by this helper.

To reconstruct inputs in a separate scratch directory, use the merge base
of the two `parents` listed in the summary for stage 1, the first parent for
stage 2, and the second parent for stage 3. For each commit, `git show
COMMIT:docs/curves/registry.json` and `git show
COMMIT:src/cryptanalysis/curve_aliases.json` reproduce the registry and alias
inputs. Write the two parent IDs, one per line, as `parents.txt`. Check every
registry input against the recorded SHA before replay. `merge-metadata.py`
is the exact corrected executed helper: its paths target the original
worktree and it must not be run casually against a newer checkout. Use an
isolated worktree for a metadata replay. The output hashes in `summary.json`
identify the actual merged bytes; `semantics-addendum.json` clarifies the
325 one-sided metadata changes and unchanged canonical identities.

This historical reconciliation is complete. It does not cover later main
commits, establish scientific correctness or authorize replaying experiments.
