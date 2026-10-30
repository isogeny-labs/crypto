# A period ECPP trial does not explain the skipped curves

26 October 2026. All 21 independently known-prime group orders received a
prime label from the recovered ECPP V3.4.1 program in the fixed full-data
configuration. Corrected certificate admission accepts 3 published and 6
skipped orders. Seven published and five skipped orders remain indeterminate
because the unchanged legacy checker reports failed theorem rows. No order
received a composite label. These are software observations, not curve
weaknesses or proof of the historical selection procedure.

![Raw CLI labels and separate certificate outcomes](reports/outcomes.png)

The graph uses exact counts in [native 005 summary](results/summary.json),
not estimates or sampled intervals. Every one of the ten published and eleven
skipped controls is retained. The [full table](results/cohort.tsv) maps each
stream/index shorthand to its unchanged group order in the
[frozen parameter table](../inputs/historical.tsv). This is an order-only
experiment; the original curves, seeds, coefficient conventions, generators
and ICV1 identities are preserved by the parent studies.

## What was tested

The fixed question was whether this recovered program accepts all published
orders but rejects all skipped orders. Each order starts a fresh process with
the complete original 1,684-record catalog, original factoring/Weber tables,
rank zero, proving and N−1/N+1 routes enabled, cyclic-only/distributed modes
disabled. No modern factorization hints enter the legacy program. Actual
arguments, data, binary hashes and input copies are retained in the archive.
The original Bigmod BmInit resets srandom(1) at every modulus initialization;
no new seed was added. An earlier caller-only RNG description was incomplete
and is corrected by protocol amendment 3.

The original static ELF32 compatibility binaries ran unchanged through
QEMU i386 user mode 7.2.0, guest reservation 256 MiB. This is a modern build of
recovered source, not an authenticated 1996 executable. The HTTPS source
archive, sibling compatibility artifacts and public emulator release are
hash-bound in [provenance](evidence/provenance.json) and the
[protocol](PROTOCOL.md). The emulator release has no publisher digest; local
hashes provide reproducibility, not independent publisher authentication.
There is no evidence linking this exact artifact/configuration to the
original curve generator.

## Admission and controls

The C++17/PARI2.17.3/OpenSSL helper reproved every input prime. Under strict
O2/Wall/Wextra/Werror compilation and one PARI worker, 50 fixed limb-boundary
pairs passed 250 addition/subtraction/multiplication/division/remainder
comparisons against the legacy arithmetic helper; ABI 4/4/4/4 was required.
Seven prover/data controls passed: primes 101/65537, composites 15/341,
CM-only 101, empty-catalog CM101, and missing-catalog CM101. Empty data gives
a false composite label for a known prime; missing data crashes and is
indeterminate. Those modified datasets were used only as controls.

Three genuine positive certificates passed both legacy checks and native
admission. Wrong-root, truncated and appended-certificate mutations were
refused even when paired with an old successful checker transcript. Admission
requires a unique prime CLI label, zero process statuses, complete root-bound
format, descending/dividing chain, recognized routes and sentinel, and every
legacy theorem row matching with success flag1. The legacy checker may exit 0
with failed mathematical rows, so its exit code alone never establishes
acceptance. The native reader checks grammar/binding and root primality; it
is not an independent general ECPP mathematical certificate verifier.

The original reader incorrectly required CM header h=−1. Original
PrintCertifEcForC emits signed cert.h; ReadCertifCEcm skips this metadata and
checks explicit curve/point/order. Actual outputs include h=−4 and h=12.
Amendment 4 removed only this unsupported reader restriction. Validation005
reused byte-identical saved 004 inputs/certificates/transcripts, rechecked the
three positive and three negative fixtures, parsed every certificate and
attempted every admission. No original certificate or legacy result was
repaired or rerun. Twelve genuine failed theorem transcripts remain unknown.

## Every historical row

