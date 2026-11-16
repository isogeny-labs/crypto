# Cached-theta xADD arm

Frozen before running the second collector arm. The first full cohort uses
the literal xADD implementation from the coordinate gate: each call
recomputes the inverses of four fixed theta constants. Its raw rows will be
retained under `results/uncached/` with exact source and price checks.

The second arm will invert those same four nonzero constants once per cold
target in the walk and once in signed Recover, cache the results, and use
otherwise identical xADD equations, quotient partition, exception repair,
Recover, graph and source verification. Initialization inversions are counted
as cold candidate work. Run all 1,260 targets and all 36 price cells for
both arms; retain each row, including losses, without screening or changing
the target order. The previous `PROTOCOL.md` remains the baseline protocol.
