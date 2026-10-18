# N83 K=16,627 prebuilt signed-Frobenius base: construction and custody

**Date:** 2026-10-17 UTC. **Protocol:** [PROTOCOL.md](PROTOCOL.md),
committed as `246a7ed01b` before construction. The separate
[peak-recovery protocol](PEAK_RECOVERY_PROTOCOL.md) was committed as part
of `8fa210d09e` before the repeat. This report describes base construction,
generic replay and durable storage only. Natural relation queries, useful
rank, target extraction and complete IC cost were not run here. This is
branch-local evidence pending PR review; no accepted pipeline row follows.

## Exact recipe and baseline

The primary curve is `icv1-f2m83-tm6151469093347-debefd74`: `a=0`,
`b=1` over `GF(2^83)` with low modulus terms `[0,1,2,45]`. The existing
native exporter selected public x candidates with `public_x_hash`, seed
`2026100801`, projected accepted points into the pinned subgroup and
expanded each representative through all 83 Frobenius phases and both signs.
No target or private scalar entered construction. The exporter source
BLAKE3 is `c0a1eb70455a4fcc856c2b951de20b941b0ef76c215f2fe783bb32622a6ac9a2`;
the Linux arm64 release binary SHA-256 is
`c40614fab6b73e59e5f29fc57d7ef53680be3de962a902a26eb25a6c12c9257b`.
The Linux build and 19/19 example tests passed under the pinned offline
image; see [build receipt](BUILD_RECEIPT.md).

The previously verified primary K=1,182 stored object is the construction
baseline named in the protocol. This K=16,627 run tests a larger declared
size and has no matched same-host speed comparison against that baseline.
The exact cardinality screen in
[SIZE_FRONTIER_V2.md](../koblitz_n83_factor_base_sweep_20261008/SIZE_FRONTIER_V2.md)
first makes the uniform nonidentity-target four-summand multiset ceiling
uninformative at K=16,627. That is a size threshold, not relation yield or
a claim that the constructed base is optimal.

## Observations and custody

| Phase | Observation | Hard envelope / outcome |
| --- | --- | --- |
| First construction | 16,627 columns, 2,760,082 point records, 33,120 public x candidates scanned; 34.164 s internal L0 wall | one CPU, 4 GiB, zero swap, 3,600 s worker and 3,900 s outer cap; exit 0, no OOM; peak not captured |
| Generic replay | all 16,627 representatives and 2,760,082 point records passed; 31.134 s container wall | one CPU, 4 GiB, zero swap; cgroup peak 834,940,928 bytes; exit 0, no OOM |
| S3 custody | upload, download and compressed BLAKE3 comparison passed | [storage receipt](upload-receipt.json) `PASS`; retrievable versioned object below |
| Precommitted peak recovery | 2,760,082 records, byte-identical object; 46.440 s internal L0 wall | same one-CPU/4-GiB/zero-swap caps; cgroup peak 507,568,128 bytes; exit 0, no OOM |

The first [manifest](manifest.json) gives **83,701,045 compressed bytes**,
compressed BLAKE3
`1aa70cc1571f4cb25af42936699378329128a869f2ae8ad35efb375663d4e59d`,
plain BLAKE3
`b556e7039f3fc397602a4889d514905e171451dad9de312484adcef68c604957`,
and sorted point-set BLAKE3
`564c6bb72b346b73112240bb0bddc4ef68b7525f7ef2556d6046e52c32f976e0`.
Compressed SHA-256 is
`2457a977ea9c05a11cc60878bdb3b29f6d905dc8db9aa26e52913a702d308369`.
The [generic replay](replay.json) used multi-limb `BinaryCurve` rather
than the producer's `Gf2_128`; it was on the same host and repository, so
an external independent replay is still open. Its cgroup peak is in
[replay.peak.bytes](replay.peak.bytes) and the clean container outcome in
[replay.container-state.json](replay.container-state.json).

The native upload routine required the matching replay receipt, checked
the local object, uploaded it to the manifest's versioned prefix, downloaded
it and matched the compressed digest. The durable object is:

```text
s3://crypto-autoresearcher/factor-bases/icv1/etc/koblitz_n83_factor_base_sweep_20261008/v2-size-frontier/a0/objects/1aa70cc1571f4cb25af42936699378329128a869f2ae8ad35efb375663d4e59d.jsonl.gz
```

The [repeat manifest](repeat-manifest.json) binds the same exporter bytes at
later commit `8fa210d09e`. `cmp` returned exit 0 on the two 83,701,045-byte
objects, and the compressed, plain, point-set, count and length fields
matched exactly. Its [peak receipt](repeat-peak.bytes) and
[container state](repeat.container-state.json) apply to that repeat only;
they do not retroactively fill in the first run's missing peak. The first
[container state](construct.container-state.json) and raw construction logs
remain separate. Both walls are shared-host L0 observations; other work ran
on the host, and their ratio is not a speed claim. S3 transfer wall and
bandwidth were not captured, so a charged cold-storage cost is still open.

## Interpretation and next measurement

This is a verified construction, replay and durable-storage result for a
large symmetric base. It demonstrates ahead-of-time reuse of one exact
object. Its 16,627 columns also create a rank and linear-algebra obligation
which this run did not measure. The result does not choose the best base or
alter the current IC baseline, matched-rho gap, or selected total runtime.
No same-host equal-output external constructor comparison was run, so no
fastest-constructor claim is made.

The [diagram](construction-diagram.svg) links the phases and open work. The
next gate is a native full-width m=4 adapter capacity test for this exact
object, with a degree-83 planted relation positive control. Then compare
natural query outcomes and useful rank against a retained near-size explicit
base under a frozen common corpus, A/A controls and complete cold charges.
The canonical IC scoreboard and progress charts remain unchanged because
this run added no verified relation, rank or end-to-end pipeline row.
