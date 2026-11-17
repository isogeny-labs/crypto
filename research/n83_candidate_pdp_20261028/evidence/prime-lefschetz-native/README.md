# Native source-bound prime first-fall certificates

Run `n83-prime-lefschetz-native-20261030-r3` completed all four phases with exit0: native source audit, native certificate production, fresh native source replay and fresh native certificate replay. All42cases (original/baseline/spike40 × m3–9 × planted/natural) are present. The raw archive was downloaded from S3; both native replays passed again and all208payload hashes matched before and after replay.

Every saved row reports first_fall_lower_bound=17, first_fall_upper_bound=17 and first_fall_degree=17, CONDITIONAL on the written Lefschetz/Koszul argument and its external SLP theorem. Independent scientific review and formalization remain pending. These are source-bound arithmetic certificates, not42independent Gröbner computations or a completed formal proof. Seven leading gates and one structure/witness certificate are shared. No new LFD, solving-degree, degree-of-regularity, arbitrary-factor-base or elite-base improvement is established. The statement concerns the exact saved polynomial presentations; it is not a general ECDLP attack result.

Native source auditing checks every input/source/base identity, complete source template, field, weights, normalization inverses and planted witness where present. The certificate binds each prior upper17 row to matching source/base identity, verifies block dimension and no-truncation arithmetic, leading-term conditions and the explicit8192term upper witness. The complete witness is in `results/upper-witness.json` within the raw archive; its compact metadata and SHA256 are adjacent here. All raw inputs, source snapshots, coefficients, individual case records, native binaries, logs and phase statuses are preserved.

Protocol v3: `../../protocol-prime-lefschetz-native-local.json`, frozen355e8228ed4ad5c85cc210e18c35ec932338a3785fdb9b42024746dbb6671133. It explicitly amended the backend only after the EKS r2 scheduling deadline; r2 remains a retained infrastructure failure with zero raw objects. No retry occurred under v2. Same native binaries and all42source/input/base hashes; four120s phases,2GiB address space,720s outer cap,zero retries. Batch timestamps are diagnostic only (wall_valid=false); no speed comparison is made.

Storage: `../../prime-lefschetz-native-production-archives/.artifacts.json`, entry r3-raw.tar.gz. SHA25634255f1d3aba276141a286952570f0ced90fbdd8a04a46d9faaf922afeed4be4;779736bytes;208payloads. Structure/encoding: `../../PRIME_LEFSCHETZ_NATIVE_RUN_FORMAT.json`. The source archive and failed EKS outcome archive are retained beside it.

Replay after restoring the raw archive to ROOT:

```
N83_LEFSCHETZ_FROZEN_PRODUCTION=1 ROOT/instrument replay ROOT ROOT/protocol-prime-lefschetz-native.json ROOT/source-audit
N83_LEFSCHETZ_FROZEN_PRODUCTION=1 ROOT/certificate-instrument replay ROOT ROOT/protocol-prime-lefschetz-native.json ROOT/results
```

Run with the frozen120s/2GiB limits. Verify manifest.sha256 before and after. Overall Koblitz/general-binary/prime m2–9 FFD/LFD and elite-factor-base work remains open.
