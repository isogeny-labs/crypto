#set page(paper: "us-letter", margin: 0.7in)
#set text(font: "Helvetica", size: 10pt)
= N83 K600 host-native supervisor build

*Observed setup:* 2026-11-16 07:44:48–08:10:19 UTC, 1,531 s outer by second-resolution endpoints; Cargo reported 1,530 s. One macOS ARM64 shared-host build, Rust 1.99.1, exit 0, no disk stop. Binary: 1,284,960 bytes, SHA-256 `3ea86decaf897e800aa3f78c5b028c1505b71fe1637cbb858ad717907f0284c0`.

#image("build-intervals.svg", width: 100%)

Raw: `build-result.json`, `stderr.log`, `exit.txt`, `disk-watch.tsv`, `SHA256SUMS`. Source and protocol links are in `REPORT.md`. The values are setup intervals, not a factor-base or one-target result.
