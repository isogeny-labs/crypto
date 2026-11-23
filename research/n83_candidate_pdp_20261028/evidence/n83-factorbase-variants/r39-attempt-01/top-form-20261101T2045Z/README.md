# r39 fixed-base top-form Dreg candidate run

This is the `spike39` candidate arm in frozen protocol `N83-FACTOR-BASE-TOP-DREG-20261031-V2`. It covers all 48 systems: Koblitz, binary and prime families; every `m=2..9`; and natural and planted targets. The complete r39 family bases are reused unchanged across all eight values of `m`.

The measured object is the original highest-homogeneous-form quotient certificate or interval. This arm does not measure affine FFD/LFD, full solving degree, solver cost or ECDLP recovery. The protocol has one attempt per candidate and an 86,400-second outer limit. This is the first r39 top-form attempt; the failed r38 attempt remains preserved under its own prefix.

Before launch, the runner rechecks the frozen protocol binding, source and binary hashes, matched-input hash, candidate-specific S3 prefix emptiness and absence of active PDP/top producers. It stages the compressed factor-base shards from the reconciled immutable r39 S3 file index, verifies every shard size and SHA-256, and writes generated inputs, certificates, fresh-process replay records and summaries to this directory. The factor-base recovery audit and file index are in `../terminal-recovery-20261101T2020Z/`.

The runner synchronizes outputs to the unique S3 prefix every 30 seconds and on exit. It preserves partial output and records the failing shell command on error. Do not rerun this candidate under the same protocol or prefix.
