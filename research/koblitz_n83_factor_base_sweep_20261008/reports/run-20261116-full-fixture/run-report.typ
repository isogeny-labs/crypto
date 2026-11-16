#set page(paper: "us-letter", margin: (x: 0.75in, y: 0.72in))
#set text(font: "Helvetica", size: 10pt)
#set par(leading: 0.7em)
= N83 full-fixture Linux release-library validation

*Run:* full-fixture attempt 2  ·  *Source:* a702129de8e5  ·  *Status:* local validation passed

The pinned Linux arm64 cargo test --release --lib --locked -j 1 completed from 2026-11-16 02:49:24 to 03:43:21 UTC, a derived whole interval of 3,237 seconds. The main suite reported *6,440 passed, zero failed, 138 ignored* in 1,244.74 test seconds. Two nested filtered checks each passed one test and are excluded from the main tally. Child, Docker and launcher exits were zero; Docker OOMKilled was false, source_clean was true, and container removal passed.

#image("fixture-validation.svg", width: 100%)

The earlier sparse-checkout run reported 6,395 passed, 45 failed and 138 ignored. Sort-unique names from the two SHA-verified raw logs each number 6,578, with zero symmetric difference; all 45 exact failed names now pass. A full Git checkout supplied the tracked files, and the archived k20-s0 records file was restored from the astronomy artifact store with SHA-256 8970667e2f18587eeb08d033123dab029b94042ac5c07c870b08988f1ee5b738 and 2,293,097 bytes. The intervening Git diff changes reports, artifact pointers and validation protocol files, with no library, test, example, Cargo, GPU or tool source changes. Input materialization cleared this validation gate; individual earlier failure causes were not isolated one by one.

The first admission attempt stopped before a container started because its full-checkout path was outside Colima's mounts; its raw receipts are preserved. Attempt 2 moved the unchanged checkout under the existing read-only input mount and froze a new launcher. The compressed attempt-2 log is 116,853 bytes, SHA-256 448f858797ba7f9d7351d9b7efd1253aa01990e48452180effba6b5e18ad13dc; the expanded log is 615,832 bytes, SHA-256 f47c59b201ef0b7ceea55453b8bf78a82cda6045566e171c8b11747d0275344e. Protocol, amendment, raw log, receipts, Docker inspect and machine summary are beside this PDF in the study directory.

This validates a local source snapshot, not a later conflict-resolved PR head. It is not a factor-base or total IC runtime result. The K600 charged build remains stopped at its 1,200-second Rust-stage cap; relation, rank, descent, answer verification, matched rho and cold one-target cost are still pending. No canonical IC graph changes.
