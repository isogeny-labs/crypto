# Prebuilt N83 K=16,627 signed-Frobenius base

The native public-x-hash exporter constructed a content-addressed primary
`GF(2^83)` factor base with **16,627 orbit columns** and **2,760,082 signed
Frobenius point records**. The compressed object is **83,701,045 bytes**.
Two capped native constructions produced byte-identical objects. The first
took 34.164 seconds of descriptive shared-host internal wall; the second
took 46.440 seconds and measured a 507,568,128-byte cgroup peak under
4 GiB/zero swap. The first construction's peak was not captured.

The separate generic multi-limb replay checked every representative and
point and returned `PASS`, with an 834,940,928-byte cgroup peak. The native
uploader returned `PASS` after a versioned S3 upload/download and compressed
digest match. The [manifest](manifest.json), [replay](replay.json),
[storage receipt](upload-receipt.json), [raw run report](RUN_REPORT.md),
[repeat manifest](repeat-manifest.json) and [diagram](construction-diagram.svg)
bind the result and its limits. The object's BLAKE3 is
`1aa70cc1571f4cb25af42936699378329128a869f2ae8ad35efb375663d4e59d`.

The K=16,627 size makes the uniform-target four-summand counting ceiling
uninformative; it does not imply relation yield. No natural relation,
useful-rank or complete cold-cost result was measured. The next step is a
native m=4 capacity and planted-relation gate on this retained object,
followed by a matched base comparison. The canonical IC scoreboard remains
unchanged.

| Requested dimension | Status after this run |
| --- | --- |
| Construct and reuse an N83 base ahead of queries | verified for this exact object: two byte-identical builds, generic replay and S3 round trip |
| Symmetry | verified structural signed-Frobenius closure with 83 phases and both signs per representative |
| m=4 base quality and cost | open: native adapter capacity, planted relation, natural yield, useful rank and full cold accounting remain |
| m=3 and m=2 quality | not attempted here; the earlier exact support review records their size thresholds, without a new oracle or rank measurement |
