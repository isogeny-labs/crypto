#set page(paper: "a4", margin: 17mm, footer: align(right)[#context counter(page).display()])
#set text(font: "DejaVu Sans", size: 8.5pt)
#set par(leading: 0.5em)
#set heading(numbering: none)
#set table(stroke: 0.3pt + luma(80%), inset: 4pt)
#heading(level: 1)[#text("R06 completion: corrected stop, unchanged factorization")]

#text("R06 stopped at 2026-10-23 02:10:32 UTC with native exit 0 after its configured 1800-second allocation. Both workers printed the corrected timeout value; native elapsed output was 1827.6341 seconds as outstanding work ended. No outer timeout signal was logged. No proper factor was returned. This is a clean bounded execution, not completed factorization.")

#image("relations.svg", width: 100%)

#text("The graph shows native counters and UTC log timestamps for one allocation, with no replicate interval, throughput comparison or completion estimate. Its ")#link("https://github.com/aburan28/crypto/blob/f3feb82d368b202f834651a3117317b89f2ace6c/research/nist_skipped_curve_dissection_20261021/factorization/rounds/R06-20261023/reports/completion-20261025T0456Z/relations.json")[#text("data")]#text(" exclude inherited relations. The final counter is 1,427,642 new raw relations; the final main-file count is 4,592,399, including 3,164,757 inherited lines. These are raw counts, not a rank claim.")

#heading(level: 2)[#text("Input, execution and verification")]

#text("The exact target is skipped_256_876-twist2, the imported P-256 block-876 512-bit degree-2 extension-twist order. Its still-composite residual has 444 bits. The certified partition remains 1657 times 45294121 times 5333949541 times that same residual. The ")#link("https://github.com/aburan28/crypto/blob/f3feb82d368b202f834651a3117317b89f2ace6c/research/nist_skipped_curve_dissection_20261021/factorization/rounds/R06-20261023/PROTOCOL.md")[#text("protocol")]#text(" binds the literal and predecessor archive checksum. Runtime source was copied and blob-checked against b08dc783f5ba6c3ef821218ff772fe16cc28cee1.")

#text("Patched YAFU and unchanged native lattice sievers used two threads, seed 20261022, -gnfs -xover 81 -pbatch 120, native 1800 seconds, outer 1860 seconds plus 30-second kill grace, 4 GiB virtual memory per process and 1 GiB per output file. Host allocation was four CPUs and 16 GiB RAM, L0 shared host. The actual native command and source/executable hashes are in the full raw archive. The original ")#link("https://github.com/aburan28/crypto/blob/f3feb82d368b202f834651a3117317b89f2ace6c/research/nist_skipped_curve_dissection_20261021/factorization/rounds/R06-20261023/reports/run-20261023T0139Z/REPORT.md")[#text("dated interim report")]#text(" and observation remain unchanged.")

#text("The separate inherited 32,684-line worker output was quarantined without editing; native range selection recomputed the missing gap. Post-run refinement and a separate prime/product replay both pass. All three proven prime certificates are retained. Fresh native checks verify the exact input and common root in both selected polynomial formats; the saved master still has 329 validated records. Existing Lean partition products remain applicable; primality is established separately by PARI certificates. No new algebraic identity or stronger factorization claim is introduced.")

#text("The ")#link("https://github.com/aburan28/crypto/blob/f3feb82d368b202f834651a3117317b89f2ace6c/research/nist_skipped_curve_dissection_20261021/factorization/rounds/R06-20261023/archive-manifest.json")[#text("archive manifest")]#text(" and ")#link("https://github.com/aburan28/crypto/blob/f3feb82d368b202f834651a3117317b89f2ace6c/research/nist_skipped_curve_dissection_20261021/factorization/rounds/R06-20261023/archive-validation.json")[#text("validation")]#text(" identify the complete raw bytes, fresh restoration, checksum checks and canonical repack. Source snapshots, commands, prior history, state, generated certificates, stdout/stderr and the quarantined worker are preserved. Bulk publication still needs the required artifact-store setup; no remote upload is claimed.")

#heading(level: 2)[#text("Subsequent check and remaining requirement")]

#text("The retained October 25 FactorDB GET response returns C and the exact residual as its only factor. It supplies no new divisor. Its original headers and timestamps are retained beside the response. This is a dated database observation, not a priority claim.")

#text("R07 separately filters an independent copy of these relations. Native duplicate removal retains 4,518,304 unique relations including generated free relations; singleton removal leaves 92, and no matrix is generated. That diagnostic preserves the original R06 source and is reported separately.")

#text("Eight of nine timeout integers and thirteen of fourteen fields remain resolved. A complete prime-certified split of the remaining 444-bit cofactor is still required. The merged PR #2379 delivered the earlier evidence and timeout correction; this completion record is its follow-up. A clean stop and more raw relations establish neither a security result nor a historical skip explanation. Existing mathematical and IC performance graphs have no changed conclusion and remain unchanged; this new operational graph records the completed allocation.")
