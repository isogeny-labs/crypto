# Fixed-difference quotient walk gate

The [PDF](TRANSITION_GATE.pdf), [graph](transition-cohort.svg),
[summary](results/summary.json), and [raw rows](results/) show a three-step
source-label gate on all 1,260 frozen targets. All 3,780 quotient transitions
matched independently computed Jacobian classes and exact labels. The
canonical-ratio partition chose branch 0 in 1,860 cases and branch 1 in 1,920.
No two-torsion xADD repair was needed in these three steps.

The initial scale-invariant Legendre-product partition chose branch 1 in
3,778 of 3,780 transitions. Its full raw rows are retained under
`results/legendre_diagnostic/`; [AMENDMENT.md](AMENDMENT.md) fixes the
canonical-ratio partition used in the subsequent run. The two partitions
must remain separate in review.

The recurrence establishes a closed quotient walk with source labels; it
does not show that relations accumulate at a useful rate. Signed relation
extraction, a complete collector, and paired rho pricing remain to be run.
All cost and ratio fields are null.
