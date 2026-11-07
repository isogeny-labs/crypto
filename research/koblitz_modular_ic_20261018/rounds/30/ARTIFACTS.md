# Raw artifact publication

The standard .artifacts.json points to runs.tar.xz in
s3://setec-astronomy-artifacts/store. SHA256:

    bd4e569b4c21fd5d6537ca31442879320acf47425230af5d17ea1c10dee92017

The archive is25600bytes and contains102 regular files: all five native and
two proof runs, original/generated source-data, and the publication-attempts
listed in PUBLICATION_RECONCILIATION.md. Fresh GetObject to a separate /var/tmp
path, every member hash and deterministic GNU tar repack all pass. This is
artifact integrity, not independent mathematical replay. See artifact-verification.json.

The38 scientific source pins bind revision
e626cb9ced1881fc348568241bce1d228e627a1b. Individual runs retain their own
prospective source revisions. source.sha256 validates all listed files;
runs.sha256 validates archive members. raw-meta.json records fresh_download=false
AT SEALING; artifact-verification.json records the subsequent successful fresh
verification. No frozen bytes are rewritten to erase that stage distinction.

Restore with the repository artifact CLI for runs.tar.xz, then extract the tar
archive inside this round directory. R30's replay scripts use fresh attempt
names and durable checkpoint gates. Existing proof dependencies are pinned in
the retained R22 cache roots and earlier archives. Never overwrite raw runs.

Four R29 recorder software logs are now durably included and match their
previously published hashes. The two isolated test source crates and initial
R30 failed renderer input/log are included too. This corrects the earlier
claim that ignored .resume scratch logs were part of the checkpoint.
