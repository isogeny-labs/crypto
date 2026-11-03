# N83 fixed-base first-fall coverage

This directory indexes the currently saved candidate first-fall results against the frozen 624-system V2 panel. It keeps first-fall, last-fall, and actual degree-of-regularity claims in separate fields and preserves the provenance of every source summary.

The index covers 432 replayed systems across `spike27`, `spike28`, `spike30`–`spike33`, and `spike35`–`spike37`. The remaining 192 protocol rows are 144 systems on known completed bases (`spike29`, `spike38`, `spike39`) plus 48 systems for `spike34`, whose factor base has not been enumerated. Across the covered rows, every Koblitz and general-binary system has a degree-2 first-fall certificate; every prime system has lower bound 5 with no exact first-fall degree. These outcomes do not vary by candidate within a family, so they cannot supply a supervised target for finding a base with a lower regularity degree.

Every candidate first-fall row has unknown LFD. A separate 12-case m3 restricted propagation panel has inherited strict lower bound 4, but no unit or exhaustive affine upper certificate; it establishes no exact LFD. Neither panel measures affine PDP degree of regularity.

Use `python3 build_coverage.py --check` to reproduce the counts and verify the source summaries and 12-case LFD panel against the stored aggregate. The spike35–spike37 archive remains in S3 and is identified in `coverage.json` and the linked custody record; it excludes the large system payloads, which remain at separately indexed S3 locations.
