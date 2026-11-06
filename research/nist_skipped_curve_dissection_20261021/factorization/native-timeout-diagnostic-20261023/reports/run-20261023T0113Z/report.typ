#set page(paper: "a4", margin: 17mm, footer: align(right)[#context counter(page).display()])
#set text(font: "DejaVu Sans", size: 8.5pt)
#set par(leading: 0.5em)
#set heading(numbering: none)
#set table(stroke: 0.3pt + luma(80%), inset: 4pt)
#heading(level: 1)[#text("Native timeout diagnostic: original 139, patched 0")]

#text("Two fresh bounded full-tool controls reproduce a timeout formatting defect in pinned YAFU c7d90283972820f99b3b4920bfc1eff05c1f6342. The original exits 139; the patched executable exits 0 and logs its integer timeout. Both use the exact R05 444-bit residual and selected polynomial. This diagnoses the failure mechanism without claiming a production backtrace or a new factor.")

#image("timeout-path.svg", width: 100%)

#heading(level: 2)[#text("Recorded controls")]

#text(size: 8pt)[#table(columns: 2,
[#text("Control")],
[#text("Observed result")],
[#text("Original sieving printf, format-as-error compile")],
[#text("rejected: uint32_t passed to string conversion")],
[#text("Original second timeout printf, format-as-error compile")],
[#text("rejected: uint32_t passed to floating conversion")],
[#text("Original extracted sieving statement, one run")],
[#text("exit 139")],
[#text("Both corrected extracted statements")],
[#text("compile and run successfully")],
[#text("Original full native NFS, one bounded control")],
[#text("exit 139")],
[#text("Patched full native NFS, one bounded control")],
[#text("exit 0; integer timeout message logged")],
[#text("Checked resume, prepare only")],
[#text("installed validated binary; native arithmetic replay passes")],
[#text("Wrong binary or active native lock")],
[#text("both rejected")])]

#text("The raw ")#link("https://github.com/aburan28/crypto/blob/codex/nist-skipped-curve-jobs/research/nist_skipped_curve_dissection_20261021/factorization/native-timeout-diagnostic-20261023/native-control-summary.json")[#text("native summary")]#text(" binds the actual executable hashes. The original matches the production R05 binary. The ")#link("https://github.com/aburan28/crypto/blob/codex/nist-skipped-curve-jobs/research/nist_skipped_curve_dissection_20261021/factorization/native-timeout-diagnostic-20261023/timeout-printf.patch")[#text("patch")]#text(" changes two timeout messages only: the sieving message's %1.4sec parses as a string conversion %1.4s, and the other message's %1.4f expects a double. Both incorrectly receive a uint32_t. Corrected messages use %u with an unsigned cast. Historical build warnings already identify both defects.")

#heading(level: 2)[#text("Inputs, bounds and evidence")]

#text("The ")#link("https://github.com/aburan28/crypto/blob/codex/nist-skipped-curve-jobs/research/nist_skipped_curve_dissection_20261021/factorization/native-timeout-diagnostic-20261023/PROTOCOL.md")[#text("protocol")]#text(" limits extracted controls to five seconds each with core dumps disabled. The ")#link("https://github.com/aburan28/crypto/blob/codex/nist-skipped-curve-jobs/research/nist_skipped_curve_dissection_20261021/factorization/native-timeout-diagnostic-20261023/NATIVE-VALIDATION-PROTOCOL.md")[#text("separate full-tool protocol")]#text(" allows two fresh native controls, two threads, seed 20261022, native limit one second, Q range 2935000 to 2935500, sieving phase only, outer bound 30 seconds plus five-second kill grace, 4 GiB address space and 64 MiB per output file. The compatible local host has four CPUs and 16 GiB RAM, L0. There is one control per native arm, no replicate interval and no timing comparison; the diagram shows recorded outcomes, not performance.")

#text("The original R05 relations and checkpoint were never imported or modified. Each control uses its own selected job, parameters, output and native executable. Both complete within the outer bound. The patched tool's final C134 label repeats the unchanged input and is not a proper factor. The first separate build attempt fails to autodetect GMP's multiarch headers; the second explicitly sets the include path and succeeds. Raw build errors, commands, compiler versions, sources/hashes, diagnostics, output/state and wrapper controls are retained in the sibling raw archive.")

#text("The ")#link("https://github.com/aburan28/crypto/blob/codex/nist-skipped-curve-jobs/research/nist_skipped_curve_dissection_20261021/factorization/native-timeout-diagnostic-20261023/resume_with_checked_tool.sh")[#text("checked wrapper")]#text(" installs only a binary matching a passing control summary, under both existing locks, with the previous executable retained. Its copied executable is rehashed before installation. Prepare-only replay verifies the unchanged original target and prime certificates. The R04 compact checkpoint is used only for this non-search preparation control; the actual later card binds the newer R05 relation checkpoint after required artifact-store publication.")

#heading(level: 2)[#text("Scope and remaining work")]

#text("The formatting defect and its corrected bounded native behavior are verified. The exact R05 production instruction pointer remains unobserved. No longer production allocation was run. Eight of nine integers remain fully factored, with the 444-bit residual still composite. This diagnostic adds no algebraic identity or Lean theorem; existing partition proofs and native prime certificates remain applicable. Existing mathematical and performance graphs have no changed result and remain unchanged.")

#text("The backlog card now requires the checked tool and the R05 relation archive, but remains draft and unqueued pending actual store publication, a merged source pin and selected lab. No paid resource, task or lease was created. The next production run must preserve all relation state and independently certify any proper factor it returns.")
