# Duplicate comparison guard

The frozen 200-cell plan lists direct-propagation versus direct-search twice:
once in the common-control comparisons and once in the per-method comparisons.
No run or raw answer is duplicated. The first audit retained both comparison
entries and consequently counted forty accepted pairs in that distribution,
despite only twenty configurations. Its v1 output remains preserved but is
superseded for aggregate claims. The other distributions and raw gates were
unaffected.

Before the corrected audit, freeze a guard that interprets each unique
(candidate,control) pair once per configuration and once in distributions.
Reopen the same raw run and PARI receipts into a fresh v2 audit directory.
Require accepted pairs <= requested pairs for every published distribution.
No scientific worker rerun or change to its inputs, outputs or instruction
counts is made. Reports use only v2.
