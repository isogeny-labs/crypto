# Signed target witnesses

REPORT.md/report.pdf present the results; PROOF.md states the exact certificate implications. systems.csv includes all 64 systems. certificate-manifest.json binds the 64 artifacts in certificates.tar.gz; each artifact contains the entire matrix, row-event origins and either weights plus a particular solution, or a solution/null direction.

Hydrate archived inputs using the repository artifact tool on this directory and ../signed-native-restarts. Run `python3 -B check_only.py` here to verify source rows, certificates and mutation controls without importing elimination or invoking a collector. run.py is the producer and is unnecessary for checking certificates. check_certificate.py is the small direct-sum checker; certificates.py is the separate elimination implementation.

conditional-v1 preserves the earlier conditional-uniqueness certificate set and source; current certificates include explicit consistency witnesses. Its archived scripts retain their original relative-path assumptions and are historical inputs, not the current checker. CONSISTENCY-AMENDMENT.md documents the strengthening.

The initial manifest-addressing failure is preserved under initial-*; PRECHECK-CORRECTION.md records the bounded correction. All native capped outcomes remain in the record. The new certificate algorithms are separate from native graph elimination, but independent source review remains pending. No new cohort or cost claim is made.
