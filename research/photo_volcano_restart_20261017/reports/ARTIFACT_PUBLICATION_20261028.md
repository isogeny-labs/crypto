# Previously inspected report exports: durable publication

The four PDFs below are exact copies of the corrected reports inspected on
October 25. Their source HTML hashes match the already published report HTML.
The October 26 owner policy keeps PDFs directly in Git. The eight full-size
heatmap PNGs were uploaded through `tools/artifacts` to the shared encrypted
content-addressed store on October 28; every object was independently downloaded
and checked against its SHA-256 and byte count. No scalar witnesses, raw RNG
journals or credentials are included.

| Report | Results PDF | Run PDF | Heatmaps |
| --- | --- | --- | --- |
| Completed graph and regularity certificates | [3 pages](completion_20261025/RESULTS_REPORT.pdf) | [5 pages](completion_20261025/RUN_REPORT.pdf) | Certificate CPU time/products; random/control ideal regularity |
| Corrected native field counters | [3 pages](field_complete_20261025_corrected/RESULTS_REPORT.pdf) | [5 pages](field_complete_20261025_corrected/RUN_REPORT.pdf) | Rho products/inversions; BSGS query/setup products |

For exact paths, digests, sizes, encryption and verified readback status, use the
[publication receipt](ARTIFACT_PUBLICATION_20261028.json). Each PNG's directory
contains its standard `.artifacts.json`; hydrate them from the repository root:

```sh
python3 tools/artifacts/artifacts.py pull research/photo_volcano_restart_20261017/reports/completion_20261025 research/photo_volcano_restart_20261017/reports/field_complete_20261025_corrected
python3 tools/artifacts/artifacts.py verify --remote research/photo_volcano_restart_20261017/reports/completion_20261025 research/photo_volcano_restart_20261017/reports/field_complete_20261025_corrected
```

These exports add access to existing evidence. They change no numerical result,
counter convention, measurement admission or ECDLP completion count. The scope
remains the selected rational degree-2 component: 46 vertices, 90 directed maps,
five levels. All 4,600 rho and 4,600 BSGS solves are verified; original-corpus IC
recoveries remain zero. The existing `inspection.json` files retain their dated
pre-publication local-only status; this receipt supersedes that storage status.

The next IC pilot has a verified source-only S3 checkpoint, but its AWS bridge
request was denied `codebuild:StartBuild`; a tested custom image and qualified
collector recovery are also still required. See the separate
[current launch status](../../volcano_ic_recovery_20261027/LAUNCH_STATUS.md).
