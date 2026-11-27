# Curve25519 selective-quotient qualification

Documentary copies of the frozen six-arm `d=19` native-Rust comparison. The
primary B policy retains quotient variables only at the last two nonlinear
squares. It lowers maximum productive degree from 5 to 4 on 8/8 random
targets, but its paired B/D geometric-mean F4-work ratio is `1.3292420766`
with descriptive 95% interval `[1.3292409735, 1.3292431924]`, against the
frozen upper target below `0.75`. Peak RSS reaches 2.2876 times D, so scaling
and `m=4` were not opened.

This entry is outside the admitted benchmark table. It has no independent
replay, validator report, matched-rho reference, or `VALIDATION.json`, and it
does not support an exponent or total-work subexponential claim.
`[pcc:unverified]`

Source of truth:
[`research/curve25519_selective_quotient_20261125`](../../research/curve25519_selective_quotient_20261125/).
Browsing copies: [`report.pdf`](report.pdf), [`figure.svg`](figure.svg),
[`figure.png`](figure.png), [`paired.csv`](paired.csv), and
[`report.typ`](report.typ).
