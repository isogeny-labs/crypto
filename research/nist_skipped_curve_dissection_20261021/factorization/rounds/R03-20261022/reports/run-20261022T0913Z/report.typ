#set page(paper: "a4", margin: 17mm, footer: align(right)[#context counter(page).display()])
#set text(font: "DejaVu Sans", size: 8.5pt)
#set par(leading: 0.5em)
#set heading(numbering: none)
#set table(stroke: 0.3pt + luma(80%), inset: 4pt)
#heading(level: 1)[#text("R03: a polynomial restore failure and its correction")]

#text("Run nist-factor-R03-20261022T0913Z, started 2026-10-22 09:13:55 UTC. The user requested continued factorization. One P-256 native NFS attempt was launched from R02's per-thread checkpoint under the separately frozen ")#link("https://github.com/aburan28/crypto/blob/codex/nist-skipped-curve-jobs/research/nist_skipped_curve_dissection_20261021/factorization/rounds/R03-20261022/PROTOCOL.md")[#text("protocol")]#text(", native limit 1800 seconds, outer 1860 seconds plus 30 seconds kill grace, seed 20261022 and two threads. Pinned YAFU: c7d90283972820f99b3b4920bfc1eff05c1f6342; PARI/GP 2.17.2.")

#image("checkpoint-flow.svg", width: 100%)

#text("Only one fixed target was attempted; an infrastructure failure prevented new coverage from being established. The explanatory diagram replaces a comparative runtime graph. The 242 count is recorded by the native exact validator, not a count of tested coefficient ranges or new independent polynomials found in R03. No sampling interval or speed claim applies.")

#heading(level: 2)[#text("What actually happened")]

#text("The restored files contained nfs.dat.0.p, while YAFU's native restore path requires master nfs.dat.p with an n: header. Its source at factor/nfs/nfs_filemanip.c checks that master file and otherwise selects NFS_STATE_STARTNEW; factor/nfs/nfs_poly.c normally merges a thread's file only after the batch returns. The interrupted R02 batch had not performed that merge. R03 therefore began at the old starting coefficient and repeated the first full candidate. A byte comparison confirms that repeat; this does not establish that every produced record is a duplicate.")

#text("SIGINT was recorded but did not stop the process. SIGTERM then ended it with exit 143; exact timestamps and reasons are in raw manual-{interrupt,terminate}.json. No native factor hint was returned. Independent PARI refinement/replay recovered the unchanged verified partial partition. The 512-bit whole target still has a 444-bit composite residual and largest currently proven prime factor of 33 bits. Eight of nine whole targets remain complete, resolving thirteen of fourteen original fields.")

#heading(level: 2)[#text("Recovery and controls")]

#text("The additive ")#link("https://github.com/aburan28/crypto/blob/codex/nist-skipped-curve-jobs/research/nist_skipped_curve_dissection_20261021/factorization/rounds/R03-20261022/PROTOCOL-CHECKPOINT-RECOVERY.md")[#text("recovery protocol")]#text(" preregistered exact checks before recovery. Native GP validated all 242 complete saved degree-5 candidates from R02: coefficient syntax, nonzero leading term, rational pair, invertibility of Y1, and zero of the algebraic polynomial at -Y0/Y1 modulo the literal input. It wrote the original candidate blocks with the correct input header and no invented timing. The first parser version mishandled the string delimiter; its error and empty proposal are retained. The corrected parser passes.")

#text("A different literal input fails the exact root check. The independent native YAFU recognition diagnostic on a separate copy, bounded to three seconds plus one second grace, prints poly file matches input and last leading coefficient was 660; exit 124 is the diagnostic bound, not a production negative. Any incidental arithmetic is setup work. The portable checkpoint restores with the same input and independently replays the prime certificates and exact whole-target product. Neither recognition nor partial replay proves complete factorization.")

#text("The resume wrapper now refuses unrecognized per-thread-only state. A configurable native -pbatch records the selected batch; R04 separately freezes a 120-coefficient batch to permit earlier durable checkpoint merging. That future search is not counted as R03 coverage.")

#heading(level: 2)[#text("Evidence, limits and remaining work")]

#text("Raw runs.tar.xz preserves both native attempts' inherited history, the intervention logs, original/repeated candidates, parser failure, recovery verdicts, wrong-input rejection and recognition diagnostic. Check its SHA-256 before extracting. The loose master file and derived checkpoint excerpt are named directly by the replay/job and remain loose under the AGENTS.md section 13a exception. The full raw archive is distinct.")

#text("The host has four CPUs and 16 GiB RAM, L0 isolation. Python only renders this document. Existing R02 product proofs and prime certificates still apply to the unchanged partition; no new factor identity is claimed. The remaining ")#link("https://github.com/aburan28/crypto/blob/codex/nist-skipped-curve-jobs/problems/nist-extension-twist-composite-cofactors.md")[#text("factorization obstruction")]#text(" and separate ")#link("https://github.com/aburan28/crypto/blob/codex/nist-skipped-curve-jobs/problems/nist-factorization-raw-archive-publication.md")[#text("bulk-publication access gap")]#text(" remain explicit. Historical generators, upstream trait defects and the historical skip reason are unresolved. No finite failed search proves impossibility or a factor-size lower bound.")

#text("The local diagram and PDF are new; IC scoreboards, timelines, performance figures and seed-study diagrams contain no affected result and are unchanged.")
