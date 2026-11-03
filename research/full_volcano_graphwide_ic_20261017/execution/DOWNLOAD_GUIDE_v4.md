# Volcano datasets and cryptanalysis: download guide v4

Frozen delivery date: 17 October 2026. Snapshots are immutable and do not
track subsequent worker progress. This guide does not claim the original
five classes or the full requested benchmark matrix are complete.

## New full selected-point-order C diagnostic

- Archive: https://drive.google.com/file/d/1HHlPDmGzwSopnwMGQ4vgvcGSH0WRa0BO/view
- Inspected report: https://drive.google.com/file/d/1Gdrpkb8n8XhBoO8Sdr3JKl_VIPMUgqdZ/view
- All-phase min/max/mean CSV: https://drive.google.com/file/d/1UHAE8NCShFWGQ1hHOR3U56KeNdW_MrQL/view
- Native aggregate CSV: https://drive.google.com/file/d/1OUbeh-phxP8vXXE_gzY6KoS6tFPapjIE/view

Archive: 9,017,364 bytes, SHA-256
`faeec1aeb755c347e1b22a5bec738e6706c1d1fd0df71683f60f67260ab47865`.
Extraction and every MANIFEST entry passed; repacking was byte-identical.
The extracted measured executable re-ran the original two-target control.

Each of 1,869 retained C models has two public one-target workloads.
EC PH/BSGS and k=1 Tate/Miller plus multiplicative PH / checked projected-prime
factor-base relations both verify all 3,738 workloads: 7,476 primary method
records. A separate replay matches every non-clock record, fixture and truth
field. Source-only controls and replays are separate populations, never pooled.
All 2/3/7 prime-power components of the selected exact point order are solved;
orders range from 2,600,529,297,408 to 1,637,958,981,148,213,248.
Largest prime is 7, so large numeric order does not imply a hard prime-order DLP.
No maximal-point-order or whole-group cyclicity certificate is claimed.

There are 26,166 setup equations / queries and 161,336 target-digit equations /
queries. Setup includes identity/dependent equations; per-component ranks,
pair tests and matrix work remain in the records. Dense yield reflects tiny
digit groups. This is not NFS, integer-smoothness IC or a large-k backend.
No planted scalar is supplied to either solver; generated scalar truth is
retained separately. Five tests include exhaustive small controls, all tiny
destination digits and rejection of a rank-two target outside the actual
cyclic subgroup, even when annihilation alone accepts it.

Cold solver intervals include order certification, setup, projections,
transfer, relations, matrix work, reconstruction and final EC verification.
Shared fixture generation is outside both intervals and separately reported.
Field operation categories are separate: curve inversions, square-root
internals and unpriced integer work are explicit. There is no combined priced
primitive total. CPU 4 was reserved, PSI unavailable; the full-prefix replay
was contended and retained. All clocks are unqualified observations. There
is no speedup, fastest-method or scaling claim. Same-implementation replay
does not establish independent arithmetic correctness.

## New 4,037-vertex repaired survey checkpoint

- Archive: https://drive.google.com/file/d/1xo7xHgABvtfG_3V4i66A5lwxD2Wr3yoK/view
- All-arm min/max/mean CSV: https://drive.google.com/file/d/1GGDuqF6R3S_HiFxPq6IjtZa5xORHUW1F/view
- Inspected report: https://drive.google.com/file/d/1JXSLCQc8h_pvLIoh2a4KONrlH10yqI-h/view
- Self-contained graph explorer: https://drive.google.com/file/d/1tK5b6cXZsR6cDq_SMP_Wmc_wemn6LHCD/view

Archive: 44,861,584 bytes, SHA-256
`2eef350b1f97efcd1840d2a403c874935859f529b0871aa2cd9457372040e70c`.
Extraction and every MANIFEST entry passed; this archive's outer repacking
was not rechecked. Download the HTML and open locally if Drive shows source.
Viewer native population/hash checks, Node interactions and real Chromium
desktop/floor/mobile checks pass; all three screenshots inspected.

4,037 / 24,823 vertices are audited, with all 51 strata sampled. There are
64,592 accepted records: 41,573 verified, 20,132 exhausted, 2,887 timeout;
2,098,636 relations / 103,036,278 source trials / 98,188,909 actual
decomposition calls. Timeouts retain unknown counters, not zeroes. Native
CSV groups by graph, stratum, method, status and phase with actual n.
Different runtime epochs have unqualified clocks. Failed and interrupted
attempts remain retained and excluded from accepted populations.

The viewer includes all 24,823 vertices and 69,300 directed kernel maps for
the three additional classes, with 51 total strata. It marks only the frozen
4,037 audited vertices. A graph-complete vertex need not have measurements.
These are complete F_p-isomorphism classes for the stated finite selected
degree sets; not infinitely many prime degrees or composed isogenies.
Exact large field values are decimal strings, parallel kernels/dual directions
are retained, and each recorded map has an inspector. Map verification is
same-implementation evidence, not an independent arithmetic replay.

