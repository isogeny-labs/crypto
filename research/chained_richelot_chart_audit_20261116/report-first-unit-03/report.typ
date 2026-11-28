#set page(paper: "a4", margin: 15mm)
#set text(font: "DejaVu Sans", size: 10.3pt)
#set par(justify: true)
#set heading(numbering: none)
#align(center)[#text(size: 17pt, weight: "bold")[Richelot chart trace audit]#linebreak()#text(size: 10pt)[Separate implementation · first bounded unit · 16 November 2026]]
#block(fill: rgb("edf2f8"), inset: 8pt)[All 5668 ordered events match. All six semantic mutations reject; elapsed-only positive accepts. Claim f88c0c is unsigned. [pcc:unverified]]
== Exact checked population
The original 287462-byte stdout contains 100 inverse, 1224 projective, 1440 forward, 1440 reverse and 1440 sign rows, plus 24 summaries. All 12 source/output pairs have 120 rational points. The separate backend computes 24 nonzero Sylvester determinants. Every recorded event is reconstructed in order from the frozen CSV. [pcc:unverified]
== Complete transcript controls
#set text(size: 8.8pt)
#table(columns:(1.15fr,0.25fr,1.55fr),inset:5pt,
[Input],[Exit],[Check],
[Original whole trace],[0],[5668 ordered events],
[Forward coordinate],[1],[First mismatch at event 204],
[Drop projective],[1],[First mismatch at event 101],
[Duplicate forward],[1],[First mismatch at event 203],
[Sign relation],[1],[First mismatch at event 4217],
[Summary coefficient],[1],[First mismatch at event 442],
[Lost infinity],[1],[First mismatch at event 202],
[Elapsed time only],[0],[Entire audit output identical])
#set text(size:10.3pt)
All generators return 0; every semantic rejection is an actual exit 1, rather than a build, I/O or timeout failure. [pcc:unverified]
== Implementation separation
The auditor imports no producer code. It uses u128 modular arithmetic and signed Euclid, iterative multiplication for the reciprocal polynomial, repeated-power evaluation, square-table point lists and Sylvester determinants. The producer uses a u64 field wrapper, Fermat inverse, binomial expansion, Horner evaluation and polynomial Euclid. Both implementations were written by the same session; this is not an independently authored trusted verifier or a generic field/compiler proof. [pcc:unverified]
== Frozen execution and whole evidence
Public source 9f3c084e/library 791dffb1 precedes compilation and native calls. Offline locked Rust 1.99.1, one compiler worker, release and 120-second caps were used. The qualifier was copied, chmod555 and hash-frozen before calls. All 24 phases match their declared exits, including strict Clippy, format and source/input/qualifier guards. A whole 319488-byte archive retains all 136 files and eight complete transcripts, with every member byte/SHA/mode checked. No producer rerun, benchmark or performance ratio. [pcc:unverified]
== Next mathematical component
Both b signs remain valid curve maps. Carry the charts through balanced divisor classes and Richelot factors, including nonrational support, exceptional/infinity fibres and dual orientation. The original fullLP tasks, admission and matched costs remain queued. Paid STOP, locked vault, original store/CI gates and separately preserved main-ledger fork remain. [pcc:unverified]
