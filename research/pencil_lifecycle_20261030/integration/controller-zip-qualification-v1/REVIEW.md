# Offline controller ZIP qualification

The ZIP contains the qualified final static controller `b0403250273d2e06e58b18a745f221f2c8d9881979afbe027fdb5f8c237628a6` and the support files required by the retained `deploy-controller` recipe. Packaging used its exact SHA256SUMS and `zip -q -X -r` commands; the deployment script itself was not run.

The existing ZIP `0bd84d4009064968365f03518adbca1bf6fd6c27eeed24721721e20018a9e85a` contains the older controller `ac7d5713…`; it was preserved. Its MCP helper and kubectl bytes were reused after its complete local manifest passed. Their provenance remains the inherited package; this operation does not establish a new source-to-binary build mapping for those tools.

The three retained scripts/config files absent from the sparse working directory were read from tracked HEAD and compared with the retained source. Current submit-job is byte-identical to its inherited version. `repository-head.txt`, `payload-files.json` and `MANIFEST.json` provide exact mappings; the static controller qualification and complete source manifest are referenced by hash and remain in the adjacent controller-progress-static-qualification-v1 directory.

`unzip -t` passed. A fresh extraction passed all seven SHA256SUMS entries. The extracted controller matches the qualified binary byte-for-byte, and all six executable files retain mode0755. The archive has exactly12 entries: four directories, seven payload files and SHA256SUMS. Directory permissions match the private packaging environment; no symlink entries exist.

No builds, binary execution, cloud operations, image/runtime assembly or deployment occurred. The earlier publication approval identified different hashes and does not authorize publication of this new ZIP. This is an offline review artifact only. No claim of ZIP byte-reproducibility across timestamps is made; archive metadata is retained in archive-modes.log.
