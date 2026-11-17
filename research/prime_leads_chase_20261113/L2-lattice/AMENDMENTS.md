# Amendments — none to the rule (2026-11-13)
Interpretation notes made after the data, recorded so they are not mistaken for rule changes:
1. summarize.py marks POS at δ=1/2 "FAILS" because its literal test is ν_1 < 1; the planted element has
   weight 2δ = 1 exactly, and ν_1 = 1.000 equals the expected 2δ at all six δ. The detector works; the
   cell is a boundary artefact of the summary script, not a control failure.
2. The "reference" column for F (chain) uses h of the eliminated S4 (12δ), as the protocol says. For
   γ(u)=1/2 that comparison is not like-for-like (the restricted chain solves a sub-problem whose
   solution count is smaller by p^{1-γ}). Reported both literally (one H1 cell, δ=1/12) and with the
   accounting derivation in REPORT.md §3 (idea F).
