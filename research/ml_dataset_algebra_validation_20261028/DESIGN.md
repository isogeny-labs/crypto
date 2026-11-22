# Independent Boolean matrix trace validation

Frozen before implementation, 2026-10-28. Conductor T-758 owns only this
research directory in a separate worktree based on
c14d4552e513f6e28bda0e0e045f5e64beb492db. The producer's separate design and
wire schema are25a46f07530 andfd0ece16169 in
research/ml_dataset_f4_trace_20261028. This is a native Rust software checker,
not a trial, performance comparison, new budget or worker launch.

Implement a standalone crate and CLI that imports no producer polynomial,
matrix, packing, trace or elimination implementation. Strict typed schemas
match crypto.boolean-f4-suffix-trace/v1. Deserialize directly from each bounded
newline-terminated record, reject duplicate/unknown fields and noncanonical
numbers/strings, then require exact canonical reserialization and a continuous
SHA256/sequence chain. Validate trusted expected caller bindings supplied
separately. Bindings establish equality of supplied identifiers, not proof of
actual source execution, platform identity or durable sink acknowledgment.

Reconstruct Boolean multiplication independently using bit-mask union and
coefficient parity. Enumerate each generator's multiplier subsets in increasing
degree and lexicographic variable-combination order. Include every zero product;
reject missing/duplicated candidates, wrong matrix row numbers, wrong support
columns or padding. Require q<=8, D<=4, every input degree<=D and canonical
unique input terms. Preserve zero-polynomial degree zero and q=0/empty input.

Replay supplied swaps and full-row XOR identities from input matrix rows,
checking slot versions, pivots, exact target order and omitted suffix words.
Use an additional scalar Boolean row-basis rank computation, separate from the
recorded elimination schedule. A complete result needs the entire candidate,
packing, pivot, output and end sequence; the independent rank and final rows
must agree. A censored prefix has no final-rank or complete-rowspace claim.
The checker does not infer a PDP encoding, whole-search UNSAT, a Groebner basis,
first/last fall degree, complete execution or total machine cost.

The parser streams bounded records: at most4MiB raw stream,32768records,
64KiB per record,64inputs,256terms per input,4096candidate rows,163columns and
3words per row. It retains only the bounded input/candidate/matrix state plus
one parsed record; it never buffers an unrestricted whole JSONL file. Check
lengths before expansion and use checked arithmetic. Checker work is separately
bounded to128million logical term/bit/row operations, with explicit refusal;
this is not a CPU instruction or RSS ceiling. Reject excessive producer limits
before allocating. Native workspace and admission counters are observations,
not independently proven allocation or total-work evidence.

An unfinished final JSONL line is an explicitly unparsed censored tail, not an
acknowledged event. A malformed complete line fails validation. A stopped record
ends the prefix; data after stopped/end is rejected. Deadline/cancellation,
memory and sink-failure reasons cannot be inferred from arithmetic and retain
stop_reason_independently_verified=false. No end record means no final rank.
Presence of a complete mathematical trace does not certify the producer's
external completion receipt or sink durability; those are separate bindings.

Meaningful tests will cover independent tiny polynomial/matrix examples,
combination order where numeric mask order differs, Boolean cancellation,
zero/constant/q=0, swaps and multiword suffixes, each supplied prefix, malformed
fields/alias/duplicate/hash/source substitutions, rehashed algebra tampering,
resource limits and a truncated final line. Final cross-implementation fixtures
must come from the separately compiled native producer and be consumed by this
standalone executable. Retain failed attempts and exact source/binary hashes.
Do not call handcrafted checker fixtures evidence of producer interoperability.

Primary references checked before implementation: Sage's official Boolean
polynomial documentation defines the GF(2) quotient by xi^2+xi, and its native
GF(2) matrix documentation distinguishes matrix rank/echelon arithmetic. The
wire order and bounded native profile come from the frozen repository schema;
no broad F4/F5 correctness theorem is imported.

- https://doc.sagemath.org/html/en/reference/polynomial_rings/sage/rings/polynomial/pbori/pbori.html
- https://doc.sagemath.org/html/en/reference/matrices/sage/matrix/matrix_mod2_dense.html

All raw events/derived ranks remain restricted validation/provenance or targets,
outside static public ML features. No cloud campaign, E12 retry, IAM mutation,
deadline or original USD200 total ceiling is changed. Cargo remains centrally
owned; no checker build begins until the current generic-v2 owner releases it.
