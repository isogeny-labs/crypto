#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
#set heading(numbering: none)
= RTX PRO 6000 normalization comparison

This is an observation report for a software diagnostic panel. It does not
certify a speedup, promote a research hypothesis, or establish an ECDLP advantage.

*Run:* RunPod pod `1lq0cd2l0en5ju`; full RTX PRO 6000 Blackwell Server Edition,
97,887 MiB; driver 595.91.07, CUDA 13.2.51, Rust 1.88.0. The pod was deleted
after successful collection. No continuing GPU workload remains.

*Source:* `47ff700720f3563ed67eb1bb9e507db8c252c1c7`.

*Protocol:* EXPLICIT-RELATIONS-NORMALIZE-20261030, frozen before data.
Four arms, three fixed instances, one warmup and nine measured repeats:
120 completed rows, 12 warmups and 108 measured. No failures or exclusions.

== Exact work and validation
The product tree performs 765 source-level field multiplications per block
in inversion preparation, versus 4608 for the old dual scan: a derived
83.3984375% reduction in this operation category. Both additionally pay one
field inversion and the same affine conversions. This is not a total-GAE,
machine-instruction or runtime reduction.

All variants matched independently checked CPU witnesses and normalization
outputs. Host/CUDA service and bounded reduction tests, 10 Rust tests and
Clippy passed. Memcheck: zero errors. Racecheck: zero hazards, errors and
warnings. Synccheck: zero errors. Fixtures covered infinity, partial blocks
and random projective scales including a 61-bit field.

== Recorded normalization medians
Milliseconds, n=9 per arm/cell. Kernel-only CUDA-event timing after warmup.

#table(columns: (1.1fr, 1fr, 1fr, 1fr, 1fr), inset: 5pt,
  [Lanes], [Old scan], [Tree], [CUB], [A/A scan],
  [257], [0.009620], [0.009036], [0.009576], [0.009580],
  [65,537], [0.011332], [0.010652], [0.011168], [0.011496],
  [262,145], [0.021668], [0.018152], [0.024024], [0.021508],
)

For four 256-point bases, four-query medians were 1.809353 ms old scan,
1.765703 ms tree, 1.814083 ms CUB and 1.807224 ms A/A. Old/tree query ranges
overlap: 1.738424–1.815144 ms and 1.707625–1.815973 ms.
Cold-pipeline medians were 143.981243 ms old scan and 145.227285 ms tree.
The candidate did not have a lower cold median in this cell.

*All clocks are L0 shared-host diagnostics; no end-to-end speedup is established.*

#pagebreak()
== Four-query diagnostics
#image("../../results/latency.svg", width: 100%)

Neutral labels: u=old scan, v=product tree, w=NVIDIA CUB BlockScan,
x=second invocation of the exact u binary. Cases 0/1/2 have 16/64/256 points
per base. Bars show medians with full observed ranges; n=9. Raw data and
the frozen native analyzer are included in the same study directory.

== Evidence and limits
The canonical checked record is `results/export-v2.json`. It corrects only
an unobserved unmasking timestamp in the preserved earlier export; raw
values, analyses and hashes are unchanged. The native analyzer consumed
neutral labels, but no human-blinded inference is claimed. Planning time
at freeze was an unmeasured allocation allowance; round 0 preserves actual
preflight costs. No rows were removed for their timing.

The GPU normalization interval excludes fixture construction and readback.
Prepare includes allocation, table build and sort. Queries include transfers,
synchronization and native witness verification. Cold cost includes context
startup, fixture construction, prepare and queries. The independent host
comparison is validation overhead outside those intervals.

Raw archive: `s3://setec-astronomy-artifacts/validation/ecbench-explicit-x/runpod-1lq0cd2l0en5ju/`.
SHA-256 of `results.tgz`:
#text(size: 8pt)[`d83cdcc92e49929fce81413ea1b2d9ebc807ff26093d2963c9735de6f1ff268b`].
Download-back verification matched. No API credential is in the archive.

The source-level count, device tests and stage diagnostics do not establish
full ECDLP speed, wider-field complete reduction, or a GPU tensor backend.
Root crypto/ecbench integration is outside this run. Chunk synchronization
and repeated right-side sums remain follow-up optimization targets.
