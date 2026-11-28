# Exact solver multiplicity fixtures

This study addresses items8,12,14 of the original fourteen-item audit.
The complete request/remaining obligations stay in
../theorem_census_20261109/README.md. No empirical scaling, timing,
generic F4 refinement, or independent-host claim is in scope.
[pcc:unverified]

For each of four two-variable inputs, run pinned msolve0.6.5 in two modes:
full reduced grevlex basis (`-g2`) and default parametrization. Run each
twice, interleaving modes. Exact sparse linear algebra (`-l2`), one thread,
default genericity handling, no saturation/signatures/normal forms, timeout
30seconds per call. All16calls and failures are retained. No retry based on
an outcome, optional stopping, input extension, or after-data parser repair.
[pcc:unverified]

| Input | Field | Expected quotient dimension | Distinct points |
|---|---|---:|---:|
| xy, x+y | F5 | 2 | 1 |
| xy, x+y−1 | F5 | 2 | 2 |
| x, y | F5 | 1 | 1 |
| x²+x, y²+y, xy+x+y | F2 | 1 | 1 |

Expectations come from the compiled Lean bivariate quotient proof and the
earlier DegreeControl/SolvingDegreeBound proof, plus elementary point
enumeration. Native analysis verifies every returned S-polynomial reduces
to zero, every input reduces to zero, normalized bases equal the registered
ones, finite staircase size, and exhaustive base-field point counts.
For the bivariate family Lean proves equality of the two generated ideals,
so containment alone is not used as an ideal-equality certificate.
The expected basis for the last two inputs is {x,y}. Lean's prior degree
control uses its explicit degree-compatible order; solver selected-pair
degrees are exploratory logs, not a certificate of minimal solving degree.
[pcc:unverified]

Read parametrization quotient dimension and eliminant degree separately.
Expect quotient dimensions2/2/1/1 and eliminant degrees1/2/1/1 on these
fixtures. A discrepancy, unsupported output, nonzero exit or timeout is
retained and fails this fixture check; it is not automatically a solver bug
or a refutation of Diem. The checker intentionally supports only the
registered small fields/exponents and documented output shape.
[pcc:unverified]

Held-out development preflight used only xy,x+y−2 over F5. Successful
calls took0.012s (GB) and0.217s (parametrization), on this unisolated host;
these clocks budget the run and support no timing claim. Calibration
validated the parser on its actual output and detected a deliberately
wrong constant and a zero basis polynomial. The c=2 outputs were seen
before freezing; no confirmatory fixture had been run. The analysis is
unblinded because its task is exact equality to proved known answers.
[pcc:unverified]

Default genericity handling can reorder/add variables. Pinned source
msolve.c:718 uses srand(time(0)); its seed is not exposed by this CLI.
Each raw call records start/end epoch seconds, preserving the containing
seed interval. Reproducibility is of verified invariant counts, not bytes
or a controlled distribution of random linear forms. No independent
machine or signed validation is claimed. The four fixed fixtures are not
a statistical population, so A/A variance, null populations and power
calculations are inapplicable. The repeated calls check exact consistency.
[pcc:unverified]

Use the approved local checkpoint exception and zero paid spend. Each call
is a bounded independent checkpoint; preserve files once created and resume
only as a new named run, with no restored clocks. No aggregate performance
or benchmark record is created. Raw stdout contains upstream timing fields
which are excluded from analysis. Primary metrics are exact basis/count
checks. All raw files and code hashes accompany the export/report.
[pcc:unverified]

Sources checked (KB): KB unavailable; use the primary/source fallback.
Diem2011 proposition's isolated-point/multiplicity context is in the prior
study's source review; this length-two control does not prove its general
n!dⁿ bound. msolve pinned source main.c documents the two modes;
msolve.c:123–181 prints qdim separately from its univariate parameter.
The published nonradical_shape-31 fixture has qdim11 and eliminant degree8,
so conflating these fields is already contraindicated by upstream data.
See provenance.json and ../theorem_audit_20261108/SOURCE_AUDIT.md for the
selection/simplify audit. No Lean proof of the C implementation is asserted.
[pcc:unverified]
