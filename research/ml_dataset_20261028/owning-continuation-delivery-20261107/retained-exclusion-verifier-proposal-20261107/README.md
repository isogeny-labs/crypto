# Retained exclusion verifier

This bounded component independently pairs every excluded Boolean partition with exactly one retained record and freshly replays its signed exhaustion. It consumes the existing complete prefix-coverage audit and classified binary verifier. It does not accept a stored ExhaustedForModel verdict as sufficient evidence.

## Inputs and record integration

verify receives the authentic CheckedMap type, original signed domain/target, original ordered Boolean equations, complete retained continuation, immutable record projection and same original owner/deadline. The map variable count must equal the continuation count. Production integration must borrow these from its original OwnedExpansion/Math rather than reconstructing them from caller inputs. The generic arithmetic component cannot authenticate how its arguments were obtained or independently prove that supplied Boolean equations were expanded from that map.

The Records trait provides allocation-free borrowed RecordView projections. For custody_driver_fix's owning LiftRecord, project partition, record, model and callback_result directly; project returned_outcome as Some(outcome) only when returned is Some(Ok(outcome)), otherwise None. In production this implementation belongs on the authentic private record slice. No verifier projection should synthesize historical callback success.

Each Excluded partition's exclusion_record is an exact record-vector index. The vector length must equal the independently checked exclusion count, every index is used exactly once, the stored record number equals its index, the partition matches, and the record model equals the exact retained native witness bit-for-bit. Missing, surplus, duplicate, misindexed or mismatched records fail. Callback result must be Some(Ok(())); retained outcome must be ExhaustedForModel. These are historical consistency checks, not a cryptographic proof of past callback execution.

After those checks the verifier allocates fresh retained Progress and invokes the actual classified signed verifier against the supplied checked map/domain/target and exact model. A replayed signed relation, malformed input, owner refusal, deadline or unclassified rejection cannot pass as exhaustion. A final admission check precedes marking that record paired. Every replay outcome/progress survives subsequent failure.

## Owner and resource semantics

The owner implements sparse Owner and checked_model VerificationAdmission on the SAME original ledger. The bounded bridge forwards all preprocessing, coverage, proof checker and signed arithmetic quotes cumulatively and preserves the original deadline. Group additions use the same owner's addition admission. Finite local record/work/byte limits only restrict the existing grant. No fresh original admission, phase reset or allowance is minted. For genuine continuation this runs while Matrix remains active, using its admitted matrix candidate-addition method.

Coverage audit is re-run rather than trusted from storage. Its conditional native equations and proofs must verify. The resulting statement is complete Boolean coverage under exactly paired exclusions, each freshly found exhausted for its supplied checked model and signed domain. Original-source authority, historical callback custody and complete exceptional-pattern coverage remain distinct integration obligations; this component does not claim all-pattern PDP unsatisfiability.

## Validation

The final bounded standalone compile used rustc +1.98 --edition=2021 --test harness.rs with timeout 180, followed by timeout 60 ./tests --test-threads=1. All 22 tests passed: six verifier controls and sixteen unchanged donor tests. Fixtures exercise real GF8 group arithmetic and actual classified verifier code under an explicit structural CheckedMap shim, not authenticated production ingress.

Controls cover exact one- and two-model replay; unique exclusion pairing; missing/extra/wrong-index/partition/model/callback rejection; stored exhaustion invalidated by changed signed domain or map; mutated coverage; finite record/deadline refusal; and selected original-owner refusal boundaries through final replay with retained progress and exact charges. Two distinct models use an unconstrained auxiliary Boolean wire in the structural fixture; no scientific measurement is implied.

The first harness compilation failed because its fixture attempted to move source equations before consuming the borrowing Session; compile-initial.log preserves the error. Fix consumed Session before moving equations. Initial tests-initial.log records 21 controls before the required two-model duplicate-index test. The final receipt binds the 22-test source/binary/log. No production or full Cargo build, cloud operation or dataset generation occurred.

Next integrate the Records projection and original-owner bridge into the authentic owning continuation packet, compile those genuine modules and exercise original-source/domain bindings. This proposal changes no frozen packet.
