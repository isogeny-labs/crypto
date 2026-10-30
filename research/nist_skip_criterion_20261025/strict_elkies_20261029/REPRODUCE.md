# Reproduce the strict-count evidence

The actual run is complete. All six phase outcomes are in export.json.
Source 043a568661ebf1c40a4ead333889d814c32d045c contains the scientific
programs; actual frozen launch source is077c41e7d414cf72faf20bf9b474b7c35dae7bd8.
Do not silently rerun failed controls or overwrite a prior run directory.
Fresh scientific execution requires a new admitted/frozen protocol and
successful uploaded checkpoint.

From the repository root, hydrate stored evidence using an authorized client:

```sh
python3 tools/artifacts/artifacts.py pull research/nist_skip_criterion_20261025/strict_elkies_20261029/evidence
cd research/nist_skip_criterion_20261025/strict_elkies_20261029
sha256sum -c evidence/runs.tar.xz.sha256
tar -xJf evidence/runs.tar.xz runs/attempt-001
python3 /workspace/crypto-autoresearcher/tools/experiment_protocol.py check-export export.json --protocol protocol.json --verify-files
```

The canonical tool is pinned at 8c5db250d4fff65eb40fffede9c8391de403897f.
The archive's MEMBERS.json binds every source, input, binary and raw member.
Verify it in a fresh extraction before treating restored data as evidence.
Individual runs/ paths are deliberately ignored by Git. Libraries are pinned
in BUILD.json rather than bundled. Native GMP/PARI programs are the science
path; Python is used only by the canonical metadata checker and renderer.

build.sh reproduces the strict C++17 compilation commands. A different build
does not automatically have the executed binary's identity. run.sh records
main exits 0 and expected negative exits 1 explicitly, using a fresh external
output directory, exact bindings and clean native environment. There are no
automatic retries. export.cpp only serializes already verified observations.

render.py lays out the saved native counts and prose. It does not compute
characters, new features or label calibration. It produces SVG and 150-dpi
PNG graphs from the same source and copies the documentary preflight record
to the benchmark browsing directory. Inspect every final PDF page. The
criterion remains empirical; reproducing it does not establish historical use.
