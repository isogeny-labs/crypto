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
