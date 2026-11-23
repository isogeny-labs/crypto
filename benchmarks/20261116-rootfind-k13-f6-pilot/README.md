# K13 local rootfinding diagnostic

This directory contains documentary copies of the fresh-target local K13
rootfinding versus F6-IC reports. The source of truth is
`research/rootfind_dense_binary_pilot_20261116/`.

The [first epoch PDF](attempt1-REPORT.pdf) and
[SVG](attempt1-paired-ratios.svg) report a complete, replayed 200-execution
L0 session whose benchmark record is classified **invalid for performance
admission** because the binary's runtime git commit came from another
worktree. The raw records and exact discrepancy remain in the study. This
directory is outside the admitted results table in `benchmarks/README.md`;
it carries no `VALIDATION.json` and changes no leaderboard.

The [custody-corrected repeat PDF](custody-rerun-REPORT.pdf) and
[SVG](custody-rerun-paired-ratios.svg) document the same frozen spec under a
worktree-local binary path. Its 200 answers and 160 measured replays passed,
and it records a local L0 complete cold process-wall comparison. The repeat
uses the same seen targets, has no independent host replay, and remains
outside the admitted results table. The two epochs are never pooled.
