# CPU numerator comparison: recorded main outcome

All **360 calls on 12 fixed public targets** completed: 36 warmups and 324
measured observations, nine measured processes per target/arm. The frozen native
analysis reports byte-identical full certificates and equal structural counters
for each target across all arms/repeats, with zero errors. All 360 external
Native replays, 360 OpenSSL positive checks and 720 intended corruption controls
pass. The original supervisor, all three phases and all 295 post-call input
hash checks pass. These are unsigned local observations,
`PCC-20261123-70a35c`. [pcc:unverified]

The original protocol-bound unified export check **fails with 324 seed-table
binding findings**. The separate schema/flow/file-only check passes. See
[EXPORT-ERRATA.md](EXPORT-ERRATA.md): preserve both gates and the original
protocol/export unchanged. The native recovery/control observation and failed
canonical metadata acceptance have separate scopes. [pcc:unverified]

## Applicable baseline and intervals

Arms a/c use the exact staged-numerator parent; b uses the reverse-pass
numerator candidate already present in the current library. Both workers use
the common 108-input snapshot, with only that library delta, m8/batch64 and
unchanged table/trial/column/work limits. The experiment measures that concrete
implementation comparison. Historical round24, unused round25 preparation,
the earlier 24-call cohort and synthetic/preflight fixtures are separate.
[pcc:unverified]

The internal complete clock starts before curve/public-target construction and
includes setup, all attempts, oracle, reduction, internal replay, an additional
Archive verification and serialization. It excludes process launch, corruption
control, output I/O, external Native replay and OpenSSL audit. The enclosing
driver clock includes worker process/corruption/I/O and external Native replay;
oracle is an included internal subphase. OpenSSL auditing is post-run. Full
complete/driver/oracle medians and ranges remain in the frozen `summary.csv`.
[pcc:unverified]

## All target comparisons

Ratios below divide the named parent's complete-time median by the compared
arm's median. Values above 1 in the first two columns mean a lower observed
candidate median. The last column is the same-parent A/A comparison. Each
median uses nine processes; the two seeds identify different targets. Values
are rounded for display; full precision and every observed range remain in the
raw records and report. [pcc:unverified]

| Field bits | Target seed | a/b | c/b | a/c (A/A) |
| ---: | ---: | ---: | ---: | ---: |
| 20 | 101 | 0.966 | 0.924 | 1.046 |
| 20 | 202 | 0.898 | 0.966 | 0.930 |
| 24 | 101 | 1.105 | 1.073 | 1.030 |
| 24 | 202 | 0.906 | 0.935 | 0.969 |
| 28 | 101 | 0.967 | 0.986 | 0.980 |
| 28 | 202 | 0.943 | 0.971 | 0.971 |
| 32 | 101 | 1.008 | 0.997 | 1.011 |
| 32 | 202 | 1.028 | 1.024 | 1.003 |
| 36 | 101 | 0.991 | 0.933 | 1.062 |
| 36 | 202 | 1.036 | 1.057 | 0.980 |
| 40 | 101 | 1.066 | 1.033 | 1.031 |
| 40 | 202 | 1.050 | 1.032 | 1.018 |

On the two largest targets, complete medians a→b are
15.889932035→14.912780968 seconds and 13.419740394→12.781461206 seconds;
same-parent c medians are 15.406181760 and 13.185250292 seconds. Smaller targets
are mixed. The 40-bit/seed202 A/A c observation at 46.241018449 seconds is
retained in its full range. No observation is dropped. [pcc:unverified]

These are shared-host L0 descriptions, with balanced execution position and an
A/A control, known source labels, separate build paths, ASLR and retained other
research. The frozen analysis makes no timing significance/equivalence decision.
CPU time, per-process peak RSS and complete counted S are unmeasured; an
asymptotic exponent is not estimated from these clocks. The reported values
do not update a ranked benchmark or canonical leaderboard. [pcc:unverified]

## Execution, custody and data

The pre-run source/control commit is b4997f9e; the frozen packet is
51ac3d745a0a7512ef3622f87fa536ae165c8cec. Protocol freeze was 14:30:46 UTC,
before launch at 14:44:47 UTC. Driver finishes 15:03:27, finalization in that
same whole-second timestamp and point audit/supervisor at 15:03:34. All phase
exit/physical-stop fields are 0. Driver sampled whole-cgroup peak is
14,327,767,040 bytes, including retained other research, rather than process
RSS. The worker-only 4 GiB virtual limit and 1500/120/120 phase ceilings remain
unchanged; no retry/resume or concurrent build/test occurred. [pcc:unverified]

The local pre-run checkpoint is 28,366,016 bytes, SHA256
`d00ef84b21ba48ff61631a162da529d804b4294082411f4c95a7b641563d373c`.
Fresh extraction checked all 302 payload files, including all 295 main inputs.
Read-only review14 bound those records before main. Original frozen inputs,
reviews10–14, warmup-only preflight and earlier cohorts remain unchanged.
[pcc:unverified]

The separate native metadata adapter was drafted during main and qualified only
after terminal. Three initial strict-lint findings are retained; corrected
Clippy passes. A source-prehashed build, two copied executables and seven-entry
pre-call manifest precede six passing controls. It makes no solver, replay or
primary-analysis calls. Every raw CSV numeric token and status JSON value is
preserved with arbitrary-precision JSON numbers. Export-v1 is 2,936,344 bytes,
SHA256 `46e507036dfcf874b65b2fd9c769c91050843a7b509e8f441fd35e8e532c0097`.
It stays local under the no-S3 instruction and JSON data policy. No encoding or
filename change moves that large JSON into Git. [pcc:unverified]

The compact published worker CSV retains the exact 24-column records in native
schedule order; the schedule supplies collector identity. Complete certificates,
status JSON, original event log, both export-check records and full referenced
payload remain local, with hashes and a reproducible local handoff. Remote-only
full replay and signed execution standing remain open. [pcc:unverified]

## Continuing research

Keep this source-bound parent/candidate comparison as a measured implementation
reference, including regressions and A/A variation. The next empirical
comparison needs a prospectively correct identity/seed protocol and complete
counted accounting on independently added public targets. Mathematical work
continues on the source/history/cost bridge already identified by the finite
precision audit; this component does not discharge those dependencies.

The bounded prior-art read recovered EXP-HZM-001 and the corrected July18
zero-minor triage in the local autoresearcher snapshot. Their explicit
enumeration/model conclusions retain their stated scope. The read also checked
the real-sign-test computational model in arXiv:2003.09533v3. These reads prevent
re-proposing the same enumerative reformulation as untried; they provide no
general novelty clearance or exclusion of every coordinate/zero-minor method.
The source-read receipt names the actual paths and hashes. [pcc:unverified]

The classical prime-field ECDLP goal remains active. S3 and quantum work remain
excluded. No additional experiment is currently running.
