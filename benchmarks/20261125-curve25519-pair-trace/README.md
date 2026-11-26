# Curve25519 pair-trace 2+1 split screen

Documentary copies of the frozen native-Rust `d=7` D/R/P comparison. The
primary P arm uses an exact unordered-pair trace quotient and two quadratized
S3 links. It lowers maximum productive degree from 5 to 4, but uses
`2371.7222843` times D's counted F4 work, with descriptive paired 95% interval
`[2371.7202857, 2371.7238337]`, against the frozen upper target below `1.00`.
Peak RSS reaches `14.4512` times D, so `d=9`, `d=19`, scaling, and `m=4` were
not opened.

This entry is outside the admitted benchmark table. It has no independent
replay, validator report, matched-rho reference, or `VALIDATION.json`, and it
does not support an exponent or total-work subexponential claim.
`[pcc:unverified]`

Source of truth:
[`research/curve25519_pair_trace_20261125`](../../research/curve25519_pair_trace_20261125/).
Browsing copies: [`report.pdf`](report.pdf), [`figure.svg`](figure.svg),
[`figure.png`](figure.png), [`paired.csv`](paired.csv), and
[`report.typ`](report.typ).
