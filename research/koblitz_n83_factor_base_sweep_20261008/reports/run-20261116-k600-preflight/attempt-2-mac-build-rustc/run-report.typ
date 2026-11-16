#set page(paper: "us-letter", margin: 0.7in)
#set text(font: "Helvetica", size: 10pt)
= N83 K600 native supervisor build: compiler-selection stop

*Observed:* 2026-11-16 07:43:40–07:43:41 UTC, exit 101 before compilation. Cargo 1.99.1 selected Homebrew rustc 1.93.1; locked dependencies require at least 1.94.1.

#image("diagram.svg", width: 100%)

Raw evidence: `stderr.log`, `exit.txt`, `start.utc`, `end.utc`, `source.commit`, `rustc.version`, and `disk-watch.tsv`. The frozen attempt-2 amendment pins the compiler path. No K600 circuit ran.
