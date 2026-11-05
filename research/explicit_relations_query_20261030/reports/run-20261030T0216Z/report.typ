#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
#set heading(numbering: none)
= RTX PRO 6000 query transfer comparison

This observation report describes a software diagnostic panel. It does not
certify a speedup, promote a research hypothesis, or establish an ECDLP advantage.

*Run:* RunPod `bbrl08j1o8gizz`, created 2026-10-30T02:16:09.784Z. One RTX PRO
6000 Blackwell Server Edition, 97,887 MiB, driver 595.91.07, CUDA 13.2.51,
Rust 1.88.0. Source `68be1081e696c232881b2da8d864d93b297e5092`.
The pod was deleted after collection; controller and validation exited zero.

*Protocol:* EXPLICIT-RELATIONS-QUERY-20261030, frozen before data. Four arms,
three cases, one warmup and nine measured repeats: 120 completed rows,
12 warmups and 108 measured. No failures, timeouts or exclusions.

== Changes and exact accounting
Each successful query now returns a radix-packed right/left witness in one
64-bit atomic result. Overflow checks preserve the sentinel and fall back to
the old match array when necessary. The reset uses device memset in the same
stream as search and readback. Explicit caps, early stop, visited counts and
deterministic witness order are preserved. All arms retain product-tree
normalization; the prior normalization panel is not pooled with this one.

For these encodable fixtures the previous path transfers 16 bytes per batch
plus 8 per hit; packing alone transfers 16 per batch; the new default transfers
8 per batch. Workload payloads previous/default/packing-only are 80/32/64 bytes
for each small case and 800/392/784 bytes for the largest. These derived API
counts exclude preparation, device writes and physical bus framing. The packed
path removes the 32,768-byte match buffer at the explicit 4096-row chunk size.

== Query diagnostic medians
Milliseconds for four queries, n=9 per cell. Bases have 16/64/256 points each;
two planted hits and two independently exhausted misses are checked per row.

#table(columns: (1fr, 1fr, 1fr, 1fr, 1fr), inset: 5pt,
  [Points/base], [Previous u], [Default v], [Packing w], [A/A x],
  [16], [0.161201], [0.138247], [0.146458], [0.163915],
  [64], [0.174400], [0.142262], [0.157365], [0.173539],
  [256], [1.854872], [1.747181], [1.840191], [1.841893],
)

Derived median differences (u-v)/u are 14.24%, 18.43% and 5.81%. These describe
this panel only. All clocks are L0 shared-host diagnostics and all exported
wall_valid flags are false. Case-0 A/A ranges also do not overlap, so disjoint
ranges are not treated as statistical evidence of a timing advantage.

Cold u/v medians were 148.872484/148.824673, 150.270249/149.864362 and
153.363963/151.367678 ms. Candidate case-2 preparation reached 9.680058 ms
and cold reached 241.054600 ms; both are retained. No cold end-to-end
speedup or fastest-tool status is established.

#pagebreak()
== Full query ranges and controls
#image("../../results/latency.svg", width: 100%)

u=previous query, v=packed winner plus device reset, w=packing only,
x=second invocation of the exact u binary. Cases 0/1/2 use 16/64/256 points
per base. Bars show medians and full observed ranges, n=9, from this run.

== Correctness and evidence
All cross-arm witness/visited and normalization digests matched. Each answer
was checked by the independent host backend. Three query variants passed
repeated reset/prepare, duplicate sum, chunk-tail, empty-plan and overflow
checks. Host/CUDA service, affine replay, bounded paper reduction/certificate
tests, 10 Rust tests and Clippy passed. Memcheck: zero errors; racecheck: zero
hazards/errors/warnings; synccheck: zero errors.

The canonical results/export.json passes file-verifying validation with zero
findings. Raw passed labels map to schema verified only in metadata. Native
analysis sees neutral labels; human blinding is not claimed. Planning runtime
was an allocation estimate, and round zero preserves actual preflight costs.

Queries include transfers, synchronization and witness verification. Prepare
includes allocation, table build and sort. Cold includes input generation,
GPU context creation, prepare and queries. Independent host comparison is
validation overhead outside these intervals. No root plug-in build was done
locally. The paper driver still has p<=65521 limits; general paper reduction
and a GPU tensor backend are outside this change. This is generic decomposition.

Raw archive: `s3://setec-astronomy-artifacts/validation/ecbench-explicit-x/runpod-bbrl08j1o8gizz/`.
Upload/download SHA-256 matched for results.tgz:
#text(size: 8pt)[`7de347dd22d60b6989e2efc9c6f26090973d6f82dd50834e6ab01aa5569e9afd`].
The next target is repeated right-side tuple construction with explicitly
bounded caching and full preparation accounting, under a new protocol.
