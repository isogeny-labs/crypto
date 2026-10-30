#set document(date: none)
#set page(paper: "a4", margin: 15mm, numbering: "1")
#set text(font: "DejaVu Sans", size: 9pt)
#set par(justify: false)
#heading(level: 1)[Complete-root instruction profiles]
#text("Original n23 planted seed230, F4/F5 complete-root calls. Five interleaved baseline/prepared/quadratic-tail profiles per arm; every root set verifies equal. Local Xeon6975P-C, Valgrind3.19, merged threads, one worker, simulated AVX2. This is instruction-count evidence and supplies no native time, cycles or IPC.")


#text("The first-tail implementation reduces median call Ir from4.525/4.556 billion to410.240/413.511 million for F4/F5. Prepared-only medians are1.505/1.523 billion. All five rounds per arm complete without failures. These are different implementations' complete instruction work; elimination word counts alone omit the newly explicit Gray assignments.")


#text("The hot-function annotation identifies remaining matrix reduction, column lookup and direct packing work. LOOKUP_PROTOCOL.md freezes a subsequent dispatch refinement before its profiles. Neither profile batch reprices the dedicated native or ecbench clocks. Sources: profiles-summary.json, runs/profiles raw data, binary/source manifests and PROFILE_PROTOCOL.md.")


#image("../../complete-root-instructions.svg", width: 100%)
#pagebreak(weak: true)
#heading(level: 2)[Recorded complete-call medians]
#set text(size: 7.5pt)
#table(columns: (1fr, 1fr, 1fr, 1fr, 1fr), inset: 3pt, table.header(text("Arm"), text("Baseline Ir"), text("Prepared Ir"), text("First-​tail Ir"), text("Five pairs")), text("solve-​f4"), text("4525036969.0"), text("1505322837.0"), text("410239680.0"), text("True"), text("solve-​f5"), text("4555545353.0"), text("1523302717.0"), text("413511483.0"), text("True"))
#set text(size: 9pt)
