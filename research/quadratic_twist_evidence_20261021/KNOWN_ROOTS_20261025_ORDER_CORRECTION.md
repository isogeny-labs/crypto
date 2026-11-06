# Preserve factor-base column order, sort only query roots

Pre-rerun correction, 2026-10-25. The first native execution at3ad11f812
rejected `distinct ascending roots` before writing a case or running the auditor.
The transported controls legitimately keep factor-base column order from the
source, so their x-coordinate vector need not be ascending. The original
protocol incorrectly required that vector to be sorted. Preserve its order:
changing it could change row coefficients/logs and violate the comparison.

The candidate now constructs a separate sorted copy for query enumeration and
its root declaration. Entry checks still validate each original base coordinate
(two checks); one adjacent strict-order check validates the sorted list and
rejects duplicates. Expanded polynomial binding is symmetric in the two roots.
Both membership polynomials are evaluated at each sorted root (four checks).
Sort requests/traversal remain explicitly unpriced, as before. This is a
specified transformation of valid setup input, not repair of an untrusted root
claim: the independent audit requires the recorded list exactly equal to the
sorted source coordinates and still rejects omissions/extras.

Only the query enumeration uses sorted roots, matching the original field scan.
Factor-base points, column indices, logs, sign ordering, witness/degree and
reference records stay unchanged. All workload, counters, controls, budgets and
acceptance conditions remain. The raw failed source at3ad11f812, build logs,
empty output directory and exit1 are retained; no result was accepted.
Commit this correction and the compiled implementations before the rerun.
