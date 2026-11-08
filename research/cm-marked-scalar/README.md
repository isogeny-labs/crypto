# Native CM marked-scalar relations and comparison

Conductor T-1888, [PR #3674](https://github.com/isogeny-labs/crypto/pull/3674). The PR was opened before implementation and comparative execution, as requested. Native closed-relation certificates and executable rational-subgroup actions now cover P-192, P-224, P-256, P-384 and P-521. P-192 is primary. The original objective of an admitted end-to-end runtime improvement over matched regular public-scalar multiplication remains open.

The [construction and exact cost ledger](CONSTRUCTION.md) explains the kernels, endpoint isomorphism, CM labels, signed orbits and all five cost vectors. The [construction report](reports/native-five-curve-20261108/report.pdf) contains the diagrams and original admission record. The subsequent [modular recoding report](reports/recoding-20261108/report.pdf) and [complete replayable census](recoding-v1/README.md) compare 30,780 schedules with symmetric direct-method optimization. P-192 saves three additions and three negations against its same-width centered fused schedule; the full direct minima remain smaller on all five curves.

For each supplied variable-base point P, the relation route evaluates phi, eta, the closing isomorphism sigma(x,y)=(x/ell²,y/ell³), then [k], with k=mu/ell modulo n. Full polynomial identities prove sigma eta phi=[ell], while the native CM checks verify a+b=mu modulo n for alpha=a+b*pi. The implementation is an explicit rational-subgroup evaluator; the much larger global noninteger quotient has unexpanded residual polynomials. Its degree and construction status are retained.

The native constructor supplies the three new curves from signed Frobenius kernels. P-384/P-521 preserve historical certificates; their historical Python producer was not rerun. The exact-token bundle and [five-curve provenance](fixtures-v2/PROVENANCE.md) keep the two origins distinct. P-192 uses its published explicit group because this OpenSSL build omits the named entry.

| Requirement | Current evidence and remaining work |
| --- | --- |
| PR before implementation/execution | Hosted PR #3674; planning commit 4afeff227516 |
| Native source and exact replay | 150 point/window cases, 15 orbit fixtures, complete polynomial/signed-kernel/label checks; native-v17 and strengthened native-v19 |
| Negative controls | Eight certificate mutations rejected at intended checks; eleven synthetic analysis-design mutations rejected |
| Explicit orbit object | Canonical affine representative and scalar transporter reproduced on all five curves |
| Costed implementation | 90 native counted rows, clocks disabled; D/A/N, explicit map-formula calls, correction/setup and conversion boundaries retained |
| Matched runtime reference | Public signed-window, OpenSSL public multiscalar and single-point references implemented; actual timing comparison pending |
| Cold end-to-end gain | Not established; local admission refused twice. Follow-up resolves EKS routing with the existing submitter identity, but quota reads remain forbidden and reported monthly spend exceeds the supplied ceiling |
| Timing protocol | Native schema validation passes for the initial P-384/P-521 draft; source/binary freeze and admitted backend pending |

Build `make -B -j1 -C research/cm-marked-scalar native-complete construct-binary qualification-tests-binary analysis-tests`; replay `/tmp/cm-marked-scalar-build/native --qualify research/cm-marked-scalar/fixtures-v2` and `/tmp/cm-marked-scalar-build/qualification-tests research/cm-marked-scalar/fixtures-v2`. Do not rerun write-once constructions into existing output directories. Use source/binary hashes rather than shared-filesystem timestamps.

See [NATIVE_STATUS.md](NATIVE_STATUS.md), [BENCHMARK_CONTRACT.md](BENCHMARK_CONTRACT.md), the [protocol notes](protocol/README.md), and the [measurement-admission problem](../../problems/cm-marked-scalar-measurement-admission.md). A five-curve correctness extension does not silently extend the initial two-curve timing draft. No new cloud job or controller was launched.
