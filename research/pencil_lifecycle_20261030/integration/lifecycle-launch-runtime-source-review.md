# Diagnostic launch/runtime source review

Observed 2026-10-30T16:37:40Z. Read-only source inspection; no builds, fixtures executed, solver runs, scientific admission, cloud changes or Git changes.

Reviewed source SHA-256:
- diagnostic_launch.rs: 86a63c3223877e0a5e569f75543aa2309970075933f6ef0681a26762676ff312
- diagnostic_runtime.rs: 5607cb88d81dcb8a7dc01719515d8dafb5a28c9b028e56a41eb496ca79b20931
- pencil.rs optional adapter: 8207cd27360051b8467f966cbea1bb164f5b2bbe6e4d8778507a51066fb414a4

No concrete defect identified within the stated stable runner-owned parent and quiescent-reader assumptions. This is not complete integration validation or successful compilation evidence.

Launch raw JSON/hash is revalidated on document/validation use, with size bound, duplicate-key rejection and strict schema. Exact three-arm role/method/profile constraints and resolved configuration digest checks are present. The raw accessors alone do not attest identity. validate_spec must receive the actual computed executable hash before validate_plan: validate_plan is a configuration check and supplies the declared hash internally. ChildEnvelope checks the declared launch and prefix bindings; the caller must additionally bind actual scheduled session, sequence, workload and arm. No receipt authenticity or compiled-source attestation is established by this metadata check.

Runtime ownership is single-threaded through a non-Send scope, unique checked token, thread-local owner, reentrant callback guard and unwind cleanup. Drop performs no publication. create_new temporary files plus exclusive hard_link publication do not overwrite existing leaves. Reader rejects symlink/nonregular leaves and uses Linux O_NOFOLLOW; directory ancestors remain the documented stable runner-owned trust boundary. Publication and durable completion counts remain distinct. Failed writes retain diagnostic failure, and reader presence does not assert durability. Temporary-only data remain diagnostic bytes rather than accepted mathematical records. Writer timing excludes other overhead by its explicit contract.

Optional adapter keeps both result owners alive, transfers report JSON with mem::take, and preserves the original solve_one route when sink is absent. A provided sink selects the separate progress producer; actual launch/runtime activation remains a separate integration obligation. Both routes retain missing allocator observation instead of inventing zero memory. This review did not independently reprove donor algorithm correspondence.

Resource module authored by this reviewer is excluded from independent-clean claims. Parent requested its correction: emitted total_active_worker_ns stays null, while externally closed elapsed wall is closed_payload_wall_ns. Independent source review and eventual software verification remain separate.
