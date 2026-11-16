#set page(paper: "us-letter", margin: 0.7in)
#set text(font: "Helvetica", size: 10pt)
= K600 supervisor build result and admission state

The host-native supervisor build passed with one measured outer interval of 1,531 seconds and one Cargo-reported build duration of 1,530 seconds. No spread or paired comparison was measured. The executable is Mach-O arm64, 1,284,960 bytes, SHA-256 `3ea86decaf897e800aa3f78c5b028c1505b71fe1637cbb858ad717907f0284c0`.

#image("build-intervals.svg", width: 100%)

The source-byte check against Linux build commit `ba620214684e` passed on Rust/Cargo paths. The 51 disk samples stayed above the 20 GiB stop floor. This result admits the next no-work `prepare` step only after its executable hash is committed. It makes no statement about relation yield, rank, descent, matched rho or cold one-target runtime. Sources and receipts: `REPORT.md`, `build-result.json`, `stderr.log`, `disk-watch.tsv`.
