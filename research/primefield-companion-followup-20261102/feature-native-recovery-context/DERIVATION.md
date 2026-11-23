# Reuse a bounded interval evaluator within one recovery

T-1860, 2026-11-07. Producer derivation and deterministic software validation; unsigned [pcc:unverified].

## Construction and correctness

Previously every CoeffStream built its own IntervalProducts, discarding the lazy block polynomial and progression weights/samples when that stream ended. StreamModes::recover_traced now creates one fresh RecoveryModes on entry and delegates to the unchanged recovery body. That local object owns a RecoveryProducts containing the exact key (r,m,B) and one RefCell<IntervalProducts>. SharedCoeffStream borrows this object with a Rust lifetime; it cannot outlive its owner. Products are evaluated under try_borrow_mut for one call only; a conflicting borrow returns Arithmetic rather than panicking. No global cache, reference counting, unsafe code or target-to-target state is used.

The generated shared coefficient and mode methods preserve their predecessor arithmetic. Only interval ownership changes. Each stream retains its own index, coefficient and remaining count. The context verifies the exact field, mode and block cap before initialization. It does not cache coefficient sequences, target signs, recovered scalars or root lists. The original CoeffStream and recover_isolated remain controls. Generation pins the predecessor and emits both versions so the arithmetic correspondence can be inspected.

For fixed (r,m,B), the block width w=min(ceil(sqrt(m)),B), block polynomial P_w, progression weights and samples are independent of e, t, b, the coefficient stream position, and the public target. Every call has the same interval arguments as before. Reusing the identical prepared polynomial/evaluator therefore preserves every interval value, coefficient, constraint and phase. Induction over stream transitions and recovery steps preserves the output and source-call trace. This is an ownership/setup change, not a change to the sign oracle or recovery logic.

## Charged work, lifetime and cap

In a successful recovery, block-polynomial setup and progression setup each occur zero or one times. Both remain lazy: an endpoint can return without either. Previously each stream that reached those paths paid its own setup. For J participating streams, the avoided construction work is the sum of their repeated setup costs after the first; this is not a timing ratio. Arithmetic for initialization, interval queries, batch inversions, residues, first C0, source grids, order validation and final public verification remains charged.

The cap B is an element/request bound, not a claimed process byte ceiling. Each request list has at most 2B intervals and each coefficient chunk at most B elements. The shared context retains at most three vectors of w+1 live field elements (block polynomial, weights, samples), plus constant-size polynomial engine, key, counters and RefCell metadata. All Vec capacities and allocator overhead must be charged; live lengths are not allocated byte counts. Setup and query trees remain temporary O(w log w) workspace. First C0 and the separate source grid retain their prior bounds. The recovery body uses streams sequentially, so sharing extends cache lifetime across streams without retaining all stream workspaces. Streams can also be interleaved safely in checked controls. The context drops on every normal return or error; the public wrapper creates a fresh context on every invocation, including repeated targets.

## Exact checks

validation01/receipt.json records optimized rustc with overflow checks enabled, compile/check exit zero, transitive source hashes and stdout hash. No timing workload was run.

- 45 sequences over r=1009,m=56 and caps 1,2,7,16,64: 420 exact coefficients and stream states match isolated evaluation. Nine sequences per cap (three repetitions of each mode).
- Isolated streams construct 36 block polynomials and 36 progression caches; shared contexts construct four of each. The cap-one context uses direct products and constructs neither. These are deterministic software controls, not workload rates.
- Fifteen mismatched context keys rejected; five conflicting borrows rejected and recovered after guard release; five pairs of interleaved streams match isolated sequences. Fresh contexts start with empty caches.
- All 281 public targets on F257, y^2=x^3+x+7, base (1,3), order281, m56: shared and isolated recovery agree on scalar, route, and complete PullStats. 14,421 source calls, zero unresolved. Each target starts cold; 224 construct one block polynomial and progression cache, 57 need neither.
- Inherited 617-target panel: 5,929 source calls, zero unresolved. Inherited cached-shift, packed-request, coefficient, residue, domain and public-recovery controls pass. The simple earlier 63-case baseline still records its 17 unresolved cases; later pulled/trace adapters recover all 63 with 511 calls. No baseline failures are relabeled.

Original 4,563-case cohort, prospective trace protocol and source calibration are unchanged. No cloud trial, budget change or fullCargo rerun; historical missing-fixture/deadline limitation remains. Independent signed review remains open.

Replay into a fresh output directory:

```sh
python3 research/primefield-companion-followup-20261102/feature-native-recovery-context/validate-recovery.py /tmp/recovery-context-replay-unique
```

## Sources and next construction

Sources checked (KB): MCP unavailable; scripts/research-kb status reports no executable. Local predecessors: feature-native-cached-shifts/{streamed.rs,stream-adapter.rs,interval.rs,progression.rs,DERIVATION.md}; scalar polynomial engine feature-native-hgcd-roots/polynomial.rs. Primary sources: Bostan-Gaudry-Schost, progression evaluation/setup, https://mathexp.eu/bostan/publications/BoGaSc04.pdf; Rust RefCell documentation, https://doc.rust-lang.org/std/cell/struct.RefCell.html. This reuses the established progression method; no novelty claim.

Next address remaining per-query inversion/initialization work or first-C0 storage with exact cold-target accounting. Current report and flow diagram describe ownership only; historical/canonical research graphs are unchanged.
