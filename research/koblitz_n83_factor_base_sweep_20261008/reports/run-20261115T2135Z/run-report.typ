#set page(paper: "a4", margin: 22mm)
#set text(font: "Arial", size: 10pt)
#set heading(numbering: none)
= N83 checkpoint retry: compile timeout

Run ID: `n83-checkpoint-retry-20261115` #linebreak()
Start: 2026-11-15 21:35:47 UTC #linebreak()
Source: `ca5ab38910489f16ee2eda9ef78f086edc3e481f`

One exact checkpoint validation test was requested under the frozen protocol. The shared macOS arm64 host used Rust 1.99.1, locked offline release Cargo, one build job, a fresh SSD target and a 1,800-second wall cap. The root `crypto_lib` test binary was still compiling at that cap. Zero tests started and no assertion verdict was produced.

#image("run-path.svg", width: 100%)

The terminal receipt records child and wrapper exit 124, no stop signal, and a 20,315-byte log. The raw log SHA-256 is `c3fd6656ed9beee2e249af7caf7682c7f6b88b29a358a627b791756d2a99b06f`. It contains no test summary. The disk guard wrote no stop marker. The 1,800-second interval is validation overhead; it is not an IC runtime sample.

Evidence: `verification/k600-m6-validation-retry-20261115/{PROTOCOL.md,launch.sh,cargo.log,receipt.txt,source-commit.txt}` in the same study. The Markdown `REPORT.md` beside this PDF gives clickable source links. The root release-library gate and charged K600 comparison remain open. No canonical IC graph changed because this run supplied no IC measurement.
