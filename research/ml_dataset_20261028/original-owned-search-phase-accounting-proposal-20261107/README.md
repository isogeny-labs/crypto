# Original-owned search phase accounting correction

This immutable external proposal extends original-owned-search-lift-proposal-20261107. It resolves the phase-accounting ambiguity with the existing Budget native contract. No new ledger, resource cap or deadline is created. The assembled adapter has not been compiled or executed.

## Policy and source basis

In the bound koblitz_m3.rs, begin_native (line 204) explicitly returns a phase ceiling and directs callers to observe at every native boundary. observe (line 237) converts cumulative phase reservations to deltas, checks phase work/bytes and shared original-owner headroom, and increments both native and phase_native. The construction caller (lines 394–440) follows that contract. reserve_checking (line 2688) instead documents setup/capture checking sharing the original work pool but does not enforce native phase ceilings.

Classify equation expansion and normalization as the construction algorithm. Classify decision search, matrix reduction, proof serialization and independent proof audit as the matrix algorithm. Both phases use native admission. Original source/sink setup and the later signed-model extraction verifier retain their existing checking admission and addition allowance. This is conservative: the proof checker's prepaid maximum envelope now consumes the matrix phase's native work/bytes. These quantities remain reservations, never measured instructions or actual peak resident memory.

## Implementation

reserve_phase adds each incremental quote to the original Budget.phase_native with checked arithmetic, then calls the existing Budget.observe. MatrixAdmission implements both the provenance admission and sparse-owner traits using that one function. Expansion now uses MatrixAdmission; normalization uses reserve_phase; search and audit already use MatrixAdmission and therefore share a cumulative matrix ceiling. No duplicated local ledger can reset between these calls. finish_native and begin_native retain their existing transitions. The matrix phase reset is only the existing phase transition; the global native total preserves all prior construction costs.

The construction byte ceiling remains min(original remaining memory, original memory/4, 64 MiB), and matrix remains min(original remaining memory, 64 MiB). Existing local component caps still apply. This may refuse an audit whose conservative envelope exceeds remaining native phase headroom; it must not be fixed by weakening or silently raising caps.

Admission precedes work/allocation, first failure remains sticky and previously admitted charges remain retained. Owner/deadline checks are unchanged. Search/audit failure still retains its attempt before returning. charged() now returns global native+checking work and native+adapter bytes, so the classification change does not hide algorithm reservations. The sums are bounded by the original Budget invariants.

The corrected registration.patch is copied exactly from owned-lift-registration-correction-20261107, SHA256 eef1f66c79cd4ded8bdd777ec7c7ce6de214680303a74148a52a20089762298d. worker-transport.patch and all mathematical/lift donors are unchanged from the bound lift packet. phase-accounting.patch is the exact adapter delta.

## Validation and next gate

Source inspection checked the direct Budget APIs, visibility through the existing parent module, cumulative accounting algebra, sticky refusal, phase transitions and unchanged extraction admission. Receipt records exact donor equality and recomputed patch equality. No standalone substitute Budget was introduced, and no full build or test execution occurred.

Next compile the genuine assembled modules. Boundary controls should show expansion+normalization sharing construction bytes; search+audit sharing matrix bytes; refusal before the first quote exceeding either ceiling; unchanged counters on refused quote; prior native costs retained after phase transition; and identical owner/deadline/addition accounting after signed lift. Test a proof-checker envelope that fits the global budget but exceeds the remaining native phase allowance. Genuine compile and runtime controls remain required before activation.

No production/frozen packet, cloud resource, scientific dataset, original budget or deadline was modified.
