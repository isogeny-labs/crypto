# Projective-law branch run, 2026-10-21 17:15 UTC

This run extends the earlier 17:01 arithmetic gate without altering that
frozen run. The Rust `complete-law-gate` binary evaluated the published
formulas on all 21 normalized exact-order Scholten models. It compared every
evaluated output with independent Cantor arithmetic: 13,608/13,608 1+1,
12,096/12,096 1+2 and 6,551/6,551 2+2 sampled additions matched. The 2+2
sample includes 1,175 targeted exceptional cases. The separate producer
rerun matched the 21 raw lines byte-for-byte (SHA-256
`32a6018c300750459e3a2ab1be39191b7e53b4e21507a5c9dd6f0f4bcbe6dec8`).

The executed command from the study directory was
`cargo run --release --locked --bin complete-law-gate` with stdout captured
as [`../../results/complete-law-branches.jsonl`](../../results/complete-law-branches.jsonl).
The figure source is [plot_law_gate.py](plot_law_gate.py); it lays out the
recorded native counters. The [PDF](report.pdf) embeds both
[branch graphs](complete-law-gate.svg) and
[2+2 graph](complete-law-2plus2.svg). The host was an Intel Xeon 6975P-C
with 32 visible CPUs; this is an exact arithmetic gate, so no wall-speed
isolation tier or cost ratio is claimed. Zero DLP targets used this law.
Exceptional branch coverage and full collector integration remain open.
