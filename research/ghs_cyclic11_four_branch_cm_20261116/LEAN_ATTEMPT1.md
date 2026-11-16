# Lean first attempt retained

The first whole-list `by decide` attempt under Lean 4.36.0 failed with
maximum recursion depth at the 910-row count and histogram. The
bound-ten equivalence also failed to synthesize a `Decidable` instance
for the opaque `paired` proposition. Its printed `sorryAx` lines were
the consequences of those failures, not accepted proofs. No limit was
raised and `native_decide` was not used. The next version checks ten
91-row slices with a Boolean `paired` predicate, then combines their
arithmetic. The original source is recoverable from the pre-edit tool
transcript; it was not committed as a passing artifact.
