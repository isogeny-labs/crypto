# Structured export limitations and correction

The scientific plan, frozen before data, is the unchanged 880-cell PLAN.json.
Its fixed design has five common arms on 160 configurations, plus a companion
arm on only 80 orbit configurations. Positive controls are development tests;
the unshifted negative control belongs to the separate geometry phase.

The initial structured phase metadata passed schema validation and freezing,
but encoded `sample_size.instances=10` (the fixture count), not the full
configuration count, and declared those separate controls as arms. The generic
checker expects every declared arm on the same instances. This was a metadata
design error, discovered during post-run binding. No task was removed or added
after seeing outcomes. The original protocols, failed checks and export-v1
remain evidence; their pre-data status must not be claimed for a correction.

The native export-v2 fixes relative file references and the maintained schema's
completion convention: a conclusive unsupported failure is a **completed
process**, although its scientific output is excluded. Thus the cost export has
775 completed processes (751 accepted plus 24 underfilled) and 105 incomplete
budget exits. The scientific audit still has exactly 751 accepted outputs out
of 880 attempts. Neither meaning is silently substituted for the other.

Standalone schema/file validation passes. Full binding to the frozen generic
factorial protocol remains rejected. This is a reporting/validation gap, not a
failed native finite census or permission to promote a benchmark row. The
frozen native plan, raw hashes, independent audit, masked analysis/unmasking and
strict PARI replay remain the evidence for the bounded diagnostic. No
VALIDATION.json, leaderboard row or complete-IC claim is created. A future
comparative run needs compatible per-panel protocols frozen before new data.

Geometry is one producer execution emitting 240 bases and 7,680 query records,
not 240 independent host executions. Preflight has 60 operational sizing
attempts excluded from primary performance; its outputs are not independently
certified as cost results or pooled into the 880-cell panel.
