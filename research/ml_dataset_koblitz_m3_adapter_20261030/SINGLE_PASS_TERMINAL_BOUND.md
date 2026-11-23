# Exact native encoding and closed terminal output

Follow-up to the archived 135-pass/2-fail runtime validation at `8708d456d3e1`. Both failures stop on the first n47 F4 plain parent event artifact; original limits and controls are preserved.

# Single-pass native encoding patch — review notes

External draft against8708d456d3e13f5501c3f1e92ecf7442168470f3. koblitz-native-single-pass.patch SHA256f5c5b658f587ae32328b060fc3bebe4bc1fc25c1396f4d6e4e239df23e2592f5. Saved proposed/base files are single-pass-{koblitz_m3,solvers,capture}.{proposed,base}.rs. Generator build-native-single-pass.py; controls native-single-pass-controls.rs. Rustfmt parse and git apply --check passed. No builds or tests executed, no source edits.

## Real change

The specialized emitter route no longer calls serde_json::to_writer on CountedEventBytes. It calculates the exact whole-envelope length using fixed JSON literals plus numeric widths, then serializes once into an exact bounded AdmittedEventBytes writer. Existing generic capture methods remain unchanged.

A concrete private-field AdmittedKoblitzNative wrapper carries only the actual native view and admitted length; its manual Serialize delegates only to the native view. The restricted solvers re-export is the visibility bridge. Only koblitz_m3 source and its descendant controls can construct the wrapper; capture accepts this concrete type, not arbitrary Serialize plus a claimed length. The only production constructor occurs after calculator and serialization-work reservations. This is a source-level closed contract, not cryptographic proof of runtime accounting.

Capture independently matches wrapper.capture_sequence to actual head.event_count.checked_add(1), validates expected length against actual buffer/capture caps, checks capture reservation before encoding, serializes once with exact limit expected-1, requires exact resulting length, appends newline, then invokes precommit and commits once. Short/long encodings and stale head fail before callback/commit. Existing producer original digest/limits/count verification, callback pending-prefix retention, capture identity, projection acknowledgement and postcommit clocks remain.

## Numeric bound and fee

native_decimal_len upper-bound searches nineteen fixed powers10. Interval length halves each iteration, so at most five comparisons/iterations;0->1digit, u64MAX->20digits. Generator usize converts to u64 with compile-time32/64-bit architecture restriction; multiplier u8 and level u32 keep their numeric widths. Bool contributes4/5bytes. No runtime string conversion or serializer is used for sizing.

At most22width calls: fifteen counters, two reservations, two stage integers, native sequence, capture sequence twice. Each search costs at most five modeled loop steps, each bounded by seven scalar primitives when array indexing includes its bounds check and assignment/update is counted with its producing operation: loop comparison, midpoint add/divide, threshold indexing, value comparison, interval update. That's770units; terminating comparisons and fixed initialization/enum/literal/sum logic receive254units. The named1024fee is a conservative source-operation model for this closed routine, not a retired-instruction count. Reviewers must not translate these units into hardware operations/timing.

The1024fee and256modeled scratch bytes are reserved BEFORE calculating any length. Existing1024+9*wire_length serialization/copy coefficient and4096scratch remain, now using exact admitted wire length. Current2048trace admission remains unchanged. Refused calculator/serialization admission keeps all prior charges; no retroactive refunds. Allocation-free sizing uses a15u64 stack array and fixed scalars; no producer heap allocation is introduced by calculator or wrapper.

Compared with variant-capped two-pass route, actual counting serialization disappears. Narrower reservation and separate calculator cost are reported distinctly;8/9per-byte coefficients are not lowered. No completion prediction is made for n47/n61.

## Controls included, not executed

Existing exhaustive all-stage numeric-max cap test additionally requires calculator length equal actual TypedEventRecord serializer bytes. New decimal-width controls cover0,1,9,10,u64MAX and below/at/above every power10. Actual generic and specialized capture output bytes are compared for an admitted native construction event. A test-only Cell inside wrapper records exactly one Serialize invocation; it is absent from production.

Exact-length minus/plus1, stale capture sequence, insufficient buffer limit and precommit callback refusal all verify no committed event or unexpected callback. Calculator fee refusal and subsequent serialization fee refusal verify exact charged delta and empty acknowledged projection/capture. Existing suite must still verify all F4/F5 paths, partial prefixes, original worker bounds, and custody controls after application.

The test-only direct wrapper construction is deliberate fault injection; no production forging constructor is exposed. Existing maximum-sequence test covers u64MAX wire representation but does not attempt to increment a live head already at u64MAX; capture's checked_add continues to enforce that error.

## Remaining validation

No compile/type-check claim. Root should run focused native/capture controls and broader original-worker gates. Additional drafted crosscheck now instantiates every variant at zero, u64MAX and below/at/above every power10, with matching native/global sequence values, narrow-field casts and both bool variants, using actual TypedEventRecord serialization. It does not claim exhaustive Cartesian coverage of all possible numeric tuples. This is a finite arithmetic/schema check, not a performance measurement.

This patch does not touch terminal outcome assembly/caps or any parent tariff. Keep terminal agent changes separate. On source changes, rebase exact generator replacements rather than silently regenerating from a moving base.


# Independent single-pass and 8 KiB terminal review

Reviewed exact external patch hashes:

- `koblitz-native-single-pass.patch`: `f5c5b658f587ae32328b060fc3bebe4bc1fc25c1396f4d6e4e239df23e2592f5`.
- `koblitz-terminal8.patch`: `9ee8eae64f80f2e4d89b3665f075a3ed6cc8a2f7f9765fe38f7283b441c7736f`.

