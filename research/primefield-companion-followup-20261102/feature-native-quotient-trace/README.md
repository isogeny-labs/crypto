# Source quotient trace instrument

2026-11-07, T-1843. Implemented and software-validated instrumentation; collector compiled, **no workload trace collected**. The prospective protocol is valid as a draft, not frozen or admitted.

## Why this boundary matters

`Modes` has its own polynomial module over scalar modulus r. The new source kernel is separately nested under `Source` over coordinate modulus p. These are different arithmetic layers. This instrument wraps only `Source`'s `quotient_canonical`, used by both general division and monic remainders. It must not be described as a whole-recovery or whole-source count.

The generator copies the preceding weighted-quotient implementation, pins its inputs and adds an opt-in `quotient_trace` configuration. Normal builds have no trace module, counters, wrappers' capture calls or trace exports; the unchanged helper arithmetic is behind a normal wrapper. The seven-file normal source dependency closure confirms trace-meter.rs is excluded. The collector build includes it.

Each event contains p, quotient length n, divisor degree d, full relevant nonzero support size, W=sum(n-j), actual route, quotient-only Poly::fm and Poly::inverse counts, and success status. Routes0/1/2/3/4 are zero dividend, direct constant, direct linear, weighted recurrence and Newton respectively. Route markers are inside actual branches; metadata is not substituted for observing dispatch. The wrapper snapshots counters around quotient construction; subsequent remainder multiplication is outside the delta. Full support inspection is audit overhead, even where production dispatch stops early. No clock or memory-performance interpretation is valid from the instrumented binary.

No scalar, polynomial coefficients or target coordinates enter event records. Input scalars exist only in the collector's target construction and reference equality check, outside the algorithm. The trace is thread-local and synchronous: begin/finish must run on the same thread as the source. Invalid nested begin and finish-without-begin return errors. Counts use checked u64 increments; any overflow marks the capture invalid. At most100000 events per target are stored; record-cap excess marks invalid and increments dropped rather than silently truncating a valid transcript. Vec capacity growth and trace allocations are instrumentation overhead, O(cap), not production storage claims. This wrapper's quotient implementation is nonrecursive in quotient_canonical; counter deltas do not double-count nested quotient events.

## Software verification and retained errors

validation-02 synthetic executable passes120 constructed polynomial cases across p5/17/97/257 and lengths1/2/3/8/32 with monic/nonmonic and empty/late/dense support. It checks uninstrumented predecessor quotient/remainder equality, exact metadata, branch identity, sparse field counts and repeat transcript equality. Additional controls exercise zero dividend, invalid divisor with no event, cap-zero drops, forced counter overflow, nested begin and independent threads. These are software tests, not a recovery workload census.

The strict prospective parser validates all617 ordered target headers and their declared events, source-modulus attribution, event indices, branch/weight consistency, exact weighted bound and zero dropped counts. Synthetic parser tests use617 fabricated headers and reject six malformed panels (missing/extra rows, dropped events, unsuccessful quotient, bound violation, incorrect event index). Fabricated rows are temporary and are not experiment outputs.

validation-01 compiled synthetic code but did not execute it: rustc adapted the requested synthetic.bin output name, and receipt hashing failed on the absent filename. validation-02 synthetic execution and collector compilation succeeded. Its normal metadata compile also succeeded, but the wrapper expected normal.rmeta instead of libnormal.rmeta. The existing metadata/dependency files were reconciled into normal.receipt.json without repeating compilation. Both wrapper versions and failure records are retained; build-check.py now handles both emitted names. A system-Python canonical-check attempt failed because its jsonschema lacks Draft202012Validator; the existing report virtualenv successfully validates the final draft. Initial draft schema/decision findings were corrected before publication; no freeze or data collection occurred.

## Prospective recovery trace

`modes.rs` builds the collector; it is not run by build-check.py. It covers the existing five tuples (p,a,b,r,P,m):

- (17,2,2,19,(0,6),2)
- (41,1,5,47,(0,13),2)
- (97,0,5,79,(1,43),3)
- (193,2,7,191,(0,59),2)
- (257,1,7,281,(1,3),5)

For each k=0..r-1, construct the public target and a fresh Source before capture, then call recover_traced once. Modes compilation is once per curve, outside capture. Lazy Source preparation during recovery is included only insofar as it invokes the quotient helper. This is a cold-source per-target boundary, distinct from the historical reused-source recovery validation. A failed recovery, overflow or dropped trace aborts complete-panel production; partial stdout must be preserved by the eventual execution wrapper.

protocol.draft.json plans two identical independent collector processes and exact transcript A/A equality, a shared120-second/one-attempt/one-CPU/1024MiB envelope, proposedUSD1 ceiling and no retries. This proposes no new allocation: reserved funds are zero and remaining allowance is unknown. Before execution: bind published source/binary, implement and validate the canonical execution/export adapter, perform admitted preflight, freeze and commit protocol before data, reconcile live role/quota/budget and obtain authentic admission within existing authorization. The environment variable in the collector is an accidental-launch guard, **not an authorization mechanism**. No claim that the current source-only artifact enforces cloud limits.

Analysis must preserve all per-target counts and histograms. The original4563-case cohort is unchanged. No aggregate source cost, frequency of sparse bands, speedup, new recovery count or canonical hypothesis status is established by these instrument tests.

## Sources and next step

KB unavailable: missing MCP and wrapper executable. Read exact weighted kernel and recovery code, prior feature-native-resultant-protocol authority/adapter, and [FLINT series division](https://flintlib.org/doc/nmod_poly/index.html#power-series-inversion-and-division). The distinction between coordinate- and scalar-field kernels follows directly from modules in modes.rs and lib.rs. Standard recurrence, no novelty claim.

Next complete the execution/export adapter for this exact trace boundary. Once an admitted frozen panel supplies data, use per-route histograms and quotient counts to choose between support specialization, quotient work in dense branches, and remaining source phases. Historical report/graph files remain immutable; this report changes no canonical research graph.
