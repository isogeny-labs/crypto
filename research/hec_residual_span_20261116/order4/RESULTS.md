# Order-four screen: two absolute-simplicity certificates produced

The once-only execution at pushed checkpoint `86581ac8f26` exited 0. All six fixed curves were smooth. Exact counts over extensions of degrees one through four, native arithmetic controls, Newton identities, curve-automorphism/group checks and the modular-certificate controls passed. No target DLP was constructed or solved in this screen, and no speedup was measured.

| p | Coefficients (a7,a5,a3,a1) | #J | Largest prime ell | Cofactor | Ordinary | Base-field simple certificate | Absolute certificate | Group/action qualified |
|---|---|---:|---:|---:|---|---|---|---|
| 13 | (1,2,3,1) | 13,600 | 17 | 800 | yes | no: elliptic factor | not attempted | no |
| 13 | (2,1,4,1) | 29,200 | 73 | 400 | yes | yes | not certified by modular screen | yes |
| 17 | (1,2,3,1) | 142,400 | 89 | 1,600 | yes | yes | yes | yes, lambda=55 |
| 17 | (2,1,4,1) | 98,368 | 53 | 1,856 | yes | yes | yes | yes, lambda=30 |
| 29 | (1,2,3,1) | 1,008,016 | 251 | 4,016 | yes | no: quartic factor | not attempted | no |
| 29 | (2,1,4,1) | 1,068,160 | 1,669 | 640 | yes | no: elliptic factor | not attempted | yes |

The two absolute certificates each contain a squarefree modular power-polynomial witness for all 127 exponents in the finite root-of-unity candidate set from THEORY.md. Their Frobenius polynomials, in descending order, are:

- p17/model0: T8+8T7+48T6+248T5+1182T4+4216T3+13872T²+39304T+83521.
- p17/model1: T8+4T7−20T6+12T5+774T4+204T3−5780T²+19652T+83521.

These claims follow the stated ordinary-factor and Howe–Zhu/power-field certificate arguments. The independent replay subsequently passed at pushed checkpoint `7903d33ad45`: all 296 positive witness records were verified, including all 127 exponents for each declared absolute model. It uses determinant evaluation/interpolation and standalone polynomial arithmetic rather than the producer’s Newton traces/native gcd. All 85 unresolved records remain unresolved. This is independent modular-arithmetic verification, not an independent rerun of point counting, a proof-assistant certificate or full scientific admission.

The p13/model1 polynomial is even. Post-run deduction: its roots occur in distinct pairs alpha,-alpha, so squaring Frobenius lowers the degree of the power field. Together with ordinary base-field simplicity, the Howe–Zhu criterion shows it is not absolutely simple. The main program conservatively reported unresolved witnesses; this deduction does not change its frozen output.

Group qualification is separate from geometry. The p13/model0 catalogue consists only of branch points and cannot reach the chosen odd prime subgroup. The p29/model0 largest prime has valuation two in #J, violating the unique prime-Sylow contract. Both are retained. All qualified cases passed the action on every projected point and signed length-two orbit reconstruction. On the two absolutely simple cases the subgroup orders are only 89 and 53, with large cofactors: they establish a valid geometric/action control, not strong-subgroup or performance coverage.

Raw point counts, field moduli, Weil polynomials, found factors and all witness/miss records are retained in POINTS.tsv, WEIL.tsv and WITNESSES.tsv. All raw SHA256 checks pass. The Homebrew Rust1.93.1 toolchain is recorded; no timing comparison with earlier builds is made. Git custody replaces unavailable S3, with no upload, retry, replacement model or threshold change. The broader tenfold genus-2+ objective remains open.
