# Primary N83 K2048 five-summand model panel · run 20261027T2012Z

**Result.** Both matched 2,048-orbit bases installed the same-size chained-S3
five-summand SAT model on the primary `EC1N83Ckb0h252dacc102a4` curve.
All four natural searches returned `Unknown` at the frozen 5,000-conflict
cap. The [figure](capped_search.svg) shows the measured capped search times;
its [data](capped_search.data.json) is copied from the
[native audit](../../evidence/panel-native-audit.json). There is one run per
arm and public point, so no sampling interval is available.

| Public point | U search (s) | D search (s) | U/D ratio (derived) | Status |
| --- | ---: | ---: | ---: | --- |
| Fixture 0 | 152.482 | 166.213 | 0.917 | Both `Unknown`, 5,000 conflicts |
| Fixture 1 | 159.272 | 153.158 | 1.040 | Both `Unknown`, 5,000 conflicts |

The ratios divide recorded U and D search milliseconds on each point. Their
direction reverses between the two fixtures. These are capped solver-stage
durations on a shared local host, without a CPU wall-time comparison tier.
They do not select a factor base or describe a completed IC pipeline.

The [protocol](../../PROTOCOL.md) fixes curve, inputs, two existing public
points, chained-S3 encoding, zero split bits, and caps; the
[ordering amendment](../../AMENDMENT_1.md) corrected the run sequence before
the second historical search. The native replay preserved U0/D0/D1/U1.
The model binary SHA-256 was
`5123ef012623a0f425f93a91979a918f7e2df369c1309352919a47e9eef52f06`.
Every arm used the pinned Linux arm64 image
`sha256:7c4ae649a84014c467d79319bbf17ce2632ae8b8be123ac2fb2ea5be46823f31`
on a Darwin arm64 shared host, with one CPU, 4 GiB memory, zero swap,
no network, read-only root filesystem, 300-second host watchdog and
240-second SAT deadline. The solver caps were 5,000 conflicts. Model
construction yielded 111,478 variables, 1,140,557 initial clauses and
4,980 XOR rows in both arms.

Six cells were requested and completed. The preflights' model construction
times were U 236.615 ms and D 94.273 ms. The four search cells' model/root
setup times and memory peaks are retained per cell in
`evidence/panel-native-audit.json`; all host guards passed without OOM or
watchdog activation. The native [negative controls](../../evidence/native-negative-controls.json)
rejected a changed compressed input before allocation and a changed target
coordinate before an audit result could be written. The Rust auditor replayed
the committed receipts byte for byte. The [native amendment](../../NATIVE_AMENDMENT_1.md)
identifies the active guard and audit path and the earlier historical receipts.

No group relation or novel rank was verified in this panel. Final matrix,
target-log and same-point rho times are unmeasured, so the candidate and
online-speedup fields remain null. The next gate is a versioned worker that
lifts and verifies a natural model as a group relation, or an improved
decomposition search that reaches one under a frozen cap. The canonical
`docs/index-calculus-scoreboard.html` and `docs/ic/progress-timeline.json`
were checked and left unchanged because this panel does not promote a
completed candidate. The [PDF](REPORT.pdf) includes the result and figure.
Visual assets are mirrored in `benchmarks/20261027-n83-k2048-m5-model/`;
the admitted `benchmarks/README.md` table awaits the validation gate's
replay certificate, validator report and matched rho reference.
