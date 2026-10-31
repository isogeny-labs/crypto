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

The draft was filed at commit 850055f73953d2adca7a403e237ce74dcc47147a
and passed the pinned official checker with zero findings. The official
freeze produced protocol file SHA-256
1dab5a5381b1b34fa884738e38dfe555eaea912299f2f59cf6b829ba544f70d8.
The rebound plan file SHA-256 is
8d2b36b9200cec6c64005f8cdfa5eff99d837538395896f186eaa062fd0fcb1a;
native preflight reported 64 planned cells, both twists, and zero generated
cells. Protocol and plan were committed at cd7ee385d0f9 before generation.

After owner review, the exact launch from the repository root is:

    PYTHONDONTWRITEBYTECODE=1 python3 research/ml_dataset_local_campaign_20261031/campaign.py check
    PYTHONDONTWRITEBYTECODE=1 python3 research/ml_dataset_local_campaign_20261031/campaign.py run

The run subcommand uses a nonblocking exclusive lock and an atomic marker per
shard before it invokes the native runner. A restart sees that marker and never
launches a second solver attempt for the same shard. Native verify, report and
strict export receive separate receipts. A completed custody checkpoint has a
restricted archive in the graph store, a small content-addressed custody
pointer in the same store, fresh GET comparisons for both, and a local
published receipt. The graph-store logical-name index at the local XFS root
is mirrored by immutable per-cell checkpoints under this research directory;
commit those checkpoints after each batch to retain discovery if the local
disk is lost. The controller halts on an active worker, changed frozen
inputs, less than 4 GiB local XFS free, custody failure, or the 12-hour frozen
admission deadline. These stops preserve unfinished indices as unattempted;
they do not replace them with different samples.
