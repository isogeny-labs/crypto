#set page(paper: "a4", margin: 17mm)
#set text(size: 9pt)
#set par(justify: true)
#show raw: set text(size: 7.6pt)
= #text("Saved-tape pruning audit: native software gate passes")

#text("The required stronger square obstruction and independent integer replay passed before this owner compiled its native runner. This report records the software gate only; the 21 scientific case receipts are separate.")


#image("../../figures/pruning-software-gate.png", width: 100%)

== #text("Actual execution and retained evidence")

#text("Managed owner: twist-joint-fp-pruning-audit-v1.service, started at 06:07 UTC. Driver: run-fp-pruning-audit.sh. Its metadata records source commit 64276f78721878ad637f52cc39225a7a19de2b0b and actual source hashes; the hashes retain the compiled source state independently of the commit label. Compilation, native tests and runner build all exit zero.")


#text("The native suite reports 77 passed, zero failed and zero ignored. Its added actual-point control verifies that the finite-residual rule must retain infinity, where a CM triangle can occur with distinct factors. This is a meaningful exception control, not a timing comparison.")


#text("The immutable runner SHA-256 is f610684f4ce5e33c10e80fe087a06c0c647e3ac906b883542a8c04eaa686d6d7. The build/copy and checksum were inspected before another native module was registered. Later edits cannot change this runner's bytes.")


#text("Evidence: runs/fp-pruning-audit-v1/metadata/ contains the executed script, commands, source hashes, test stdout, exit receipts and runner checksum. The compiled suite runs serially with one thread, an 8 GiB address-space limit and a 1,200-second watchdog. Heavy phases use the shared busy lock; compilation is separate. Queue delay and the test log's elapsed duration are not admitted solver timings. No isolation tier or speedup is claimed.")


== #text("Scientific boundary")

#text("The subsequent audit reuses all 9,658 saved historical residuals and the existing phi, doubling and pure group-reference sets. It verifies every saved physical presentation, distinct tuple, projective row, independent RREF, first full-rank query and recovered point. It does not repeat a polynomial collector or BSGS oracle. A software pass establishes none of the final rejection counts, rank preservation or complete-cost gain. No filter has been deployed and no benchmark or leaderboard row changes.")
