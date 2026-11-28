# Bounded shared parity factoring

Every selected pair of generator functions is replaced in target XOR forms
by a new wire equal to their XOR. This preserves each target function exactly,
whether generators are independent inputs or include the selector product.
The new wire depends only on previous wires, so all circuits stay acyclic.
Early termination at a round/work cap remains exact: remaining expressions
are emitted directly. A cap does not imply zero roots or an incomplete circuit.

A prepass identifies primary/selector-affine forms required at nonlinear uses
and outputs. Unexpected later forms use the original exact emitter. Gate
induction and pruning preserve every old predicate. The complete tiny union
oracle is rerun with this compiler; raw linear transforms have independent
exhaustive bit-parity checks. Actual tests compare every source output plus
all14numeric predicates and original recovered guards.

Maximum64greedy substitutions and8000000pair events; deterministic pair ties.
Only69initial generator functions, no exponentially large state table. One
core4096MiB,90s outer supervisor,78/84s finalization checks. This is known
common-subexpression logic synthesis, not a novel rootfinding claim. Promotion
requires both total variables/clauses below the same C105source control;
qualified whole-system time/memory performance remains a separate gate.
