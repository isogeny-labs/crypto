# Patched reproduction of archived reports

Rounds 07, 09 and 10 of `research/koblitz_modular_ic_20261018` now select
PyMuPDF 1.26.7 in their active report requirements; Typst remains at 0.15.0.
The historical PyMuPDF 1.26.5 requirements bytes, original hashes and source
revision are preserved in `tools/report_reproduction_v1.json`. No archived
PDF, image, Typst source, ledger, protocol, native replay or run archive changes.
This follows the isolated round-08 upgrade documented in
`ci-repair-report-dependencies-20261027.md`.

From the repository root, in a fresh virtual environment:

```sh
python -m pip install -r research/koblitz_modular_ic_20261018/rounds/07/report-requirements.txt
python tools/reproduce_archived_reports_v1.py --output /absolute/new/report-reproduction
```

The output parent must exist. The command rejects an existing destination,
repository destinations, and symlink destinations or ancestors. Use `--round 07`,
`--round 09` or `--round 10` to select one report. It reads pinned committed
plaintext through the required Git filters, checks input SHA-256 values, and
runs byte-identical copies of each builder and its three data/prose inputs in
a fresh `study/rounds/<round>` tree. Rendering times out after 120 seconds per
round. It neither invokes `REPLAY.sh` nor launches scientific trials.

`REPRODUCTION.json` records actual Python/package versions, input and output
hashes, and comparisons. The command requires exact SVG and Typst source,
page count and per-page extracted text. It also records pixel equality and
PDF-byte equality without assuming them: PDF metadata can change while pages
remain identical. Preserve new outputs separately from the frozen evidence.

Local validation on 2026-11-15 used Python 3.13.1, Typst 0.15.0 and PyMuPDF
1.26.7. The three reports had 4, 4 and 5 pages. Generated SVG and Typst sources,
per-page text, page counts and all rendered page pixels matched their frozen
counterparts; PDF container bytes differed. All 13 pages were visually reviewed.
Four isolation/custody controls passed. A separate temporary attachment fixture
reproduced relative traversal, absolute traversal and overwriting with 1.26.5;
1.26.7 rejected all three by default while ordinary extraction still passed.
Explicit upstream `-output` and `-unsafe` options intentionally permit selected
destinations; this patch does not redefine those upstream APIs.

The inspected builders use PyMuPDF's rendering library, not its attachment CLI.
The affected CLI was nevertheless installed by the previous requirements.
Upstream sources: [advisory](https://github.com/advisories/GHSA-cxqh-p2w9-fmr7)
and [patch](https://github.com/pymupdf/PyMuPDF/commit/603cafe38a183b8bab34f16d05043b4185d8d40a).
This is report reproduction and dependency validation, with no new scientific
measurement or performance claim. The filesystem isolation controls assume a
trusted local parent directory; they are not a sandbox against concurrent
hostile filesystem mutation.
