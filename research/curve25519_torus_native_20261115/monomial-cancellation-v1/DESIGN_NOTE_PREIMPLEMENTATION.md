# Control adaptation before implementation

The old rollback polynomial pair shares its quadratic terms and differs by a
multiple of1-z. This suggests that the new degree-two row space can derive a
unit in z before branching. That is a hypothesis to check natively, not an
observed solver outcome yet. Keeping an assertion that this solver must Undo
would require it to ignore the very consequence under study.

Keep the original rollback system/query/outcome in the same35-system controls.
Do not require the new arm to traverse the old forbidden branch. Instead retain
a separately labelled AUTHORED coefficient/lease trace over the SAME rollback
system: assignz0, admit the constant consequence using its original-row
weights, Drop, Undo, then assign the z1 root. The existing independent reader
must accept that valid trace and reject omitted Drop, wrong Undo, duplicate
assignment, premise/coefficient/origin corruption and bad root. This is a trace
fixture, not a measured solver path, node count or completeness argument.

This adaptation is frozen before code or native results. It preserves the
checker contract and all original inputs; it does not relax correctness or
resource/pruning gates. Record both the actual solver trace and the authored
trace explicitly. Future node comparisons use actual solver output only.
