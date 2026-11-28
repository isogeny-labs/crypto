# Bounded packet status

The source-qualified v1 packet completed. Its frozen analyzer verdict is
`INDETERMINATE_CAP`, not an algorithmic pass or an impossibility result.
[pcc:unverified]

- The independently reconstructed formal-target S5 polar has 16,752 nonzero
  sparse coefficients. Its first canonical pair norm reached the prospective
  262,144-slot cap before an output was emitted.
- The explicit quadratic-tower DAG has 221 nodes for the polar, 4,261 after
  one pair norm and 61,309 after two. The second observed state ratio is
  14.388406477. The third norm stopped before exceeding the 262,144-node cap.
- The verifier independently matched the complete S5 coefficient map, checked
  all three emitted DAGs structurally, and matched the common H DAG/polynomial
  cell on nine deterministic assignments. It reported zero errors. The v1
  verifier did not independently reconstruct the two norm DAGs, exceptional
  charts, rational signs or a complete pair query, so coverage remains false.
- `seed_manifest=null`; no random stream or cross-target amortization exists.

`PACKAGE_SHA256SUMS` hashes the stored files. `SHA256SUMS` retains the original
uncompressed candidate-stream digest; decompressing `candidate.txt.gzip`
reproduces that digest. `verify_evidence.sh` rebuilt the frozen verifier and
analyzer and reproduced both plain-text outputs byte-for-byte.