| Stream/index | Role | Certificate admission |
|---|---|---|
| p192-139 | Published | Indeterminate |
| p192-2198 | Skipped | Indeterminate |
| p192-5228 | Published | Indeterminate |
| p192-5405 | Skipped | Accepted |
| p192-6013 | Published | Accepted |
| p224-777 | Skipped | Accepted |
| p224-1485 | Skipped | Accepted |
| p224-5736 | Skipped | Accepted |
| p224-7026 | Published | Indeterminate |
| p239-36 | Skipped | Accepted |
| p239-1562 | Skipped | Accepted |
| p239-2185 | Published | Indeterminate |
| p239-2320 | Published | Indeterminate |
| p239-2491 | Skipped | Indeterminate |
| p239-2682 | Skipped | Indeterminate |
| p239-2863 | Skipped | Indeterminate |
| p239-3048 | Published | Accepted |
| p256-876 | Skipped | Indeterminate |
| p256-1842 | Published | Indeterminate |
| p384-2688 | Published | Indeterminate |
| p521-1136 | Published | Accepted |

Every row has prover exit 0, checker exit 0 and one raw prime label. Accepted
means the defined software/certificate admission passed; indeterminate is not
rejection, compositeness or weakness. All three originally supplied skipped
P224 curves pass. Both skipped and published P256 are indeterminate.

## Flow, failures and resource envelope

| Round | Actual execution and outcome |
|---|---|
| replay001 / source7355b1871 | Build failed 1: helper name collided with filesystem::status; no input preparation or legacy execution. |
| replay002 / sourceb209482f9 | Build passed; PARI preparation watchdog124 after 300 s with worker-stack warnings; no completed input or legacy execution. |
| replay003 / sourcea13b5891b | Prepared all 21 primes; direct ELF32 arithmetic returned 2/no ABI marker; admission failed 1, no prover/checker execution. |
| replay004 / sourcef187cce03 | All prerequisites passed;21/21 real prover/checker pairs completed;0 CLI rejections; first reader admitted 2 published/3 skipped. |
| validation 005 / sourcec0cf8c5a9 | No legacy rerun;21/21 saved parses and admission attempts completed; corrected admission 3 published/6 skipped;12 retained unknowns. |

Each round has its own [run report and PDF](reports/INDEX.md), including
failures. For the004 named-order pass: planned 21, attempted 21, process-completed 21,
not-completed 0, interrupted 0, resumed 0, excluded 0. Admission separately has 9
accepted and12 indeterminate. For005 saved validation: planned 21, attempted 21,
completed 21, excluded 0; these are the same observations revalidated, not21 new
independent samples. Earlier attempts contribute no legacy cohort results.
The human-readable protocol and source were committed/checkpointed before the
affected executions; no machine-schema preregistration or validated unified
experiment.export/v1 was produced. No such certification is claimed.

This was a serial functional experiment on the shared managed four-core/16 GiB
host. No performance isolation tier, A/A timing interval, rusage, total cost,
1997 runtime or speed comparison was measured. Watchdogs were300s for native
phases,600s for the prover and180s for the checker, with 10 s kill grace. Child
virtual-address cap 768 MiB and disabled core dumps apply to legacy execution
and helper phases.004 standalone native admissions were timed but accidentally
omitted that address cap;005 reapplied it to every call. This discrepancy is
preserved, not retroactively described as a capped measurement.

## Comparison with prior findings

The [completed factorization follow-up](../factor_followup_20261026/REPORT.md)
proves all 84 nearby-order/twist/discriminant factorizations over all 21 rows,
including the four previously unresolved targets. Its shortest fitted rules
remain empirical: generation two tests, structural three tests. Generation
as-good calibration 1,973/4,480 and coefficient-root dependence prevent treating
an exact fit as a historical explanation. The earlier19-row two-test
structural fit fails the published P521 control.

