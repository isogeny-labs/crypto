# Next increment: signed-domain prefixes and independent replay

Implementation plan after the25-test input-verifier gate. No group producer,
certificate publisher or runtime activation is claimed by this document.

The original domain has four canonical signed points. Enumerate64 ordered
triples lexicographically, including repeated points. For tuple cursor t,
indices are [t/16,(t/4)%4,t%4]. Start each tuple at infinity and charge every
actual add call, including the identity start. Emit one boundary per addition:
up to192 events, including infinity intermediate results. Derive cursor and
summand from sequence position in the verifier rather than trusting saved indices.

Use existing FastBinaryCurve addition for the producer and the separate general
binary_ecc::curve::point_add for historical prefix verification. Convert only
validated original points to the latter's field representation. Never feed
producer-selected intermediate points back into the reference arithmetic;
compute each prefix afresh and compare the recorded result. Replay must check
indices, cursor, prefix and match flags, including exceptional infinity prefixes.

Track enumeration add calls separately from positive-witness checking calls.
Both consume the shared original group-add allowance. Native group_op_count
omits identity starts and distinguishes doubling, so it is not interchangeable
with this explicit API-call count. A192-call exact boundary may complete an
unwitnessed exhaustive enumeration; an additional independent positive check
has its own accounted calls. End-cursor completeness is not inferred merely
from exhausting the allowance.

Retain acknowledged events and an explicit optional unacknowledged event, with
performed and acknowledged counters. A failed callback or deadline seals the
ledger; later native work cannot continue. Partial prefixes are evidence, not
solver checkpoints. A negative verdict requires all64 tuples, no matches, no
failure, and uncensored outer completion. A nonempty native matrix report alone
never supplies that verdict.

Distinguish a candidate match from a durably published independently checked
witness. On the first match, verify the three original summands with separate
group arithmetic and publish a bound certificate before risking later work.
Keep the first successful receipt if subsequent event persistence, time, work
or byte checks fail. Parent witness helpers currently set solver_completed=true;
do not copy that unconditional outcome update into a censored-prefix path.
Reuse bound payload publication and the existing outer supervisor claim policy.
The artifact can survive even when outer completion claims are cleared.

Pre-admit retained event storage with fallible allocation, and charge callback
storage/serialization against shared memory and trace limits. Native field setup
and generic positive-check scratch require explicit accounting before runtime
activation. Do not invent an RSS or retired-instruction count from event counts.
The bounded group-replay JSON object may contain up to192 addition events for
64 triples; its strict schema and role remain distinct from live JSONL events.

Controls: fixed original negative target; positive target equal to an original
domain point (including inverse-pair infinity prefixes); repeated points;
exact/one-less call allowance; callback refusal before and after the first
published witness; input/role/encoding/cursor/index/prefix/counter mutations;
missing witness, mutated witness and false complete/negative claims. Software
controls remain separate from the frozen matrix-reduction experiment and
scientific corpus generation. No target resampling to turn a failure into a pass.
