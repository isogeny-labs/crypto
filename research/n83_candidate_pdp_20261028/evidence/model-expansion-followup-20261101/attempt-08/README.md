# Ten-base exploratory PDP model extension, attempt 08

This post-label exploratory follow-up extends the existing spike27-spike33 panel with fixed 2^27 bases spike35-spike37. Protocol v5 is `protocol-model-full-domain-v5.json` (SHA-256 `3a041db64c7d6d70aa99200fe46500770998de5faffaf2acb7a2e366c855eb57`). New inputs and labels have not yet been produced.

Attempt 07 is preserved as a failed metadata preflight: it rejected multiple identical recovered copies of an r36 manifest. Attempt 08 accepts multiple copies only when the expected digest matches. The preflight binds all runners, binaries, prior labels, factor-base indexes and family manifests.

Planned sequence: prepare metadata, generate 144 matched-selector inputs, produce and independently replay 144 top-form interval certificates, then train grouped leave-one-base-out ridge over all ten bases. This is exploratory. Labels are finite top-form interval midpoint proxies/statuses, not exact regularity, FFD, LFD, solving degree or ECDLP cost.
