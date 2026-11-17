#set page(paper: "us-letter", margin: (x: 0.75in, y: 0.72in))
#set text(font: "Helvetica", size: 10pt)
#set par(leading: 0.7em)
= K600 charged build: solver archive extraction stop

*Source:* 98d8e7da5492  ·  *Status:* setup stopped at solver stage  ·  *Scope:* one local attempt

No-work admission passed. The charged build ran 05:38:08–06:16:52 UTC (2,324 seconds). Rust binaries passed in an uncapped 1,654,022 ms wall-to-receipt-start interval. Solver extraction stopped with exit 2 in an uncapped 304,526 ms interval. Container inspection records no OOM; cleanup and source-clean checks passed.

#image("stage-path.svg", width: 90%)

The pinned CryptoMiniSat archive contains two symlinks. Extraction onto the macOS bind-mounted output emitted permission errors for both paths and left mode-000 zero-byte files. The next source-bound attempt must extract and build on Linux-local storage, then copy the finished solver to shared output. This needs a committed script and plan hash and a fresh admission. The failed attempt is retained without reinterpretation.

Evidence: admission and build receipts, stage JSON, raw stderr, Docker inspection and SHA256SUMS beside this report. This was a build setup stop. It does not supply relation, rank, descent, matched rho or total cold IC cost; no canonical factor-base graph changes.
