#set page(paper: "a4", margin: 16mm, numbering: "1 / 1")
#set text(font: "DejaVu Sans", size: 10pt)
#set par(justify: true, leading: 0.58em)
#set heading(numbering: none)
#align(center)[#text(size: 17pt, weight: "bold")[Restart custody gap]#linebreak()#text(size: 9pt)[Bounded owned-path observations: 23 November 2026 · PCC-20261123-5e3cc7, unsigned]]
#text("The completed native cost16B data checkpoint survives in GitHub, and all 128 study blobs matched it before new report/resume edits. The previously recorded full raw stdout and whole private archive are absent at their recorded owned tmpfs paths. This is a custody loss at the inspected locations, not a fresh execution or a global irrecoverability finding. [pcc:unverified]")

#figure(image("custody.svg", width: 100%), caption: [Different custody states of the historical packet and its public records; the missing branch remains open. [pcc:unverified]])

#text("The original /dev/shm/ct812-first-cost16B tree was deliberately reclaimed after whole-packet comparison on 17 November. Its complete archive backing was /run/lock/chained-transfers-ct669/entire-native16B-all-source-deps-ELFs-and-raw-streams.tar.xz, linked from /dev/net/chained-transfers-owned-storage-20261117. Both recorded archive paths are now absent. The old owned Gitdir and index are also absent; the original worktree's .git remains a dangling link. [pcc:unverified]")

#text("The public record still pins the 161,784-byte stdout7d528378, the 66-member archiveeefd5d1a, exact public sourcee7efdbf1, all recorded guard manifests and 15 native-emitted aggregate lines. Those metadata do not replace the full raw bytes. No original timestamp, PID or elapsed time has been reconstructed, and no science call was repeated. [pcc:unverified]")

#text("New work uses an isolated partial clone and the untouched current-main PCC prefix. The persistent /workspace overlay reports zero available bytes; new state is therefore checkpointed promptly through GitHub. No protected original, shared Git objects/index, keyring, verifier, budget, provider identity or vault was changed. [pcc:unverified]")

#text(size: 8.5pt)[#link("https://github.com/isogeny-labs/crypto/blob/918120b5e21f6338c93345a6b49faf9a0faa93a1/research/chained_primefield_cost_20261117/restart-custody-20261123/OBSERVED_RESTART_CUSTODY_GAP.json")[Bounded path observations and exact hashes]]
