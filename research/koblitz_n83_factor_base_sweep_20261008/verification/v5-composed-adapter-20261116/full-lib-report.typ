#set page(paper: "a4", margin: 18mm)
#set text(font: "Helvetica", size: 10pt)
#set par(justify: true)

= N83 V5 release-library validation stopped at disk floor

The full-fixture macOS ARM64 checkout at
`a8821d64550526c3151eca205a9751ca4d8aa8a5` has the same
library-affecting files as merged source
`a91b8b66d0f1ffc175fdce984d1059144ce232f6`. The V5 branch leaves those
files unchanged. This is equivalent-source validation, not an exact V5-head
test.

The frozen command was `cargo test --release --lib --locked --offline -j 1`
under Rust 1.99.1 and a 7,200-second process cap. It ran from 2026-11-16
13:16:30 to 13:36:01 UTC and exited 143 when the boot-space watcher recorded
1,947,868 KiB free, below its 2,097,152 KiB floor. The 19m31s endpoint
interval is validation overhead, not an IC timing.

#image("full-lib-flow.svg", width: 100%)

The test binary announced 6,565 tests. Before termination the log recorded
2,462 `ok` and 46 `ignored` terminal lines, zero `FAILED` lines, and no
`test result:` summary. The arithmetic remainder is 4,057 tests without
terminal lines. These are incomplete observations, not a suite pass or a
zero-failure verdict. The earlier completed macOS merged-source run had
6,413 passed, 17 failed, and 135 ignored; it remains separate and failed.

The raw `full-lib.log` is 259,693 bytes, SHA-256
`66e7ffcb926ba1310749e459a241694e576fb2e20acb0eec3ce999ea1e1e0106`.
The disk series has SHA-256
`d1dbed6917e0c28981a4ffb570c5f4b6e32e76d963d222c7cc5fbfd149af519b`.
Its minimum boot and SSD free space were 1,947,868 and 3,153,416 KiB.
The cause of the boot-space drop on the shared host is unknown. The required
full-fixture Linux ARM64 gate remains unrun after two AWS account-level
admission blocks. No factor-base or IC total-runtime result follows, and
canonical IC graphs were left unchanged.
