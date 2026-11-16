#set page(paper: "us-letter", margin: 0.7in)
#set text(font: "Helvetica", size: 10pt)
= N83 K600 source-locked zero-trial preflight

*Observed gate:* 2026-11-16 08:23:50–08:23:55 UTC; wrapper exit 0, native status `PREFLIGHT_ONLY`, worker exit 0. The native outer wall was 4,040.215208 ms. This one run used no relation trials; it is a capacity and input-identity gate, not a search result.

#image("diagram.svg", width: 100%)

H2 `a30e67492f82` is a clean, one-lock child of C `ba620214684e`. The fresh 3,025,798-byte S3 object matched SHA-256 `3e85c9e4…`. The container used one CPU, a 4 GiB memory and no-swap cap, and no network; observed cgroup peak was 105,897,984 bytes. All seven source hashes, two executable hashes, config/summary hashes, limits, terminal status and cleanup passed the independent receipt cross-check in `audit.txt`.

The producer reported 0 trials, 0 relations, 0 SAT calls, 0 matrix rows and rank 0. Its zero-row rank check is preflight overhead only. The earlier stopped attempts and builds remain charged setup. Full cold one-target runtime, a winning factor base and rho ratios remain unset. The source-linked report and raw receipts are in `REPORT.md` and adjacent files.
