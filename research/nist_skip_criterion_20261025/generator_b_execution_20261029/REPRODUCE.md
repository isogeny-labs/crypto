# Reproduce and verify the completed finite comparison

The actual launch used run-v2.sh under frozen protocol-v2.json at source
1b18bec823d730ef8f30ef0761595cac633c411e. Do not rerun a failed attempt
in place. A new scientific execution requires a fresh admitted protocol,
checkpoint, output directory and source/binary bindings.

## Hydrate the saved raw files

From the repository root, with an authorized artifact-store client:

```sh
python3 tools/artifacts/artifacts.py pull research/nist_skip_criterion_20261025/generator_b_execution_20261029/evidence
cd research/nist_skip_criterion_20261025/generator_b_execution_20261029
sha256sum -c evidence/full-raw-002.tar.xz.sha256
tar -xJf evidence/full-raw-002.tar.xz --wildcards 'runs/*'
```

The deterministic archive also has MEMBERS.json, pinned native sources,
inputs and exact executed binaries. Check every regular member against that
manifest in a fresh scratch extraction; never overlay unrelated source files.
The final delivery receipt records the actual remote readback and 106 checked
members. Extracted runs/ is intentionally ignored by Git. Do not replace
textual NA with zero or remove P-384/P-521 controls.

Use the canonical sibling tool (crypto-autoresearcher revision
8c5db250d4fff65eb40fffede9c8391de403897f), not a copied local schema:

```sh
python3 /workspace/crypto-autoresearcher/tools/experiment_protocol.py check-export export.json --protocol protocol-v2.json --verify-files
```

Inspect the tool's --help if its path/version differs. The saved invocation
and zero-findings output are in evidence/export-v2-admission.txt. Version 1
must remain refused with its real shortfall: verification was never attempted.

## Native recomputation

The prepared sibling's source.sha256, input.sha256 and binaries.sha256 pin
the original producer and separate GMP verifier. Build dependencies are
C++17, PARI 2.17.3, GMP 6.3.0 and the recorded OpenSSL; library files are
pinned in plan-v2.json. The full archive contains the exact original binaries
but not those libraries. Do not describe a different rebuild as the executed
binary. Read the prepared build-controls.sh and actual run-v2.sh before any
fresh authorized replay. The original primary science path is native; Python
here is canonical metadata checking or document layout only.

## Render the reports

```sh
python3 render.py
```

This uses installed Matplotlib, ReportLab and PyMuPDF with DejaVu fonts. It
plots saved native records and lays out prose; it does not perform curve
arithmetic, search or timing. SVG and 150-dpi PNG copies share the plotting
source. Inspect every generated PDF page. The planning PDF/figure is copied
to benchmarks/20261029-generator-preflight/ by the same renderer; it is an
unadmitted documentary record, not an admitted benchmark row.
