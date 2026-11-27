# Curve25519 symmetric-remainder quotient qualification

Documentary copies of the counted native-Rust comparison between direct
remainder equations and explicit quotient auxiliaries. At the frozen `d=19`
decision point, the quotient/direct paired geometric-mean F4-work ratio is
`1.2864624565` with descriptive 95% interval
`[1.2864612925, 1.2864635223]`; the required upper endpoint was below `0.50`.
The quotient arm lowers maximum productive degree from 5 to 4 but raises peak
RSS by as much as 2.4275 times, so scaling and `m=4` were not opened.

This entry is outside the admitted benchmark table. It has no independent
replay, validator report, matched-rho reference, or `VALIDATION.json`, and it
does not support an exponent or total-work subexponential claim.
`[pcc:unverified]`

Source of truth:
[`research/curve25519_symmetric_remainder_20261125`](../../research/curve25519_symmetric_remainder_20261125/).
Browsing copies: [`report.pdf`](report.pdf), [`figure.svg`](figure.svg),
[`figure.png`](figure.png), [`paired.csv`](paired.csv), and
[`report.typ`](report.typ).
