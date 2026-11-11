#set page(paper: "a4", margin: (x: 18mm, y: 16mm))
#set text(font: "DejaVu Sans", size: 9pt, fill: rgb("17202a"))
#set heading(numbering: none)
#set par(justify: true, leading: 0.55em)

#text(size: 18pt, weight: "bold")[Root cache snapshot renewal]
#text(size: 11pt, fill: rgb("52606d"))[Terminal evidence packet · 2026-11-11]

= Result

The bounded snapshot attempt reached a terminal *passed* receipt. Native,
controller, and caller elapsed times were 3.1129651480005123 s,
3.461129906005226 s, and 3.629120421013795 s. The systemd result is `success`,
the execution status is 0, and the owned cgroup was empty or absent at terminal
observation.

The attempt charged 60 worker-seconds at 1 CPU and 1 GiB, with swap disabled
and `TasksMax=32`. Campaign accounting is 3,780 / 129,600 worker-seconds after
the attempt, leaving 125,820. The preserved receipts contain no dollar-cost
value; the standardized dollar-cost field is null.

#image("timing.svg", width: 100%)

#pagebreak()

= Exact cache-manifest receipt

#table(
  columns: (31%, 69%),
  stroke: rgb("d5d9dd"),
  inset: 5pt,
  [*Field*], [*Recorded value*],
  [SHA-256], [`9eae2a2fc003071692c5058e52b6e712f2d3ee0ad87e5c2086bc08240479c850`],
  [Receipt size], [593,986 bytes],
  [Inventory], [2,165 files in 415 directories],
  [Aggregate file bytes], [2,618,767,578],
  [Initial manifest SHA-256], [`c5178d2138d79f1ea8cac1fc96cb8d968913ba403d68ee66fa36afdd60fdf3f2`],
)

The cache roots are local physical paths under
`/var/tmp/pencil-root-materialize-t1953-20261109/cache-stage-v2/`, recorded as
reproduced from published sources. This packet preserves the exact 593,986-byte
manifest and small receipts. It does not contain or claim upload of the
2,618,767,578-byte cache body.

= Comparison to initial target-only manifest

#table(
  columns: (25%, 25%, 25%, 25%),
  stroke: rgb("d5d9dd"),
  inset: 5pt,
  align: center,
  [*Added*], [*Changed*], [*Missing*], [*Unchanged*],
  [33], [1,395], [1], [737],
)

The raw comparison receipt states that additions in other mutable roots are not
attributed solely to the failed compiler. The snapshot result records
`compiler_invoked=false`.

= Remote-publication readbacks

- Pre-execution binding: commit = remote head
  `36b55a2fb224720fd043b5728cb15c84f79646d1`.
- Caller observation: commit = remote head
  `e5e262cafa7f05c1c5fa7769d999447fc4bc69d5`.

These readbacks bind source publication at their recorded observation times;
they do not assert publication of the cache body.

= Review map

Machine-readable rows are in `metrics.csv` and `summary.json`. Editable graph
inputs are `graph-data.csv` and `make_graph.py`. Exact receipts are under
`raw/`; `raw-receipts.tar.gz` contains only those receipt files. Integrity is
indexed by `MANIFEST.json` and `SHA256SUMS`.
