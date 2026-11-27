# Read-only incidence and phase-accounting inspection

Read the unchanged original attempt-v1 matrix. Scan rows into a list of
(row,column,coefficient) occurrences on columns225..428, sort by column, and
require each column exactly once, no new incidence on the six original rows,
and two incidences on each added row. Compare the complete sorted list with
the instrumented collector's emitted certificate. This differs from the
producer's per-column owner scan and uses only small integer/JSON arithmetic.
It does not replace the already executed independent elliptic/map checker.

Reject wrong owner, missing certificate, changed sign and out-of-domain new
coefficient. Check that all nonnegative phase clocks sum to the native inner
clock interval. No new point, residual root, map evaluation or collection call.
Strict C++17 build cap300seconds/1GiB, inspector cap30seconds/1GiB on the same
CPU. Retain exact sources, binary/input hashes, output, exits and process costs.
All resulting records remain producer-unverified pending independent signing.
