#set page(paper: "a4", margin: 22mm)
#set text(font: "Arial", size: 10pt)
#set heading(numbering: none)
= N83 exact checkpoint validation: pass

Run ID: `n83-checkpoint-extended-20261115` #linebreak()
Start: 2026-11-15 22:12:23 UTC #linebreak()
Source: `1858073d1e7336ee63136da75a471f668ff5fb5f`

One exact release-library checkpoint test was requested under the frozen protocol. The shared macOS arm64 host used Rust 1.99.1, locked offline release Cargo, one build job, a fresh SSD target and a 5,400-second cap. The release build finished in 57 minutes 27 seconds; the test runner reported one passed, zero failed, in 0.08 seconds.

#image("run-path.svg", width: 100%)

The terminal receipt records child and wrapper exit 0, no stop signal, and a 21,721-byte log with SHA-256 `37dc7c5967b2725ec853a194f02e2ce81b1253ce2a2b6e55acd98b929fe6b66d`. A `rust-objcopy` missing-library warning did not prevent compilation or execution. The interval is validation overhead, not IC runtime.

Evidence: `verification/k600-m6-validation-extended-20261115/{PROTOCOL.md,launch.sh,cargo.log,receipt.txt,source-commit.txt}` in the same study. The Markdown `REPORT.md` beside this PDF gives clickable links. The full release-library gate and charged K600 IC comparison remain open; no canonical IC graph changed.
