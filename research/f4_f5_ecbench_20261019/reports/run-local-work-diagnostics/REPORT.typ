#set document(date: none)
#set page(paper: "a4", margin: 15mm, numbering: "1")
#set text(font: "DejaVu Sans", size: 9pt)
#set par(justify: false)
#heading(level: 1)[Construction and bounded-tail work diagnostics]
#text("Same n17/n23 generator systems; baseline, prepared layout/top-degree/linear-tail/direct-packing package, and bounded quadratic-tail package. Every original root set verifies equal. Shared-host native wall values are retained raw but not admitted or quoted.")


#text("The n23 complete-root tree uses1022 matrices before,511 with prepared policies and63 with the quadratic tail. The tail also performs4194304 Gray assignments. Those are different work units; fewer matrices alone is not a speed result. Both matrix engines show the same count pattern here.")


#text("These diagnostics motivated the frozen dedicated-host runs; they do not substitute for five paired native timings or full DLP recovery. Sources: diagnostic-*.jsonl, frozen baseline and candidate manifests.")


#image("../../local-matrix-work.svg", width: 100%)
#pagebreak(weak: true)
#heading(level: 2)[Recorded work, including enumeration]
#set text(size: 7.5pt)
#table(columns: (1fr, 1fr, 1fr, 1fr, 1fr, 1fr), inset: 3pt, table.header(text("Arm"), text("n"), text("Policy"), text("Matrices"), text("Word XORs"), text("Gray assignments")), text("solve-​f4"), text("17"), text("baseline"), text("126"), text("2705998"), text("0"), text("solve-​f4"), text("17"), text("prepared"), text("63"), text("754446"), text("0"), text("solve-​f4"), text("17"), text("quadratic-​tail"), text("1"), text("39685"), text("65536"), text("solve-​f4"), text("23"), text("baseline"), text("1022"), text("112863703"), text("0"), text("solve-​f4"), text("23"), text("prepared"), text("511"), text("26680925"), text("0"), text("solve-​f4"), text("23"), text("quadratic-​tail"), text("63"), text("6779923"), text("4194304"), text("solve-​f5"), text("17"), text("baseline"), text("126"), text("2705998"), text("0"), text("solve-​f5"), text("17"), text("prepared"), text("63"), text("754446"), text("0"), text("solve-​f5"), text("17"), text("quadratic-​tail"), text("1"), text("39685"), text("65536"), text("solve-​f5"), text("23"), text("baseline"), text("1022"), text("112863703"), text("0"), text("solve-​f5"), text("23"), text("prepared"), text("511"), text("26680925"), text("0"), text("solve-​f5"), text("23"), text("quadratic-​tail"), text("63"), text("6779923"), text("4194304"))
#set text(size: 9pt)
