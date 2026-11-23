# Archive and report reproduction

Hydrate official artifact-store entries before running sha256sum -c manifest.sha256 in this directory. Raw source bytes are preserved; .lean is archived as .lean.txt. Compiled .olean caches and the audit executable are omitted; audit.rs and raw outputs remain. Original review manifest and run scripts refer to the original private paths. For a separately assigned replay, restore source suffixes in a new private tree and preserve compiler/helper identities while adapting paths. Do not execute archived scripts automatically.

encoded-source-map.json records any exact diff bytes stored as base64 solely to preserve significant patch-context trailing whitespace through Git whitespace checks. Decode with base64 -d and verify its recorded original SHA256. An empty map means no encoding was needed. This is not a size-policy exemption; encoded files remain subject to official artifact policy.

The rejected reviewer-attempt1 may contain sorryAx because compilation failed; it is explicitly excluded from accepted proof results. The final successful readbacks and checker acceptance are separate. Render REPORT.md with python3 render-report.py using rsvg-convert and pdflatex and inspect report.pdf. Rendering grants no experiment authorization.
