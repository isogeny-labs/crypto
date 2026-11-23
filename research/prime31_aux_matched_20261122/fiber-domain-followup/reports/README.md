# Fiber-domain visual supplement

Six-page tutorial source. It supplements the frozen sparse-H2 report without
changing that earlier document or any native source/evidence.

`build_report.py` reads parent-supplied `report-data.json`; the adjacent example
defines its schema. Do not replace missing test or campaign counts with zero.
The script stages `render-data.json`, writes exact SVG illustrations with
`make_figures.py`, and renders `REPORT.typ` through the existing Typst Python
module. These scripts perform document layout only.

Render a pending layout without a qualification receipt:

```sh
python3 research/prime31_aux_matched_20261122/fiber-domain-followup/reports/build_report.py --allow-pending
```

After the parent supplies `report-data.json` with source-bound counts and status:

```sh
python3 research/prime31_aux_matched_20261122/fiber-domain-followup/reports/build_report.py
```

Inspect all six rendered pages with Poppler or a PDF renderer before delivery.
The data gate intentionally rejects an assertion of established end-to-end
advantage: this prose would need a separately reviewed revision supported by
complete-cold execution evidence. Mathematical component counts are explicitly
source-derived and must not be relabeled as measured speedups.

The path-reuse refactor removes the former second map traversal. Page 3 now
shows one validation cost C plus separately charged index construction and
temporary path storage. Its optional `path_reuse_qualification_status` metadata
field must describe the fresh source-bound qualification; without that field,
the page explicitly marks this later refactor pending. Earlier 103-test evidence
remains evidence for its original source commit and is not silently reassigned
to the refactor. Only the parent should finalize counts, source, PR and status.