No concrete integration blocker found on source review. No worktree changes, compilation, Rust tests, CLI generation or cloud operations performed. I independently recomputed compact JSON byte arithmetic from the saved maximal outcome without writing it: outcome5071, frame5169 and conservative nested guard-cap frame6461, below8192. This arithmetic is not a Rust execution test.

## Native single-pass

The calculator matches the actual closed serde shapes: internally tagged lowercase construction/matrix phase; externally tagged matrix-stage variants; reservation work/bytes; all15 Counters fields; inner schema/binding/sequence/capture-sequence/event; outer live-event schema, two64hex digests, sequence, kind/payload and trailing newline. Decimal widths are computed by an upper-bound search over19 thresholds, at most5 iterations. The maximum number of widths is22 (15 counters,2 reservation words,2 stage integers, and3 inner/outer sequence positions). Booleans account for4/5 bytes, and fixed digests cannot require escaping. The supported usize-width assertion prevents widening a generator beyond the modeled u64 domain. Small closed-schema lengths cannot overflow supported usize widths.

The explicit1024-work/256-byte calculator admission occurs before calculating length. The documented1024 model covers at most22*5*7 scalar steps plus254 fixed operations; this is a declared modeled operation tariff, not a claim about machine instructions or measured stack. No heap allocation or serde pass occurs in that calculator. The subsequent existing1024+9*wire coefficient uses the computed actual byte bound and is admitted before serialization. A failure at the second admission retains the calculator charge; it does not refund or start capture.

The specialized capture method accepts only the concrete wrapper whose fields/construction remain private to the native module. It checks the next head sequence, event/buffer limits and capture reservation before one serialization. A bounded writer rejects underestimation, and exact final-length equality rejects overestimation before the precommit callback or event commit. The newline remains included in admission. Generic arbitrary-Serialize entry points retain their original two-pass behavior. The trusted binding precheck in BoundNativeEvents remains before specialized invocation.

This removes the real counting serialization, rather than merely reducing an unchanged pass's tariff. Existing acknowledgement timing, canonical precommit-copy rules, root custody and original resource limits remain unchanged. Serializer-once controls, exact byte comparison with the legacy encoder, short/long length refusal, wrong sequence, insufficient buffer, callback refusal and precharge failure controls directly exercise the contract. All-variant zero/MAX/decimal-boundary comparisons use the actual TypedEventRecord serializer, and field-length additivity supports mixed numeric widths beyond the shared-value test vectors. These controls require compilation and execution before validation is claimed.

## Terminal8 closed domain

The new guard enumerates35 named scalar fields plus4 nested fields, each with its actual type, finite enum or fixed literal. I compared these against the current constructor and finalizer: no production field is dropped; no free-form OS/error string, equation array or public-instance object is admitted. Root receipts retain all5 fields and fixed identity checks; all11 known identities fit256bytes (longest215), with12 slots retained. Diagnostics retain the original four limits, quote, ledger, path and remaining-work value under exact named nested keys. Finalization retains its five fields, both error strings remain permitted, and absent optional fields continue to support private partial controls.

The saved maximal outcome independently recomputes to5071bytes and frame5169. Replacing exact receipt, diagnostic, error and finalization lengths with their respective guard caps yields6461bytes, leaving1731bytes below8192. The outcome contains pending and finalization fields simultaneously, making this conservative for the actual frame/return stages. The private arbitrary-Value schema-violation preservation remains outside this closed production-domain bound, as already agreed for the32KiB predecessor.

Coefficients remain32 work per output byte,8 per guard byte and4 per trace byte, with unchanged fixed terms. The fixed-key lookup/finite-schema checks are bounded guard work, not measured instruction counts. Combined future reservation is397312work and36864trace bytes, reductions983040work and98304trace bytes relative to32KiB. Original memory, work/deadline limits,12 receipt slots and16KiB parent-delivery cap are unchanged. No existing charge is refunded, no evidence is truncated and no completion follows just from the headroom arithmetic.

The existing guards count the assembled outcome, frame and final outcome; draft controls cover every fixed root identity and all Stop/Phase values with maximal counters, plus unknown-key/type/literal refusal. Existing real worker/partial/collision/central-replay tests remain necessary to establish that the stricter guard accepts every current runtime path.


Integration note: both reviewed changes are applied. The terminal test append was separated from its final EOF hunk because the single-pass patch also appends controls. No semantic change was made. The stale external-draft source comment was removed. Compilation and runtime validation are pending.

## Terminal validation at d6f3865a9301

Native suite:141 passed,2 failed,0 ignored,exit101. Capture suite:40 passed,0 failed,exit0. All seven source hashes remained unchanged. New serializer/closed-schema controls passed. Frozen n47 F4 plain direct/collision controls and actual worker now pass. The direct suite next fails at n47 F4 twist: event117539bytes stored after3826784work reservation, then group24066bytes quote835648 refused with265330work remaining. Actual-worker suite next fails at n47 F5 plain: nativeevents100660bytes quote3286656 refused with2803789remaining. Both complete192producer group additions; no original limits changed. Later cases remain unexecuted. Two failed raw fixtures and terminal evidence archived with verified S3 readback, SHA c1bca20f999dfe013d99b53b279ad00d2b197f2dafdc92323a9450cd3dd92948.

Next: review actual dedicated parent-storage operation accounting; keep original controls. Full-library and real CLI gates remain pending.
