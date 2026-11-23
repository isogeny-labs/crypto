#set page(paper: "us-letter", margin: (x: 0.75in, y: 0.72in))
#set text(font: "Helvetica", size: 10pt)
#set par(leading: 0.7em)
= N83 touched-example release validation

*Source:* 8e5ffc56747d  ·  *Status:* five example targets passed  ·  *Scope:* local validation

All five existing example files touched by PR #4295 passed their locked Linux arm64 release tests: factor-base export *27/27*, budget controller *5/5*, Linux build producer *2/2*, native arms *15/15*, native preflight *6/6*. Total: *55 passed, zero failed*. Two other changed example files are deleted on this head and have no runnable target.

#image("example-tests.svg", width: 73%)

The frozen launcher ran from 2026-11-16 03:56:14 to 04:15:08 UTC, 1,134 seconds derived. Cargo reported 10m55s for release compilation. This used the completed full-library run's Cargo cache, so neither interval is a cold-build performance comparison. The source-path diff gate against the 6,440-pass library source a702129de8e5 passed. Child/Docker/launcher exits were zero, Docker OOMKilled was false, source_clean was true, and container removal passed. The container had 12 GiB hard memory, zero swap, four CPUs and 512 PIDs.

The raw example log is 15,020 bytes, SHA-256 709d7915355a68eb357dc841cbd9ea48f2bdd5a2c9b529a665be3c531bac0bd6. The protocol and launcher are in the verification directory; the raw log, receipts, Docker inspect, source commit and machine summary are beside this PDF. A later base merge needs affected validation on its new head. This run does not measure relation yield, rank, descent, answer verification, matched rho or cold one-target IC cost, and it does not select a factor base.
