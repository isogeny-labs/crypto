# Bounded-memory Boolean Macaulay diagnostic

This directory copies the combined [run/results PDF](report.pdf),
[SVG](passes.svg) and 150 dpi [PNG](passes.png) from the
[source study](../../research/ideas_execution_20261018/bounded_memory/README.md).
The study remains the source of truth for protocols, inputs and raw attempts.

Eight degree-4 passes completed; four small passes agreed exactly with an
independent dense RREF. The n17/ell6/m4 diagnostic processed 31,110 rows with
recorded rank 30,888 and a 1,048,576-byte pivot-payload cache. Large-pass
independent verification remains open. Original n15 reductions recorded
26,932/27,048 KiB peak RSS for first/minimum-weight policies.

Shared four-CPU managed host, campaign L0/Tier 0 diagnostic notation; no host
isolation receipt and no comparative wall-time, speedup, full F4, solving-degree
or end-to-end claim. Count plots cover committed checkpoint history and exclude
rolled-back suffix work; all failed attempts remain in the source study.
