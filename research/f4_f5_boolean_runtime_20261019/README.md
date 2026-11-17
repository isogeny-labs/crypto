# Native Boolean solver comparisons

The [results report](REPORT.md) and [PDF](REPORT.pdf) compare matching
16/22-variable public Boolean systems and their first-witness decomposition
wrappers. The final eighteen-arm comparison uses five admitted native samples
per method/fixture on one dedicated c7i.2xlarge. Exact original root sets and
group witnesses are checked. All costs are solver-boundary measurements;
common curve/base construction and the independent post-timing audit are
outside those intervals.

On the n23 planted fixture, cold decomposition medians are F4 26.3691 ms,
F5 26.2727 ms, F6 7.3539 ms, generalized typed 1.4781 ms and batched Semaev
46.3986 ms. On the fixed generator control, they are 370.4942, 375.1102,
18.6067, 1.4931 and 45.6419 ms. Complete-root enumeration is a separate
boundary: planted F4/F5 360.1913/363.9447 ms, typed 1.4494 ms. F6's
complete-original-root interface is not benchmarked here.

The [bounded native generalized solver](../../tools/generalized-rootfinder/README.md)
includes certified fields/domains, exact counts independent of output caps,
complete original-coordinate roots and the full eight-coordinate S3 separator.
Its typed Boolean entry avoids JSON conversion; fresh paired confirmation
qualifies all sixteen typed cells faster than the previous wrapper. The
existing scalar/wide kernels remain the search backends. One n23 JSON wrapper
control regresses about 3.5%; that outcome is retained.

The original single-write matrix optimization qualifies about 1.04x/1.036x
on the primary F4/F5 matrix. Extra SIMD table construction is neutral.
Batching Semaev's cold setup qualifies about 3.6x while preserving every
table key, first witness and abstract group charge. Each ratio is from its
own same-host experiment; none multiplies different hosts.

The initial serial-clearing trial improves all eight one-worker matrix
cells, but its n17 F4 four-worker control regresses (paired ratio 0.8436).
That result is retained in [its run report](reports/run-20261019T030905Z/REPORT.md).
The [refined protocol](SERIAL_GUARD_PROTOCOL.md) restricts activation to an
explicit one-worker process before querying the current pool, and confirms
the change with fresh controls and direct legacy-kernel ablations. All eight
one-worker cells qualify faster and all four four-worker controls are neutral.
Primary n23 incremental ratios are 1.0802 F4 and 1.0730 F5; direct legacy to
candidate ratios on that same host are 1.1141 and 1.1209. Eleven rejected
attempts remain archived. No full recursive-solver gain is inferred.
All matrix/setup switches remain opt-in.

| Requested work | Status and evidence |
|---|---|
| Native F4/F5 versus F6 and both existing root-solver families | Verified on the original n17/n23 fixtures: eighteen arms, five samples per arm/fixture, exact outputs; [final table](REPORT.md). |
| Finish the missing generalized native runtime | Implemented and verified for its documented bounded field/domain contracts; thirteen tests and exact-count CLI fixtures. Hard implicit-domain search remains open. |
| Optimize measured bottlenecks | Verified paired gains for single-write tables, guarded serial clearing, typed generalized input and batched Semaev setup; neutral and slower controls retained. |
| n19 native matrix result | Blocked by the existing 36-variable matrix admission limit; refusals retained, with no speed result inferred. |
| Broader generalized-rootfinding research gates | Partial: hardest field families, external comparison, independent-host qualification and original hard-family gates remain in the linked open problem. |

Use [RAW_EVIDENCE.md](RAW_EVIDENCE.md) for archive extraction, source/binary
provenance and native analysis replay. The Rust analyzers prepare admission,
medians, ranges, paired ratios and bootstrap decisions; Python only renders
those recorded values. [VALIDATION.md](VALIDATION.md) records every actual
test invocation, including failures and targeted reruns.

The old `random=true` flag denotes a fixed generator in these single-target
runs, as explained in [the correction](TARGET_LABEL_CORRECTION.md). n19 has
36 Boolean variables and its retained matrix admission refusals supply no
timing. External fastest comparisons, Arm64/GPU, whole-IC/rho, m5-separator
performance and the original hard-family 2x/5x/8x gates remain outside this
qualification; [the open problem](../../problems/generalized-rootfinding-all-roots-8x.md)
preserves the latter requirements.
