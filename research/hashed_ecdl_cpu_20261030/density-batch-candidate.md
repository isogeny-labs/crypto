# Density-aware residual scheduling

The preceding five-arm panel showed unconditional geometric growth lowered the
small-fixture median but raised both larger-fixture medians. Before new timing,
choose one structural threshold: when distinct suffix sums cover at least1/8
of the validated group order, grow batches1,2,4,... up to the configured cap;
otherwise use full batches immediately. Count unique keys while constructing
the existing index, so duplicate tuples cannot inflate coverage.

This is a heuristic scheduling decision, not a probability guarantee: query
residuals need not be uniform or independent. Every prefix is still visited in
order until the first exact match, and absent queries still visit all prefixes.
The rule never inspects known fixture scalars or curve-specific IDs. No extra
heap table is retained. Batch1 behavior is unchanged.

Literature context: standard batch-inversion cost tradeoffs, reviewed in
Bernstein–Lange precomputation arithmetic during earlier iterations. Motivation
is the matched round10 fixed/progressive diagnostic, not a new mathematical
claim. Threshold1/8 is chosen before the next panel; it is not fitted during it.

All 29 release tests and strict Clippy pass. The duplicate-sum test now checks
unique counts against an independent set, and all-target exact first-witness,
wide arithmetic, reset, bound and full-certificate checks pass. Freeze a new
five-arm comparison with baseline2bd4588fba60 and previous fixed candidate
abb2795c95cb. Retain all full recovery phases, A/A and batch1 controls.
