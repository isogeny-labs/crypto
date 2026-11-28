# Curve25519 quotient-layout qualification

Documentary copies of the frozen five-arm `d=19` native-Rust comparison.
The primary F arm permutes the selective-cut quotient variables to the leading
positions under ordinary grevlex. It retains maximum productive degree 4 but
uses `1.2934390725` times D's counted F4 work, with descriptive paired 95%
interval `[1.2934369447, 1.2934413625]`, against the frozen upper target below
`0.75`. Peak RSS reaches `2.3766` times D, so scaling and `m=4` were not opened.

This entry is outside the admitted benchmark table. It has no independent
replay, validator report, matched-rho reference, or `VALIDATION.json`, and it
does not support an exponent or total-work subexponential claim.
`[pcc:unverified]`

Source of truth:
[`research/curve25519_quotient_layout_20261125`](../../research/curve25519_quotient_layout_20261125/).
Browsing copies: [`report.pdf`](report.pdf), [`figure.svg`](figure.svg),
[`figure.png`](figure.png), [`paired.csv`](paired.csv), and
[`report.typ`](report.typ).
