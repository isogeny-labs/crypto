# Round25: exact original quadratic quotient

The original source involution and a complete quotient presentation on the
registered b-unit affine open are now explicit. See [CM.md](CM.md),
[full quotient equations](registered-quotient.sing),
[generic invariant coordinates](quotient-coordinates.sing),
[results](results.json), [report](REPORT.md), [PDF](report.pdf),
[proof scope](PROOF_MAP.md), [failures](FAILURES.md) and [next checks](NEXT.md).
The desired order11 modular-factor action and proper polarized embedding
are still unverified; no cost or relation-rank change is reported.

[PROTOCOL.md](PROTOCOL.md) was committed before data with prospective
implementation/registered-chart amendments. Its machine index is the
inherited custom schema in protocol.json; the generic cross-repository
checker is unavailable. Native cap4GiB virtual/1200s, no EKS job or new claim.

Hydrate runs.tar.xz through tools/artifacts/artifacts.py pull, then extract
it here and run sha256sum -c runs.tar.xz.sha256 and sha256sum -c runs.sha256.
VERIFY_RETAINED.sh checks the retained source hashes, complete successful
blocks, failed diagnostics, control and proof audit without launching trials.
source.sha256 binds the final science source frame named by source-revision.txt.
A fresh S3 download, byte comparison and extraction verification are retained.

For a genuinely new, authorized native replay, first reconcile ownership,
read the protocol and push a current scripts/checkpoint.sh checkpoint.
REPLAY_NATIVE.sh requires a fresh basename and native Singular4.4.1. Cases
positive,quotient,corrupt are separate. The correct negative-control outcome
is wrapper exit1. REPLAY_LEAN.sh recompiles the separate pinned Lean project
and runs the repository's transitive axiom audit. Its third-party source
roots are inherited from the R22 cache receipt; rehydrate that pinned cache
if the original paths are absent. This does not claim independent review.

/tmp ran out of inodes at the final prelaunch checkpoint. The continuation
used task-owned /var/tmp/koblitz-r25-checkpoint via TMPDIR. The original tiny
committed checkpoint-evidence.tar.xz stays byte-for-byte unchanged in history;
mutable transport is checkpoint-live.tar.xz and is not a final evidence seal.
R24's science archive/source pins are unchanged. Its software validation
archive is separate and explicitly not a green full library result.
