# Publication custody snapshot

Initial read-only snapshot: 2026-10-31 00:24:23–00:24:50 UTC. Scope is followup-continuation-v2 and canonical integration/lean/same-support only.

All 798 included paths in 32 snapshotted custody maps matched their exact SHA256 and byte count. The maps explicitly account for 96 omissions. All 18 artifact-managed included files also matched their local artifact manifest pins. No remote download/readback was performed by this audit; local retained bytes and recorded pins were checked.

Two forest packets appeared additively during the audit. A separate addendum checked their 12 included files, all matching. Existing custody-map hashes did not change during the initial snapshot. Later additions remain outside these dated observations.

All 13 canonical Lean modules match their reviewed producer/replay sources, including Constructor. All 35 README/INDEX/index.json link occurrences resolve. README explicitly says root typed tests are in progress; no current-final-Rust-check completion claim was found. Earlier STATUS open threads still mention completed graph and recovery work; append current status for clarity without changing frozen evidence.

One publication-size action was identified: packets/geometric-cycle-obstruction/archive.jsonl (568,983 bytes) lacked a local artifact manifest at the initial snapshot. Apply repository store policy before staging it. Other oversized hits are relation-adapter/target build cache; omit those from publication, not artifact-store them. git check-ignore returned1 for the checked release binary path, so stage explicit paths and exclude this cache.

Files: snapshot.json binds the initial maps; included-checks.json lists every initial included-file check; audit.json records links, canonical comparisons and raw size findings; addendum.json resolves the final canonical source and additive forest packets. No root edits, builds, external jobs or scientific runs occurred.
