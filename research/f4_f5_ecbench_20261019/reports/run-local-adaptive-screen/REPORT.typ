#set document(date: none)
#set page(paper: "a4", margin: 15mm, numbering: "1")
#set text(font: "DejaVu Sans", size: 9pt)
#set par(justify: false)
#heading(level: 1)[Cheap degree-two refutations: local work screen]
#text("Source frozen-adaptive-screen/koblitz_groebner.rs (pre-version guarded source). Same registered curves, standard bases and public target0/round1 as the full panel; one warmup and one measured round, four arms per curve. All16 executions verify, including8 warmups; all8 measured replays reproduce. Gate passed: True. The native replay-all mode excludes warmups. This corrects earlier prose equating total verified records with measured replays. No L0 elapsed-time or pipeline speedup claim follows.")


#text("The failed all-width degree-three policy spends3.000 billion word XORs on the n23 F4 target; the guarded policy spends62.421 million, against79.418 million baseline. The old native full-pipeline regression remains in its own report; logical work counts here are not native wall time.")


#text("Both original16/22-variable fixtures retain identical complete roots and all checked deterministic counters against source71c4c99d85. The final corrected policy receives its own identity; pre-version screen records keep their sealed bytes and frozen-source replay. Sources: the adjacent Rust-prepared summary, ADAPTIVE_PROTOCOL.md and the raw screen tree.")


#image("../../screen-v0-word-work.svg", width: 100%)
#pagebreak(weak: true)
#heading(level: 2)[Matched target work]
#set text(size: 7.5pt)
#table(columns: (1fr, 1fr, 1fr, 1fr, 1fr, 1fr, 1fr), inset: 3pt, table.header(text("n"), text("Engine"), text("Calls"), text("Baseline XORs"), text("Failed XORs"), text("Adaptive XORs"), text("Queries/answer equal")), text("17"), text("f4"), text("2398"), text("3355654"), text("15190651"), text("2062875"), text("True"), text("17"), text("f5"), text("2398"), text("3350611"), text("15107801"), text("2058669"), text("True"), text("23"), text("f4"), text("135075"), text("79417986"), text("3000248688"), text("62421379"), text("True"), text("23"), text("f5"), text("135075"), text("79343067"), text("2994993068"), text("62338849"), text("True"))
#set text(size: 9pt)
