# F = 1 walk with the target inside it, single-target ecbench (2026-11-07)

Copies of the study figures and report; the study is `research/curve25519_f1_targetwalk_20261108/`.
No `benchmarks/README.md` row: the validation gate (replay certificate, validator report,
`VALIDATION.json`) is not filled. All numbers are unverified assertions [pcc:unverified].

The target-in-walk F = 1 arm with chained jumps (`wwalk:x=1.5,g=1,grow=2,tw=1,cj=1`) costs 0.909x
[0.885, 0.934] the current F = 1 table-then-probe arm at k = 20 and 0.888x [0.846, 0.932] at k = 24
(w = 0) [pcc:unverified]. It stays 1.40x and 1.41x from rho-neg at w = 0 [pcc:unverified]. The frozen
w-band shows it below rho-neg at w = 1 only because rho's per-step canonicalisation is priced and the
walk's per-step hash inserts are not; priced symmetrically, rho-neg stays ahead at every weight
(1.36x and 1.39x at w = 1, post hoc) [pcc:unverified]. The method is one unknown away from rho (with
P1 := G it is a stored rho walk) and is not a decomposition-based index-calculus improvement
[pcc:unverified].

Caveat (validator erratum E3): the progress report's `s_twcj_w1` series (0.83 of the matched rho at
k = 20, 0.75 at k = 24) is an artifact of unpriced table inserts: the frozen band prices rho's
per-step canonicalisation and not the walk's per-step hash inserts [pcc:unverified]. ecbench's own
band verdict for rho-neg against twcj-x15 is "unresolved inside the band" in 16 of 16 sessions
[pcc:unverified]. With inserts priced, rho-neg is ahead at every weight (1.36x and 1.39x at w = 1)
[pcc:unverified]. The closed records and the generated progress report cannot be edited, so the
caveat is stated here. Also with inserts priced, the w = 1 value of twcj / f1-x07 is 0.896 at k = 20
and 0.883 at k = 24, not 0.734 and 0.675 [pcc:unverified]. The method is single-target only: its
walk is built from the target, so no table is reusable across targets [pcc:unverified]. The report
`report-validated.pdf` is the version with the validator errata applied; `report.pdf` is the file
the closed records name and is left unchanged [pcc:unverified].
