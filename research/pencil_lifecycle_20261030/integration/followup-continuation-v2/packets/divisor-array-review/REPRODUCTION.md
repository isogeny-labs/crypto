# Reproducing the archive and review

Hydrate any official artifact-store entries using the repository tools/artifacts/artifacts.py pull interface before checking manifest.sha256. Run sha256sum -c manifest.sha256 from this packet directory. The publication manifest binds archived paths; the nested original review manifest refers to the original private paths and names.

The review is completed evidence, not a request to rerun it. For an explicitly admitted replay, restore .lean.txt files as .lean in a new private directory, preserve exact compiler and helper bindings, and adapt original absolute paths in lean-path.txt/run.sh. Compiled .olean files and the native audit executable are excluded; audit.rs and all raw outputs are preserved. Rebuilding all imports or independent binary/kernel checking would be a separate review. Original target source bytes are unchanged despite archive suffixes.

Run python3 render-report.py to render the neutral REPORT.md and flow.svg into report.pdf with rsvg-convert and pdflatex. Inspect the PDF. No execution or resource authorization is supplied by rendering or this archive. Large original binding inventories are stored through the official artifact mechanism without truncation or splitting.

The exact refreshed-main-rules.diff is stored as refreshed-main-rules.diff.base64.txt because unified-diff context-only lines trigger repository whitespace checks. Decode with base64 --decode to a separate private restoration path. Original SHA256: c83c36a4af07b13bfb09e321d949eab7278cd005a2e4ec2987aa6040212986d6. Nested original manifests refer to the decoded bytes; the outer manifest binds the encoded archive. This encoding preserves the original bytes and is not used for size-policy avoidance.