## Original inputs and older separate cohorts

- Three complete graph databases: https://drive.google.com/file/d/1SSyQ5bY0RPTK_abRW6MCG78cdANonBVi/view
- Five original retained prefixes and local conductor certificates: https://drive.google.com/file/d/1zeGSq3GWne9yTH0R37EZ3YI89xG_yR2L/view
- Local-conductor report: https://drive.google.com/file/d/17jC_A1uSRHtJc5c8Sy9AkH9jt6UJTbBX/view
- Earlier C order-7-only cohort: https://drive.google.com/file/d/1Xi7mjLCskYEauXlwzAfQQtOoHw6waop-/view
- Earlier 2,935-vertex checkpoint: https://drive.google.com/file/d/1IoPt5C-EYlo488pPVWPx-qpDHo7t1GwY/view
- Historical complete B multipart delivery: https://drive.google.com/drive/folders/1pTN-Ov-4Q2N8nqZaTsKQC_WuRgS1cqyu

These old cohorts stay unchanged and are not pooled with the new data.
The original source End(E) conductors are 268,435,456 / 129,140,163 /
967,458,816 / 544,195,584 / 644,972,544 for A/B/C/D/E. All 15 calibration
cuts pass. Unknown prefix conductors remain unknown. C has 517 exact local
labels across 24 observed strata and 1,352 unknown, not all 165 full strata.

| Original | Expected vertices | Strata | Current full enumeration |
| --- | ---: | ---: | --- |
| A | 268,435,456 | 29 | incomplete |
| B | 129,140,163 | 18 | historical complete |
| C | 1,934,901,248 | 165 | incomplete |
| D | 1,631,523,870 | 143 | incomplete |
| E | 3,869,658,230 | 160 | incomplete |

Total: 7,833,658,967 expected vertices / 515 strata. Original full-vertex
method comparisons remain open, including the requested 8 targets, 5 rounds,
warmup, four base sizes, million trials, transported targets, exact subgroup
representation registration, independent arithmetic replay, isolation/A-A
qualification and external fastest references. The bounded survey's two
targets, one round, bases 16/64, 4,096 source trials and 3-second child watchdog
do not satisfy that full matrix. EC/finite-field PH on the C prefix does not
discharge the other originals or graph-wide transported targets.

## Code and storage limits

Repository: https://github.com/aburan28/crypto
CLI: `crypto isogeny class`, with plan/build/resume/verify and explicit maps
for its supported ordinary prime-field models, CM bounds and finite degree
sets. It is not a universal arbitrary-field or unbounded-isogeny enumerator.
New C prototype: `research/full_volcano_graphwide_ic_20261017/execution/full_point_order/`.
Its frozen protocol and archive provide exact executable, native source,
dependency digests, inputs and fresh-output replay commands.

Drive upload metadata confirms names, sizes and parent folder; local hashes
are retained. Remote whole-file content hashes have not been independently
checked. Snapshots do not upload themselves as the worker advances.
Normal SDK credential discovery at 20:08 UTC found no AWS signing identity;
no S3 upload is claimed. The existing target is
`s3://crypto-autoresearcher/store`. The reports-to-S3 main-branch workflow
does not attach that identity or a working volume to this runtime. Do not
send credentials in chat; an existing attached role/worker would unblock it.

The 32-GiB workspace cannot hold the remaining original graphs in the current
generic SQLite format. Drive/S3 archives are not its random-access working
volume. A connected durable volume/worker, or a validated external-memory
backend, remains needed. The old B mmap generator is specialized to D_K=-11
and 3-power conductor. Large-k finite-field IC remains an implementation gap:
original A/B/D/E largest-prime embedding degrees are 238827252966622 /
341460 / 101139959082 / 17426350368. The additional three have k=30875 /
3953777913734 / 2126. Dense representation bounds are not universal
impossibility results. No new paid cloud compute/storage was provisioned.

## Actual resumed local worker

PID 97646 launched at 2026-10-17T20:13:41.722914+00:00; checked live at 2026-10-17T20:13:55.172771+00:00. Readback confirms PPID 1, its own session, a non-zombie process, two newly audited vertices and an active next job. The outer execution guard is now 72 hours / 6 GiB (previously 24 hours / 2 GiB), a separately recorded operational envelope. Frozen queue, solver, targets, rounds, bases, source-trial budgets and 3-second child watchdog are unchanged. This is a new runtime epoch with unqualified clocks; process survival after a host interruption is not guaranteed. There is no unattended upload claim. Source and evidence follow-up: https://github.com/aburan28/crypto/pull/2106 . Receipts are in execution/resume-after-4037/.
The fixed checkpoint above remains 4,037 vertices regardless of later progress.
