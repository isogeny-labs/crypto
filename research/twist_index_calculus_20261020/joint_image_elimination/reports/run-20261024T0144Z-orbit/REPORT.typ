#set page(paper: "a4", margin: 17mm)
#set text(size: 9pt)
#set par(justify: true)
#show raw: set text(size: 7.6pt)
= #text("Combined polynomial proof gate v3")

#text("Run lean-gate-v3 starts 2026-10-24 01:44:51 UTC. Its exact theorem source, source SHA-256, toolchain and command remain in runs/lean-gate-v3/metadata/. The one-worker wrapper fixes the prior thread-allocation issue.")


#image("../../figures/joint-mechanism.png", width: 100%)

#text("The build reports INTERNAL PANIC: out of memory under the unchanged 8 GiB address-space bound. The proof gate returns a build failure; no theorem in the combined module is promoted on this receipt. Three independently checked integer/IDC identities still have their separate exact scope; they are not Lean proof receipts.")


#text("This bounded correctness attempt uses the busy reservation, one worker and a 900 s watchdog. The later split coordinate and quartic targets retain the theorem statements and get separate receipts. No performance comparison is admitted; the diagram shows the verification boundary.")
