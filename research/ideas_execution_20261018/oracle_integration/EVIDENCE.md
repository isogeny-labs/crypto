# Raw evidence package

This workstream is unmerged and unreviewed. Its repository-relative raw
evidence package is `raw-evidence-20261018.tar.gz` (6,320,305 bytes), SHA256
`59bb5ed51227aa373740c6f87abffeae1eb707d3129510e8c2124e070fdc967c`.
The archive contains all 181 original files below `runs/`, including native
sources, source/binary hashes, both complete factorial sessions, replay
receipts, stdout/stderr, return codes, failed builds and expected negative
controls. It also contains `raw-evidence.files.sha256`.

From this study directory, verify and extract with:

```sh
sha256sum -c raw-evidence.archive.sha256
tar -xzf raw-evidence-20261018.tar.gz
sha256sum -c raw-evidence.files.sha256
```

`raw-evidence.archive.members.txt` lists the package contents. The original
unpacked files are retained locally; stage the archive and compact derived
reports rather than also duplicating its full raw tree in Git. A repository
PR must carry this package or record another durable, accessible location
before anyone treats the evidence as delivered. No remote upload, commit
or PR was made by this workstream.

The report PDF and all four SVG/150dpi PNG pairs have separate checksums in
`reports/deliverables.sha256`. Benchmark copies are handled by the campaign's
benchmark workstream. The frozen protocols, `status.json` and `README.md`
list the unmet original criteria; archiving these records does not close
those criteria or admit a performance claim.
