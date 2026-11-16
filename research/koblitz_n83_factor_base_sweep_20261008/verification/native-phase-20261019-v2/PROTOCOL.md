# Native phase diagnostic v2: explicitly specified orbit witness

Frozen before any v2 execution. The original 19 October control card stopped
at command3: small-field original-group and phase controls passed; N83 fixed
coordinates reached fixed_units_installed at113.114676ms, then hit180s wall
without a terminal result. Its gate was **not met**. Both original retained
cards remained unlaunched. Preserve their cards/specs and the failed control
record under `../native-phase-20261019/exec/`; do not label them successful.

This is an explicit change of the positive-control predicate, not a rerun or
replacement of that control. The new native example
`koblitz_n83_selector_witness_control` uses the same public fixture, full
1182/866 orbit domains, circuit, first-pair ordering and eight fixed x blocks,
and additionally fixes each known one-hot representative choice and seven-bit
Frobenius phase. Representative/phase lookup must be unique, and allocation
ranges must match the frozen encoder's public size calculation. An independent
native polynomial-word verifier replays the selected Frobenius images, domain
indices and original five-point group sum. Require SAT_verified_witness_x
and model verification, at most5000conflicts, same180s wall and resources.
This proves consistency of this fully specified witness only. It does not
establish that the earlier free-selector N83 search can find the witness.
The earlier control remains an open search/propagation bottleneck.

The traced exact small-field test and its phase checker run first again,
using the same exact name and native binary pinned by the director. Its
ordered/unordered m5/m6 assertions test the unpinned selector search at n7,
not natural N83 yield. All control failures stop their card and are retained.

After that distinct control card succeeds, run **new separately frozen**
K2048 and K4096 cards, each one public natural trial, m5 orbit-phase chained
S3, public_x_sequential seed2026100801, 32 models, 150000 variables,
2000000 domain clauses, 1000 conflicts,600s outer wall. No selectors are
pinned in either natural arm. No automatic retry. K4096 is an independently
requested arm after controls even if K2048 yields an unsupported disposition.

All curve/fixture identities, SHA256 archive pins, immutable image, isolation,
source/input/worker freeze, native execution and custody requirements of the
original protocol apply. Image is
`sha256:541096ae9b05d3b850341b78790e2ed31e94d5aa0ea086933c6ff0c09ffa2e63`;
Docker network none,1CPU quota,4GiB RAM,no swap,64PIDs, task UID, RO inputs,
all capabilities dropped,no-new-privileges, and independent inner hard timeout.
Live/final inspections and cleanup remain mandatory. Command allowances120s
for each small-field/control-check command,240s for N83 control,660s per arm.
New container envelope180+600+600=1380s; previous capped180s is additional
spent work, never reset or erased. Build/verification/storage/tests/rendering
remain separate overhead; no fully charged cold total is established.

The v2 runner is built into a **separate target directory**. Original runner,
worker binaries, specs, cards and failed receipt are retained unchanged.
Native specs again pre-pin current source commit and complete content hashes;
new cards bind their SHA256s before launch. An ignored separate exec directory
permits source-clean sequential cards. Director compiles; parent relays the
native prompt verbatim; mechanical executor returns native receipt stdout;
director assesses execution and relay. The card transport and schemas use the
explicit strict native replacement, never Python.

The native audit still admits only a checked zero-relation solver-cap terminal
or a censored phase observation for natural arms. Positive natural relations
or scalars require the future full native matrix/relation verifier. Missing
final counters remain null. Empty traces localize no phase; solver_enter
brackets the solve call, including preprocessing. No A/A, full cold pipeline,
matched external benchmark, fastest-base selection, asymptotic claim or
nonexistence conclusion is established. Preserve partial/failed outcomes.
After a passing selector control, forge one reported selector phase while
updating stream custody hashes and require independent native rejection.
Native runner/card tests and relevant native release tests remain required.
All results get source-linked visual reports and inspected PDFs outside the
frozen audit directories; report both control variants separately.
