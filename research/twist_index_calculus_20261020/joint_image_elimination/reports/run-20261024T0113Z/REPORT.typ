#set page(paper: "a4", margin: 17mm)
#set text(size: 9pt)
#set par(justify: true)
#show raw: set text(size: 7.6pt)
= #text("Joint verification v6: retained partial result")

#text("Run verification-v6, started 2026-10-24 01:13:33 UTC. Its executed script, commands, source hashes and per-phase exits remain in the raw run metadata.")


#image("../../figures/joint-mechanism.png", width: 100%)

#text("All 68 native tests passed. The independent construction validator passed 297 tasks and 694 system files. Three exact integer identities had zero remainders and their false-statement mutation controls rejected. IDC records are listed in the proof map (../../proof/PROOF_MAP.md).")


#text("The Lean invocation supplied an absolute directory where the repository gate required a relative standalone file; no theorem was verified by that invocation. The later pure geometric export had inconsistent exponent dimensions and panicked in the msolve writer. The finite pure export was written, but the geometric file is not a valid input. Neither mixed variant completed in this run. The outer exit is 101; this is not a completed gate.")


#text("The defect is repaired in a later source revision and a fresh v8 run. The failed outputs remain historical evidence and are not silently reused. This is correctness verification under the busy reservation, with one thread and bounded scientific phases; it admits no performance result.")
