# Implementation handoff

Design-first commit b8a13bfac560; base60e185001207e3c5cebfdaf3887ab87cfcb39f4c. Source authored and scoped rustfmt/diff checks only. No builds, tests, fixtures, cloud activity or adapter activation performed by author. Parent owns serialized validation. This owned handoff replaces writes to the conflicted shared session log.

## Files and API

`prime_root_trace.rs` adds only the `bounded` child declaration; legacy solve/verify and their tests are untouched. New `prime_root_bounded.rs` exposes RootBudget, Stop, Metrics, inline Poly17 and BoundedRootEvent, solve_bounded, and Replay::{new,push,finish}. Borrowed Fp64Arithmetic supplies a previously validated modulus; no primality/sqrt setup is repeated. The native polynomial operations remain canonical u128 arithmetic, porting legacy convolution/division/gcd/right-to-left powering and linear splitting into fixed storage. It preserves nonmonic normalization and repeated-root semantics.

New stream events are Begin, LinearPart, Split, LinearRoot and Finish. They are not legacy v1 transcripts. Fixed-size public decoding helper Poly17::from_coefficients performs<=17 comparisons/copies without allocation; callers must admit encoded parsing/storage before calling it. No serde or unbounded native parsing path is introduced.

Replay is genuinely separate arithmetic/control flow: output-diagonal convolution; fixed high-to-low remainder loops; left-to-right modular exponentiation; independent Euclidean normalization; explicit pending-factor state; final independent product(X-r)=gcd(f,X^p-X). It calls no Producer algebra method or solve. Shared Gate is resource/deadline accounting only. Seeded SplitMix64 schedule is independently checked per accepted transcript event. No uniform/expected-time/deterministic-completion guarantee is asserted. No Fp scan fallback exists.

## Storage and work accounting

No non-test Vec/String/Box/map, allocation or recursion. Poly17 is at most144 bytes on the ordinary 64bit target. Producer owns eight pending polynomials, eight roots and one optional failed event. Replay owns eight pending polynomials, input/g and eight roots. A compile-time size expression sums BOTH Producer and Replay structures (conservative, though separate calls), a full Report and TWO event objects, PLUS64 Poly17 equivalents for simultaneous nonrecursive helper frames/scratch/copies/scalars; it must fit16KiB. Maximum producer nesting is execute->power->division->inverse->scalar; execute->gcd->division->inverse is shallower. Power holds base/acc/product, division holds q/r, gcd holds a/b/r; no frame allocates an unbounded object. Checker nesting push->check_event->modular_power->remainder->reciprocal is analogous. Sixty-four polynomial equivalents conservatively cover live inputs, return temporaries and fixed scalar state across those paths; explicit persistent structures are counted separately. This is a requested modeled-storage bound, NOT compiler stack-frame/RSS measurement. Unavoidable refusal/return ABI metadata exists even before storage admission and is explicitly outside admitted-arithmetic storage semantics.

Entry admits4096work+16KiB before input copy/callback. Scalar arithmetic precharges32 modeled units plus one field operation. Polynomial helpers precharge fixed17-coefficient copy envelopes; their internal scalar arithmetic is independently charged. Event metadata/copy allowance is admitted by prepare_event BEFORE constructing each owned event, then observer is called. Fixed scans/sorts/stack operations have conservative precharges. Field inverse1 takes an explicitly charged constant shortcut, like legacy; other inversions are bounded by61 exponent bits. Attempts count before callback, acknowledgement only onOk, then original deadline is checked. Failed callback is sticky, retains one diagnostic unacknowledged event and no final roots. Native arithmetic_invoked is set only by successful scalar admission immediately before actual modular arithmetic.

Root degree8, fieldwork1m, events/splits100k and work16,777,216 ceilings remain; no higher global cap. Successful calls expose consumed modeled-work/storage so adapter can deduct balances across nodes. The native module cannot enforce caller-owned cumulative ledgers across separate objects. Each producer/replay receives the original caller deadline; replay uses its own admitted execution deadline, never the expired historical timestamp. Any late acknowledged Finish remains censored.

## Assurance boundaries

Complete proves all DISTINCT Fp roots; AllFieldElements is separate and never lists p values. No multiplicity or full irreducible factorization claim. Censored native reports clear final roots; pending-factor count is diagnostic, not a resumable checkpoint or independently proven operation history. The checker validates acknowledged algebraic events and final identities, with its own resource budget. Its checked roots on an incomplete prefix are individual diagnostics only, not a complete root set.

Producer metrics/stop-cause metadata are range/shape checked, not independently proven exact execution counts. In particular producer_operation_trajectory_verified is alwaysfalse. Unacknowledged event contents are retained diagnostics, explicitly not certified by replay. A prefix without an acknowledged Begin is an admission/I/O failure, not a mathematical certificate; Replay::finish rejects it. This avoids manufacturing proof from an empty failed attempt. Adapter must preserve these outer failure records separately.

Six authored test functions cover exhaustive F7 quadratics; repeated derivative-zero F3; constants/zero/irreducible/mixed nonlinear factors; fixed nonmonic/repeated/degree8 roots at47/53/61bits; actual split events; exact/one-less work,field,bytes,event and split controls; invalid inputs before callback; sticky Begin/middle/Finish observer refusal; Begin and actualFinish callback expiry; original deadline already expired; independent replay input/gcd/delta/order/omission/duplication/root/canonical-tail tampering; replay tiny-budget refusal and sticky invalid-prefix state. Tests collect streams in test-owned Vecs, outside native memory claims. All tests are authored, unrun. Fixed wide degree8 completion gates must pass without resampling or increased ceilings; if not, report the failed gate and improve bounded implementation under review.

Remaining: parent source review/build/focused/full validation; bounded adapter serialization and replay envelopes; nonlinear multivariate residual visitor; group/domain/original-equation checks; exact-role persistence and rehashed dataset controls; source-bound CLI and S3 restore evidence. None is implied by this native commit.

## Independent review correction: replay split accounting

The independent review of d630bc682d9e found that Replay checked and incremented
its internal split-event counter without updating the returned replay metrics.
The correction increments replay_metrics.splits after each successfully checked
Split event. The existing fixed wide nonmonic/degree-eight controls now compare
that metric with the recorded Split event count and require a nonzero count.
This reports checked replay splits; it does not certify the producer's complete
operation trajectory. Source formatting and diff checks are separate from
runtime validation, which remains queued behind the combined regression suite.

The same review identified expiry between event reservation and callback entry.
The emitter now rechecks the original deadline before constructing/counting the
event and calling the observer. A deterministic control expires the deadline
between preparation and emission and requires zero callbacks/attempts. The wide
fixed-root panel now covers every degree2 through8, without altered ceilings or
seeds. A caller-ledger control subtracts the first call's work, field, storage,
event and split consumption before a second call under the same deadline; its
remaining work forces censorship before any second callback. These tests are
newly authored, not yet executed. Remaining review coverage suggestions and
runtime acceptance still need reconciliation before declaring validation done.
