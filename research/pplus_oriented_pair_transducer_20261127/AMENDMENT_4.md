# Amendment 4: remove unavailable timing utility

Date frozen: 2026-11-27 UTC, after failed attempt 1 and before any candidate
census row existed.

## Observed wrapper failure

The pushed-Git checkpoint, all 13 source hashes, and the locked release build
passed. The runner then exited 127 at its first execution line because this
executor has no `/usr/bin/time`. The retained `candidate.txt` is zero bytes;
`candidate.stderr` contains only the missing-utility shell error. Therefore no
candidate depth row, query classification, verifier output, analysis result, or
promotion information was observed.

Failed attempt 1 is retained byte-for-byte with SHA-256 under
`failed-attempt-1/`.

## Prospective runner-only fix

Replace `/usr/bin/time` with a shell wrapper around the unchanged binaries. It
records elapsed nanoseconds from `date +%s%N`, the exact child exit status, and
`max_rss_kib=unavailable`. Peak RSS was only a diagnostic and is not used by
any correctness, growth, cost, or promotion rule.

One explicit retry is permitted only after this amendment, the repaired runner,
an updated source manifest, and the failed-attempt receipt are committed and
pushed with local/remote head equality. The retry still refuses any existing
successful `evidence/` directory.

The candidate, verifier, analyzer, curve, targets, operation counters, cost
panels, and stop rule are unchanged. [pcc:unverified]
