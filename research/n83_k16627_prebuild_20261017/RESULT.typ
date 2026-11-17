#set page(paper: "a4", margin: 17mm)
#set text(font: "New Computer Modern", size: 10pt)
= Stored N83 K = 16,627 signed-Frobenius base

*2026-10-17 UTC · Primary a = 0 curve · Native public-x-hash recipe*

#image("construction-diagram.svg", width: 100%)

The exporter constructed one 16,627-column base with 2,760,082 signed
Frobenius point records in an 83,701,045-byte compressed object. Its
first descriptive construction wall was 34.164 seconds. A preregistered
repeat produced byte-identical bytes in 46.440 seconds and measured a
507,568,128-byte cgroup peak. The first run's peak is missing.

The generic multi-limb replay checked every representative and point and
passed; its cgroup peak was 834,940,928 bytes. The native uploader passed
an S3 upload/download and compressed digest check. The exact object BLAKE3
is `1aa70cc1571f4cb25af42936699378329128a869f2ae8ad35efb375663d4e59d`.
The manifests, replay, storage and container receipts are linked in
`RUN_REPORT.md`.

The size was chosen from the m = 4 cardinality threshold. Its construction
does not measure natural relation yield, useful rank, individual logarithms
or complete cold IC cost. The next experiment gates the full-width m = 4
adapter and a degree-83 planted relation on this retained object. No
factor-base winner or IC baseline change is selected.
