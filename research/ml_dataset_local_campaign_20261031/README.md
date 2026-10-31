# Prime n23 BSGS 64-cell local campaign

This is a separate fixed-N generation contract for all 64 cells of the native
prime-field n23, arity-2, BSGS matrix: eight curve seeds, two generators, two
target keys and both twist choices. It preserves every terminal outcome. It
does not claim a speed comparison or extend the two-cell local-XFS pilot.

The original pilot plan is bound to the pilot protocol SHA-256 and cannot be
reused by a new protocol. After official freeze, generate a new native plan
with identical axes and the new frozen protocol file SHA-256. The protocol
and plan must be committed before the first generation attempt. The controller
must then pass native preflight, verify the pinned clean binary and source
identity, and run one shard index at a time on local XFS. Each shard gets at
most one solver attempt. Interrupted attempts are retained as censored; a
resume may finalize verification and custody but cannot launch that shard
again.

The per-shard sequence is native generate, independent native verify, native
partial report, strict cyclic-subgroup export, restricted archive, conditional
content-addressed S3 PUT, and fresh GET with byte and hash comparison. A failed
stage is retained and halts advancement until the existing attempt is
reconciled. S3 archives include restricted witness material and must not be
used as public ML features. The strict export separates public feature rows
from restricted labels. The main corpus design remains in
docs/ml-dataset/DESIGN.md; native operation and S3 limits are in
docs/ml-dataset/OPERATIONS.md.

Relevant internal sources checked: the native dataset CLI help and clean
binary at source commit c41bec150e16e4c4f9a00634002b633b821de262;
the two-cell pilot's frozen protocol, outcome receipts and S3 readbacks; the
official experiment protocol checker at f755085ff951888dbe64682c210c9ef7d167b40c;
and the corpus design/operations documents above. No comparative speed or
method novelty claim is proposed.
