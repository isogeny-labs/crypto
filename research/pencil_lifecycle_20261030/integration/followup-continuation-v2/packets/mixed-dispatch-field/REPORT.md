# Mixed degree-one plus degree-two dispatcher composition

Three compiled theorems compose the four reviewed branches into a source-order field-value dispatcher. Under original normalized point/divisor validity for the monic quintic and four nonzero representation weights, source_order_total_valid returns some degree-one or degree-two payload with nonzero weights and normalized validity; its tag is not mismatch. none_excluded rules out all modeled None paths under those same premises. returned_payload_valid applies directly to any returned payload. No output-validity or branch-positive assumption is added.

## Exact modeled decisions

1. rawK != 0: call the generic dispatcher, preserve its Option result, tag common.
2. Otherwise cancellation equality: return extracted point, tag cancellation.
3. Otherwise ordinate mismatch: return None, tag mismatch.
4. Otherwise repeated cross-scaled support: return literal tripling field payload, tag tripling.
5. Otherwise: double the original point, call generic with extracted point and doubled divisor, preserve its Option result, tag positiveDistinct.

Tag mappings to source strings are common/common, cancellation/cancellation, mismatch/coincident_y_mismatch, tripling/tripling, positiveDistinct/coincident_double_plus_other. Sum.inl models DegreeOne and Sum.inr models DegreeTwo. Identity is absent because none of these source return constructors produces it. Source dbl unconditionally constructs DegreeTwo; the following let-else unreachable arm is constructor-excluded in this mapping, not replaced with an invented y-zero return. There is no Rust Result/Err return in this source function: modeled failure is Option None, including the nested generic call. This packet does not claim arbitrary-invalid-input panic safety or compiler refinement.

## Composition proof

Split rawK zero/nonzero first. The nonzero case uses genericK_same and the reviewed generic dispatch/validity theorem. In the shared-support case, cancellation returns the reviewed valid extracted point. If cancellation fails, original validity implies positive ordinate equality, excluding mismatch. Repeated support uses the reviewed tripling theorem; distinct support uses the reviewed doubled-plus-other theorem, which already proves its second generic call is Some. Thus both generic None possibilities and the explicit mismatch None are excluded under the original inputs. The field may have characteristic2: earlier cancellation is respected, and the positive subcases derive characteristic!=2 rather than assume it globally.

## Checks and retained failures

Final main attempt3, controls3 and readback2 pass. The three named theorems use only propext, Classical.choice, Quot.sound. Ten fixed controls check all four successful tags, zero-y cancellation precedence, invalid-input mismatch, and full original-input aggregate theorem invocations for each of the four successful branches. The branch fixtures are rational: generic f=T5+T3+1, cancellation/positive-distinct f=T5-T+1, repeated f=T5+1. No sampled trajectories or performance measurements occurred.

Main attempt1 retained a simplifier ordering failure: rewriting positive equality too early hid cancellationfalse. Attempt2 passed after staged guard rewriting; attempt3 adds the returned-payload corollary. Controls1 ran before the main module existed and failed import; controls2 retained an ambiguous support name, corrected by qualifying MixedCancellation.support. All failures remain in their receipt directories; no evidence was overwritten.

This is source-order field-value composition with explicit tags, not merely another isolated branch. It still depends on the reviewed field formula mappings; it does not establish UInt64 dispatcher composition, side effects/counters, divisor-class addition, or full walker closure. Independent review is pending. No root edits, scientific runs or cloud actions occurred. Cost ratios remain null.
