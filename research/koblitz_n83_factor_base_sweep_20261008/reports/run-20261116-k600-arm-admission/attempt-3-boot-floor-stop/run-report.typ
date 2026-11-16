#set page(paper: "us-letter", margin: 0.7in)
#set text(font: "Helvetica", size: 10pt)
= N83 K600 arm admission attempt 3: disk-floor stop

The GNU-pinned sparse checkout and offline Cargo metadata gate passed. The Rust 1.99.1 H2 host-native arm build ran from 2026-11-16 08:54:32 to 09:00:18 UTC: 346 seconds by endpoint seconds. The disk watcher sent TERM when boot free space was 4,388,412 KiB, below the frozen 5 GiB floor; the child exited 143. The 2,700-second process cap did not fire.

#image("diagram.svg", width: 100%)

The SSD sample was 24,294,888 KiB free at stop, above its 20 GiB floor. H2 remained clean, and no arm binary, S3 stage, relation search or solver resulted. The partial Cargo target is an external setup artifact, not an admitted executable. This failed interval remains charged. The source-linked report, 24 disk samples, terminal exit, Cargo stderr and sparse/metadata receipts are adjacent to `REPORT.md`.
