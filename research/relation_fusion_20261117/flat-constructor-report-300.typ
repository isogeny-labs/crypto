#set page(paper: "a4", margin: 1.5cm)
#set text(font: "DejaVu Sans", size: 10pt)
#set par(leading: 0.5em, spacing: 0.75em)
#heading(level: 1, "Packed flat maps: exact-output qualification")
*No comparative speed result. Original 15× goal remains open.*

The opt-in sorted-vector backend shares one packed constructor body and all six unchanged algebraic kernels with the retained tree backend. The original packed assertions remain. Lookup uses binary search; insertion/removal shifts entries and may regress as maps grow. Stable in-place coalescing preserves duplicate-value/key contracts. No unsafe code, cached answer or checker arithmetic constructs results.
#parbreak()
#image("visual-300/flat-layout.svg", width: 100%)
#parbreak()
#table(columns: (1.2fr, 2.1fr), inset: 5pt,
[*Qualification*], [*Recorded outcome*],
[Producer / counter suites], [74 / 75 passed],
[Separate checker], [14 passed],
[Direct matrix 277], [36 cases; 108 full replays; legacy/tree/flat exact wire, metadata, basis and stats equality],
[Polynomial mutations], [126 rejected],
[Real worker 281], [72 calls; 36 launcher identities; four negative controls],
[Saved audit 297], [180 full replays; 126 stored mutations; two receipt saboteurs],
[Copied-data 298 / 299b], [Missing mutation and altered control summary both reject, exit 1],
)
Unsigned producer data. Audits do not independently re-execute OS limits or attest native counters, clocks or execution. Raw worker diagnostic clocks remain unanalysed. [pcc:unverified]

Both primes 2147483647 / 9007199254740881, n3/4/6, seed7 LAST-star and three kernels stay fixed. Complete original membership, input containment, reduced monicity and S-pair obligations remain. Identical certificates establish no smaller proof or lower regularity. No F4 solve; observed solving degree stays null.

*Actual-root gate open:* new opt-in forms and the unchanged-assertion native matrix are prepared, but attempts 279/284/286 execute zero cases because of upstream source corruption. Reviewed BHEC/EFD and six valid-parent support restorations retain APIs and test names; structural inventories do not prove semantic equivalence. Further source unions and actual compilation/runtime checks remain. No module/tests were disabled. [pcc:unverified]

*Before any speed claim:* preserve strongest old/current controls with equivalent launchers, full boundary, source/tool/input pins, repeats, A/A and all six original thresholds. Canonical protocol/export access remains pending: required files are encrypted and no KEY binding is configured. No gate is waived.

*Provenance:* 274 omitted temporarily off-disk Cargo paths; unchanged pins are supplemented by 278. Earlier 269/270 contexts, build 295, every failure and first 299 shell receipt remain; direct 299b captures the expected rejection. Sources and full raw artifacts accompany the archive. No signed execution verdict or newly admitted performance fact. LocalCPU0 / one thread / zero cloud; checkpoint/S3 waiver persists.
#link("https://github.com/isogeny-labs/crypto/pull/4989")[Continuation PR 4989] · FLAT_CONSTRUCTOR_REPORT_300.md
