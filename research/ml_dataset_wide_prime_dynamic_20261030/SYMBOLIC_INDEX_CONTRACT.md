# Symbolic frontier index: concrete implementation contract

2026-10-30. Outside-worktree specification for independent root/author acceptance before implementation. Native baseline9bfe5bcae18fb3007730bf5bae8cea098138647d retains the reviewed342 engine. Completes proposal v2 SHA54ee7f2ab938cc95396abb3d88215af55c7b30606bb685c9ebf49651e9a5ea50. No engine edits, execution, new scientific authorization or cap changes.

## Fixed semantic boundary

Only replace Engine::add_frontier membership lookup and add its batch-local state. Preserve existing append-order frontier, processed cursor, lowest-basis-ID reducer scan, shifted rows, full minimum-degree pair selection, row-limited stable selected prefix, pair removal only after successful batch, selected/pending counters and step semantics. Never use table traversal as algebra order. Destroy index before frontier sorting. No changes to field arithmetic, sorting, reduction, native terminal/callback behavior or adapter observer split.

## Exact hash and equality

For V active canonical u16 exponents, initialize h=0xcbf29ce484222325u64 XOR (V as u64). For axis0..V in ascending order perform h=(h XOR u64::from(exponent[axis])).wrapping_mul(0x00000100000001b3). Final hash=h XOR (h>>32). This is frozen deterministic word mixing, not cryptography or probabilistic equality. Initial slot=(hash as usize)&mask; mask=S-1 and supported target usize is64bits. Do not pack/truncate exponents or mix coefficients/basis IDs.

Collision resolution is linear probing: occupied slots contain frontier index+1; validate decoded index<frontier.len(), then compare all V active exponent coordinates with per-coordinate deadline checks, stopping on first difference. Canonical zero unused tails are an inherited engine invariant; tests include all28 active axes and highu16 values. Equal active coordinates therefore mean exact Mono28 equality. Do not add a full-tail scan per probe without accounting for it. If implementation cannot preserve the invariant, reject invalid monomials at the existing boundary rather than weakening equality.

At most S probes, with wrapping progression(slot+1)&mask. S is a power of two<=16384 and slot<S, so arithmetic cannot overflow. Exhaustion is InvalidInput, never retry/reseed/fallback. The test-only collision hook substitutes the initial hash only; production hash is fixed above.

## Concrete requested scalar layout

Use one batch-local FrontierIndex owning Vec<u32> slots and usize mask. Additional fixed helper scratch comprises exactly twelve64bit scalar lanes: hash, slot, probes, decoded_front_index, axis, left_exponent, right_exponent, init_position, init_end, publish_code, active_variables, logical_column_limit. All may use u64 for a simple explicit96byte layout; usize conversions are checked at boundary or justified by validated<=16384 indices. Reuse lanes across initialization and lookup; no second scratch object or duplicate Mono28 is introduced. The existing borrowed monomial argument and frontier are already owned by their current callers.

On the supported64bit target, Vec header24+mask8+scratch96=128bytes. Require compile-time size assertions for the actual chosen structure, size_of::<usize>()==8, size_of::<Mono28>()==56 and total index/header/scratch<=128. If representation differs, halt source acceptance and explicitly correct the charged envelope; do not silently rely on compiler stack layout. This envelope is requested mathematical state, not compiler spills, allocator metadata, call frames or RSS. The table's existing128byte gate.vec overhead remains separate; it is not reused to hide this scalar state. No temporary exponent arrays or owned monomial keys.

## Admission and initialization

Lazy allocation on first attempted nonempty insertion. Cmax=limits.columns is validated<=8192. If0, return ColumnLimit without a table. Calculate S=checked_next_power_of_two(checked_mul(Cmax,2)); require2<=S<=16384. Admit128fixed requested bytes before creating helper state, then use gate.vec::<u32>(S), which already charges128+4*S before allocation. No extra charge for that same Vec header/allocation. Failure leaves all accepted earlier reservations consumed. Initialize only S logical slots even if allocator grants excess capacity. Store mask=S-1. Do not allocate/reinitialize on later lookups within a batch.

