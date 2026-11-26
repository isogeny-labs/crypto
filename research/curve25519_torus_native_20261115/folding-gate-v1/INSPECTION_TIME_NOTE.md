# Inspection timestamp correction

INSPECTION.json, bound by report statement PCC-20261123-77cb93, contains a
manually entered inspected_at2026-11-23T23:03:00Z. That value was entered
before the clock returned2026-11-23T23:02:10Z, so it is NOT an observed
inspection timestamp. Both final-page image views had already occurred before
that clock read. Exact inspection time is unknown; retain only this upper
bound. The two-page inspection findings and artifact hashes are unchanged.
The original inspection/claim bytes remain frozen. This additive note corrects
the producer's metadata error without rewriting the old record.
[pcc:unverified]
