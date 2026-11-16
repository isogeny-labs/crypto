#set page(paper: "us-letter", margin: 0.7in)
#set text(font: "Helvetica", size: 10pt)
= N83 K600 arm admission attempt 4: second disk-floor stop

The cached H2 host-native arm build ran from 2026-11-16 09:09:41 to 09:11:41 UTC: 120 seconds by second-resolution endpoints. The disk watcher sent TERM when boot free space reached 4,908,536 KiB, below the unchanged 5 GiB floor; the child exited 143. The SSD held 21,476,640 KiB free, above its 20 GiB floor. The 2,700-second process cap did not fire.

#image("diagram.svg", width: 100%)

`TMPDIR` was on the SSD and the prior partial Cargo target was reused as an unsealed cache, but the boot free-space fluctuation still triggered the stop. No completed arm executable, read-only S3 stage, relation search or solver resulted. H2 remained clean. The 120-second interval and earlier failed setup remain charged; raw receipts and the source-linked analysis are adjacent to `REPORT.md`.
