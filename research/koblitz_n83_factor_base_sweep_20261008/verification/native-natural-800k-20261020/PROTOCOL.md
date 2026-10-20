# Native N83 continuation: 800000 conflicts, UTC 2026-10-20

The user requests higher conflict caps for K2048 and K4096, continued native
N83 factor-base sweep, and all raw evidence in setec-astronomy-artifacts.
This separately frozen round raises 400000 to 800000 conflicts and 2400 to
4800 worker seconds per base. Run K2048 once, assess, then independently
run K4096 once. Each mechanical command has a 5400-second deadline; maximum
new worker allocation is 9600 seconds. No automatic timed retry. Every
failed attempt or censor remains evidence and is assessed before proceeding.
Earlier observations, including the unadmitted K4096-v10 inspect failure,
remain immutable; no pooling, time ranking or inference of UNSAT from zeros.

Retain the single public query, public_x_hash seed2026100801, m5, identitymask0,
primary a=0 icv1-f2m83-tm6151469093347-debefd74, field polynomial
x^83+x^45+x^2+x+1, subgroup order2417851639230796216685689, and public
K2048/K4096 panels. The positive-trace a=1 holdout is distinct and EC1
identity equivalence remains unresolved. No unknown-scalar recovery claim.
Retain worker SHA256 3a31bd2bedee5519d30b517f39ec27f72561ce898213657688c55b51bbcdf7b6,
its nine watched sources matching660f0f4b10; do not rebuild it. The new
source freeze identifies guard/protocol only. Guard-v12 changes only natural
budget admission to800000/4800; previous400000/2400 stays accepted, zero and
just-above-limit reject. Witness100000/900 and external1000000/900 stay fixed.
Keep v11's 60-second read-only Docker inspect timeout, 15 seconds for other
RPCs, no retry, and preserved status/stderr. Original v10 failure root cause
remains unknown; timeout was an inference, not a preserved exit124.

Retain image sha256:541096ae9b05d3b850341b78790e2ed31e94d5aa0ea086933c6ff0c09ffa2e63,
1CPU quota,4GiB memory,no extra swap,network none,read-only root/source/input/
worker,capdropALL,no-new-privileges,64PIDs. CPU reservation[14,30], full SMT
core14/30, runCPU14, strict native topology/cgroup/sibling checks and exclusive
/tmp/crypto-bench.lock. Evict/restore movable user threads only, preserve
quiet samples and bounded30-second settling. Do not change peers' cpusets or
stop peers. No builds/tests/heavy work during measured arms. Fixed kernel
threads and missing worker context-switch/fault counters preclude automatic
L2 admission. n=1 per K/cap; recorded clocks are descriptive, no fastest claim.

The resumed host uses EFS at /home/ec2-user and a fresh Docker daemon; the
exact retained image was absent, restored by its immutable repo digest, then
same-image /bin/true readiness passed. Record this environment change rather
than claim a matched-host timing comparison against earlier rounds.
Native guard tests precede timing. No library/worker change: inherit original
4398-pass/118-ignored library and post-main-merge socket-denial/38-module/
19-exporter/2-model-check qualifications from the400K publication; these are
not new clean full-suite runs. Preserve fresh guard build/test logs and all
relevant sources/manifests/lockfiles.

Commit clean source/protocol before freezing specs/cards. Exec parent exists
and is ignored, including relay/receipt paths. Native-only director compile,
verbatim prompt relay to mechanical executor, actual write-once stdout,
director assessment at unchanged frozen HEAD. No tracked edits or HEAD moves
while a card awaits assessment. Every failure is preserved and assessed;
corrective attempts require a separately frozen addendum and fresh paths.
Native replay checks query aG+bQ in the original field/group, phase order,
input/source/ELF/stream custody, budgets, exit/OOM and cleanup. Killed prefixes
have null terminal counters. Completed zero-yield unknown is bounded, not
UNSAT. Positive natural models fail closed until independent original-group,
rank and scalar checks support them. No cold/rho/identity promotion.

Research execution, replay and derived summaries use native Rust/C/C++.
Python only plots recorded values and renders/inspects PDFs. Preserve every
new actual command/spec/card/receipt/relay/assessment, stdout/stderr/events/
summary/config/custody/resource record, failure/censor, exact ELF/public
inputs/sources/build inputs and host/toolchain metadata. Deliver native
summary, source-linked report, editable graph with SVG and same-source150dpi
PNG, inspected PDF and benchmark copies/index. Upload complete raw archive,
fresh GET and archive/payload checks, downloaded-record native replay and
deterministic repack; upload proof sidecar with fresh byte-checked GET.
Use UTC2026-10-20 fixed archive timestamp. Prior content addresses remain
unchanged. Canonical cold/rho/identity graphs stay unchanged absent their
separate gates. Commit/push and update existing PR2036 without force.