The [exact embedding study](https://github.com/isogeny-labs/crypto/pull/2648)
certifies all 21 embedding degrees. Maximal degree accepts 0/10 published and
2/11 skipped; the modern RFC7748 bound accepts 10/10 published and10/11 skipped.
It rejects only skipped p239-1562, whose index is0x7E. Opposite-label P256
orders and several P224 orders share index0x3. The SEA proxy also fails a
uniform larger-or-smaller preference. None of these observations supplies an
ECDLP attack. No measured DLP experiment was requested or performed.

Six admitted skipped certificates already contradict a universal rejection
filter for this fixed ECPP artifact/configuration, regardless of the twelve
unknowns. Raw CLI labels alone also give no separation. This does not exclude
other configurations, other implementations, historical resource filters or
unrecovered generation schedules. It does not explain why any particular
curve was selected, establish intent, prove security or refute every possible
weakness hypothesis.

## Evidence, reproduction and requirement status

[runs.tar.xz](runs.tar.xz) has SHA256
e037fcbb2d646e59454e873f846b97f0103b83769f0ed60f44f6af2181a458f2,
28,589,060 bytes. Its 1,081 listed files contain all five closed raw attempts,
source snapshots, exact vendor bytes/data, commands, statuses, inputs,
certificates, transcripts and outer launcher logs. The external/internal
[per-file manifest](evidence/manifest.json) and
[full GET/member receipt](evidence/archive-delivery.json) bind custody. No
whole workspace, credential wrapper or secret was archived. Raw directories
were ignored only after full remote verification. Hydrate with the standard
artifact tool, check runs.tar.xz.sha256, extract into a fresh directory and
check every MANIFEST.json member; never execute recovered programs merely
to inspect the archive. Repacking uses sorted GNU tar, fixed26October UTC
mtime/owner/group/mode and xz−6/T1, as documented in [reproduction](REPRODUCE.md).

| Requested requirement | Status and remaining gap |
|---|---|
| Original four curves in ICV1 and DiSSECT | Complete in the frozen parent assessment; no identities changed here. |
| Four outstanding factorizations and full comparison | Complete:84/84 records proved;21 controls retained. |
| PDF and PR evidence | This study supplies a new inspected PDF and ready PR; previous PDFs remain frozen. Hosted gates are tracked separately from local validation. |
| Criterion explaining historical skips | Unresolved. No tested fixed rule establishes the historical choice. Need original caller/acceptance records or independently testable historical predictions. |
| Either weakness hypothesis | Unresolved: no applicable attack construction and costed validation established. Trait differences are not attacks. |

Checked canonical index-calculus scoreboard, progress timeline, curve diagrams,
performance-gain pages and figures; none records this order-only software
trial or gains a measured attack/crossover result. Those graphs are unchanged.
Study-local graphs and all run PDFs reflect their own frozen outcomes. No
leaderboard or performance promotion is made. The existing
[problem record](../../../problems/nist-candidate-selection-weakness-hypotheses.md)
retains the original unresolved requirement and checkable exit conditions.

## Primary sources and provenance limits

Atkin and Morain, [Elliptic curves and primality proving, 1993](https://doi.org/10.1090/S0025-5718-1993-1199989-X),
provides the mathematical method, not evidence that the historical generator
used these bytes. The [source audit](https://github.com/isogeny-labs/crypto-autoresearcher/blob/main/research/nist_seed_ecpp_source_20261024/report.md)
and [runtime audit](https://github.com/isogeny-labs/crypto-autoresearcher/blob/main/research/nist_seed_ecpp_compat_20261024/runtime-report.md)
were read before choosing/executing the trial; the latter contained zero
named-order runs. Original CLI/catalog-exhaustion paths, certif.c formatter,
checkcertif.c reader, Bigmod/mod.c RNG reset and N−1 checker were inspected.
The original archive is retained verbatim. Installed PARI2.17.3 documentation
informed the nbthreads correction. [RFC7748 section A.2](https://www.rfc-editor.org/rfc/rfc7748#appendix-A.2)
is a modern comparison criterion, not the original NIST selection contract.