Initialization reserves16*S work once before writes. The modeled schedule is8units per zero store plus8units per cursor/control step. Implement bounded contiguous zero-fill chunks<=64 entries with gate.check before each chunk and after all initialization; no unmetered whole-table resize. Chunk cursor arithmetic/control is covered by the cursor/control half of this schedule, not a separate parent reservation; document that modeled combined control primitive rather than claiming a machine-instruction count. Use checked admission arithmetic. The initialized table length must be S on successful completion. Partial initialization on refusal is dropped and cannot be consulted. A gate.check failure never refunds the upfront initialization reservation.

At maximum S, table+scalar requested bytes=128+65536+128=65792 per batch. At most64 entered batches imply4210688 cumulative bytes for these additions; original frontier/rows/basis/other allocations remain charged separately under unchanged B. A failed batch terminates; no65th batch bypasses the step gate.

## Per-insertion tariff and transaction

Keep all accepted v2 tariffs, using checked arithmetic:

- Hash reservation64*(V+1) before reading exponents. Per axis: coordinate read8, widening8, xor8, wrapping multiply8, combined loop/control<=16, spare16. The extra64 covers seed/V mix, final shift/xor and initial mask. Check deadline each coordinate. No field operation or setup is involved.
- Each probe reservation32+64*(V+1) before slot access. Per compared axis: two reads16, comparison8, up tofour8unit index/control primitives32, spare8. Fixed96 overhead covers slot load, zero test, checked decode/range validation, loop bookkeeping and next-slot arithmetic. Check deadline each probe and each compared coordinate. Empty and early-mismatch probes reserve the same worst-case charge; native exposes reserved work, not falsely inferred executed operations. If diagnostic actual visits are added, charge their updates within this control schedule and keep them distinct from reserved work.
- A duplicate returns success without frontier/table mutation, including at full logical Cmax.
- On empty slot, enforce frontier.len()<Cmax, then reserve16*(ceil(size_of(Mono28)/8)+1)+32 for monomial copy and publication. With56byte Mono28 this is160work. Existing gate.push independently admits frontier growth and its movement tariff; leave it unchanged, neither duplicate its growth charge nor bypass it.
- Calculate checked publish_code=u32::try_from(frontier.len()+1) before mutation, within the admitted store/index schedule. gate.push(frontier,monomial) may fail before append; then index remains unchanged. After successful append, assign slots[slot]=publish_code with no intervening fallible gate/check. Any later deadline refusal preserves this committed pair of mutations. All source indices are<=8192 and fit u32.

No writer/serializer is added by this helper. Empty-write handling and byte-meter fragmentation are therefore inapplicable; keep existing event serialization untouched. The native Gate::permit has cumulative reserved counters only. Do not invent performed-operation counts from its reservations.

## Fixed validation and acceptance

1. Differential helper sequences compare exact append order with legacy contains after every accepted addition; fixed mixed order, duplicates, all28axes, exponents65535, and test-only forced collisions/wraparound. No resampling.
2. Cmax0/1/8192, duplicate at full cap, new distinct refusal, allocator overcapacity not legal columns; no mutation on refused append/growth.
3. Exact/one-less initialization/hash/probe/append work and table+scalar/frontier-growth bytes. Confirm nonrefundable failure receipts. Test expiry before initialization and within chunk/probe/equality loops, preserving original absolute deadline.
4. Completed small native fixtures compare canonical basis, stop, selected batches/pairs, reducer/matrix rows and pivot counters against legacy helper under unchanged existing caps. Account for expected resource-receipt differences. Do not require identical censored progress after changing costs.
5. All existing native cap/observer/regression controls remain. Freeze implementation source before parent-owned compilation. Retain original342 failed432 panel; any later explicitly scheduled full panel preserves inputs/caps and useful-row/pivot predicate with every case logged. No useful-work success, speedup, activation, extraction coverage or scientific production is inferred from this specification.

Root/author independent acceptance of this concrete document precedes code. Scope remains native helper and dedicated notes only when separately assigned; this draft does not authorize concurrent mutation of frozen validation source.
