# F = 1 walk-table oracle, single-target ecbench (2026-11-07)

Copies of the study figures and report; the study is `research/curve25519_f1_walk_20261107/`.
No `benchmarks/README.md` row yet: the validation gate (replay certificate, validator report,
`VALIDATION.json`) is not filled. All numbers are unverified assertions [pcc:unverified].

Corrected after validation (warm-up records excluded, 64 measured rounds per arm): the primary F = 1 arm
costs 0.809x the F = 2 incumbent at k = 20 and 0.855x at k = 24 (w = 0), and stays 1.50x and 1.57x
from rho-neg at w = 0 [pcc:unverified]. It is a generic walk-table gain that moves toward rho, not a
decomposition-based index-calculus improvement [pcc:unverified]. See the study README, section 7
[pcc:unverified].
