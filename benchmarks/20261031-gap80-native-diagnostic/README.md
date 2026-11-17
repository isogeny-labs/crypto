# Retained gap80 native-helper and partial-tile diagnostics

Documentary browsing copies of the
[study's frozen observations](../../research/prime_conductor_gap80_20261031/README.md).
These are outside the admitted benchmark table: no `VALIDATION.json`, matched
rho comparison, independent timing replay or full-map measurement is claimed.

Historical native-helper medians were 0.469961 → 0.395023 s (degree 4,096)
and 2.453493 → 2.087622 s (degree 16,384), seven unpinned alternating pairs
each. The same partial 80-bit tile was 3.735434 → 3.431381 s, three retained
CPU-0 pairs after the documented exclusions. The host was AMD EPYC 9V74;
these are shared-host observations, and the tile harness used Sage/Python.

The study retains raw receipts, sources, certificates and an unchanged
archive. [Run report](run-report.pdf), [results report](results-report.pdf),
[SVG](historical-timings.svg) and [150-dpi PNG artifact pointer](.artifacts.json) are
copies for browsing; the study is the source of truth. No benchmark-admission
or leaderboard status follows from copying the reports.

Hydrate the PNG from the repository root with
`python3 tools/artifacts/artifacts.py pull benchmarks/20261031-gap80-native-diagnostic`.
