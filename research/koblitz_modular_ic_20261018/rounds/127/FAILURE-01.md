# R127 negative-control fixture setup failure

The first changed-source test copied R127 into `<tmp>/127` and the donor into `<tmp>/51`. The producer resolves its protocol path relative to the study root, so it looked for `<tmp>/rounds/51/...`; it exited on a missing file. The test's assertion that this was the intended hash rejection failed. No result was accepted from that attempt. The corrected fixture copied both into `<tmp>/study/rounds/{127,51}`; after one byte was appended to the donor, the producer raised its source-hash assertion and wrote no `result.json`.
