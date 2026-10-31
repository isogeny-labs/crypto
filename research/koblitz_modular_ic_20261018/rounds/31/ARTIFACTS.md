# Immutable raw artifact

.artifacts.json points to runs.tar.xz in setec-astronomy-artifacts/store.
SHA2569a421aa1ebf7395c550e7a362a851f42e238ba7ea3f56099eec0c726eb10c705,
425776bytes,102 regular files. It contains all seven native runs, the Lean
compile/audit, frozen source snapshots, generated inputs, original paired
inputs/Phi11 data, and initial/final formatting logs. No secrets are included.

Fresh GetObject to /var/tmp/koblitz-r31-fresh-download.tar.xz, safe extraction,
every expected member hash and deterministic GNU tar repack pass. This is
artifact integrity; no independent mathematical replay or geometry proof.
See artifact-verification.json. raw-meta.json correctly retains the earlier
fresh_download_verified=false sealing state; the later receipt records success.

Scientific source revision548ab0b10eb3b0cb8c93e207c789eddefed80281 has38
source pins. Native/proof runs retain their own prospective source revisions.
The source files and archive member files have separate complete hash lists.
Proof dependency roots are retained, with mathlibd13f23b and Lean4.34.1.
The exact native source before formatting is in cm31-01/source; final source
formatting changes no arithmetic. All original failed sources/archives remain.

Restore using the repository artifact CLI and extract inside this round.
Use new attempt names for any authorized replay. Existing managed jobs are
terminal and must not be restarted merely to reproduce this package.
