# Selective quotient cuts: completed negative qualification

Study `EXP-C25519-SELCUT-001` tested whether retaining quotient variables at
only the last two nonlinear squares in the `d=19` cubic-remainder circuit
could keep the predecessor's degree reduction without its work and memory
penalty.

The primary B arm did keep maximum productive degree at 4 instead of D's 5
on all 8 random targets. It nevertheless used `1.3292420766` times D's
counted F4 field operations; the descriptive paired 95% interval is
`[1.3292409735, 1.3292431924]`, versus a frozen upper limit below `0.75`.
Its maximum per-pair peak-RSS ratio was `2.2876417617`, versus a limit of
`1.25`. B therefore stops before scaling and `m=4`.

All 72 rows completed. Every planted row evaluated exactly and re-added to
the target point. D and A exactly reproduced all 12 predecessor direct and
all-quotient rows, respectively, on targets, counted operations, degrees,
matrix extrema, steps, and verdicts. The U negative control never lowered
degree below D.

This is a `[pcc:unverified]` relation-system stage diagnostic. It does not
price relation probability, setup, filtering, sparse linear algebra,
individual logarithms, or target recovery, and makes no subexponential ECDLP
claim. The instrument, analyzer, raw rows, protocol, and evidence are native
Rust/repository artifacts; no JavaScript is used.

## Layout

- `PROTOCOL.md`, `protocol.json`, `frozen.sha256`: frozen question and gates.
- `preflight/`: build, self-test, and analysis receipts.
- `smoke/`: non-qualification smoke rows.
- `raw/`: the 72 target-major qualification rows.
- `evidence/analysis.json`: machine-readable gate decision.
- `evidence/paired.csv`: every arm paired to D by exact target.
- `evidence/figure.svg`, `figure.png`: editable and rendered comparison.
- `evidence/report.typ`, `report.pdf`: inspected human-readable report.
- `manifest.sha256`: complete retained-artifact binding.
